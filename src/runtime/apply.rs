//! Runtime-owned bounded-memory Apply preparation and one-shot publication.

use std::io::{Read, Seek, SeekFrom, Write};

use cap_fs_ext::{
    FollowSymlinks, OpenOptionsFollowExt, OpenOptionsMaybeDirExt, OpenOptionsSyncExt,
};
use cap_std::fs::{Dir, File, OpenOptions};
#[cfg(unix)]
use cap_std::fs::{Permissions, PermissionsExt};

use crate::backwriter::anddress::{
    Anddress, AnddressIssuer, AnddressTarget, ParagraphGeometry, ParentGeometry, TargetGeometry,
};
use crate::backwriter::{
    apply::{ApplyError, EditReceipt},
    edit::{Edit, EditError, Position},
};
use crate::hash::transcript_hex;

use super::{
    AnchorPlanEntry, CurrentProof, SourceProofEvidence, WorkspaceRuntime, is_backwriter_spill,
    mark_anchor_collisions,
    source_scan::{
        CurrentObservation, ObservationBuilder, READ_BUFFER_SIZE, SourceScanError, observe_source,
        validate_source_exact,
    },
    structural_cursor::{LineSpan, StructuralSink},
};

pub(super) fn finish_publication(
    runtime: &mut WorkspaceRuntime,
    path: &str,
    plan: Vec<AnchorPlanEntry>,
    proof: Option<CurrentProof>,
    publication: Result<(), ApplyError>,
) -> Result<(), ApplyError> {
    match publication {
        Ok(()) => {
            runtime.install_prepared_current_proof(proof);
            runtime.reflect_anchors(plan);
            Ok(())
        }
        Err(ApplyError::PublicationUncertain) => {
            runtime.invalidate_source_state(path);
            Err(ApplyError::PublicationUncertain)
        }
        Err(error) => Err(error),
    }
}

fn same_path_bindings(runtime: &WorkspaceRuntime, path: &str) -> Result<Vec<Anddress>, ApplyError> {
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
            .ok_or(ApplyError::Unavailable)?
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
    result: Option<TargetGeometry>,
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

    fn record(&mut self, markers: Markers, geometry: TargetGeometry) {
        if self.multiple {
            return;
        }
        let qualifies = match self.relation {
            Relation::Containing => (markers.replacement || markers.binding) && !markers.other,
            Relation::Outside => markers.binding && !markers.replacement && !markers.other,
            Relation::Nested => false,
        };
        if !qualifies {
            return;
        }
        if self.result.is_some() {
            self.result = None;
            self.multiple = true;
        } else {
            self.result = Some(geometry);
        }
    }
}

#[derive(Clone, Copy)]
enum Emission {
    Original { byte_start: usize },
    Copied,
    Replacement,
}

struct AfterProjector<'a> {
    candidates: Vec<Candidate<'a>>,
    in_paragraph: bool,
    emission: Emission,
    emission_start: usize,
}

impl<'a> AfterProjector<'a> {
    fn new(bindings: &'a [Anddress], edit: &Edit) -> Result<Self, ApplyError> {
        let relations = source_relations(edit, bindings)?;
        let count = bindings
            .iter()
            .filter(|binding| binding.target() != AnddressTarget::File)
            .count();
        let mut candidates = Vec::new();
        candidates
            .try_reserve_exact(count)
            .map_err(|_| ApplyError::Unavailable)?;
        for (binding, (relation, source_member)) in bindings.iter().zip(relations) {
            if binding.target() != AnddressTarget::File {
                candidates.push(Candidate::new(binding, relation, source_member));
            }
        }
        Ok(Self {
            candidates,
            in_paragraph: false,
            emission: Emission::Replacement,
            emission_start: 0,
        })
    }

    fn begin_emission(&mut self, emission: Emission, byte_start: usize) {
        self.emission = emission;
        self.emission_start = byte_start;
    }

    fn finish(self) -> Result<Vec<Option<TargetGeometry>>, ApplyError> {
        let mut results = Vec::new();
        results
            .try_reserve_exact(self.candidates.len())
            .map_err(|_| ApplyError::Unavailable)?;
        for candidate in self.candidates {
            results.push(candidate.result);
        }
        Ok(results)
    }

    fn begin_line(&mut self) {
        for candidate in &mut self.candidates {
            if !candidate.multiple {
                candidate.line = Markers::default();
            }
        }
    }

