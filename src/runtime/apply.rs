//! Runtime-owned bounded-memory Apply preparation and one-shot publication.

use std::io::{self, Read, Write};

use cap_fs_ext::{
    FollowSymlinks, OpenOptionsFollowExt, OpenOptionsMaybeDirExt, OpenOptionsSyncExt,
};
use cap_std::fs::{Dir, File, OpenOptions};
#[cfg(unix)]
use cap_std::fs::{Permissions, PermissionsExt};

use crate::backwriter::anddress::{
    Anddress as PublicAnddress, AnddressTarget as PublicAnddressTarget, LineBodyClass,
    construct_anddress as construct_public_anddress, construct_source_identity,
};
use crate::backwriter::{
    apply::ApplyError,
    edit::{Edit as PublicEdit, EditError, Position as PublicPosition},
};
use crate::hash::{Sha256, transcript_hex};

use super::{
    AnchorPlanEntry, WorkspaceRuntime, is_backwriter_spill, mark_anchor_collisions,
    source_scan::{
        ExactTargetTracker, READ_BUFFER_SIZE, SourceEvent, SourceFramer, SourceScanError,
        SourceState, scan_source,
    },
};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Natural(usize);

impl Natural {
    fn zero() -> Self {
        Self(0)
    }
}

#[derive(Clone, Debug)]
struct DecimalOrdinal(Natural);

impl DecimalOrdinal {
    fn zero() -> Result<Self, SourceScanError> {
        Ok(Self(Natural::zero()))
    }

    fn increment(&mut self) -> Result<(), SourceScanError> {
        self.0.0 = self.0.0.checked_add(1).ok_or(SourceScanError::Resource)?;
        Ok(())
    }

