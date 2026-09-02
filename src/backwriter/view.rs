//! Exact current self-or-ancestor projection for one caller-provided Anddress.

use thiserror::Error;

use crate::backwriter::anddress::{Anddress, AnddressError, AnddressTarget, LineTerminator};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewOutcome {
    File {
        anddress: Anddress,
        text: String,
    },
    Paragraph {
        anddress: Anddress,
        text: String,
        file: Anddress,
    },
    Line {
        anddress: Anddress,
        content: String,
        terminator: LineTerminator,
        file: Anddress,
        paragraph: Option<Anddress>,
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

pub(crate) fn validate_request(
    anddress: &Anddress,
    projection: AnddressTarget,
) -> Result<(), ViewError> {
    anddress.validate().map_err(map_input_error)?;
    validate_projection(anddress.target(), projection)
}

pub(crate) fn validate_projection(
    input: AnddressTarget,
    projection: AnddressTarget,
) -> Result<(), ViewError> {
    if matches!(
        (input, projection),
        (AnddressTarget::File, AnddressTarget::File)
            | (
                AnddressTarget::Paragraph,
                AnddressTarget::Paragraph | AnddressTarget::File
            )
            | (
                AnddressTarget::Line,
                AnddressTarget::Line | AnddressTarget::Paragraph | AnddressTarget::File
            )
    ) {
        Ok(())
    } else {
        Err(ViewError::InvalidInput)
    }
}

fn map_input_error(error: AnddressError) -> ViewError {
    match error {
        AnddressError::UnsupportedVersion => ViewError::UnsupportedVersion,
        AnddressError::Invalid | AnddressError::Encoding => ViewError::InvalidInput,
        AnddressError::Resource => ViewError::Unavailable,
    }
}