    fn mark(&mut self, byte_start: usize, index: usize) -> Result<(), SourceScanError> {
        let output_offset = byte_start
            .checked_add(index)
            .ok_or(SourceScanError::Resource)?;
        let emission_index = output_offset
            .checked_sub(self.emission_start)
            .ok_or(SourceScanError::InvalidSource)?;
        let original_offset = match self.emission {
            Emission::Original { byte_start } => Some(
                byte_start
                    .checked_add(emission_index)
                    .ok_or(SourceScanError::Resource)?,
            ),
            Emission::Copied | Emission::Replacement => None,
        };
        for candidate in &mut self.candidates {
            if candidate.multiple {
                continue;
            }
            match self.emission {
                Emission::Original { .. } => {
                    let offset = original_offset.expect("original emission has an offset");
                    if candidate.binding.byte_start() <= offset
                        && offset < candidate.binding.byte_end()
                    {
                        candidate.line.binding = true;
                    } else {
                        candidate.line.other = true;
                    }
                }
                Emission::Copied if !candidate.source_member => {
                    candidate.line.replacement = true;
                }
                Emission::Copied => {}
                Emission::Replacement => candidate.line.replacement = true,
            }
        }
        Ok(())
    }

    fn finish_line(&mut self, line: LineSpan) {
        let geometry = line.file_geometry();
        for candidate in &mut self.candidates {
            if candidate.binding.target() == AnddressTarget::Line {
                candidate.record(candidate.line, geometry);
            }
        }
        if line.body_class == crate::backwriter::anddress::LineBodyClass::Text {
            if !self.in_paragraph {
                self.in_paragraph = true;
                for candidate in &mut self.candidates {
                    if !candidate.multiple {
                        candidate.paragraph = Markers::default();
                    }
                }
            }
            for candidate in &mut self.candidates {
                if !candidate.multiple {
                    candidate.paragraph.include(candidate.line);
                }
            }
        }
    }

    fn finish_paragraph(&mut self, paragraph: ParagraphGeometry) -> Result<(), SourceScanError> {
        if !self.in_paragraph {
            return Ok(());
        }
        for candidate in &mut self.candidates {
            if let Some(TargetGeometry::Line {
                byte_start,
                byte_end,
                line_offset_in_parent,
                parent,
                ..
            }) = candidate.result.as_mut()
                && paragraph.byte_start <= *byte_start
                && *byte_end <= paragraph.byte_end
            {
                *line_offset_in_parent = line_offset_in_parent
                    .checked_sub(paragraph.file_line_offset)
                    .ok_or(SourceScanError::Resource)?;
                *parent = ParentGeometry::Paragraph(paragraph);
            }
            if candidate.binding.target() == AnddressTarget::Paragraph {
                candidate.record(candidate.paragraph, TargetGeometry::Paragraph(paragraph));
            }
        }
        self.in_paragraph = false;
        Ok(())
    }
}

impl StructuralSink for AfterProjector<'_> {
    fn begin_line(
        &mut self,
        _byte_start: usize,
        _file_line_offset: usize,
    ) -> Result<(), SourceScanError> {
        self.begin_line();
        Ok(())
    }

    fn segment(
        &mut self,
        bytes: &[u8],
        byte_start: usize,
        _is_content: bool,
    ) -> Result<(), SourceScanError> {
        for index in 0..bytes.len() {
            self.mark(byte_start, index)?;
        }
        Ok(())
    }

    fn line(&mut self, line: LineSpan) -> Result<(), SourceScanError> {
        self.finish_line(line);
        Ok(())
    }

    fn paragraph(&mut self, paragraph: ParagraphGeometry) -> Result<(), SourceScanError> {
        self.finish_paragraph(paragraph)
    }
}

fn source_relations(
    edit: &Edit,
    bindings: &[Anddress],
) -> Result<Vec<(Relation, bool)>, ApplyError> {
    let target = edit_target(edit);
    let mut relations = Vec::new();
    relations
        .try_reserve_exact(bindings.len())
        .map_err(|_| ApplyError::Unavailable)?;
    for binding in bindings {
        let Some(target) = target else {
            relations.push((Relation::Outside, false));
            continue;
        };
        let binding_contains = contains(binding, target);
        let target_contains = contains(target, binding);
        let overlaps = ranges_overlap(binding, target);
        let source_member = binding_contains || target_contains || overlaps;
        let binding_is_container = binding_contains
            && !(target_contains
                && binding.target() == AnddressTarget::Line
                && target.target() == AnddressTarget::Paragraph);
        let relation = if binding_is_container {
            Relation::Containing
        } else if !source_member {
            Relation::Outside
        } else {
            match edit {
                Edit::Move { .. } => Relation::Containing,
                Edit::Copy { .. } => Relation::Outside,
                Edit::Replace { .. } | Edit::Delete { .. } => Relation::Nested,
                Edit::Insert { .. } => Relation::Outside,
            }
        };
        relations.push((relation, source_member));
    }
    Ok(relations)
}