    fn as_natural(&self) -> Result<Natural, SourceScanError> {
        Ok(self.0.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Anddress {
    workspace_coordinate: String,
    logical_path: String,
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum AnddressTarget {
    File,
    Paragraph {
        ordinal: Natural,
    },
    Line {
        ordinal: Natural,
        exact_extent: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Position {
    Before(Anddress),
    After(Anddress),
    StartOf(Anddress),
    EndOf(Anddress),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Edit {
    Insert {
        position: Position,
        content: String,
    },
    Replace {
        target: Anddress,
        content: String,
    },
    Delete {
        target: Anddress,
    },
    Move {
        target: Anddress,
        position: Position,
    },
    Copy {
        target: Anddress,
        position: Position,
    },
}

pub(super) fn finish_publication(
    runtime: &mut WorkspaceRuntime,
    path: &str,
    plan: Vec<super::AnchorPlanEntry>,
    publication: Result<(), ApplyError>,
) -> Result<(), ApplyError> {
    match publication {
        Ok(()) => {
            runtime.reflect_anchors(plan);
            Ok(())
        }
        Err(ApplyError::PublicationUncertain) => {
            runtime.invalidate_anchors_for_path(path);
            Err(ApplyError::PublicationUncertain)
        }
        Err(error) => Err(error),
    }
}

fn same_path_bindings(
    runtime: &WorkspaceRuntime,
    path: &str,
) -> Result<Vec<PublicAnddress>, ApplyError> {
    let count = runtime
        .anchors
        .iter()
        .filter(|binding| binding.anddress.logical_path() == path)
        .count();
    let mut inputs = Vec::new();
    inputs
        .try_reserve_exact(count)
        .map_err(|_| ApplyError::Unavailable)?;
    for binding in &runtime.anchors {
        if binding.anddress.logical_path() == path {
            inputs.push(binding.anddress.clone());
        }
    }
    Ok(inputs)
}

fn indexes(length: usize) -> Result<Vec<usize>, ApplyError> {
    let mut indexes = Vec::new();
    indexes
        .try_reserve_exact(length)
        .map_err(|_| ApplyError::Unavailable)?;
    for index in 0..length {
        indexes.push(index);
    }
    Ok(indexes)
}

fn map_scan_error(_error: SourceScanError) -> ApplyError {
    ApplyError::Unavailable
}

struct Temporary<'a> {
    parent: &'a Dir,
    name: String,
    file: Option<File>,
    armed: bool,
}

impl<'a> Temporary<'a> {
    fn create(parent: &'a Dir, name: String) -> Result<Self, ApplyError> {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        options
            .follow(FollowSymlinks::No)
            .maybe_dir(false)
            .nonblock(true);
        let file = parent
            .open_with(&name, &options)
            .map_err(|_| ApplyError::Unavailable)?;
        Ok(Self {
            parent,
            name,
            file: Some(file),
            armed: true,
        })
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), ApplyError> {
        self.file
            .as_mut()
            .expect("temporary is open before publication")
            .write_all(bytes)
            .map_err(|_| ApplyError::Unavailable)
    }

    #[cfg(unix)]
    fn set_mode(&mut self, mode: u32) -> Result<(), ApplyError> {
        self.file
            .as_ref()
            .expect("temporary is open before publication")
            .set_permissions(Permissions::from_mode(mode))
            .map_err(|_| ApplyError::Unavailable)
    }

    fn close(&mut self) -> Result<(), ApplyError> {
        self.file
            .as_mut()
            .expect("temporary is open before publication")
            .flush()
            .map_err(|_| ApplyError::Unavailable)?;
        drop(self.file.take());
        Ok(())
    }

    fn remove(&mut self) -> Result<(), ApplyError> {
        if self.file.is_some() {
            self.close()?;
        }
        self.parent
            .remove_file(&self.name)
            .map_err(|_| ApplyError::Unavailable)?;
        self.armed = false;
        Ok(())
    }

    fn open_read(&self) -> Result<File, ApplyError> {
        if self.file.is_some() {
            return Err(ApplyError::Unavailable);
        }
        let mut options = OpenOptions::new();
        options.read(true);
        options
            .follow(FollowSymlinks::No)
            .maybe_dir(false)
            .nonblock(true);
        self.parent
            .open_with(&self.name, &options)
            .map_err(|_| ApplyError::Unavailable)
    }
}

impl Drop for Temporary<'_> {
    fn drop(&mut self) {
        if self.armed {
            drop(self.file.take());
            let _ = self.parent.remove_file(&self.name);
        }
    }
}

fn edit_temporary_name(
    runtime: &WorkspaceRuntime,
    path: &str,
    purpose: &str,
) -> Result<String, ApplyError> {
    let digest = transcript_hex(
        "artext.backwriter-apply-edit-v1-temporary",
        [
            runtime.workspace_coordinate.as_bytes(),
            path.as_bytes(),
            purpose.as_bytes(),
        ],
    );
    let prefix = ".env.artext-apply-edit-";
    let length = prefix
        .len()
        .checked_add(digest.len())
        .ok_or(ApplyError::Unavailable)?;
    let mut name = String::new();
    name.try_reserve_exact(length)
        .map_err(|_| ApplyError::Unavailable)?;
    name.push_str(prefix);
    name.push_str(&digest);
    Ok(name)
}

fn publish(
    parent: &Dir,
    destination: &str,
    #[cfg(unix)] source_mode: u32,
    mut temporary: Temporary<'_>,
) -> Result<(), ApplyError> {
    #[cfg(unix)]
    temporary.set_mode(source_mode)?;
    temporary.close()?;
    parent
        .rename(&temporary.name, parent, destination)
        .map_err(|_| ApplyError::PublicationUncertain)?;
    temporary.armed = false;
    Ok(())
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Relation {
    Outside,
    Containing,
    Nested,
}

#[derive(Clone, Copy, Default)]
struct Markers {
    binding: bool,
    other: bool,
    replacement: bool,
}

impl Markers {
    fn include(&mut self, other: Self) {
        self.binding |= other.binding;
        self.other |= other.other;
        self.replacement |= other.replacement;
    }
}

struct Candidate<'a> {
    binding: &'a Anddress,
    relation: Relation,
    source_member: bool,
    line: Markers,
    paragraph: Markers,
    result: Option<Anddress>,
    multiple: bool,
}

impl<'a> Candidate<'a> {
    fn new(binding: &'a Anddress, relation: Relation, source_member: bool) -> Self {
        Self {
            binding,
            relation,
            source_member,
            line: Markers::default(),
            paragraph: Markers::default(),
            result: None,
            multiple: false,
        }
    }

    fn qualifies(&mut self, markers: Markers) -> bool {
        if self.multiple {
            return false;
        }
        let qualifies = match self.relation {
            Relation::Containing => (markers.replacement || markers.binding) && !markers.other,
            Relation::Outside => markers.binding && !markers.replacement && !markers.other,
            Relation::Nested => false,
        };
        if !qualifies {
            return false;
        }
        if self.result.is_some() {
            self.result = None;
            self.multiple = true;
            return false;
        }
        true
    }

    fn into_result(self) -> Option<Anddress> {
        self.result
    }
}

#[derive(Clone, Copy)]
enum Emission<'a> {
    Source(Option<&'a Natural>, Option<&'a Natural>),
    Mutation,
    Moved(Option<&'a Natural>, Option<&'a Natural>),
    Copied,
}

struct AfterPlanner<'a> {
    candidates: Vec<Candidate<'a>>,
    framer: Option<SourceFramer>,
    line_bytes: Option<Vec<u8>>,
    line_ordinal: DecimalOrdinal,
    paragraph_ordinal: DecimalOrdinal,
    paragraph_start: usize,
    paragraph_end: usize,
    in_paragraph: bool,
    hash: Sha256,
    byte_length: usize,
}

impl<'a> AfterPlanner<'a> {
    fn for_edit(
        bindings: &'a [Anddress],
        relations: Vec<(Relation, bool)>,
    ) -> Result<Self, ApplyError> {
        if relations.len() != bindings.len() {
            return Err(ApplyError::Unavailable);
        }
        let mut candidates = Vec::new();
        candidates
            .try_reserve_exact(bindings.len())
            .map_err(|_| ApplyError::Unavailable)?;
        for (binding, (relation, source_member)) in bindings.iter().zip(relations) {
            if !matches!(binding.target, AnddressTarget::File) {
                candidates.push(Candidate::new(binding, relation, source_member));
            }
        }
        Ok(Self {
            candidates,
            framer: Some(SourceFramer::new().map_err(map_scan_error)?),
            line_bytes: None,
            line_ordinal: DecimalOrdinal::zero().map_err(map_scan_error)?,
            paragraph_ordinal: DecimalOrdinal::zero().map_err(map_scan_error)?,
            paragraph_start: 0,
            paragraph_end: 0,
            in_paragraph: false,
            hash: Sha256::new(),
            byte_length: 0,
        })
    }

    fn feed_source(
        &mut self,
        bytes: &[u8],
        line: Option<&Natural>,
        paragraph: Option<&Natural>,
    ) -> Result<(), SourceScanError> {
        self.feed(bytes, Emission::Source(line, paragraph))
    }

    fn feed_replacement(&mut self, bytes: &[u8]) -> Result<(), SourceScanError> {
        self.feed(bytes, Emission::Mutation)
    }

    fn feed_copy(&mut self, bytes: &[u8]) -> Result<(), SourceScanError> {
        self.feed(bytes, Emission::Copied)
    }

    fn feed_moved(
        &mut self,
        bytes: &[u8],
        line: Option<&Natural>,
        paragraph: Option<&Natural>,
    ) -> Result<(), SourceScanError> {
        self.feed(bytes, Emission::Moved(line, paragraph))
    }

    fn feed(&mut self, bytes: &[u8], source: Emission<'_>) -> Result<(), SourceScanError> {
        self.hash.update(bytes);
        self.byte_length = self
            .byte_length
            .checked_add(bytes.len())
            .ok_or(SourceScanError::Resource)?;
        let mut framer = self.framer.take().expect("after framer is present");
        let mut mark_pending = true;
        let result = framer.push(bytes, &mut |event| {
            self.consume(event, source, &mut mark_pending)
        });
        self.framer = Some(framer);
        result
    }

    fn finish(&mut self) -> Result<(SourceState, Vec<Option<Anddress>>), ApplyError> {
        let mut framer = self.framer.take().expect("after framer is present");
        let mut mark_pending = false;
        let result =
            framer.finish(&mut |event| self.consume(event, Emission::Mutation, &mut mark_pending));
        self.framer = Some(framer);
        result.map_err(map_scan_error)?;
        self.finish_paragraph().map_err(map_scan_error)?;
        let mut results = Vec::new();
        results
            .try_reserve_exact(self.candidates.len())
            .map_err(|_| ApplyError::Unavailable)?;
        for candidate in std::mem::take(&mut self.candidates) {
            results.push(candidate.into_result());
        }
        let state = SourceState {
            hash: std::mem::replace(&mut self.hash, Sha256::new())
                .finish()
                .to_hex(),
            byte_length: self.byte_length,
        };
        Ok((state, results))
    }

    fn consume(
        &mut self,
        event: SourceEvent,
        source: Emission<'_>,
        mark_pending: &mut bool,
    ) -> Result<(), SourceScanError> {
        match event {
            SourceEvent::StartLine { .. } => {
                *mark_pending = true;
                self.line_bytes = self.needs_line_bytes(source).then(Vec::new);
                for candidate in &mut self.candidates {
                    if !candidate.multiple {
                        candidate.line = Markers::default();
                    }
                }
            }
            SourceEvent::Byte { byte, .. } => {
                if let Some(bytes) = self.line_bytes.as_mut() {
                    bytes
                        .try_reserve(1)
                        .map_err(|_| SourceScanError::Resource)?;
                    bytes.push(byte);
                }
                if *mark_pending {
                    self.mark_candidates(source);
                    *mark_pending = false;
                }
            }
            SourceEvent::EndLine {
                byte_start,
                byte_end,
                body_class,
                ..
            } => self.finish_line(byte_start, byte_end, body_class)?,
        }
        Ok(())
    }

    fn mark_candidates(&mut self, source: Emission<'_>) {
        for candidate in &mut self.candidates {
            if candidate.multiple {
                continue;
            }
            let markers = &mut candidate.line;
            match source {
                Emission::Mutation => markers.replacement = true,
                Emission::Moved(line, paragraph) => {
                    if candidate.source_member {
                        Self::mark_source(candidate.binding, markers, line, paragraph);
                    } else {
                        markers.replacement = true;
                    }
                }
                Emission::Copied => {
                    if !candidate.source_member {
                        markers.replacement = true;
                    }
                }
                Emission::Source(line, paragraph) => {
                    Self::mark_source(candidate.binding, markers, line, paragraph);
                }
            }
        }
    }

    fn needs_line_bytes(&self, source: Emission<'_>) -> bool {
        self.candidates
            .iter()
            .filter(|candidate| !candidate.multiple && candidate.result.is_none())
            .any(|candidate| match source {
                Emission::Source(Some(line), _) => matches!(
                    (&candidate.binding.target, candidate.relation),
                    (AnddressTarget::Line { ordinal, .. }, Relation::Outside | Relation::Containing)
                        if ordinal == line
                ),
                Emission::Source(None, _) => false,
                Emission::Mutation => matches!(
                    (&candidate.binding.target, candidate.relation),
                    (AnddressTarget::Line { .. }, Relation::Containing)
                ),
                Emission::Moved(Some(line), _) => {
                    candidate.source_member
                        && matches!(
                            (&candidate.binding.target, candidate.relation),
                            (AnddressTarget::Line { ordinal, .. }, Relation::Outside | Relation::Containing)
                                if ordinal == line
                        )
                }
                Emission::Moved(None, _) => false,
                Emission::Copied => {
                    candidate.source_member
                        && matches!(candidate.binding.target, AnddressTarget::Line { .. })
                }
            })
    }

    fn mark_source(
        binding: &Anddress,
        markers: &mut Markers,
        line: Option<&Natural>,
        paragraph: Option<&Natural>,
    ) {
        match &binding.target {
            AnddressTarget::File => unreachable!("same-path candidates exclude File targets"),
            AnddressTarget::Line { ordinal, .. } => {
                if line == Some(ordinal) {
                    markers.binding = true;
                } else {
                    markers.other = true;
                }
            }
            AnddressTarget::Paragraph { ordinal } => match paragraph {
                Some(paragraph) if ordinal == paragraph => markers.binding = true,
                Some(_) => markers.other = true,
                None => {}
            },
        }
    }

    fn finish_line(
        &mut self,
        byte_start: usize,
        byte_end: usize,
        body_class: LineBodyClass,
    ) -> Result<(), SourceScanError> {
        for candidate in &mut self.candidates {
            if !matches!(candidate.binding.target, AnddressTarget::Line { .. })
                || !candidate.qualifies(candidate.line)
            {
                continue;
            }
            let address = legacy_retarget(
                candidate.binding,
                AnddressTarget::Line {
                    ordinal: self.line_ordinal.as_natural()?,
                    exact_extent: copy_utf8(
                        self.line_bytes
                            .as_deref()
                            .ok_or(SourceScanError::Resource)?,
                    )?,
                },
                byte_start,
                byte_end,
            )?;
            candidate.result = Some(address);
        }
        self.line_bytes = None;
        if body_class == LineBodyClass::Text {
            if !self.in_paragraph {
                self.in_paragraph = true;
                self.paragraph_start = byte_start;
                for candidate in &mut self.candidates {
                    if !candidate.multiple {
                        candidate.paragraph = Markers::default();
                    }
                }
            }
            self.paragraph_end = byte_end;
            for candidate in &mut self.candidates {
                if !candidate.multiple {
                    candidate.paragraph.include(candidate.line);
                }
            }
        } else if self.in_paragraph {
            self.finish_paragraph()?;
            self.paragraph_ordinal.increment()?;
            self.in_paragraph = false;
        }
        self.line_ordinal.increment()?;
        Ok(())
    }

    fn finish_paragraph(&mut self) -> Result<(), SourceScanError> {
        if !self.in_paragraph {
            return Ok(());
        }
        for candidate in &mut self.candidates {
            if !matches!(candidate.binding.target, AnddressTarget::Paragraph { .. })
                || !candidate.qualifies(candidate.paragraph)
            {
                continue;
            }
            let address = legacy_retarget(
                candidate.binding,
                AnddressTarget::Paragraph {
                    ordinal: self.paragraph_ordinal.as_natural()?,
                },
                self.paragraph_start,
                self.paragraph_end,
            )?;
            candidate.result = Some(address);
        }
        Ok(())
    }
}

fn reflection_plan(
    runtime: &WorkspaceRuntime,
    path: &str,
    state: SourceState,
    candidates: Vec<Option<Anddress>>,
) -> Result<Vec<AnchorPlanEntry>, ApplyError> {
    let source = construct_source_identity(
        &runtime.workspace_coordinate,
        path,
        &state.hash,
        state.byte_length,
    )
    .map_err(|_| ApplyError::Unavailable)?;
    let mut plan = Vec::new();
    plan.try_reserve_exact(runtime.anchors.len())
        .map_err(|_| ApplyError::Unavailable)?;
    let mut candidates = candidates.into_iter();
    for binding in &runtime.anchors {
        if binding.anddress.logical_path() != path {
            plan.push(AnchorPlanEntry::Preserve);
            continue;
        }
        if binding.anddress.target() == PublicAnddressTarget::File {
            plan.push(AnchorPlanEntry::Rebind {
                anddress: construct_public_anddress(
                    &source,
                    PublicAnddressTarget::File,
                    0,
                    state.byte_length,
                )
                .map_err(|_| ApplyError::Unavailable)?,
                collides: false,
            });
            continue;
        }
        let candidate = candidates.next().expect("same-path candidate is prepared");
        match candidate {
            Some(anddress) => {
                let target = match anddress.target {
                    AnddressTarget::Paragraph { .. } => PublicAnddressTarget::Paragraph,
                    AnddressTarget::Line { .. } => PublicAnddressTarget::Line,
                    AnddressTarget::File => unreachable!("same-path File anchors are handled"),
                };
                plan.push(AnchorPlanEntry::Rebind {
                    anddress: construct_public_anddress(
                        &source,
                        target,
                        anddress.byte_start,
                        anddress.byte_end,
                    )
                    .map_err(|_| ApplyError::Unavailable)?,
                    collides: false,
                });
            }
            None => plan.push(AnchorPlanEntry::Remove),
        }
    }
    debug_assert!(candidates.next().is_none());
    mark_anchor_collisions(&mut plan);
    Ok(plan)
}

fn copy_utf8(bytes: &[u8]) -> Result<String, SourceScanError> {
    let value = std::str::from_utf8(bytes).map_err(|_| SourceScanError::InvalidSource)?;
    let mut copy = String::new();
    copy.try_reserve_exact(value.len())
        .map_err(|_| SourceScanError::Resource)?;
    copy.push_str(value);
    Ok(copy)
}

fn legacy_retarget(
    input: &Anddress,
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
) -> Result<Anddress, SourceScanError> {
    Ok(Anddress {
        workspace_coordinate: copy_utf8(input.workspace_coordinate.as_bytes())?,
        logical_path: copy_utf8(input.logical_path.as_bytes())?,
        target,
        byte_start,
        byte_end,
    })
}

// Private Apply execution body included by `runtime::apply`.

pub(super) fn execute(runtime: &mut WorkspaceRuntime, edit: &PublicEdit) -> Result<(), ApplyError> {
    edit.validate().map_err(map_edit_error)?;
    let (first, second) = public_operands(edit);
    if second.is_some_and(|second| !same_source(first, second)) {
        return Err(ApplyError::InvalidInput);
    }
    if first.workspace_coordinate() != runtime.workspace_coordinate
        || is_backwriter_spill(first.logical_path())
    {
        return Err(ApplyError::Unavailable);
    }

    runtime.prune_dead_anchors();
    let public_bindings = same_path_bindings(runtime, first.logical_path())?;
    let (inputs, edit_count) = tracker_inputs(first, second, &public_bindings)?;
    let indexes = indexes(inputs.len())?;
    let mut tracker = ExactTargetTracker::new(&inputs, &indexes).map_err(map_scan_error)?;
    let mut resolver = LegacyResolver::new(&inputs)?;
    let mut source = runtime
        .open_admitted_source(first.logical_path())
        .map_err(|_| ApplyError::Unavailable)?;
    let (parent, destination) = runtime
        .open_admitted_parent(first.logical_path())
        .map_err(|_| ApplyError::Unavailable)?;
    let mut staging = Temporary::create(
        &parent,
        edit_temporary_name(runtime, first.logical_path(), "staging")?,
    )?;
    let state = match scan_staged(&mut source, &mut staging, &mut tracker, &mut resolver) {
        Ok(state) => state,
        Err(SourceScanError::InvalidSource) => {
            runtime.invalidate_anchors_for_path(first.logical_path());
            return Err(ApplyError::Unavailable);
        }
        Err(SourceScanError::Read | SourceScanError::Resource) => {
            return Err(ApplyError::Unavailable);
        }
    };
    tracker.finish(&state);
    let current = tracker.into_current();
    if current[edit_count..].iter().any(|current| !current) {
        runtime.invalidate_anchors_for_path(first.logical_path());
        return Err(ApplyError::Unavailable);
    }
    if current[..edit_count].iter().any(|current| !current) {
        return Err(ApplyError::Unavailable);
    }
    let resolved = resolver.finish()?;
    let edit = resolve_edit(edit, &resolved[..edit_count])?;
    let bindings = &resolved[edit_count..];
    staging.close()?;

    if matches!(&edit, Edit::Insert { content, .. } if content.is_empty()) {
        staging.remove()?;
        return Ok(());
    }

    let move_layout = matches!(&edit, Edit::Move { .. })
        .then(|| classify_move(&staging, &edit))
        .transpose()?;
    match move_layout {
        Some(MoveLayout::Interior) => return Err(ApplyError::InvalidInput),
        Some(MoveLayout::Start | MoveLayout::End) => {
            staging.remove()?;
            return Ok(());
        }
        Some(MoveLayout::Before | MoveLayout::After) | None => {}
    }

    if matches!(&edit, Edit::Replace { .. } | Edit::Move { .. }) {
        let comparison = staging.open_read()?;
        let mut probe_output = Output::probe(comparison);
        replay(&staging, &edit, &mut probe_output)?;
        let identical = probe_output.is_identical()?;
        drop(probe_output);
        if identical {
            staging.remove()?;
            return Ok(());
        }
    }

    let relations = source_relations(&staging, &edit, bindings)?;
    let planner = AfterPlanner::for_edit(bindings, relations)?;
    let temporary = Temporary::create(
        &parent,
        edit_temporary_name(runtime, first.logical_path(), "after")?,
    )?;
    let mut output = Output::after(temporary, planner);
    replay(&staging, &edit, &mut output)?;
    let (temporary, after_state, candidates) = output.finish()?;
    #[cfg(unix)]
    let source_mode = source
        .metadata()
        .map_err(|_| ApplyError::Unavailable)?
        .permissions()
        .mode()
        & 0o777;
    let plan = reflection_plan(runtime, first.logical_path(), after_state, candidates)?;
    staging.remove()?;
    finish_publication(
        runtime,
        first.logical_path(),
        plan,
        publish(
            &parent,
            destination,
            #[cfg(unix)]
            source_mode,
            temporary,
        ),
    )
}

fn map_edit_error(error: EditError) -> ApplyError {
    match error {
        EditError::UnsupportedVersion => ApplyError::UnsupportedVersion,
        EditError::InvalidInput => ApplyError::InvalidInput,
        EditError::Resource => ApplyError::Unavailable,
    }
}

fn public_operands(edit: &PublicEdit) -> (&PublicAnddress, Option<&PublicAnddress>) {
    match edit {
        PublicEdit::Insert { position, .. } => (public_position_target(position), None),
        PublicEdit::Replace { target, .. } | PublicEdit::Delete { target } => (target, None),
        PublicEdit::Move { target, position } | PublicEdit::Copy { target, position } => {
            (target, Some(public_position_target(position)))
        }
    }
}

fn public_position_target(position: &PublicPosition) -> &PublicAnddress {
    match position {
        PublicPosition::Before(target)
        | PublicPosition::After(target)
        | PublicPosition::StartOf(target)
        | PublicPosition::EndOf(target) => target,
    }
}

fn edit_target(edit: &Edit) -> Option<&Anddress> {
    match edit {
        Edit::Insert { .. } => None,
        Edit::Replace { target, .. }
        | Edit::Delete { target }
        | Edit::Move { target, .. }
        | Edit::Copy { target, .. } => Some(target),
    }
}

fn same_source(left: &PublicAnddress, right: &PublicAnddress) -> bool {
    left.workspace_coordinate() == right.workspace_coordinate()
        && left.logical_path() == right.logical_path()
        && left.source_state_hash() == right.source_state_hash()
        && left.source_byte_length() == right.source_byte_length()
}

fn tracker_inputs(
    first: &PublicAnddress,
    second: Option<&PublicAnddress>,
    bindings: &[PublicAnddress],
) -> Result<(Vec<PublicAnddress>, usize), ApplyError> {
    let edit_count = 1 + usize::from(second.is_some());
    let mut inputs = Vec::new();
    inputs
        .try_reserve_exact(edit_count + bindings.len())
        .map_err(|_| ApplyError::Unavailable)?;
    inputs.push(first.clone());
    if let Some(second) = second {
        inputs.push(second.clone());
    }
    for binding in bindings {
        inputs.push(binding.clone());
    }
    Ok((inputs, edit_count))
}

fn resolve_edit(edit: &PublicEdit, operands: &[Anddress]) -> Result<Edit, ApplyError> {
    let first = operands.first().ok_or(ApplyError::Unavailable)?.clone();
    let second = operands.get(1).cloned();
    Ok(match edit {
        PublicEdit::Insert { position, content } => Edit::Insert {
            position: resolve_position(position, first),
            content: copy_utf8(content.as_bytes()).map_err(map_scan_error)?,
        },
        PublicEdit::Replace { content, .. } => Edit::Replace {
            target: first,
            content: copy_utf8(content.as_bytes()).map_err(map_scan_error)?,
        },
        PublicEdit::Delete { .. } => Edit::Delete { target: first },
        PublicEdit::Move { position, .. } => Edit::Move {
            target: first,
            position: resolve_position(position, second.ok_or(ApplyError::Unavailable)?),
        },
        PublicEdit::Copy { position, .. } => Edit::Copy {
            target: first,
            position: resolve_position(position, second.ok_or(ApplyError::Unavailable)?),
        },
    })
}

fn resolve_position(position: &PublicPosition, target: Anddress) -> Position {
    match position {
        PublicPosition::Before(_) => Position::Before(target),
        PublicPosition::After(_) => Position::After(target),
        PublicPosition::StartOf(_) => Position::StartOf(target),
        PublicPosition::EndOf(_) => Position::EndOf(target),
    }
}

struct LegacyResolver<'a> {
    inputs: &'a [PublicAnddress],
    results: Vec<Option<Anddress>>,
    line: Natural,
    paragraph: Natural,
    line_start: usize,
    line_bytes: Vec<u8>,
    capture_line: bool,
    paragraph_start: usize,
    paragraph_end: usize,
    in_paragraph: bool,
}

impl<'a> LegacyResolver<'a> {
    fn new(inputs: &'a [PublicAnddress]) -> Result<Self, ApplyError> {
        let mut results = Vec::new();
        results
            .try_reserve_exact(inputs.len())
            .map_err(|_| ApplyError::Unavailable)?;
        results.resize(inputs.len(), None);
        Ok(Self {
            inputs,
            results,
            line: Natural::zero(),
            paragraph: Natural::zero(),
            line_start: 0,
            line_bytes: Vec::new(),
            capture_line: false,
            paragraph_start: 0,
            paragraph_end: 0,
            in_paragraph: false,
        })
    }

    fn consume(&mut self, event: SourceEvent) -> Result<(), SourceScanError> {
        match event {
            SourceEvent::StartLine { byte_start, .. } => {
                self.line_start = byte_start;
                self.line_bytes.clear();
                self.capture_line = self.inputs.iter().any(|input| {
                    input.target() == PublicAnddressTarget::Line && input.byte_start() == byte_start
                });
            }
            SourceEvent::Byte { byte, .. } if self.capture_line => {
                self.line_bytes
                    .try_reserve(1)
                    .map_err(|_| SourceScanError::Resource)?;
                self.line_bytes.push(byte);
            }
            SourceEvent::Byte { .. } => {}
            SourceEvent::EndLine {
                byte_start,
                byte_end,
                body_class,
                ..
            } => self.finish_line(byte_start, byte_end, body_class)?,
        }
        Ok(())
    }

    fn finish_line(
        &mut self,
        byte_start: usize,
        byte_end: usize,
        body_class: LineBodyClass,
    ) -> Result<(), SourceScanError> {
        debug_assert_eq!(byte_start, self.line_start);
        for (index, input) in self.inputs.iter().enumerate() {
            if input.target() == PublicAnddressTarget::Line
                && input.byte_start() == byte_start
                && input.byte_end() == byte_end
            {
                self.results[index] = Some(legacy_address(
                    input,
                    AnddressTarget::Line {
                        ordinal: self.line.clone(),
                        exact_extent: copy_utf8(&self.line_bytes)?,
                    },
                )?);
            }
        }
        if body_class == LineBodyClass::Text {
            if !self.in_paragraph {
                self.in_paragraph = true;
                self.paragraph_start = byte_start;
            }
            self.paragraph_end = byte_end;
        } else {
            self.finish_paragraph()?;
        }
        self.line.0 = self
            .line
            .0
            .checked_add(1)
            .ok_or(SourceScanError::Resource)?;
        Ok(())
    }

    fn finish_paragraph(&mut self) -> Result<(), SourceScanError> {
        if !self.in_paragraph {
            return Ok(());
        }
        for (index, input) in self.inputs.iter().enumerate() {
            if input.target() == PublicAnddressTarget::Paragraph
                && input.byte_start() == self.paragraph_start
                && input.byte_end() == self.paragraph_end
            {
                self.results[index] = Some(legacy_address(
                    input,
                    AnddressTarget::Paragraph {
                        ordinal: self.paragraph.clone(),
                    },
                )?);
            }
        }
        self.paragraph.0 = self
            .paragraph
            .0
            .checked_add(1)
            .ok_or(SourceScanError::Resource)?;
        self.in_paragraph = false;
        Ok(())
    }

    fn finish(mut self) -> Result<Vec<Anddress>, ApplyError> {
        self.finish_paragraph().map_err(map_scan_error)?;
        for (index, input) in self.inputs.iter().enumerate() {
            if input.target() == PublicAnddressTarget::File {
                self.results[index] =
                    Some(legacy_address(input, AnddressTarget::File).map_err(map_scan_error)?);
            }
        }
        let mut resolved = Vec::new();
        resolved
            .try_reserve_exact(self.results.len())
            .map_err(|_| ApplyError::Unavailable)?;
        for result in self.results {
            resolved.push(result.ok_or(ApplyError::Unavailable)?);
        }
        Ok(resolved)
    }
}

fn legacy_address(
    input: &PublicAnddress,
    target: AnddressTarget,
) -> Result<Anddress, SourceScanError> {
    Ok(Anddress {
        workspace_coordinate: copy_utf8(input.workspace_coordinate().as_bytes())?,
        logical_path: copy_utf8(input.logical_path().as_bytes())?,
        target,
        byte_start: input.byte_start(),
        byte_end: input.byte_end(),
    })
}

fn scan_staged(
    source: &mut impl Read,
    staging: &mut Temporary<'_>,
    tracker: &mut ExactTargetTracker<'_>,
    resolver: &mut LegacyResolver<'_>,
) -> Result<SourceState, SourceScanError> {
    let mut staged = StagedRead { source, staging };
    scan_source(&mut staged, |event| {
        tracker.consume(event)?;
        resolver.consume(event)
    })
}

struct StagedRead<'read, 'temporary, 'parent, R> {
    source: &'read mut R,
    staging: &'temporary mut Temporary<'parent>,
}

impl<R: Read> Read for StagedRead<'_, '_, '_, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let count = self.source.read(buffer)?;
        if count != 0 {
            self.staging
                .write(&buffer[..count])
                .map_err(|_| io::Error::other("staging write failed"))?;
        }
        Ok(count)
    }
}

