use std::{ffi::NulError, io};
use thiserror::Error;

/// A possible error occurring during [`MumbleLink`](crate::MumbleLink) creation.
#[derive(Debug, Error)]
pub enum Error {
    /// MumbleLink is disabled.
    #[error("mumblelink disabled")]
    Disabled,

    /// MumbleLink name has interior nul byte.
    #[error(transparent)]
    NulError(#[from] NulError),

    #[error(transparent)]
    WinError(#[from] windows::core::Error),

    #[error(transparent)]
    IoError(#[from] io::Error),
}