fn contains(outer: &Anddress, inner: &Anddress) -> bool {
    outer.byte_start() <= inner.byte_start() && inner.byte_end() <= outer.byte_end()
}

fn ranges_overlap(left: &Anddress, right: &Anddress) -> bool {
    left.byte_start() < right.byte_end() && right.byte_start() < left.byte_end()
}

fn reflection_plan(
    runtime: &WorkspaceRuntime,
    path: &str,
    issuer: &AnddressIssuer,
    candidates: Vec<Option<TargetGeometry>>,
) -> Result<Vec<AnchorPlanEntry>, ApplyError> {
    let mut plan = Vec::new();
    plan.try_reserve_exact(runtime.anchors.len())
        .map_err(|_| ApplyError::Unavailable)?;
    let mut candidates = candidates.into_iter();
    for binding in &runtime.anchors {
        if binding.anddress.logical_path() != path {
            plan.push(AnchorPlanEntry::Preserve);
        } else if binding.anddress.target() == AnddressTarget::File {
            plan.push(AnchorPlanEntry::Rebind {
                anddress: issuer
                    .issue(TargetGeometry::File)
                    .map_err(|_| ApplyError::Unavailable)?,
                collides: false,
            });
        } else {
            match candidates.next().expect("same-path candidate is prepared") {
                Some(geometry) => plan.push(AnchorPlanEntry::Rebind {
                    anddress: issuer
                        .issue(geometry)
                        .map_err(|_| ApplyError::Unavailable)?,
                    collides: false,
                }),
                None => plan.push(AnchorPlanEntry::Remove),
            }
        }
    }
    debug_assert!(candidates.next().is_none());
    mark_anchor_collisions(&mut plan);
    Ok(plan)
}

fn changed_receipt(
    issuer: &AnddressIssuer,
    projected_geometry: Option<TargetGeometry>,
    target: AnddressTarget,
) -> Result<EditReceipt, ApplyError> {
    let geometry = match target {
        AnddressTarget::File => Some(TargetGeometry::File),
        AnddressTarget::Paragraph | AnddressTarget::Line => projected_geometry,
    };
    let anddress = geometry
        .map(|geometry| issuer.issue(geometry))
        .transpose()
        .map_err(|_| ApplyError::Unavailable)?;
    Ok(EditReceipt::Changed { anddress })
}

fn stage_source(
    source: &mut impl Read,
    staging: &mut Temporary<'_>,
) -> Result<CurrentObservation, SourceScanError> {
    observe_source(source, |bytes, _| {
        staging.write(bytes).map_err(|_| SourceScanError::Resource)
    })
}

fn stage_source_trusted(
    source: &mut impl Read,
    staging: &mut Temporary<'_>,
    expected_length: usize,
) -> Result<(), SourceScanError> {
    validate_source_exact(source, expected_length, |bytes, _| {
        staging.write(bytes).map_err(|_| SourceScanError::Resource)
    })
}