fn scan_events(
    reader: &mut impl Read,
    mut on_event: impl FnMut(SourceEvent) -> Result<(), ApplyError>,
) -> Result<(), ApplyError> {
    let mut failure = None;
    let scan = scan_source(reader, |event| match on_event(event) {
        Ok(()) => Ok(()),
        Err(error) => {
            failure = Some(error);
            Err(SourceScanError::Resource)
        }
    });
    if let Some(error) = failure {
        return Err(error);
    }
    scan.map(|_| ()).map_err(map_scan_error)
}

enum OutputProvenance<'a> {
    Source(Option<&'a Natural>, Option<&'a Natural>),
    Replacement,
    Moved(Option<&'a Natural>, Option<&'a Natural>),
    Copy,
}

struct Output<'parent, 'bindings> {
    temporary: Option<Temporary<'parent>>,
    comparison: Option<File>,
    identical: bool,
    after: Option<AfterPlanner<'bindings>>,
}

impl<'parent, 'bindings> Output<'parent, 'bindings> {
    fn probe(comparison: File) -> Self {
        Self {
            temporary: None,
            comparison: Some(comparison),
            identical: true,
            after: None,
        }
    }

    fn after(temporary: Temporary<'parent>, after: AfterPlanner<'bindings>) -> Self {
        Self {
            temporary: Some(temporary),
            comparison: None,
            identical: false,
            after: Some(after),
        }
    }

