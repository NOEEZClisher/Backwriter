//! Runtime admission, capability execution, and optional Host source proofs.

use std::{
    fmt,
    path::{Path, PathBuf},
    sync::{Mutex, Weak},
};

#[cfg(any(unix, windows))]
use std::fs;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[cfg(windows)]
use std::path::Component;

use thiserror::Error;

use crate::backwriter::anchor::{Anchedress, AnchorError, AnchorOutcome};
use crate::backwriter::anddress::{Anddress, AnddressTarget};
use crate::backwriter::apply::{ApplyError, EditReceipt};
use crate::backwriter::check::{CheckError, CheckOutcome, CheckStatus};
use crate::backwriter::edit::Edit;
use crate::backwriter::pick::PickOutcome;
use crate::backwriter::search::{SearchError, SearchOutcome, SearchRequest};
use crate::backwriter::view::{ViewError, ViewOutcome};
#[cfg(any(unix, windows))]
use crate::hash::transcript_hex;
#[cfg(any(unix, windows))]
use crate::safe_path::open_workspace_root;
use crate::safe_path::{
    SafeReadError, WorkspaceRoot, classify_child, open_directory, open_regular,
};
use crate::source::validate_logical_path;

mod anchor;
mod apply;
mod check;
mod search;
mod source_scan;
mod structural_cursor;
mod view;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AdmissionRoot(String);