pub(super) fn execute(
    runtime: &mut WorkspaceRuntime,
    edit: &Edit,
    receipt_target: Option<&Anddress>,
) -> Result<Option<EditReceipt>, ApplyError> {
    let (first, second) = operands(edit);
    edit.validate().map_err(map_edit_error)?;
    if second.is_some_and(|second| !same_coordinate_path(first, second)) {
        return Err(ApplyError::InvalidInput);
    }
    if first.workspace_coordinate() != runtime.workspace_coordinate
        || is_backwriter_spill(first.logical_path())
    {
        return Err(ApplyError::Unavailable);
    }
    runtime
        .selected_root(first.logical_path())
        .map_err(|_| ApplyError::Unavailable)?;
    let proof = runtime.select_current_proof(first.logical_path());
    if proof.is_some_and(|proof| {
        !matches_proof(first, &proof) || second.is_some_and(|second| !matches_proof(second, &proof))
    }) {
        return Err(ApplyError::Unavailable);
    }

    runtime.prune_dead_anchors();
    let mut bindings = same_path_bindings(runtime, first.logical_path())?;
    if proof.is_some_and(|proof| {
        bindings
            .iter()
            .any(|binding| !matches_proof(binding, &proof))
    }) {
        runtime.invalidate_source_state(first.logical_path());
        return Err(ApplyError::Unavailable);
    }
    let mut source = match runtime.open_admitted_source(first.logical_path()) {
        Ok(source) => source,
        Err(_) => {
            if proof.is_some() {
                runtime.invalidate_current_proof(first.logical_path());
            }
            return Err(ApplyError::Unavailable);
        }
    };
    let (parent, destination) = match runtime.open_admitted_parent(first.logical_path()) {
        Ok(parent) => parent,
        Err(_) => {
            if proof.is_some() {
                runtime.invalidate_current_proof(first.logical_path());
            }
            return Err(ApplyError::Unavailable);
        }
    };
    let mut staging = Temporary::create(
        &parent,
        edit_temporary_name(runtime, first.logical_path(), "staging")?,
    )?;
    let before_length = if let Some(proof) = proof {
        match stage_source_trusted(&mut source, &mut staging, proof.byte_length) {
            Ok(()) => proof.byte_length,
            Err(SourceScanError::InvalidSource) => {
                runtime.invalidate_source_state(first.logical_path());
                return Err(ApplyError::Unavailable);
            }
            Err(SourceScanError::Read) => {
                runtime.invalidate_current_proof(first.logical_path());
                return Err(ApplyError::Unavailable);
            }
            Err(SourceScanError::Resource) => return Err(ApplyError::Unavailable),
        }
    } else {
        let before = match stage_source(&mut source, &mut staging) {
            Ok(state) => state,
            Err(SourceScanError::InvalidSource) => {
                runtime.invalidate_source_state(first.logical_path());
                return Err(ApplyError::Unavailable);
            }
            Err(SourceScanError::Read | SourceScanError::Resource) => {
                return Err(ApplyError::Unavailable);
            }
        };
        if bindings
            .iter()
            .any(|binding| !matches_state(binding, &before))
        {
            runtime.invalidate_source_state(first.logical_path());
            return Err(ApplyError::Unavailable);
        }
        if !matches_state(first, &before)
            || second.is_some_and(|second| !matches_state(second, &before))
        {
            return Err(ApplyError::Unavailable);
        }
        before.byte_length
    };
    staging.close()?;

    let geometry = Geometry::new(edit, before_length)?;
    if geometry.direct_noop(edit) {
        staging.remove()?;
        return Ok(receipt_target.map(|target| EditReceipt::Unchanged {
            anddress: target.clone(),
        }));
    }

    let receipt_projection =
        receipt_target.is_some_and(|target| target.target() != AnddressTarget::File);
    if receipt_projection {
        bindings
            .try_reserve(1)
            .map_err(|_| ApplyError::Unavailable)?;
        bindings.push(
            receipt_target
                .expect("non-File receipt target is present")
                .clone(),
        );
    }
    let projector = AfterProjector::new(&bindings, edit)?;
    let temporary = Temporary::create(
        &parent,
        edit_temporary_name(runtime, first.logical_path(), "after")?,
    )?;
    let comparison = staging.open_read()?;
    let mut output = Output::new(temporary, comparison, projector)?;
    assemble(&staging, edit, geometry, &mut output)?;
    let CompletedOutput {
        mut temporary,
        observation: after,
        mut candidates,
        identical,
    } = output.finish()?;
    if identical {
        temporary.remove()?;
        staging.remove()?;
        return Ok(receipt_target.map(|target| EditReceipt::Unchanged {
            anddress: target.clone(),
        }));
    }

    let receipt_geometry = receipt_projection
        .then(|| candidates.pop().expect("receipt projection is last"))
        .flatten();

    #[cfg(unix)]
    let source_mode = source
        .metadata()
        .map_err(|_| ApplyError::Unavailable)?
        .permissions()
        .mode()
        & 0o777;
    let after_issuer = AnddressIssuer::new(
        &runtime.workspace_coordinate,
        first.logical_path(),
        &after.hash,
        after.byte_length,
        after.line_count,
    )
    .map_err(|_| ApplyError::Unavailable)?;
    let receipt = receipt_target
        .map(|target| changed_receipt(&after_issuer, receipt_geometry, target.target()))
        .transpose()?;
    let plan = reflection_plan(runtime, first.logical_path(), &after_issuer, candidates)?;
    let next_proof = runtime.prepare_current_proof_installation(
        first.logical_path(),
        after.hash,
        after.byte_length,
        after.line_count,
    )?;
    staging.remove()?;
    finish_publication(
        runtime,
        first.logical_path(),
        plan,
        next_proof,
        publish(
            &parent,
            destination,
            #[cfg(unix)]
            source_mode,
            temporary,
        ),
    )?;
    Ok(receipt)
}