    fn source(
        &mut self,
        bytes: &[u8],
        line: Option<&Natural>,
        paragraph: Option<&Natural>,
    ) -> Result<(), ApplyError> {
        self.emit(bytes, OutputProvenance::Source(line, paragraph))
    }

    fn replacement(&mut self, bytes: &[u8]) -> Result<(), ApplyError> {
        self.emit(bytes, OutputProvenance::Replacement)
    }

    fn copy(&mut self, bytes: &[u8]) -> Result<(), ApplyError> {
        self.emit(bytes, OutputProvenance::Copy)
    }

    fn moved(
        &mut self,
        bytes: &[u8],
        line: Option<&Natural>,
        paragraph: Option<&Natural>,
    ) -> Result<(), ApplyError> {
        self.emit(bytes, OutputProvenance::Moved(line, paragraph))
    }

    fn emit(&mut self, bytes: &[u8], provenance: OutputProvenance<'_>) -> Result<(), ApplyError> {
        if let Some(temporary) = self.temporary.as_mut() {
            temporary.write(bytes)?;
        }
        self.compare(bytes)?;
        if let Some(after) = self.after.as_mut() {
            match provenance {
                OutputProvenance::Source(line, paragraph) => after
                    .feed_source(bytes, line, paragraph)
                    .map_err(map_scan_error)?,
                OutputProvenance::Replacement => {
                    after.feed_replacement(bytes).map_err(map_scan_error)?
                }
                OutputProvenance::Moved(line, paragraph) => after
                    .feed_moved(bytes, line, paragraph)
                    .map_err(map_scan_error)?,
                OutputProvenance::Copy => after.feed_copy(bytes).map_err(map_scan_error)?,
            }
        }
        Ok(())
    }

    fn compare(&mut self, bytes: &[u8]) -> Result<(), ApplyError> {
        let Some(reader) = self.comparison.as_mut() else {
            return Ok(());
        };
        compare_exact(reader, bytes, &mut self.identical)
    }

    fn is_identical(&mut self) -> Result<bool, ApplyError> {
        if !self.identical {
            return Ok(false);
        }
        let Some(reader) = self.comparison.as_mut() else {
            return Ok(false);
        };
        comparison_exhausted(reader)
    }

    fn finish(
        mut self,
    ) -> Result<(Temporary<'parent>, SourceState, Vec<Option<Anddress>>), ApplyError> {
        let (state, candidates) = self
            .after
            .as_mut()
            .expect("final output has an after planner")
            .finish()?;
        Ok((
            self.temporary
                .take()
                .expect("final output owns an after temporary"),
            state,
            candidates,
        ))
    }
}

fn compare_exact(
    reader: &mut impl Read,
    bytes: &[u8],
    identical: &mut bool,
) -> Result<(), ApplyError> {
    if !*identical {
        return Ok(());
    }
    let mut offset = 0;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    while offset < bytes.len() {
        let length = (bytes.len() - offset).min(scratch.len());
        let count = reader
            .read(&mut scratch[..length])
            .map_err(|_| ApplyError::Unavailable)?;
        if count == 0 || scratch[..count] != bytes[offset..offset + count] {
            *identical = false;
            return Ok(());
        }
        offset += count;
    }
    Ok(())
}

fn comparison_exhausted(reader: &mut impl Read) -> Result<bool, ApplyError> {
    let mut extra = [0_u8; 1];
    reader
        .read(&mut extra)
        .map(|count| count == 0)
        .map_err(|_| ApplyError::Unavailable)
}

