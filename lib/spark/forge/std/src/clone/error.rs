//! Error types for crystal-optimized cloning

use std::error::Error;
use std::fmt;

/// Error type for clone operations
#[derive(Debug)]
pub enum CloneError {
    /// Failed to allocate memory
    AllocationFailed,
    /// Invalid clone strategy for type
    InvalidStrategy,
    /// Type cannot be cloned
    Uncloneable,
    /// Alignment error during clone
    AlignmentError,
}

impl fmt::Display for CloneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloneError::AllocationFailed => write!(f, "failed to allocate memory for clone"),
            CloneError::InvalidStrategy => write!(f, "invalid clone strategy for type"),
            CloneError::Uncloneable => write!(f, "type cannot be cloned"),
            CloneError::AlignmentError => write!(f, "alignment error during clone"),
        }
    }
}

impl Error for CloneError {}
