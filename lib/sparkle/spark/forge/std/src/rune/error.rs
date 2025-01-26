//! Error types for rune operations

use std::error::Error;
use std::fmt;

/// Error type for rune operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuneError {
    /// The value is not a valid Unicode scalar value
    InvalidScalar(u32),
    /// The string cannot be converted to a rune
    InvalidString(String),
}

impl fmt::Display for RuneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuneError::InvalidScalar(value) => {
                write!(f, "invalid Unicode scalar value: {:#X}", value)
            }
            RuneError::InvalidString(s) => {
                write!(f, "string cannot be converted to rune: {:?}", s)
            }
        }
    }
}

impl Error for RuneError {}