fn replay(
    staging: &Temporary<'_>,
    edit: &Edit,
    output: &mut Output<'_, '_>,
) -> Result<(), ApplyError> {
    if let Edit::Replace {
        target: Anddress {
            target: AnddressTarget::File,
            ..
        },
        content,
    } = edit
    {
        return output.replacement(content.as_bytes());
    }
    let mut reader = staging.open_read()?;
    let mut replay = Replay::new(staging, edit, output)?;
    replay.start()?;
    scan_events(&mut reader, |event| replay.event(event))?;
    replay.finish()
}

struct Replay<'staging, 'edit, 'output, 'parent, 'bindings> {
    staging: &'staging Temporary<'staging>,
    edit: &'edit Edit,
    output: &'output mut Output<'parent, 'bindings>,
    paragraph: DecimalOrdinal,
    in_paragraph: bool,
    target_active: bool,
    line: Option<Natural>,
    line_paragraph: Option<Natural>,
    line_started: bool,
    line_prepared: bool,
    line_selected: bool,
    line_skip: bool,
    leading: Vec<u8>,
    pending_paragraph: bool,
    source_batch: [u8; READ_BUFFER_SIZE],
    source_batch_len: usize,
}

impl<'staging, 'edit, 'output, 'parent, 'bindings>
    Replay<'staging, 'edit, 'output, 'parent, 'bindings>
{
    fn new(
        staging: &'staging Temporary<'staging>,
        edit: &'edit Edit,
        output: &'output mut Output<'parent, 'bindings>,
    ) -> Result<Self, ApplyError> {
        Ok(Self {
            staging,
            edit,
            output,
            paragraph: DecimalOrdinal::zero().map_err(map_scan_error)?,
            in_paragraph: false,
            target_active: false,
            line: None,
            line_paragraph: None,
            line_started: false,
            line_prepared: false,
            line_selected: false,
            line_skip: false,
            leading: Vec::new(),
            pending_paragraph: false,
            source_batch: [0; READ_BUFFER_SIZE],
            source_batch_len: 0,
        })
    }

    fn start(&mut self) -> Result<(), ApplyError> {
        if matches!(position(self.edit), Some(Position::StartOf(_))) {
            self.destination()?;
        }
        Ok(())
    }

    fn event(&mut self, event: SourceEvent) -> Result<(), ApplyError> {
        match event {
            SourceEvent::StartLine { line_index, .. } => {
                self.line = Some(Natural(line_index));
                self.line_paragraph = None;
                self.line_started = false;
                self.line_prepared = false;
                self.line_selected = false;
                self.line_skip = false;
                self.leading.clear();
                self.pending_paragraph = self.may_resolve_paragraph()?;
                if !self.pending_paragraph {
                    self.prepare_line(false)?;
                }
            }
            SourceEvent::Byte { byte, content } => {
                if !self.line_started {
                    if content && !matches!(byte, b' ' | b'\t') {
                        self.start_line(true)?;
                        self.line_started = true;
                        self.emit_leading()?;
                    } else if self.pending_paragraph {
                        self.leading
                            .try_reserve(1)
                            .map_err(|_| ApplyError::Unavailable)?;
                        self.leading.push(byte);
                        return Ok(());
                    }
                }
                self.emit_source_byte(byte)?;
            }
            SourceEvent::EndLine { body_class, .. } => {
                if !self.line_started {
                    self.start_line(body_class == LineBodyClass::Text)?;
                    self.line_started = true;
                    self.emit_leading()?;
                } else {
                    debug_assert_eq!(body_class, LineBodyClass::Text);
                }
                self.flush_source()?;
                let line_target = matches!(
                    edit_target(self.edit).map(|target| &target.target),
                    Some(AnddressTarget::Line { .. })
                );
                if self.line_selected && line_target {
                    self.target_active = false;
                }
                let line = self.line.take().expect("line start precedes line end");
                if self.position_after_line(&line) {
                    self.destination()?;
                }
                self.line_paragraph = None;
                self.line_started = false;
                self.line_prepared = false;
                self.line_selected = false;
                self.line_skip = false;
                self.pending_paragraph = false;
            }
        }
        Ok(())
    }

    fn start_line(&mut self, text: bool) -> Result<(), ApplyError> {
        let paragraph_start = if text {
            let paragraph_start = !self.in_paragraph;
            self.in_paragraph = true;
            self.line_paragraph = Some(self.paragraph.as_natural().map_err(map_scan_error)?);
            paragraph_start
        } else {
            self.close_paragraph()?;
            false
        };
        if !self.line_prepared {
            self.prepare_line(paragraph_start)?;
        }
        Ok(())
    }

    fn prepare_line(&mut self, paragraph_start: bool) -> Result<(), ApplyError> {
        let line = self
            .line
            .as_ref()
            .expect("line start precedes bytes")
            .clone();
        let paragraph = self.line_paragraph.clone();
        let selected = self.selected(&line, paragraph.as_ref());
        let source_start = selected && !self.target_active;
        if self.position_before(&line, paragraph.as_ref(), paragraph_start) {
            self.destination()?;
        }
        if source_start {
            self.target_active = true;
            if let Edit::Replace { content, .. } = self.edit {
                self.output.replacement(content.as_bytes())?;
            }
        }

        self.line_skip = selected
            && matches!(
                self.edit,
                Edit::Replace { .. } | Edit::Delete { .. } | Edit::Move { .. }
            );
        self.line_selected = selected;
        self.line_prepared = true;
        Ok(())
    }

    fn may_resolve_paragraph(&self) -> Result<bool, ApplyError> {
        let target = edit_target(self.edit).map(|target| &target.target);
        let position = position(self.edit);
        if !matches!(target, Some(AnddressTarget::Paragraph { .. }))
            && !matches!(
                position,
                Some(Position::Before(Anddress {
                    target: AnddressTarget::Paragraph { .. },
                    ..
                })) | Some(Position::After(Anddress {
                    target: AnddressTarget::Paragraph { .. },
                    ..
                }))
            )
        {
            return Ok(false);
        }
        let paragraph = self.paragraph.as_natural().map_err(map_scan_error)?;
        let target = matches!(
            target,
            Some(AnddressTarget::Paragraph { ordinal }) if ordinal == &paragraph
        );
        let position = matches!(
            position,
            Some(Position::Before(Anddress { target: AnddressTarget::Paragraph { ordinal }, .. }))
                if !self.in_paragraph && ordinal == &paragraph
        ) || matches!(
            position,
            Some(Position::After(Anddress { target: AnddressTarget::Paragraph { ordinal }, .. }))
                if self.in_paragraph && ordinal == &paragraph
        );
        Ok(target || position)
    }

    fn emit_leading(&mut self) -> Result<(), ApplyError> {
        for byte in std::mem::take(&mut self.leading) {
            self.emit_source_byte(byte)?;
        }
        Ok(())
    }

    fn emit_source_byte(&mut self, byte: u8) -> Result<(), ApplyError> {
        if self.line_skip {
            return Ok(());
        }
        self.source_batch[self.source_batch_len] = byte;
        self.source_batch_len += 1;
        if self.source_batch_len == self.source_batch.len() {
            self.flush_source()?;
        }
        Ok(())
    }

    fn flush_source(&mut self) -> Result<(), ApplyError> {
        let length = std::mem::take(&mut self.source_batch_len);
        if length == 0 {
            return Ok(());
        }
        let line = self
            .line
            .as_ref()
            .expect("line start precedes source bytes");
        self.output.source(
            &self.source_batch[..length],
            Some(line),
            self.line_paragraph.as_ref(),
        )
    }

    fn close_paragraph(&mut self) -> Result<(), ApplyError> {
        if !self.in_paragraph {
            return Ok(());
        }
        let paragraph = self.paragraph.as_natural().map_err(map_scan_error)?;
        if self.target_active
            && matches!(edit_target(self.edit).map(|target| &target.target), Some(AnddressTarget::Paragraph { ordinal }) if ordinal == &paragraph)
        {
            self.target_active = false;
        }
        if self.position_after_paragraph(&paragraph) {
            self.destination()?;
        }
        self.paragraph.increment().map_err(map_scan_error)?;
        self.in_paragraph = false;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), ApplyError> {
        self.close_paragraph()?;
        if matches!(position(self.edit), Some(Position::EndOf(_))) {
            self.destination()?;
        }
        Ok(())
    }

    fn selected(&self, line: &Natural, paragraph: Option<&Natural>) -> bool {
        match edit_target(self.edit).map(|target| &target.target) {
            Some(AnddressTarget::Line { ordinal, .. }) => ordinal == line,
            Some(AnddressTarget::Paragraph { ordinal }) => paragraph == Some(ordinal),
            Some(AnddressTarget::File) | None => false,
        }
    }

    fn position_before(
        &self,
        line: &Natural,
        paragraph: Option<&Natural>,
        paragraph_start: bool,
    ) -> bool {
        matches!(
            position(self.edit),
            Some(Position::Before(Anddress { target: AnddressTarget::Line { ordinal, .. }, .. })) if ordinal == line
        ) || matches!(
            position(self.edit),
            Some(Position::Before(Anddress { target: AnddressTarget::Paragraph { ordinal }, .. }))
                if paragraph_start && paragraph == Some(ordinal)
        )
    }

    fn position_after_line(&self, line: &Natural) -> bool {
        matches!(
            position(self.edit),
            Some(Position::After(Anddress { target: AnddressTarget::Line { ordinal, .. }, .. })) if ordinal == line
        )
    }

    fn position_after_paragraph(&self, paragraph: &Natural) -> bool {
        matches!(
            position(self.edit),
            Some(Position::After(Anddress { target: AnddressTarget::Paragraph { ordinal }, .. })) if ordinal == paragraph
        )
    }

    fn destination(&mut self) -> Result<(), ApplyError> {
        match self.edit {
            Edit::Insert { content, .. } => self.output.replacement(content.as_bytes()),
            Edit::Move { target, .. } => match (&target.target, self.output.comparison.is_some()) {
                (AnddressTarget::Line { exact_extent, .. }, true) => {
                    self.output.moved(exact_extent.as_bytes(), None, None)
                }
                _ => extract_target(self.staging, target, false, self.output),
            },
            Edit::Copy { target, .. } => match &target.target {
                AnddressTarget::Line { exact_extent, .. } => {
                    self.output.copy(exact_extent.as_bytes())
                }
                AnddressTarget::Paragraph { .. } => {
                    extract_target(self.staging, target, true, self.output)
                }
                AnddressTarget::File => unreachable!("Edit validation excludes File Copy"),
            },
            Edit::Replace { .. } | Edit::Delete { .. } => Ok(()),
        }
    }
}

