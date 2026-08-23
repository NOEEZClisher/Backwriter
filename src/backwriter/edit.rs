//! Inert, caller-owned Edit value validation.

use thiserror::Error;

use crate::backwriter::anddress::{Anddress, AnddressError, AnddressTarget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Position {
    Before(Anddress),
    After(Anddress),
    StartOf(Anddress),
    EndOf(Anddress),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Edit {
    Insert { position: Position, content: String },
    Replace { target: Anddress, content: String },
    Delete { target: Anddress },
    Move { target: Anddress, position: Position },
    Copy { target: Anddress, position: Position },
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum EditError {
    #[error("Anddress version is unsupported")]
    UnsupportedVersion,
    #[error("Edit input is invalid")]
    InvalidInput,
    #[error("Edit resource allocation failed")]
    Resource,
}

impl Edit {
    pub fn validate(&self) -> Result<(), EditError> {
        match self {
            Self::Insert { position, content } => {
                validate_position(position)?;
                validate_content(content)
            }
            Self::Replace { target, content } => {
                validate_anddress(target)?;
                validate_content(content)
            }
            Self::Delete { target } => validate_non_file_target(target),
            Self::Move { target, position } | Self::Copy { target, position } => {
                validate_non_file_target(target)?;
                validate_position(position)
            }
        }
    }
}

fn validate_position(position: &Position) -> Result<(), EditError> {
    match position {
        Position::Before(target) | Position::After(target) => {
            validate_non_file_target(target)
        }
        Position::StartOf(target) | Position::EndOf(target) => {
            validate_anddress(target)?;
            matches!(&target.target, AnddressTarget::File)
                .then_some(())
                .ok_or(EditError::InvalidInput)
        }
    }
}

fn validate_non_file_target(target: &Anddress) -> Result<(), EditError> {
    validate_anddress(target)?;
    matches!(
        &target.target,
        AnddressTarget::Paragraph { .. } | AnddressTarget::Line { .. }
    )
    .then_some(())
    .ok_or(EditError::InvalidInput)
}

fn validate_anddress(target: &Anddress) -> Result<(), EditError> {
    target.validate().map_err(|error| match error {
        AnddressError::UnsupportedVersion => EditError::UnsupportedVersion,
        AnddressError::Invalid | AnddressError::Encoding => EditError::InvalidInput,
        AnddressError::Resource => EditError::Resource,
    })
}

fn validate_content(content: &str) -> Result<(), EditError> {
    (!content.contains('\0'))
        .then_some(())
        .ok_or(EditError::InvalidInput)
}
