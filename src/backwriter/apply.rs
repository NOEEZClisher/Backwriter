//! Public Apply error contract.

use thiserror::Error;

use crate::backwriter::anddress::Anddress;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditReceipt {
    Unchanged { anddress: Anddress },
    Changed { anddress: Option<Anddress> },
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ApplyError {
    #[error("Anddress version is unsupported")]
    UnsupportedVersion,
    #[error("Apply input is invalid")]
    InvalidInput,
    #[error("current source is unavailable")]
    Unavailable,
    #[error("source replacement result is uncertain")]
    PublicationUncertain,
}