fn position(edit: &Edit) -> Option<&Position> {
    match edit {
        Edit::Insert { position, .. }
        | Edit::Move { position, .. }
        | Edit::Copy { position, .. } => Some(position),
        Edit::Replace { .. } | Edit::Delete { .. } => None,
    }
}

fn extract_target(
    staging: &Temporary<'_>,
    target: &Anddress,
    copy: bool,
    output: &mut Output<'_, '_>,
) -> Result<(), ApplyError> {
    let mut reader = staging.open_read()?;
    let mut extractor = Extractor::new(target, copy, output)?;
    scan_events(&mut reader, |event| extractor.event(event))?;
    extractor.finish()
}

struct Extractor<'target, 'output, 'parent, 'bindings> {
    target: &'target Anddress,
    copy: bool,
    output: &'output mut Output<'parent, 'bindings>,
    paragraph: DecimalOrdinal,
    in_paragraph: bool,
    line: Option<Natural>,
    line_paragraph: Option<Natural>,
    line_started: bool,
    selected: bool,
    leading: Vec<u8>,
    pending_paragraph: bool,
    batch: [u8; READ_BUFFER_SIZE],
    batch_len: usize,
}

impl<'target, 'output, 'parent, 'bindings> Extractor<'target, 'output, 'parent, 'bindings> {
    fn new(
        target: &'target Anddress,
        copy: bool,
        output: &'output mut Output<'parent, 'bindings>,
    ) -> Result<Self, ApplyError> {
        Ok(Self {
            target,
            copy,
            output,
            paragraph: DecimalOrdinal::zero().map_err(map_scan_error)?,
            in_paragraph: false,
            line: None,
            line_paragraph: None,
            line_started: false,
            selected: false,
            leading: Vec::new(),
            pending_paragraph: false,
            batch: [0; READ_BUFFER_SIZE],
            batch_len: 0,
        })
    }

    fn event(&mut self, event: SourceEvent) -> Result<(), ApplyError> {
        match event {
            SourceEvent::StartLine { line_index, .. } => {
                self.line = Some(Natural(line_index));
                self.line_paragraph = None;
                self.line_started = false;
                self.leading.clear();
                self.selected = matches!(
                    &self.target.target,
                    AnddressTarget::Line { ordinal, .. }
                        if self.line.as_ref() == Some(ordinal)
                );
                self.pending_paragraph = self.may_select_paragraph()?;
            }
            SourceEvent::Byte { byte, content } => {
                if !self.line_started {
                    if content && !matches!(byte, b' ' | b'\t') {
                        self.start_line(true)?;
                        self.line_started = true;
                        self.emit_leading()?;
                    } else if self.pending_paragraph {
                        self.leading
                            .try_reserve(1)
                            .map_err(|_| ApplyError::Unavailable)?;
                        self.leading.push(byte);
                        return Ok(());
                    }
                }
                self.emit_selected_byte(byte)?;
            }
            SourceEvent::EndLine { body_class, .. } => {
                if !self.line_started {
                    self.start_line(body_class == LineBodyClass::Text)?;
                    self.line_started = true;
                    self.emit_leading()?;
                } else {
                    debug_assert_eq!(body_class, LineBodyClass::Text);
                }
                self.flush()?;
                self.pending_paragraph = false;
            }
        }
        Ok(())
    }

    fn may_select_paragraph(&self) -> Result<bool, ApplyError> {
        let AnddressTarget::Paragraph { ordinal } = &self.target.target else {
            return Ok(false);
        };
        Ok(self.paragraph.as_natural().map_err(map_scan_error)? == *ordinal)
    }

    fn start_line(&mut self, text: bool) -> Result<(), ApplyError> {
        if text {
            self.in_paragraph = true;
            self.line_paragraph = Some(self.paragraph.as_natural().map_err(map_scan_error)?);
        } else if self.in_paragraph {
            self.paragraph.increment().map_err(map_scan_error)?;
            self.in_paragraph = false;
        }
        let line = self.line.as_ref().expect("line start precedes bytes");
        self.selected = matches!(
            &self.target.target,
            AnddressTarget::Line { ordinal, .. } if ordinal == line
        ) || matches!(
            (&self.target.target, self.line_paragraph.as_ref()),
            (AnddressTarget::Paragraph { ordinal }, Some(current)) if ordinal == current
        );
        Ok(())
    }

    fn emit_leading(&mut self) -> Result<(), ApplyError> {
        for byte in std::mem::take(&mut self.leading) {
            self.emit_selected_byte(byte)?;
        }
        Ok(())
    }

    fn emit_selected_byte(&mut self, byte: u8) -> Result<(), ApplyError> {
        if !self.selected {
            return Ok(());
        }
        self.batch[self.batch_len] = byte;
        self.batch_len += 1;
        if self.batch_len == self.batch.len() {
            self.flush()?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), ApplyError> {
        let length = std::mem::take(&mut self.batch_len);
        if length == 0 {
            return Ok(());
        }
        if self.copy {
            self.output.copy(&self.batch[..length])
        } else {
            self.output.moved(
                &self.batch[..length],
                self.line.as_ref(),
                self.line_paragraph.as_ref(),
            )
        }
    }

    fn finish(&mut self) -> Result<(), ApplyError> {
        self.flush()
    }
}

#[derive(Clone, Copy, Debug)]
enum MoveLayout {
    Before,
    Start,
    Interior,
    End,
    After,
}

fn classify_move(staging: &Temporary<'_>, edit: &Edit) -> Result<MoveLayout, ApplyError> {
    let Edit::Move { target, position } = edit else {
        unreachable!("only Move has a move layout");
    };
    let mut reader = staging.open_read()?;
    let mut classifier = MoveClassifier::new(target, position)?;
    scan_events(&mut reader, |event| classifier.event(event))?;
    classifier.finish()
}

fn source_relations(
    staging: &Temporary<'_>,
    edit: &Edit,
    bindings: &[Anddress],
) -> Result<Vec<(Relation, bool)>, ApplyError> {
    let Some(target) = edit_target(edit) else {
        return outside_relations(bindings.len());
    };
    if matches!(target.target, AnddressTarget::File)
        || bindings
            .iter()
            .all(|binding| matches!(binding.target, AnddressTarget::File))
    {
        return outside_relations(bindings.len());
    }
    let same_kind_only = match &target.target {
        AnddressTarget::Line { .. } => bindings.iter().all(|binding| {
            matches!(
                binding.target,
                AnddressTarget::File | AnddressTarget::Line { .. }
            )
        }),
        AnddressTarget::Paragraph { .. } => bindings.iter().all(|binding| {
            matches!(
                binding.target,
                AnddressTarget::File | AnddressTarget::Paragraph { .. }
            )
        }),
        AnddressTarget::File => unreachable!("File target returns before relation classification"),
    };
    if same_kind_only {
        let mut relations = outside_relations(bindings.len())?;
        for (binding, relation) in bindings.iter().zip(&mut relations) {
            if matches!(
                (&target.target, &binding.target),
                (
                    AnddressTarget::Line { ordinal: target, .. },
                    AnddressTarget::Line { ordinal, .. },
                ) | (
                    AnddressTarget::Paragraph { ordinal: target },
                    AnddressTarget::Paragraph { ordinal },
                ) if ordinal == target
            ) {
                *relation = (Relation::Containing, true);
            }
        }
        return Ok(relations);
    }
    let mut reader = staging.open_read()?;
    let mut classifier = SourceRelations::new(target, edit, bindings)?;
    scan_events(&mut reader, |event| classifier.event(event))?;
    Ok(classifier.relations)
}

fn outside_relations(length: usize) -> Result<Vec<(Relation, bool)>, ApplyError> {
    let mut relations = Vec::new();
    relations
        .try_reserve_exact(length)
        .map_err(|_| ApplyError::Unavailable)?;
    relations.resize(length, (Relation::Outside, false));
    Ok(relations)
}

struct SourceRelations<'a> {
    target: &'a Anddress,
    edit: &'a Edit,
    bindings: &'a [Anddress],
    relations: Vec<(Relation, bool)>,
    paragraph: DecimalOrdinal,
    in_paragraph: bool,
    line: Option<Natural>,
}

impl<'a> SourceRelations<'a> {
    fn new(
        target: &'a Anddress,
        edit: &'a Edit,
        bindings: &'a [Anddress],
    ) -> Result<Self, ApplyError> {
        Ok(Self {
            target,
            edit,
            bindings,
            relations: outside_relations(bindings.len())?,
            paragraph: DecimalOrdinal::zero().map_err(map_scan_error)?,
            in_paragraph: false,
            line: None,
        })
    }

