//! Exact current lookup for one caller-provided Anddress.

use thiserror::Error;

use crate::backwriter::anddress::{Anddress, AnddressError, LineTerminator};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewOutcome {
    File {
        text: String,
    },
    Paragraph {
        text: String,
        file: Anddress,
    },
    Line {
        content: String,
        terminator: LineTerminator,
        file: Anddress,
        paragraph: Option<Anddress>,
    },
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

pub(crate) fn validate_input(anddress: &Anddress) -> Result<(), ViewError> {
    anddress.validate().map_err(map_input_error)
}

fn map_input_error(error: AnddressError) -> ViewError {
    match error {
        AnddressError::UnsupportedVersion => ViewError::UnsupportedVersion,
        AnddressError::Invalid | AnddressError::Encoding => ViewError::InvalidInput,
        AnddressError::Resource => ViewError::Unavailable,
    }
}
