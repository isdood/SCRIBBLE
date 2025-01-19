//! Error Types Module
//! ================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 08:55:47 UTC
//! Last Updated: 2025-01-19 08:55:47 UTC
//! Version: 0.1.0
//! License: MIT

/// Math operation errors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MathError {
    /// Domain error (e.g., sqrt of negative number)
    DomainError,
    /// Overflow error
    Overflow,
    /// Underflow error
    Underflow,
    /// Division by zero
    DivisionByZero,
    /// Convergence failure
    ConvergenceFailure,
}

pub type MathResult<T> = Result<T, MathError>;
