//! Opaque Runtime-local continuity handles.

use std::{
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Weak},
};

use thiserror::Error;

/// A non-cloneable handle for one Runtime-local current target association.
pub struct Anchedress {
    token: Arc<()>,
    _runtime_local: PhantomData<Rc<()>>,
}

impl Anchedress {
    pub(crate) fn new() -> Self {
        Self {
            token: Arc::new(()),
            _runtime_local: PhantomData,
        }
    }

    pub(crate) fn weak(&self) -> Weak<()> {
        Arc::downgrade(&self.token)
    }
}

pub enum AnchorOutcome {
    Anchored(Anchedress),
    AlreadyLive,
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum AnchorError {
    #[error("Anddress version is unsupported")]
    UnsupportedVersion,
    #[error("Anchor input is invalid")]
    InvalidInput,
    #[error("current source is unavailable")]
    Unavailable,
}
