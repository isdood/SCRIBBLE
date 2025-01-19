//! Error Types for MeshMath Operations
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 10:01:28 UTC
//! Last Updated: 2025-01-19 10:01:28 UTC
//! Version: 0.1.0
//! License: MIT

#[derive(Debug)]
pub enum MathError {
    /// Overflow error
    Overflow,
    /// Underflow error
    Underflow,
    /// Division by zero
    DivisionByZero,
    /// Invalid operation
    InvalidOperation,
}

impl core::fmt::Display for MathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overflow => write!(f, "Mathematical overflow"),
            Self::Underflow => write!(f, "Mathematical underflow"),
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::InvalidOperation => write!(f, "Invalid mathematical operation"),
        }
    }
}