impl AdmissionRoot {
    pub fn new(value: impl AsRef<str>) -> Result<Self, RuntimeError> {
        let value = value.as_ref();
        if value != "." && validate_logical_path(value).is_err() {
            return Err(RuntimeError::InvalidAdmission);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for AdmissionRoot {
    type Error = RuntimeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceAdmission {
    roots: Vec<AdmissionRoot>,
}

impl WorkspaceAdmission {
    pub fn new(roots: impl IntoIterator<Item = AdmissionRoot>) -> Result<Self, RuntimeError> {
        let mut roots: Vec<_> = roots.into_iter().collect();
        if roots.is_empty() {
            return Err(RuntimeError::InvalidAdmission);
        }
        roots.sort_unstable_by(|left, right| {
            left.as_str().as_bytes().cmp(right.as_str().as_bytes())
        });
        if roots.windows(2).any(|roots| roots[0] == roots[1]) {
            return Err(RuntimeError::InvalidAdmission);
        }
        if roots.len() > 1
            && roots
                .binary_search_by(|root| root.as_str().as_bytes().cmp(b"."))
                .is_ok()
        {
            return Err(RuntimeError::InvalidAdmission);
        }
        for root in &roots {
            for (index, _) in root.as_str().match_indices('/') {
                let ancestor = &root.as_str()[..index];
                if roots
                    .binary_search_by(|candidate| {
                        candidate.as_str().as_bytes().cmp(ancestor.as_bytes())
                    })
                    .is_ok()
                {
                    return Err(RuntimeError::InvalidAdmission);
                }
            }
        }
        Ok(Self { roots })
    }

    pub fn roots(&self) -> &[AdmissionRoot] {
        &self.roots
    }

    fn root_index_for_path(&self, logical_path: &str) -> Option<usize> {
        if let Ok(index) = self
            .roots
            .binary_search_by(|root| root.as_str().as_bytes().cmp(b"."))
        {
            return Some(index);
        }
        let mut end = logical_path.len();
        loop {
            let candidate = &logical_path[..end];
            if let Ok(index) = self
                .roots
                .binary_search_by(|root| root.as_str().as_bytes().cmp(candidate.as_bytes()))
            {
                return Some(index);
            }
            end = candidate.rfind('/')?;
        }
    }
}

pub struct WorkspaceRuntime {
    pub(crate) workspace_root: PathBuf,
    pub(crate) workspace: WorkspaceRoot,
    pub(crate) workspace_coordinate: String,
    pub(crate) admission: WorkspaceAdmission,
    pub(crate) anchors: Vec<AnchorBinding>,
    authority: ObservationAuthority,
    current_proofs: Mutex<Vec<CurrentProof>>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ObservationAuthority {
    Untrusted,
    HostAuthoritative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CurrentProofMatch {
    Missing,
    Matching,
    Mismatched,
}

#[derive(Clone, Copy)]
struct SourceProofEvidence {
    hash: [u8; 64],
    byte_length: usize,
    line_count: usize,
}

fn source_state_matches(
    hash: &[u8],
    byte_length: usize,
    line_count: usize,
    input: &Anddress,
) -> bool {
    hash == input.source_state_hash().as_bytes()
        && byte_length == input.source_byte_length()
        && line_count == input.source_line_count()
}

fn compare_source_keys(left: &Anddress, right: &Anddress) -> std::cmp::Ordering {
    left.workspace_coordinate()
        .as_bytes()
        .cmp(right.workspace_coordinate().as_bytes())
        .then_with(|| {
            left.logical_path()
                .as_bytes()
                .cmp(right.logical_path().as_bytes())
        })
}

struct CurrentProof {
    logical_path: String,
    hash: String,
    byte_length: usize,
    line_count: usize,
}

impl CurrentProof {
    fn new(
        logical_path: &str,
        hash: String,
        byte_length: usize,
        line_count: usize,
    ) -> Result<Self, SearchError> {
        Self::prepare(logical_path, hash, byte_length, line_count).ok_or(SearchError::Unavailable)
    }

    fn prepare(
        logical_path: &str,
        hash: String,
        byte_length: usize,
        line_count: usize,
    ) -> Option<Self> {
        let mut owned_path = String::new();
        owned_path.try_reserve_exact(logical_path.len()).ok()?;
        owned_path.push_str(logical_path);
        Some(Self {
            logical_path: owned_path,
            hash,
            byte_length,
            line_count,
        })
    }
}

pub(crate) struct AnchorBinding {
    pub(crate) token: Weak<()>,
    pub(crate) anddress: Anddress,
}

pub(crate) enum AnchorPlanEntry {
    Preserve,
    Remove,
    Rebind { anddress: Anddress, collides: bool },
}

impl fmt::Debug for WorkspaceRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkspaceRuntime")
            .field("workspace_root", &self.workspace_root)
            .field("workspace_coordinate", &self.workspace_coordinate)
            .field("admission", &self.admission)
            .field("anchors", &"<redacted>")
            .finish()
    }
}

impl WorkspaceRuntime {
    #[cfg(any(unix, windows))]
    pub fn open(
        workspace_root: impl AsRef<Path>,
        admission: WorkspaceAdmission,
    ) -> Result<Self, RuntimeError> {
        Self::open_with_authority(
            workspace_root.as_ref(),
            admission,
            ObservationAuthority::Untrusted,
        )
    }

    /// Opens a Runtime whose host coordinates every source-visible writer and
    /// logical-path replacement. The host must call [`Self::invalidate_source`]
    /// synchronously before mutation and exclude mutation while any Runtime
    /// capability call is executing; metadata or later notification is not a
    /// substitute for that contract.
    #[cfg(any(unix, windows))]
    pub fn open_host_authoritative(
        workspace_root: impl AsRef<Path>,
        admission: WorkspaceAdmission,
    ) -> Result<Self, RuntimeError> {
        Self::open_with_authority(
            workspace_root.as_ref(),
            admission,
            ObservationAuthority::HostAuthoritative,
        )
    }

    #[cfg(any(unix, windows))]
    fn open_with_authority(
        workspace_root: &Path,
        admission: WorkspaceAdmission,
        authority: ObservationAuthority,
    ) -> Result<Self, RuntimeError> {
        if !workspace_root.is_absolute() {
            return Err(RuntimeError::InvalidWorkspace);
        }
        let metadata =
            fs::symlink_metadata(workspace_root).map_err(|_| RuntimeError::InvalidWorkspace)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(RuntimeError::InvalidWorkspace);
        }
        let canonical =
            fs::canonicalize(workspace_root).map_err(|_| RuntimeError::InvalidWorkspace)?;
        #[cfg(not(windows))]
        if canonical != workspace_root {
            return Err(RuntimeError::InvalidWorkspace);
        }
        #[cfg(windows)]
        if workspace_root
            .components()
            .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
        {
            return Err(RuntimeError::InvalidWorkspace);
        }
        let workspace =
            open_workspace_root(&canonical).map_err(|_| RuntimeError::InvalidWorkspace)?;
        let workspace_coordinate = workspace_coordinate(&canonical);
        Ok(Self {
            workspace_root: canonical,
            workspace,
            workspace_coordinate,
            admission,
            anchors: Vec::new(),
            authority,
            current_proofs: Mutex::new(Vec::new()),
        })
    }

    #[cfg(not(any(unix, windows)))]
    pub fn open(
        _workspace_root: impl AsRef<Path>,
        _admission: WorkspaceAdmission,
    ) -> Result<Self, RuntimeError> {
        Err(RuntimeError::UnsupportedPlatform)
    }

    #[cfg(not(any(unix, windows)))]
    pub fn open_host_authoritative(
        _workspace_root: impl AsRef<Path>,
        _admission: WorkspaceAdmission,
    ) -> Result<Self, RuntimeError> {
        Err(RuntimeError::UnsupportedPlatform)
    }

    /// Searches current admitted Workspace Source without retaining source
    /// bytes or result state after this call returns.
    pub fn search(&self, request: &SearchRequest) -> Result<SearchOutcome, SearchError> {
        search::execute(self, request)
    }

    /// Checks one caller-provided target against current admitted Workspace Source.
    pub fn check(&self, input: Anddress) -> Result<CheckOutcome<Option<Anddress>>, CheckError> {
        check::check_one(self, input)
    }

    /// Checks ordered caller-provided targets, preserving one status per input occurrence.
    pub fn check_batch(&self, inputs: &[Anddress]) -> Result<Vec<CheckStatus>, CheckError> {
        check::check_batch(self, inputs)
    }

    /// Checks a Search outcome without interpreting its query or payload provenance.
    pub fn check_search(
        &self,
        input: SearchOutcome,
    ) -> Result<CheckOutcome<SearchOutcome>, CheckError> {
        check::check_search(self, input)
    }

    /// Checks a Pick outcome without interpreting its predicate provenance.
    pub fn check_pick(&self, input: PickOutcome) -> Result<CheckOutcome<PickOutcome>, CheckError> {
        check::check_pick(self, input)
    }

    /// Projects one caller-provided target upward from current admitted source.
    pub fn view(
        &self,
        anddress: &Anddress,
        projection: AnddressTarget,
    ) -> Result<ViewOutcome, ViewError> {
        view::execute(self, anddress, projection)
    }

    /// Projects an ordered Anddress collection with per-source observation reuse.
    /// None keeps each input's target; Some selects one common upward target.
    pub fn view_batch(
        &self,
        anddresses: &[Anddress],
        projection: Option<AnddressTarget>,
    ) -> Result<Vec<ViewOutcome>, ViewError> {
        view::execute_batch(self, anddresses, projection)
    }

    /// Applies one caller-owned Edit to one current admitted logical source.
    pub fn apply(&mut self, edit: &Edit) -> Result<(), ApplyError> {
        apply::execute(self, edit, None).map(drop)
    }

    /// Applies one Replace and returns its exact current-state result.
    pub fn apply_replace(&mut self, edit: &Edit) -> Result<EditReceipt, ApplyError> {
        let Edit::Replace { target, .. } = edit else {
            return Err(ApplyError::InvalidInput);
        };
        apply::execute(self, edit, Some(target))?.ok_or(ApplyError::Unavailable)
    }

    pub fn anchor(&mut self, anddress: &Anddress) -> Result<AnchorOutcome, AnchorError> {
        anchor::anchor(self, anddress)
    }

    pub fn view_anchored(
        &mut self,
        anchedress: &Anchedress,
        projection: AnddressTarget,
    ) -> Result<ViewOutcome, ViewError> {
        anchor::view_anchored(self, anchedress, projection)
    }

    pub fn invalidate_anchored_source(&mut self, path: &str) -> Result<(), AnchorError> {
        anchor::invalidate_source(self, path)
    }

    /// Invalidates Host-authoritative proof and live Anchor state for one
    /// admitted logical source before a host-coordinated mutation.
    pub fn invalidate_source(&mut self, path: &str) -> Result<(), AnchorError> {
        anchor::invalidate_source(self, path)
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn admission(&self) -> &WorkspaceAdmission {
        &self.admission
    }

    pub(crate) fn selected_root(&self, logical_path: &str) -> Result<&AdmissionRoot, RuntimeError> {
        self.admission
            .root_index_for_path(logical_path)
            .map(|index| &self.admission.roots[index])
            .ok_or(RuntimeError::UnadmittedPath)
    }

    pub(crate) fn open_admitted_directory(
        &self,
        path: &str,
    ) -> Result<cap_std::fs::Dir, DirectoryAccessError> {
        self.selected_root(path)
            .map_err(|_| DirectoryAccessError::Unadmitted)?;
        let mut directory = self
            .workspace
            .directory()
            .try_clone()
            .map_err(|_| DirectoryAccessError::Unavailable)?;
        if path == "." {
            return Ok(directory);
        }
        for component in path.split('/') {
            let classified = classify_child(&directory, component).map_err(map_safe_read_error)?;
            directory =
                open_directory(&directory, component, classified).map_err(map_safe_read_error)?;
        }
        Ok(directory)
    }

    pub(crate) fn open_admitted_source(
        &self,
        path: &str,
    ) -> Result<cap_std::fs::File, DirectoryAccessError> {
        let (parent, name) = self.open_admitted_parent(path)?;
        let classified = classify_child(&parent, name).map_err(map_safe_read_error)?;
        open_regular(&parent, name, classified).map_err(map_safe_read_error)
    }

    pub(crate) fn open_admitted_parent<'a>(
        &self,
        path: &'a str,
    ) -> Result<(cap_std::fs::Dir, &'a str), DirectoryAccessError> {
        self.selected_root(path)
            .map_err(|_| DirectoryAccessError::Unadmitted)?;
        let (parent_path, name) = path.rsplit_once('/').unwrap_or((".", path));
        let parent = self.open_admitted_directory(parent_path)?;
        Ok((parent, name))
    }

    pub(crate) fn invalidate_anchors_for_path(&mut self, path: &str) {
        self.anchors
            .retain(|binding| binding.anddress.logical_path() != path);
    }

    fn invalidate_current_proof(&self, path: &str) {
        if self.authority == ObservationAuthority::Untrusted {
            return;
        }
        match self.current_proofs.lock() {
            Ok(mut proofs) => proofs.retain(|proof| proof.logical_path != path),
            Err(poisoned) => poisoned.into_inner().clear(),
        }
    }

    fn match_current_proof(&self, input: &Anddress) -> CurrentProofMatch {
        match self.select_current_proof(input.logical_path()) {
            Some(proof)
                if source_state_matches(
                    &proof.hash,
                    proof.byte_length,
                    proof.line_count,
                    input,
                ) =>
            {
                CurrentProofMatch::Matching
            }
            Some(_) => CurrentProofMatch::Mismatched,
            None => CurrentProofMatch::Missing,
        }
    }

    fn select_current_proof(&self, path: &str) -> Option<SourceProofEvidence> {
        if self.authority == ObservationAuthority::Untrusted {
            return None;
        }
        let current = match self.current_proofs.lock() {
            Ok(current) => current,
            Err(mut poisoned) => {
                poisoned.get_mut().clear();
                return None;
            }
        };
        let index = current
            .binary_search_by(|proof| proof.logical_path.as_bytes().cmp(path.as_bytes()))
            .ok()?;
        let proof = &current[index];
        let hash: [u8; 64] = proof.hash.as_bytes().try_into().ok()?;
        Some(SourceProofEvidence {
            hash,
            byte_length: proof.byte_length,
            line_count: proof.line_count,
        })
    }

    fn invalidate_source_state(&mut self, path: &str) {
        self.invalidate_current_proof(path);
        self.invalidate_anchors_for_path(path);
    }

    fn install_search_proofs(&self, mut observed: Vec<CurrentProof>) -> Result<(), SearchError> {
        if self.authority == ObservationAuthority::Untrusted {
            return Ok(());
        }
        observed.sort_unstable_by(|left, right| {
            left.logical_path
                .as_bytes()
                .cmp(right.logical_path.as_bytes())
        });
        debug_assert!(
            observed
                .windows(2)
                .all(|pair| pair[0].logical_path != pair[1].logical_path)
        );
        let mut current = match self.current_proofs.lock() {
            Ok(current) => current,
            Err(mut poisoned) => {
                poisoned.get_mut().clear();
                return Err(SearchError::Unavailable);
            }
        };
        let additional = observed
            .iter()
            .filter(|proof| {
                current
                    .binary_search_by(|candidate| {
                        candidate
                            .logical_path
                            .as_bytes()
                            .cmp(proof.logical_path.as_bytes())
                    })
                    .is_err()
            })
            .count();
        current
            .try_reserve(additional)
            .map_err(|_| SearchError::Unavailable)?;
        for proof in observed {
            match current.binary_search_by(|candidate| {
                candidate
                    .logical_path
                    .as_bytes()
                    .cmp(proof.logical_path.as_bytes())
            }) {
                Ok(index)
                    if current[index].hash != proof.hash
                        || current[index].byte_length != proof.byte_length
                        || current[index].line_count != proof.line_count =>
                {
                    current[index] = proof;
                }
                Ok(_) => {}
                Err(index) => current.insert(index, proof),
            }
        }
        Ok(())
    }

    fn prepare_current_proof_installation(
        &mut self,
        path: &str,
        hash: String,
        byte_length: usize,
        line_count: usize,
    ) -> Result<Option<CurrentProof>, ApplyError> {
        if self.authority == ObservationAuthority::Untrusted {
            return Ok(None);
        }
        let proof = CurrentProof::prepare(path, hash, byte_length, line_count)
            .ok_or(ApplyError::Unavailable)?;
        let current = match self.current_proofs.get_mut() {
            Ok(current) => current,
            Err(poisoned) => {
                poisoned.into_inner().clear();
                return Ok(None);
            }
        };
        if current
            .binary_search_by(|candidate| candidate.logical_path.as_bytes().cmp(path.as_bytes()))
            .is_err()
        {
            current
                .try_reserve(1)
                .map_err(|_| ApplyError::Unavailable)?;
        }
        Ok(Some(proof))
    }

    fn install_prepared_current_proof(&mut self, proof: Option<CurrentProof>) {
        let Some(proof) = proof else {
            return;
        };
        let current = match self.current_proofs.get_mut() {
            Ok(current) => current,
            Err(_) => unreachable!("prepared current proof state remains usable"),
        };
        match current.binary_search_by(|candidate| {
            candidate
                .logical_path
                .as_bytes()
                .cmp(proof.logical_path.as_bytes())
        }) {
            Ok(index) => current[index] = proof,
            Err(index) => current.insert(index, proof),
        }
    }

    pub(crate) fn prune_dead_anchors(&mut self) {
        self.anchors
            .retain(|binding| binding.token.upgrade().is_some());
    }

    pub(crate) fn reflect_anchors(&mut self, plan: Vec<AnchorPlanEntry>) {
        debug_assert_eq!(self.anchors.len(), plan.len());
        // `retain` is allocation-free and performs removal only after all
        // successful publication state is available.
        let mut actions = plan.into_iter();
        self.anchors.retain_mut(|binding| match actions.next() {
            Some(AnchorPlanEntry::Preserve) => true,
            Some(AnchorPlanEntry::Remove) | None => false,
            Some(AnchorPlanEntry::Rebind { anddress, collides }) => {
                if collides {
                    false
                } else {
                    binding.anddress = anddress;
                    true
                }
            }
        });
    }
}

fn prospective_address(entry: &AnchorPlanEntry) -> Option<&Anddress> {
    match entry {
        AnchorPlanEntry::Rebind { anddress, .. } => Some(anddress),
        AnchorPlanEntry::Preserve | AnchorPlanEntry::Remove => None,
    }
}

fn mark_collision(entry: &mut AnchorPlanEntry) {
    if let AnchorPlanEntry::Rebind { collides, .. } = entry {
        *collides = true;
    }
}

pub(crate) fn mark_anchor_collisions(plan: &mut [AnchorPlanEntry]) {
    for left in 0..plan.len() {
        if prospective_address(&plan[left]).is_none() {
            continue;
        }
        for right in left + 1..plan.len() {
            let collides = prospective_address(&plan[left])
                .zip(prospective_address(&plan[right]))
                .is_some_and(|(left, right)| left == right);
            if collides {
                mark_collision(&mut plan[left]);
                mark_collision(&mut plan[right]);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DirectoryAccessError {
    Unadmitted,
    NotCurrent,
    Unavailable,
}

fn map_safe_read_error(error: SafeReadError) -> DirectoryAccessError {
    match error {
        SafeReadError::NotCurrent => DirectoryAccessError::NotCurrent,
        SafeReadError::Unavailable => DirectoryAccessError::Unavailable,
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum RuntimeError {
    #[error("workspace Runtime is unsupported on this platform")]
    UnsupportedPlatform,
    #[error("workspace admission is invalid")]
    InvalidAdmission,
    #[error("workspace root is invalid")]
    InvalidWorkspace,
    #[error("logical path is not admitted")]
    UnadmittedPath,
}

fn path_is_within_root(logical_path: &str, root: &str) -> bool {
    root == "."
        || logical_path == root
        || logical_path
            .strip_prefix(root)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

#[cfg(unix)]
fn workspace_coordinate(workspace_root: &Path) -> String {
    transcript_hex(
        "artext.backwriter-workspace-coordinate.v3",
        [b"unix".as_slice(), workspace_root.as_os_str().as_bytes()],
    )
}

#[cfg(windows)]
fn workspace_coordinate(workspace_root: &Path) -> String {
    let root_bytes: Vec<u8> = workspace_root
        .as_os_str()
        .encode_wide()
        .flat_map(u16::to_le_bytes)
        .collect();
    transcript_hex(
        "artext.backwriter-workspace-coordinate.v3",
        [b"windows".as_slice(), root_bytes.as_slice()],
    )
}

#[cfg(not(windows))]
pub(crate) fn is_backwriter_spill(path: &str) -> bool {
    path == ".bw"
        || path.starts_with(".bw/")
        || path == ".artext/bw"
        || path.starts_with(".artext/bw/")
}

#[cfg(all(test, any(unix, windows)))]
mod tests {
    use super::{
        AdmissionRoot, AnchorBinding, AnchorPlanEntry, CurrentProof, WorkspaceAdmission,
        WorkspaceRuntime, mark_anchor_collisions, workspace_coordinate,
    };

    fn address(
        path: &str,
        target: crate::backwriter::anddress::AnddressTarget,
        byte_start: usize,
        byte_end: usize,
    ) -> crate::backwriter::anddress::Anddress {
        use crate::backwriter::anddress::{
            AnddressIssuer, ParagraphGeometry, ParentGeometry, TargetGeometry,
        };

        let issuer = AnddressIssuer::new(&"a".repeat(64), path, &"b".repeat(64), 10, 1).unwrap();
        issuer
            .issue(match target {
                crate::backwriter::anddress::AnddressTarget::File => TargetGeometry::File,
                crate::backwriter::anddress::AnddressTarget::Paragraph => {
                    TargetGeometry::Paragraph(ParagraphGeometry {
                        byte_start,
                        byte_end,
                        file_line_offset: 0,
                        line_count: 1,
                    })
                }
                crate::backwriter::anddress::AnddressTarget::Line => TargetGeometry::Line {
                    byte_start,
                    byte_end,
                    terminator: crate::backwriter::anddress::LineTerminator::None,
                    line_offset_in_parent: 0,
                    parent: ParentGeometry::File,
                },
            })
            .unwrap()
    }

    #[test]
    fn collision_marking_invalidates_every_member_of_two_and_three_way_collisions() {
        let address = address(
            "note.txt",
            crate::backwriter::anddress::AnddressTarget::File,
            0,
            10,
        );
        let mut two_way = vec![
            AnchorPlanEntry::Rebind {
                anddress: address.clone(),
                collides: false,
            },
            AnchorPlanEntry::Rebind {
                anddress: address.clone(),
                collides: false,
            },
        ];
        mark_anchor_collisions(&mut two_way);
        assert!(
            two_way
                .iter()
                .all(|entry| { matches!(entry, AnchorPlanEntry::Rebind { collides: true, .. }) })
        );

        let mut three_way = vec![
            AnchorPlanEntry::Rebind {
                anddress: address.clone(),
                collides: false,
            },
            AnchorPlanEntry::Rebind {
                anddress: address.clone(),
                collides: false,
            },
            AnchorPlanEntry::Rebind {
                anddress: address,
                collides: false,
            },
        ];
        mark_anchor_collisions(&mut three_way);
        assert!(
            three_way
                .iter()
                .all(|entry| { matches!(entry, AnchorPlanEntry::Rebind { collides: true, .. }) })
        );
    }

    #[test]
    fn collision_marking_starts_only_from_prospective_rebinds() {
        let address = |path: &str| {
            address(
                path,
                crate::backwriter::anddress::AnddressTarget::File,
                0,
                10,
            )
        };
        let mut no_rebinds = vec![AnchorPlanEntry::Preserve, AnchorPlanEntry::Remove];
        mark_anchor_collisions(&mut no_rebinds);
        assert!(matches!(
            no_rebinds.as_slice(),
            [AnchorPlanEntry::Preserve, AnchorPlanEntry::Remove]
        ));

        let mut single = vec![
            AnchorPlanEntry::Preserve,
            AnchorPlanEntry::Rebind {
                anddress: address("one.txt"),
                collides: true,
            },
            AnchorPlanEntry::Remove,
        ];
        mark_anchor_collisions(&mut single);
        assert!(matches!(
            single.as_slice(),
            [
                AnchorPlanEntry::Preserve,
                AnchorPlanEntry::Rebind { collides: true, .. },
                AnchorPlanEntry::Remove,
            ]
        ));

        let a = address("a.txt");
        let b = address("b.txt");
        let mut nonadjacent = vec![
            AnchorPlanEntry::Preserve,
            AnchorPlanEntry::Rebind {
                anddress: a.clone(),
                collides: true,
            },
            AnchorPlanEntry::Remove,
            AnchorPlanEntry::Rebind {
                anddress: b,
                collides: false,
            },
            AnchorPlanEntry::Rebind {
                anddress: a,
                collides: false,
            },
        ];
        mark_anchor_collisions(&mut nonadjacent);
        assert!(matches!(
            nonadjacent.as_slice(),
            [
                AnchorPlanEntry::Preserve,
                AnchorPlanEntry::Rebind { collides: true, .. },
                AnchorPlanEntry::Remove,
                AnchorPlanEntry::Rebind {
                    collides: false,
                    ..
                },
                AnchorPlanEntry::Rebind { collides: true, .. },
            ]
        ));
    }

    #[test]
    fn publication_uncertainty_invalidates_only_the_exact_logical_path() {
        let fixture = tempfile::tempdir().unwrap();
        let mut runtime = WorkspaceRuntime::open_host_authoritative(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        runtime
            .install_search_proofs(vec![
                CurrentProof::new("first.txt", "a".repeat(64), 1, 1).unwrap(),
                CurrentProof::new("second.txt", "b".repeat(64), 2, 1).unwrap(),
            ])
            .unwrap();
        let first = crate::backwriter::anchor::Anchedress::new();
        let second = crate::backwriter::anchor::Anchedress::new();
        let address = |path: &str| {
            address(
                path,
                crate::backwriter::anddress::AnddressTarget::File,
                0,
                10,
            )
        };
        runtime.anchors.push(AnchorBinding {
            token: first.weak(),
            anddress: address("first.txt"),
        });
        runtime.anchors.push(AnchorBinding {
            token: second.weak(),
            anddress: address("second.txt"),
        });
        assert_eq!(
            super::apply::finish_publication(
                &mut runtime,
                "first.txt",
                Vec::new(),
                Some(CurrentProof::new("first.txt", "c".repeat(64), 3, 1).unwrap()),
                Err(crate::backwriter::apply::ApplyError::PublicationUncertain),
            ),
            Err(crate::backwriter::apply::ApplyError::PublicationUncertain)
        );
        assert_eq!(runtime.anchors.len(), 1);
        assert_eq!(runtime.anchors[0].anddress.logical_path(), "second.txt");
        let proofs = runtime.current_proofs.lock().unwrap();
        assert_eq!(proofs.len(), 1);
        assert_eq!(proofs[0].logical_path, "second.txt");
    }

    #[test]
    fn reflection_removes_every_two_and_three_way_collision() {
        let fixture = tempfile::tempdir().unwrap();
        let mut runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let handles = [
            crate::backwriter::anchor::Anchedress::new(),
            crate::backwriter::anchor::Anchedress::new(),
            crate::backwriter::anchor::Anchedress::new(),
        ];
        let address = |index: usize| {
            address(
                "note.txt",
                crate::backwriter::anddress::AnddressTarget::Paragraph,
                index,
                index + 1,
            )
        };
        for (index, handle) in handles.iter().enumerate() {
            runtime.anchors.push(AnchorBinding {
                token: handle.weak(),
                anddress: address(index),
            });
        }
        let collision = address(9);
        runtime.reflect_anchors(vec![
            AnchorPlanEntry::Rebind {
                anddress: collision.clone(),
                collides: true,
            },
            AnchorPlanEntry::Rebind {
                anddress: collision.clone(),
                collides: true,
            },
            AnchorPlanEntry::Rebind {
                anddress: collision,
                collides: true,
            },
        ]);
        assert!(runtime.anchors.is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn unix_workspace_coordinate_uses_raw_root_bytes_and_fixed_domain() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        let path =
            std::path::PathBuf::from(OsString::from_vec(b"/tmp/artext-\xff-coordinate".to_vec()));
        assert_eq!(
            workspace_coordinate(&path),
            "63fdd0e55582896460d5350af06bd95fcb0cc387373ca56c11f195d153aef11d"
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_workspace_coordinate_uses_utf16_little_endian_fixed_kat() {
        assert_eq!(
            workspace_coordinate(std::path::Path::new(r"C:\artext")),
            "8eddb1b867f05712a7f7d5ca1594733cbc325e1d1513784918add02e60a4aedc"
        );
    }

    #[test]
    fn spill_boundary_uses_exact_root_components_and_platform_case() {
        for path in [
            ".bw",
            ".bw/file",
            ".bw/nested/file",
            ".artext/bw",
            ".artext/bw/file",
        ] {
            assert!(super::is_backwriter_spill(path), "{path}");
        }
        for path in [
            ".bw-notes",
            ".bw2",
            ".artext/bw2",
            ".artext/other",
            "x/.bw",
            "x/.bw/file",
            "x/.artext/bw",
        ] {
            assert!(!super::is_backwriter_spill(path), "{path}");
        }
        for path in [".BW", ".Bw/file", ".ARTEXT/BW", ".artext/Bw/file"] {
            assert_eq!(super::is_backwriter_spill(path), cfg!(windows), "{path}");
        }
    }
}

#[cfg(windows)]
pub(crate) fn is_backwriter_spill(path: &str) -> bool {
    let mut components = path.split('/');
    match components.next() {
        Some(root) if root.eq_ignore_ascii_case(".bw") => true,
        Some(root) if root.eq_ignore_ascii_case(".artext") => {
            matches!(components.next(), Some(child) if child.eq_ignore_ascii_case("bw"))
        }
        _ => false,
    }
}