    fn event(&mut self, event: SourceEvent) -> Result<(), ApplyError> {
        match event {
            SourceEvent::StartLine { line_index, .. } => {
                self.line = Some(Natural(line_index));
            }
            SourceEvent::Byte { .. } => {}
            SourceEvent::EndLine { body_class, .. } => {
                let line = self.line.as_ref().expect("line start precedes line end");
                let paragraph = (body_class == LineBodyClass::Text)
                    .then(|| self.paragraph.as_natural())
                    .transpose()
                    .map_err(map_scan_error)?;
                let selected = matches!(
                    (&self.target.target, paragraph.as_ref()),
                    (AnddressTarget::Line { ordinal, .. }, _) if ordinal == line
                ) || matches!(
                    (&self.target.target, paragraph.as_ref()),
                    (AnddressTarget::Paragraph { ordinal }, Some(current)) if ordinal == current
                );
                if selected {
                    for (index, binding) in self.bindings.iter().enumerate() {
                        let member = matches!(
                            (&self.target.target, &binding.target, paragraph.as_ref()),
                            (AnddressTarget::Line { .. }, AnddressTarget::Line { ordinal, .. }, _)
                                if ordinal == line
                        ) || matches!(
                            (&self.target.target, &binding.target, paragraph.as_ref()),
                            (AnddressTarget::Line { .. }, AnddressTarget::Paragraph { ordinal }, Some(current))
                                if ordinal == current
                        ) || matches!(
                            (&self.target.target, &binding.target, paragraph.as_ref()),
                            (AnddressTarget::Paragraph { .. }, AnddressTarget::Paragraph { ordinal }, Some(current))
                                if ordinal == current
                        ) || matches!(
                            (&self.target.target, &binding.target, paragraph.as_ref()),
                            (AnddressTarget::Paragraph { .. }, AnddressTarget::Line { ordinal, .. }, Some(_))
                                if ordinal == line
                        );
                        if !member {
                            continue;
                        }
                        let relation = match (&self.target.target, &binding.target, self.edit) {
                            (
                                AnddressTarget::Paragraph { .. },
                                AnddressTarget::Line { .. },
                                Edit::Copy { .. },
                            ) => Relation::Outside,
                            (
                                AnddressTarget::Paragraph { .. },
                                AnddressTarget::Line { .. },
                                Edit::Move { .. },
                            ) => Relation::Containing,
                            (AnddressTarget::Paragraph { .. }, AnddressTarget::Line { .. }, _) => {
                                Relation::Nested
                            }
                            _ => Relation::Containing,
                        };
                        self.relations[index] = (relation, true);
                    }
                }
                if body_class == LineBodyClass::Text {
                    self.in_paragraph = true;
                } else if self.in_paragraph {
                    self.paragraph.increment().map_err(map_scan_error)?;
                    self.in_paragraph = false;
                }
            }
        }
        Ok(())
    }
}

struct MoveClassifier<'a> {
    target: &'a Anddress,
    position: &'a Position,
    paragraph: DecimalOrdinal,
    in_paragraph: bool,
    active: bool,
    seen: bool,
    location: Option<MoveLayout>,
    pending_after_line: bool,
    line: Option<Natural>,
}

impl<'a> MoveClassifier<'a> {
    fn new(target: &'a Anddress, position: &'a Position) -> Result<Self, ApplyError> {
        Ok(Self {
            target,
            position,
            paragraph: DecimalOrdinal::zero().map_err(map_scan_error)?,
            in_paragraph: false,
            active: false,
            seen: false,
            location: None,
            pending_after_line: false,
            line: None,
        })
    }

    fn event(&mut self, event: SourceEvent) -> Result<(), ApplyError> {
        match event {
            SourceEvent::StartLine { line_index, .. } => {
                self.line = Some(Natural(line_index));
            }
            SourceEvent::Byte { .. } => {}
            SourceEvent::EndLine { body_class, .. } => {
                let line = self
                    .line
                    .as_ref()
                    .expect("line start precedes bytes")
                    .clone();
                if body_class != LineBodyClass::Text {
                    self.resolve_pending(true)?;
                    self.close_paragraph()?;
                    self.visit_line(&line, None, false)?;
                } else {
                    self.resolve_pending(false)?;
                    let paragraph_start = !self.in_paragraph;
                    self.in_paragraph = true;
                    let paragraph = self.paragraph.as_natural().map_err(map_scan_error)?;
                    self.visit_line(&line, Some(&paragraph), paragraph_start)?;
                }
            }
        }
        Ok(())
    }

    fn visit_line(
        &mut self,
        line: &Natural,
        paragraph: Option<&Natural>,
        paragraph_start: bool,
    ) -> Result<(), ApplyError> {
        let selected = self.selected(line, paragraph);
        if self.before_matches(line, paragraph, paragraph_start) {
            self.record(if selected && !self.active {
                MoveLayout::Start
            } else if self.active {
                MoveLayout::Interior
            } else if self.seen {
                MoveLayout::After
            } else {
                MoveLayout::Before
            });
        }
        if selected {
            self.active = true;
            self.seen = true;
        }
        let selected_line = matches!(self.target.target, AnddressTarget::Line { .. }) && selected;
        if self.after_line_matches(line) {
            if selected_line {
                self.record(MoveLayout::End);
                self.active = false;
            } else if self.active {
                self.pending_after_line = true;
            } else if self.seen {
                self.record(MoveLayout::After);
            } else {
                self.record(MoveLayout::Before);
            }
        }
        if selected_line {
            self.active = false;
        }
        Ok(())
    }

    fn close_paragraph(&mut self) -> Result<(), ApplyError> {
        if !self.in_paragraph {
            return Ok(());
        }
        let paragraph = self.paragraph.as_natural().map_err(map_scan_error)?;
        let selected = matches!(&self.target.target, AnddressTarget::Paragraph { ordinal } if ordinal == &paragraph);
        if self.after_paragraph_matches(&paragraph) {
            self.record(if selected || self.active {
                MoveLayout::End
            } else if self.seen {
                MoveLayout::After
            } else {
                MoveLayout::Before
            });
        }
        if selected {
            self.active = false;
        }
        self.paragraph.increment().map_err(map_scan_error)?;
        self.in_paragraph = false;
        Ok(())
    }

    fn resolve_pending(&mut self, source_ends: bool) -> Result<(), ApplyError> {
        if self.pending_after_line {
            self.record(if source_ends {
                MoveLayout::End
            } else {
                MoveLayout::Interior
            });
            self.pending_after_line = false;
        }
        Ok(())
    }

    fn finish(mut self) -> Result<MoveLayout, ApplyError> {
        self.resolve_pending(true)?;
        self.close_paragraph()?;
        Ok(self.location.unwrap_or(MoveLayout::After))
    }

    fn record(&mut self, location: MoveLayout) {
        self.location = Some(location);
    }

    fn selected(&self, line: &Natural, paragraph: Option<&Natural>) -> bool {
        matches!(&self.target.target, AnddressTarget::Line { ordinal, .. } if ordinal == line)
            || matches!(
                (&self.target.target, paragraph),
                (AnddressTarget::Paragraph { ordinal }, Some(paragraph)) if ordinal == paragraph
            )
    }

    fn before_matches(
        &self,
        line: &Natural,
        paragraph: Option<&Natural>,
        paragraph_start: bool,
    ) -> bool {
        matches!(
            self.position,
            Position::Before(Anddress { target: AnddressTarget::Line { ordinal, .. }, .. }) if ordinal == line
        ) || matches!(
            self.position,
            Position::Before(Anddress { target: AnddressTarget::Paragraph { ordinal }, .. })
                if paragraph_start && paragraph == Some(ordinal)
        )
    }

    fn after_line_matches(&self, line: &Natural) -> bool {
        matches!(
            self.position,
            Position::After(Anddress { target: AnddressTarget::Line { ordinal, .. }, .. }) if ordinal == line
        )
    }

    fn after_paragraph_matches(&self, paragraph: &Natural) -> bool {
        matches!(
            self.position,
            Position::After(Anddress { target: AnddressTarget::Paragraph { ordinal }, .. }) if ordinal == paragraph
        )
    }
}

#[cfg(test)]
mod edit_tests {
    use std::{
        fs,
        io::{self, Read},
    };

    use crate::{
        backwriter::{
            anchor::{Anchedress, AnchorOutcome},
            anddress::{Anddress as PublicAnddress, AnddressTarget as PublicAnddressTarget},
            apply::ApplyError,
            edit::Edit,
        },
        runtime::{
            AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime,
            source_scan::{ExactTargetTracker, SourceScanError},
        },
    };

    use super::{
        AfterPlanner, Anddress, AnddressTarget, LegacyResolver, Natural, Relation, Temporary,
        compare_exact, comparison_exhausted, edit_temporary_name, execute, publish, scan_staged,
    };

