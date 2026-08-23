//! Stateless Runtime admission plus Search, View, Apply, and Check execution.

use std::{
    fmt,
    path::{Path, PathBuf},
    sync::Weak,
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
use crate::backwriter::anddress::Anddress;
use crate::backwriter::apply::ApplyError;
use crate::backwriter::check::{CheckError, CheckOutcome};
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
        let workspace_root = workspace_root.as_ref();
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
        })
    }

    #[cfg(not(any(unix, windows)))]
    pub fn open(
        _workspace_root: impl AsRef<Path>,
        _admission: WorkspaceAdmission,
    ) -> Result<Self, RuntimeError> {
        Err(RuntimeError::UnsupportedPlatform)
    }

    /// Searches current admitted Workspace Source without retaining source or
    /// result state after this call returns.
    pub fn search(&self, request: &SearchRequest) -> Result<SearchOutcome, SearchError> {
        search::execute(self, request)
    }

    /// Checks one caller-provided target against current admitted Workspace Source.
    pub fn check(&self, input: Anddress) -> Result<CheckOutcome<Option<Anddress>>, CheckError> {
        check::check_one(self, input)
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

    /// Reconstructs one current admitted source target without retaining state.
    pub fn view(&self, anddress: &Anddress) -> Result<ViewOutcome, ViewError> {
        view::execute(self, anddress)
    }

    /// Applies one caller-owned Edit to one current admitted logical source.
    pub fn apply(&mut self, edit: &Edit) -> Result<(), ApplyError> {
        apply::execute(self, edit)
    }

    pub fn anchor(&mut self, anddress: &Anddress) -> Result<AnchorOutcome, AnchorError> {
        anchor::anchor(self, anddress)
    }

    pub fn view_anchored(&mut self, anchedress: &Anchedress) -> Result<ViewOutcome, ViewError> {
        anchor::view_anchored(self, anchedress)
    }

    pub fn invalidate_anchored_source(&mut self, path: &str) -> Result<(), AnchorError> {
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
            .retain(|binding| binding.anddress.logical_path != path);
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
    path == ".artext/bw" || path.strip_prefix(".artext/bw/").is_some()
}

#[cfg(all(test, any(unix, windows)))]
mod tests {
    use super::{
        AdmissionRoot, AnchorBinding, AnchorPlanEntry, WorkspaceAdmission, WorkspaceRuntime,
        mark_anchor_collisions, workspace_coordinate,
    };

    #[test]
    fn collision_marking_invalidates_every_member_of_two_and_three_way_collisions() {
        let address = crate::backwriter::anddress::Anddress {
            version: crate::backwriter::anddress::ANDDRESS_VERSION.to_owned(),
            workspace_coordinate: "a".repeat(64),
            logical_path: "note.txt".to_owned(),
            target: crate::backwriter::anddress::AnddressTarget::File,
        };
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
        let address = |path: &str| crate::backwriter::anddress::Anddress {
            version: crate::backwriter::anddress::ANDDRESS_VERSION.to_owned(),
            workspace_coordinate: "a".repeat(64),
            logical_path: path.to_owned(),
            target: crate::backwriter::anddress::AnddressTarget::File,
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
        let mut runtime = WorkspaceRuntime::open(
            fixture.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let first = crate::backwriter::anchor::Anchedress::new();
        let second = crate::backwriter::anchor::Anchedress::new();
        let address = |path: &str| crate::backwriter::anddress::Anddress {
            version: crate::backwriter::anddress::ANDDRESS_VERSION.to_owned(),
            workspace_coordinate: "a".repeat(64),
            logical_path: path.to_owned(),
            target: crate::backwriter::anddress::AnddressTarget::File,
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
                Err(crate::backwriter::apply::ApplyError::PublicationUncertain),
            ),
            Err(crate::backwriter::apply::ApplyError::PublicationUncertain)
        );
        assert_eq!(runtime.anchors.len(), 1);
        assert_eq!(runtime.anchors[0].anddress.logical_path, "second.txt");
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
        let address = |ordinal: usize| crate::backwriter::anddress::Anddress {
            version: crate::backwriter::anddress::ANDDRESS_VERSION.to_owned(),
            workspace_coordinate: "a".repeat(64),
            logical_path: "note.txt".to_owned(),
            target: crate::backwriter::anddress::AnddressTarget::Paragraph {
                ordinal: crate::backwriter::anddress::Natural::parse(&ordinal.to_string()).unwrap(),
            },
        };
        for (ordinal, handle) in handles.iter().enumerate() {
            runtime.anchors.push(AnchorBinding {
                token: handle.weak(),
                anddress: address(ordinal),
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

    #[cfg(windows)]
    #[test]
    fn windows_spill_boundary_is_ascii_case_insensitive_and_root_relative() {
        assert!(super::is_backwriter_spill(".ARTEXT/BW"));
        assert!(!super::is_backwriter_spill(".artext/bw2"));
        assert!(!super::is_backwriter_spill("x/.artext/bw"));
    }
}

#[cfg(windows)]
pub(crate) fn is_backwriter_spill(path: &str) -> bool {
    let mut components = path.split('/');
    matches!(components.next(), Some(root) if root.eq_ignore_ascii_case(".artext"))
        && matches!(components.next(), Some(child) if child.eq_ignore_ascii_case("bw"))
}