fn map_edit_error(error: EditError) -> ApplyError {
    match error {
        EditError::UnsupportedVersion => ApplyError::UnsupportedVersion,
        EditError::InvalidInput => ApplyError::InvalidInput,
        EditError::Resource => ApplyError::Unavailable,
    }
}

fn operands(edit: &Edit) -> (&Anddress, Option<&Anddress>) {
    match edit {
        Edit::Insert { position, .. } => (position_target(position), None),
        Edit::Replace { target, .. } | Edit::Delete { target } => (target, None),
        Edit::Move { target, position } | Edit::Copy { target, position } => {
            (target, Some(position_target(position)))
        }
    }
}

fn position_target(position: &Position) -> &Anddress {
    match position {
        Position::Before(target)
        | Position::After(target)
        | Position::StartOf(target)
        | Position::EndOf(target) => target,
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

fn same_coordinate_path(left: &Anddress, right: &Anddress) -> bool {
    left.workspace_coordinate() == right.workspace_coordinate()
        && left.logical_path() == right.logical_path()
}

fn matches_state(input: &Anddress, state: &CurrentObservation) -> bool {
    input.source_state_hash() == state.hash
        && input.source_byte_length() == state.byte_length
        && input.source_line_count() == state.line_count
}

fn matches_proof(input: &Anddress, proof: &SourceProofEvidence) -> bool {
    super::source_state_matches(&proof.hash, proof.byte_length, proof.line_count, input)
}

#[derive(Clone, Copy)]
struct Geometry {
    source_length: usize,
    target: Option<(usize, usize)>,
    destination: Option<usize>,
    adjusted_move_destination: Option<usize>,
}

impl Geometry {
    fn new(edit: &Edit, source_length: usize) -> Result<Self, ApplyError> {
        let target = edit_target(edit).map(|target| (target.byte_start(), target.byte_end()));
        let destination = match edit {
            Edit::Insert { position, .. }
            | Edit::Move { position, .. }
            | Edit::Copy { position, .. } => Some(position_boundary(position, source_length)?),
            Edit::Replace { .. } | Edit::Delete { .. } => None,
        };
        let adjusted_move_destination = match (edit, target, destination) {
            (Edit::Move { .. }, Some((start, end)), Some(destination)) => {
                if start < destination && destination < end {
                    return Err(ApplyError::InvalidInput);
                }
                let removed = end.checked_sub(start).ok_or(ApplyError::Unavailable)?;
                Some(if destination > end {
                    destination
                        .checked_sub(removed)
                        .ok_or(ApplyError::Unavailable)?
                } else {
                    destination
                })
            }
            _ => None,
        };
        Ok(Self {
            source_length,
            target,
            destination,
            adjusted_move_destination,
        })
    }

    fn direct_noop(self, edit: &Edit) -> bool {
        match edit {
            Edit::Insert { content, .. } => content.is_empty(),
            Edit::Replace { target, content } => {
                target.byte_start() == target.byte_end() && content.is_empty()
            }
            Edit::Delete { target } | Edit::Copy { target, .. } => {
                target.byte_start() == target.byte_end()
            }
            Edit::Move { target, .. } => {
                target.byte_start() == target.byte_end()
                    || self.destination == Some(target.byte_start())
                    || self.destination == Some(target.byte_end())
            }
        }
    }
}

fn position_boundary(position: &Position, source_length: usize) -> Result<usize, ApplyError> {
    let boundary = match position {
        Position::Before(target) => target.byte_start(),
        Position::After(target) => target.byte_end(),
        Position::StartOf(_) => 0,
        Position::EndOf(_) => source_length,
    };
    (boundary <= source_length)
        .then_some(boundary)
        .ok_or(ApplyError::Unavailable)
}

struct Output<'parent, 'bindings> {
    temporary: Option<Temporary<'parent>>,
    comparison: File,
    identical: bool,
    observation: ObservationBuilder,
    projector: Option<AfterProjector<'bindings>>,
}

struct CompletedOutput<'parent> {
    temporary: Temporary<'parent>,
    observation: CurrentObservation,
    candidates: Vec<Option<TargetGeometry>>,
    identical: bool,
}

impl<'parent, 'bindings> Output<'parent, 'bindings> {
    fn new(
        temporary: Temporary<'parent>,
        comparison: File,
        projector: AfterProjector<'bindings>,
    ) -> Result<Self, ApplyError> {
        Ok(Self {
            temporary: Some(temporary),
            comparison,
            identical: true,
            observation: ObservationBuilder::new().map_err(map_scan_error)?,
            projector: Some(projector),
        })
    }

    fn emit(&mut self, bytes: &[u8], emission: Emission) -> Result<(), ApplyError> {
        let chunk_start = self.observation.byte_offset();
        let projector = self.projector.as_mut().expect("output owns its projector");
        projector.begin_emission(emission, chunk_start);
        self.observation
            .push_structural(bytes, projector)
            .map_err(map_scan_error)?;
        self.temporary
            .as_mut()
            .expect("output owns its temporary")
            .write(bytes)?;
        compare_exact(&mut self.comparison, bytes, &mut self.identical)?;
        Ok(())
    }

    fn finish(self) -> Result<CompletedOutput<'parent>, ApplyError> {
        let Self {
            temporary,
            mut comparison,
            mut identical,
            observation,
            projector,
        } = self;
        if identical {
            identical = comparison_exhausted(&mut comparison)?;
        }
        let mut projector = projector.expect("output owns its projector");
        let state = observation
            .finish_structural(&mut projector)
            .map_err(map_scan_error)?;
        let candidates = projector.finish()?;
        Ok(CompletedOutput {
            temporary: temporary.expect("output owns its temporary"),
            observation: state,
            candidates,
            identical,
        })
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
        offset = offset.checked_add(count).ok_or(ApplyError::Unavailable)?;
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

fn assemble(
    staging: &Temporary<'_>,
    edit: &Edit,
    geometry: Geometry,
    output: &mut Output<'_, '_>,
) -> Result<(), ApplyError> {
    let mut reader = staging.open_read()?;
    match edit {
        Edit::Insert { content, .. } => {
            let destination = geometry.destination.expect("Insert has a destination");
            emit_range(&mut reader, 0, destination, false, output)?;
            output.emit(content.as_bytes(), Emission::Replacement)?;
            emit_range(
                &mut reader,
                destination,
                geometry.source_length,
                false,
                output,
            )
        }
        Edit::Replace { content, .. } => {
            let (start, end) = geometry.target.expect("Replace has a target");
            emit_range(&mut reader, 0, start, false, output)?;
            output.emit(content.as_bytes(), Emission::Replacement)?;
            emit_range(&mut reader, end, geometry.source_length, false, output)
        }
        Edit::Delete { .. } => {
            let (start, end) = geometry.target.expect("Delete has a target");
            emit_range(&mut reader, 0, start, false, output)?;
            emit_range(&mut reader, end, geometry.source_length, false, output)
        }
        Edit::Copy { .. } => {
            let (start, end) = geometry.target.expect("Copy has a target");
            let destination = geometry.destination.expect("Copy has a destination");
            emit_range(&mut reader, 0, destination, false, output)?;
            emit_range(&mut reader, start, end, true, output)?;
            emit_range(
                &mut reader,
                destination,
                geometry.source_length,
                false,
                output,
            )
        }
        Edit::Move { .. } => {
            let (start, end) = geometry.target.expect("Move has a target");
            let destination = geometry.destination.expect("Move has a destination");
            if destination < start {
                debug_assert_eq!(geometry.adjusted_move_destination, Some(destination));
                emit_range(&mut reader, 0, destination, false, output)?;
                emit_range(&mut reader, start, end, false, output)?;
                emit_range(&mut reader, destination, start, false, output)?;
                emit_range(&mut reader, end, geometry.source_length, false, output)
            } else {
                let removed = end.checked_sub(start).ok_or(ApplyError::Unavailable)?;
                let adjusted = geometry
                    .adjusted_move_destination
                    .ok_or(ApplyError::Unavailable)?;
                let original_destination = adjusted
                    .checked_add(removed)
                    .ok_or(ApplyError::Unavailable)?;
                if original_destination != destination {
                    return Err(ApplyError::Unavailable);
                }
                emit_range(&mut reader, 0, start, false, output)?;
                emit_range(&mut reader, end, original_destination, false, output)?;
                emit_range(&mut reader, start, end, false, output)?;
                emit_range(
                    &mut reader,
                    original_destination,
                    geometry.source_length,
                    false,
                    output,
                )
            }
        }
    }
}

fn emit_range(
    reader: &mut File,
    start: usize,
    end: usize,
    copied: bool,
    output: &mut Output<'_, '_>,
) -> Result<(), ApplyError> {
    if start > end {
        return Err(ApplyError::Unavailable);
    }
    reader
        .seek(SeekFrom::Start(
            u64::try_from(start).map_err(|_| ApplyError::Unavailable)?,
        ))
        .map_err(|_| ApplyError::Unavailable)?;
    let mut position = start;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    while position < end {
        let length = end
            .checked_sub(position)
            .ok_or(ApplyError::Unavailable)?
            .min(scratch.len());
        let count = reader
            .read(&mut scratch[..length])
            .map_err(|_| ApplyError::Unavailable)?;
        if count == 0 {
            return Err(ApplyError::Unavailable);
        }
        let emission = if copied {
            Emission::Copied
        } else {
            Emission::Original {
                byte_start: position,
            }
        };
        output.emit(&scratch[..count], emission)?;
        position = position.checked_add(count).ok_or(ApplyError::Unavailable)?;
    }
    Ok(())
}

#[cfg(test)]
mod apply_tests {
    use std::{
        fs,
        io::{self, Cursor, Read},
    };

    use crate::{
        backwriter::{
            anchor::AnchorOutcome,
            anddress::{Anddress, AnddressTarget},
            apply::ApplyError,
            edit::Edit,
            view::ViewOutcome,
        },
        runtime::{
            AdmissionRoot, CurrentProof, WorkspaceAdmission, WorkspaceRuntime,
            source_scan::observe_source,
        },
    };

    use super::{
        SourceScanError, Temporary, edit_temporary_name, execute, publish, stage_source,
        stage_source_trusted,
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
            let count = buffer.len().min(self.bytes.len() - self.position);
            buffer[..count].copy_from_slice(&self.bytes[self.position..self.position + count]);
            self.position += count;
            Ok(count)
        }
    }

    fn runtime(root: &std::path::Path) -> WorkspaceRuntime {
        WorkspaceRuntime::open(
            root,
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap()
    }

    fn host_runtime(root: &std::path::Path) -> WorkspaceRuntime {
        WorkspaceRuntime::open_host_authoritative(
            root,
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap()
    }

    fn file(runtime: &WorkspaceRuntime, bytes: &[u8]) -> Anddress {
        use crate::backwriter::anddress::{AnddressIssuer, TargetGeometry};

        let mut reader = bytes;
        let state = observe_source(&mut reader, |_, _| Ok(())).unwrap();
        AnddressIssuer::new(
            &runtime.workspace_coordinate,
            "note.txt",
            &state.hash,
            state.byte_length,
            state.line_count,
        )
        .unwrap()
        .issue(TargetGeometry::File)
        .unwrap()
    }

    #[test]
    fn late_source_read_failure_removes_partial_staging() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "source").unwrap();
        let runtime = runtime(fixture.path());
        let (parent, _) = runtime.open_admitted_parent("note.txt").unwrap();
        let name = edit_temporary_name(&runtime, "note.txt", "staging").unwrap();
        let mut staging = Temporary::create(&parent, name.clone()).unwrap();
        let mut reader = FailingReader {
            bytes: b"partial".to_vec(),
            position: 0,
        };

        assert_eq!(
            stage_source(&mut reader, &mut staging),
            Err(SourceScanError::Read)
        );
        drop(staging);
        assert!(!fixture.path().join(name).exists());
        assert_eq!(
            fs::read(fixture.path().join("note.txt")).unwrap(),
            b"source"
        );

        let trusted_name = edit_temporary_name(&runtime, "note.txt", "staging").unwrap();
        let mut trusted_staging = Temporary::create(&parent, trusted_name.clone()).unwrap();
        let mut trusted_reader = FailingReader {
            bytes: b"partial".to_vec(),
            position: 0,
        };
        assert_eq!(
            stage_source_trusted(&mut trusted_reader, &mut trusted_staging, 7),
            Err(SourceScanError::Read)
        );
        drop(trusted_staging);
        assert!(!fixture.path().join(trusted_name).exists());
        assert_eq!(
            fs::read(fixture.path().join("note.txt")).unwrap(),
            b"source"
        );
    }

    #[test]
    fn staging_write_failure_is_recoverable_and_removes_the_temporary() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), "source").unwrap();
        let runtime = runtime(fixture.path());
        let (parent, _) = runtime.open_admitted_parent("note.txt").unwrap();
        let name = edit_temporary_name(&runtime, "note.txt", "staging").unwrap();
        let mut staging = Temporary::create(&parent, name.clone()).unwrap();
        staging.close().unwrap();
        let mut reader = Cursor::new(b"source".as_slice());

        assert_eq!(
            stage_source(&mut reader, &mut staging),
            Err(SourceScanError::Resource)
        );
        drop(staging);
        assert!(!fixture.path().join(name).exists());
        assert_eq!(
            fs::read(fixture.path().join("note.txt")).unwrap(),
            b"source"
        );

        let trusted_name = edit_temporary_name(&runtime, "note.txt", "staging").unwrap();
        let mut trusted_staging = Temporary::create(&parent, trusted_name.clone()).unwrap();
        trusted_staging.close().unwrap();
        let mut trusted_reader = Cursor::new(b"source".as_slice());
        assert_eq!(
            stage_source_trusted(&mut trusted_reader, &mut trusted_staging, 6),
            Err(SourceScanError::Resource)
        );
        drop(trusted_staging);
        assert!(!fixture.path().join(trusted_name).exists());
        assert_eq!(
            fs::read(fixture.path().join("note.txt")).unwrap(),
            b"source"
        );
    }

    #[test]
    fn staging_and_after_collisions_preserve_source_proof_anchor_and_collision() {
        for purpose in ["staging", "after"] {
            let fixture = tempfile::tempdir().unwrap();
            let bytes = b"source";
            fs::write(fixture.path().join("note.txt"), bytes).unwrap();
            let mut runtime = host_runtime(fixture.path());
            let name = edit_temporary_name(&runtime, "note.txt", purpose).unwrap();
            fs::write(fixture.path().join(&name), "collision").unwrap();
            let target = file(&runtime, bytes);
            let handle = match runtime.anchor(&target).unwrap() {
                AnchorOutcome::Anchored(handle) => handle,
                AnchorOutcome::AlreadyLive => panic!("File Anchor"),
            };
            runtime
                .install_search_proofs(vec![
                    CurrentProof::new(
                        "note.txt",
                        target.source_state_hash().to_owned(),
                        target.source_byte_length(),
                        target.source_line_count(),
                    )
                    .unwrap(),
                ])
                .unwrap();

            assert_eq!(
                execute(
                    &mut runtime,
                    &Edit::Replace {
                        target: target.clone(),
                        content: "changed".to_owned(),
                    },
                    Some(&target),
                ),
                Err(ApplyError::Unavailable)
            );
            assert_eq!(fs::read(fixture.path().join("note.txt")).unwrap(), bytes);
            assert_eq!(
                fs::read_to_string(fixture.path().join(name)).unwrap(),
                "collision"
            );
            let proofs = runtime.current_proofs.lock().unwrap();
            assert_eq!(proofs.len(), 1);
            assert_eq!(proofs[0].logical_path, "note.txt");
            assert_eq!(proofs[0].hash, target.source_state_hash());
            drop(proofs);
            assert!(matches!(
                runtime.view_anchored(&handle, AnddressTarget::File),
                Ok(ViewOutcome::File { text, .. }) if text == "source"
            ));
        }
    }

    #[test]
    fn failed_rename_is_uncertain_and_removes_the_after_temporary() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("source.txt"), "before").unwrap();
        fs::create_dir(fixture.path().join("destination")).unwrap();
        let runtime = runtime(fixture.path());
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
        assert!(fixture.path().join("destination").is_dir());
        assert!(!fixture.path().join(name).exists());
        assert_eq!(
            fs::read_to_string(fixture.path().join("source.txt")).unwrap(),
            "before"
        );
    }
}