    struct FailingReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl Read for FailingReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if self.position == self.bytes.len() {
                return Err(io::Error::other("late read failure"));
            }
            let length = buffer.len().min(self.bytes.len() - self.position);
            buffer[..length].copy_from_slice(&self.bytes[self.position..self.position + length]);
            self.position += length;
            Ok(length)
        }
    }

    struct ShortReader {
        bytes: Vec<u8>,
        position: usize,
        chunk: usize,
    }

    impl Read for ShortReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if self.position == self.bytes.len() {
                return Ok(0);
            }
            let count = buffer
                .len()
                .min(self.chunk)
                .min(self.bytes.len() - self.position);
            buffer[..count].copy_from_slice(&self.bytes[self.position..self.position + count]);
            self.position += count;
            Ok(count)
        }
    }

    fn line(runtime: &WorkspaceRuntime, ordinal: &str, extent: &str) -> PublicAnddress {
        use crate::hash::Sha256;

        let source = fs::read(runtime.workspace_root.join("note.txt")).unwrap();
        let ordinal: usize = ordinal.parse().unwrap();
        let mut ranges = Vec::new();
        let mut start = 0;
        for (index, byte) in source.iter().enumerate() {
            if *byte == b'\n' {
                ranges.push((start, index + 1));
                start = index + 1;
            }
        }
        if start < source.len() {
            ranges.push((start, source.len()));
        }
        let (byte_start, byte_end) = ranges[ordinal];
        assert_eq!(&source[byte_start..byte_end], extent.as_bytes());
        PublicAnddress::new(
            &runtime.workspace_coordinate,
            "note.txt",
            &{
                let mut hash = Sha256::new();
                hash.update(&source);
                hash.finish().to_hex()
            },
            source.len(),
            PublicAnddressTarget::Line,
            byte_start,
            byte_end,
        )
        .unwrap()
    }

    fn anchor(runtime: &mut WorkspaceRuntime, input: &PublicAnddress) -> Anchedress {
        match runtime.anchor(input) {
            Ok(AnchorOutcome::Anchored(handle)) => handle,
            _ => panic!("anchor"),
        }
    }

    fn planned_line(extent: &str) -> Anddress {
        Anddress {
            workspace_coordinate: "0".repeat(64),
            logical_path: "note.txt".to_owned(),
            target: AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: extent.to_owned(),
            },
            byte_start: 0,
            byte_end: extent.len(),
        }
    }

    #[test]
    fn after_planner_marks_each_feed_segment_and_physical_line() {
        let binding = planned_line("a");
        let line = Natural::zero();
        let mut planner = AfterPlanner::for_edit(
            std::slice::from_ref(&binding),
            vec![(Relation::Outside, false)],
        )
        .unwrap();
        planner.feed_source(b"a", Some(&line), None).unwrap();
        assert_eq!(planner.finish().unwrap().1, vec![Some(binding)]);

        let binding = planned_line("ab");
        let line = Natural::zero();
        let mut planner = AfterPlanner::for_edit(
            std::slice::from_ref(&binding),
            vec![(Relation::Outside, false)],
        )
        .unwrap();
        planner.feed_source(b"a", Some(&line), None).unwrap();
        planner.feed_replacement(b"X").unwrap();
        planner.feed_source(b"b", Some(&line), None).unwrap();
        assert_eq!(planner.finish().unwrap().1, vec![None]);

        let binding = planned_line("a\r");
        let line = Natural::zero();
        let mut planner = AfterPlanner::for_edit(
            std::slice::from_ref(&binding),
            vec![(Relation::Outside, false)],
        )
        .unwrap();
        planner.feed_source(b"a\r", Some(&line), None).unwrap();
        planner.feed_replacement(b"\n").unwrap();
        assert_eq!(planner.finish().unwrap().1, vec![None]);

        let binding = planned_line("old\n");
        let mut planner = AfterPlanner::for_edit(
            std::slice::from_ref(&binding),
            vec![(Relation::Containing, false)],
        )
        .unwrap();
        planner.feed_replacement(b"x\r\ny\nz").unwrap();
        assert_eq!(planner.finish().unwrap().1, vec![None]);
    }

    #[test]
    fn no_op_comparison_advances_by_observed_short_reads() {
        let mut exact = ShortReader {
            bytes: b"abc".to_vec(),
            position: 0,
            chunk: 1,
        };
        let mut identical = true;
        compare_exact(&mut exact, b"abc", &mut identical).unwrap();
        assert!(identical);
        assert!(comparison_exhausted(&mut exact).unwrap());

        let mut shorter = ShortReader {
            bytes: b"ab".to_vec(),
            position: 0,
            chunk: 1,
        };
        let mut identical = true;
        compare_exact(&mut shorter, b"abc", &mut identical).unwrap();
        assert!(!identical);

        let mut longer = ShortReader {
            bytes: b"abcd".to_vec(),
            position: 0,
            chunk: 1,
        };
        let mut identical = true;
        compare_exact(&mut longer, b"abc", &mut identical).unwrap();
        assert!(identical);
        assert!(!comparison_exhausted(&mut longer).unwrap());

        let mut mismatch = ShortReader {
            bytes: b"axc".to_vec(),
            position: 0,
            chunk: 1,
        };
        let mut identical = true;
        compare_exact(&mut mismatch, b"abc", &mut identical).unwrap();
        assert!(!identical);
        let position = mismatch.position;
        compare_exact(&mut mismatch, b"remaining", &mut identical).unwrap();
        assert_eq!(mismatch.position, position);

        let mut unavailable = FailingReader {
            bytes: Vec::new(),
            position: 0,
        };
        compare_exact(&mut unavailable, b"remaining", &mut identical).unwrap();
        assert_eq!(unavailable.position, 0);
    }

    #[test]
    fn stale_binding_is_invalidated_before_a_current_edit_operand() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "one\nb\n").unwrap();
        let mut runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let edit = line(&runtime, "0", "one\n");
        let binding = line(&runtime, "1", "b\n");
        let _handle = anchor(&mut runtime, &binding);
        fs::write(fixture.path().join("note.txt"), "one\nchanged\n").unwrap();

        assert_eq!(
            execute(&mut runtime, &Edit::Delete { target: edit }),
            Err(ApplyError::Unavailable)
        );
        assert!(runtime.anchors.is_empty());
    }

    #[test]
    fn stale_source_state_invalidates_same_path_bindings() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "one\nb\n").unwrap();
        let mut runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let edit = line(&runtime, "0", "one\n");
        let binding = line(&runtime, "1", "b\n");
        let _handle = anchor(&mut runtime, &binding);
        fs::write(fixture.path().join("note.txt"), "changed\nb\n").unwrap();

        assert_eq!(
            execute(&mut runtime, &Edit::Delete { target: edit }),
            Err(ApplyError::Unavailable)
        );
        assert!(runtime.anchors.is_empty());
    }

    #[test]
    fn two_stale_inputs_invalidate_exact_path_bindings() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "one\nb\n").unwrap();
        let mut runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let edit = line(&runtime, "0", "one\n");
        let binding = line(&runtime, "1", "b\n");
        let _handle = anchor(&mut runtime, &binding);
        fs::write(fixture.path().join("note.txt"), "changed\nother\n").unwrap();

        assert_eq!(
            execute(&mut runtime, &Edit::Delete { target: edit }),
            Err(ApplyError::Unavailable)
        );
        assert!(runtime.anchors.is_empty());
    }

    #[test]
    fn after_staging_collision_preserves_source_and_removes_staging() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
        let mut runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let staging = edit_temporary_name(&runtime, "note.txt", "staging").unwrap();
        let after = edit_temporary_name(&runtime, "note.txt", "after").unwrap();
        fs::write(fixture.path().join(&after), "collision").unwrap();
        let edit = Edit::Delete {
            target: line(&runtime, "0", "one\n"),
        };

        assert_eq!(execute(&mut runtime, &edit), Err(ApplyError::Unavailable));
        assert_eq!(
            fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
            "one\n"
        );
        assert!(!fixture.path().join(staging).exists());
        assert_eq!(
            fs::read_to_string(fixture.path().join(after)).unwrap(),
            "collision"
        );
    }

    #[test]
    fn staging_collision_preserves_source_without_replacing_the_collision() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
        let mut runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let staging = edit_temporary_name(&runtime, "note.txt", "staging").unwrap();
        fs::write(fixture.path().join(&staging), "collision").unwrap();
        let edit = Edit::Delete {
            target: line(&runtime, "0", "one\n"),
        };

        assert_eq!(execute(&mut runtime, &edit), Err(ApplyError::Unavailable));
        assert_eq!(
            fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
            "one\n"
        );
        assert_eq!(
            fs::read_to_string(fixture.path().join(staging)).unwrap(),
            "collision"
        );
    }

    #[test]
    fn late_staged_read_failure_leaves_no_partial_temporary() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
        let runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let (parent, _) = runtime.open_admitted_parent("note.txt").unwrap();
        let name = edit_temporary_name(&runtime, "note.txt", "staging").unwrap();
        let mut temporary = Temporary::create(&parent, name.clone()).unwrap();
        use crate::hash::Sha256;
        let expected = b"one\nlate";
        let input = PublicAnddress::new(
            &runtime.workspace_coordinate,
            "note.txt",
            &{
                let mut hash = Sha256::new();
                hash.update(expected);
                hash.finish().to_hex()
            },
            expected.len(),
            PublicAnddressTarget::File,
            0,
            expected.len(),
        )
        .unwrap();
        let inputs = [input];
        let indexes = [0];
        let mut tracker = ExactTargetTracker::new(&inputs, &indexes).unwrap();
        let mut resolver = LegacyResolver::new(&inputs).unwrap();
        let mut reader = FailingReader {
            bytes: b"one\nlate".to_vec(),
            position: 0,
        };

        assert_eq!(
            scan_staged(&mut reader, &mut temporary, &mut tracker, &mut resolver),
            Err(SourceScanError::Read)
        );
        drop(temporary);
        assert!(!fixture.path().join(name).exists());
    }

    #[test]
    fn failed_publication_removes_the_after_temporary() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("source.txt"), "before").unwrap();
        fs::create_dir(fixture.path().join("destination")).unwrap();
        let runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let (parent, _) = runtime.open_admitted_parent("source.txt").unwrap();
        let name = edit_temporary_name(&runtime, "source.txt", "after").unwrap();
        let mut temporary = Temporary::create(&parent, name.clone()).unwrap();
        temporary.write(b"after").unwrap();

        assert_eq!(
            publish(
                &parent,
                "destination",
                #[cfg(unix)]
                0o600,
                temporary,
            ),
            Err(ApplyError::PublicationUncertain)
        );
        assert_eq!(
            fs::read_to_string(fixture.path().join("source.txt")).unwrap(),
            "before"
        );
        assert!(fixture.path().join("destination").is_dir());
        assert!(!fixture.path().join(&name).exists());

        drop(Temporary::create(&parent, name).unwrap());
    }

    #[test]
    fn staging_remove_failure_is_unavailable_without_source_publication() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
        let runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let (parent, _) = runtime.open_admitted_parent("note.txt").unwrap();
        let name = edit_temporary_name(&runtime, "note.txt", "staging").unwrap();
        let mut staging = Temporary::create(&parent, name.clone()).unwrap();
        staging.close().unwrap();
        parent.remove_file(&name).unwrap();
        parent.create_dir(&name).unwrap();

        assert_eq!(staging.remove(), Err(ApplyError::Unavailable));
        assert_eq!(
            fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
            "one\n"
        );
        drop(staging);
        fs::remove_dir(fixture.path().join(name)).unwrap();
    }
}
