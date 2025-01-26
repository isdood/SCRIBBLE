//! Error types for crystal-optimized heap allocation

use std::error::Error;
use std::fmt;

/// Error type for cube operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeError {
    /// Failed to allocate memory
    AllocationFailed,
    /// Failed to align memory
    AlignmentFailed,
    /// Invalid alignment requested
    InvalidAlignment,
}

impl fmt::Display for CubeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CubeError::AllocationFailed => write!(f, "failed to allocate memory"),
            CubeError::AlignmentFailed => write!(f, "failed to align memory"),
            CubeError::InvalidAlignment => write!(f, "invalid alignment requested"),
        }
    }
}

impl Error for CubeError {}
