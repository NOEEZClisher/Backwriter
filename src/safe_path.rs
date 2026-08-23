//! Capability-relative, no-follow workspace access.

use std::io::ErrorKind;
use std::path::Path;

use cap_fs_ext::{
    FollowSymlinks, OpenOptionsFollowExt, OpenOptionsMaybeDirExt, OpenOptionsSyncExt,
};
use cap_std::fs::{Dir, File, OpenOptions};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SafeReadError {
    NotCurrent,
    Unavailable,
}

#[derive(Debug)]
pub(crate) struct WorkspaceRoot {
    directory: Dir,
}

impl WorkspaceRoot {
    pub(crate) fn directory(&self) -> &Dir {
        &self.directory
    }
}

pub(crate) fn open_workspace_root(path: &Path) -> Result<WorkspaceRoot, SafeReadError> {
    Dir::open_ambient_dir(path, cap_std::ambient_authority())
        .map(|directory| WorkspaceRoot { directory })
        .map_err(|_| SafeReadError::Unavailable)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClassifiedChild {
    Directory,
    Regular,
    Excluded,
}

pub(crate) fn directory_names(directory: &Dir) -> Result<Vec<std::ffi::OsString>, SafeReadError> {
    let entries = directory
        .read_dir(".")
        .map_err(|_| SafeReadError::Unavailable)?;
    let mut names = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|_| SafeReadError::Unavailable)?;
        names
            .try_reserve(1)
            .map_err(|_| SafeReadError::Unavailable)?;
        names.push(entry.file_name());
    }
    names.sort_unstable_by(|left, right| left.as_encoded_bytes().cmp(right.as_encoded_bytes()));
    Ok(names)
}

pub(crate) fn classify_child(
    parent: &Dir,
    component: &str,
) -> Result<ClassifiedChild, SafeReadError> {
    let metadata = parent.symlink_metadata(component).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            SafeReadError::NotCurrent
        } else {
            SafeReadError::Unavailable
        }
    })?;
    if metadata.file_type().is_symlink() || (!metadata.is_dir() && !metadata.is_file()) {
        return Ok(ClassifiedChild::Excluded);
    }
    Ok(if metadata.is_dir() {
        ClassifiedChild::Directory
    } else {
        ClassifiedChild::Regular
    })
}

pub(crate) fn open_directory(
    parent: &Dir,
    component: &str,
    classified: ClassifiedChild,
) -> Result<Dir, SafeReadError> {
    match classified {
        ClassifiedChild::Directory => {}
        ClassifiedChild::Regular | ClassifiedChild::Excluded => {
            return Err(SafeReadError::NotCurrent);
        }
    }
    let file = open_no_follow(parent, component)?;
    let directory = Dir::from_std_file(file.into_std());
    let metadata = directory
        .dir_metadata()
        .map_err(|_| SafeReadError::Unavailable)?;
    metadata
        .is_dir()
        .then_some(directory)
        .ok_or(SafeReadError::Unavailable)
}

pub(crate) fn open_regular(
    parent: &Dir,
    component: &str,
    classified: ClassifiedChild,
) -> Result<File, SafeReadError> {
    match classified {
        ClassifiedChild::Regular => {}
        ClassifiedChild::Directory | ClassifiedChild::Excluded => {
            return Err(SafeReadError::NotCurrent);
        }
    }
    let file = open_no_follow(parent, component)?;
    let metadata = file.metadata().map_err(|_| SafeReadError::Unavailable)?;
    metadata
        .is_file()
        .then_some(file)
        .ok_or(SafeReadError::Unavailable)
}

fn open_no_follow(parent: &Dir, component: &str) -> Result<File, SafeReadError> {
    let mut options = OpenOptions::new();
    options.read(true);
    options
        .follow(FollowSymlinks::No)
        .maybe_dir(true)
        .nonblock(true);
    parent
        .open_with(component, &options)
        .map_err(|_| SafeReadError::Unavailable)
}
