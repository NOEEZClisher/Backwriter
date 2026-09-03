//! Exact current self-or-ancestor projection for one caller-provided Anddress.

use thiserror::Error;

use crate::backwriter::anddress::{Anddress, AnddressError, AnddressTarget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewOutcome {
    Projected {
        anddress: Anddress,
        content: String,
    },
    /// The requested Line-to-Paragraph relation does not exist.
    RelationAbsent,
}
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ViewError {
    #[error("Anddress version is unsupported")]
    UnsupportedVersion,
    #[error("Anddress input is invalid")]
    InvalidInput,
    #[error("current source is unavailable")]
    Unavailable,
}

pub(crate) fn project_request(
    anddress: &Anddress,
    projection: AnddressTarget,
) -> Result<Option<Anddress>, ViewError> {
    anddress.validate().map_err(map_input_error)?;
    anddress.project(projection).map_err(map_input_error)
}

fn map_input_error(error: AnddressError) -> ViewError {
    match error {
        AnddressError::UnsupportedVersion => ViewError::UnsupportedVersion,
        AnddressError::Invalid | AnddressError::Encoding => ViewError::InvalidInput,
        AnddressError::Resource => ViewError::Unavailable,
    }
}
