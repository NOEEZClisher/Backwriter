//! Batch currentness reporting for caller-provided v4 addresses.

use thiserror::Error;

use crate::backwriter::anddress::Anddress;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckOutcome<T> {
    pub filtered: T,
    pub report: CheckReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckReport {
    current_count: usize,
    removed: Vec<Anddress>,
    unavailable: Vec<Anddress>,
}

impl CheckReport {
    pub fn current_count(&self) -> usize {
        self.current_count
    }

    pub fn removed_count(&self) -> usize {
        self.removed.len()
    }

    pub fn unavailable_count(&self) -> usize {
        self.unavailable.len()
    }

    pub fn checked_count(&self) -> usize {
        self.current_count + self.removed.len() + self.unavailable.len()
    }

    pub fn removed(&self) -> &[Anddress] {
        &self.removed
    }

    pub fn unavailable(&self) -> &[Anddress] {
        &self.unavailable
    }

    pub(crate) fn from_parts(
        current_count: usize,
        removed: Vec<Anddress>,
        unavailable: Vec<Anddress>,
    ) -> Self {
        Self {
            current_count,
            removed,
            unavailable,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CheckError {
    #[error("Anddress version is unsupported")]
    UnsupportedVersion,
    #[error("Check input is invalid")]
    InvalidInput,
    #[error("Check resource allocation failed")]
    Resource,
}
