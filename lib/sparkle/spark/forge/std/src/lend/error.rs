//! Lending system error types

use std::error::Error;
use std::fmt;

/// Error type for lending operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LendError {
    /// The value is already mutably lent
    AlreadyLent,
    /// Cannot create mutable lend when immutable lends exist
    ImmutableLendsExist,
    /// The value has been moved or dropped
    ValueMoved,
}

impl fmt::Display for LendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LendError::AlreadyLent => write!(f, "value is already mutably lent"),
            LendError::ImmutableLendsExist => write!(f, "cannot create mutable lend when immutable lends exist"),
            LendError::ValueMoved => write!(f, "value has been moved or dropped"),
        }
    }
}

impl Error for LendError {}
