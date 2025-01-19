//! Error Types for Crystal Computing
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:53:17 UTC
//! Version: 0.1.0
//! License: MIT

use core::fmt;
use crate::align::AlignmentState;

/// Core error type for quantum operations
#[derive(Debug)]
pub enum QuantumError {
    /// Invalid quantum state
    InvalidState,
    /// Boundary violation in crystal structure
    BoundaryViolation,
    /// Loss of quantum coherence
    CoherenceLoss,
    /// Phase misalignment
    PhaseMisalignment,
    /// Resonance failure
    ResonanceFailure,
    /// Alignment failure
    AlignmentFailure(AlignmentState),
    /// Vector operation error
    VectorError(VectorError),
}

/// Error type for vector operations
#[derive(Debug)]
pub enum VectorError {
    /// Division by zero
    DivisionByZero,
    /// Invalid dimension
    InvalidDimension,
    /// Overflow error
    Overflow,
    /// Normalization error
    NormalizationError,
}

/// Error type for coherence operations
#[derive(Debug)]
pub enum CoherenceError {
    /// Invalid coherence value
    InvalidValue,
    /// Phase alignment failure
    PhaseAlignmentFailure,
    /// Boundary violation
    BoundaryViolation,
    /// Resonance failure
    ResonanceFailure,
}

impl fmt::Display for QuantumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState => write!(f, "Invalid quantum state"),
            Self::BoundaryViolation => write!(f, "Crystal boundary violation"),
            Self::CoherenceLoss => write!(f, "Loss of quantum coherence"),
            Self::PhaseMisalignment => write!(f, "Phase misalignment detected"),
            Self::ResonanceFailure => write!(f, "Resonance failure"),
            Self::AlignmentFailure(state) => write!(f, "Alignment failure: {:?}", state),
            Self::VectorError(e) => write!(f, "Vector error: {:?}", e),
        }
    }
}

impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::InvalidDimension => write!(f, "Invalid vector dimension"),
            Self::Overflow => write!(f, "Vector operation overflow"),
            Self::NormalizationError => write!(f, "Vector normalization error"),
        }
    }
}

impl fmt::Display for CoherenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue => write!(f, "Invalid coherence value"),
            Self::PhaseAlignmentFailure => write!(f, "Phase alignment failure"),
            Self::BoundaryViolation => write!(f, "Boundary violation"),
            Self::ResonanceFailure => write!(f, "Resonance failure"),
        }
    }
}

/// Result type alias for quantum operations
pub type QuantumResult<T> = Result<T, QuantumError>;

/// Result type alias for coherence operations
pub type CoherenceResult<T> = Result<T, CoherenceError>;

impl From<VectorError> for QuantumError {
    fn from(error: VectorError) -> Self {
        Self::VectorError(error)
    }
}

impl From<CoherenceError> for QuantumError {
    fn from(error: CoherenceError) -> Self {
        match error {
            CoherenceError::InvalidValue => Self::InvalidState,
            CoherenceError::PhaseAlignmentFailure => Self::PhaseMisalignment,
            CoherenceError::BoundaryViolation => Self::BoundaryViolation,
            CoherenceError::ResonanceFailure => Self::ResonanceFailure,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_error_display() {
        let error = QuantumError::InvalidState;
        assert_eq!(error.to_string(), "Invalid quantum state");
    }

    #[test]
    fn test_vector_error_display() {
        let error = VectorError::DivisionByZero;
        assert_eq!(error.to_string(), "Division by zero");
    }

    #[test]
    fn test_coherence_error_display() {
        let error = CoherenceError::InvalidValue;
        assert_eq!(error.to_string(), "Invalid coherence value");
    }

    #[test]
    fn test_error_conversion() {
        let vec_error = VectorError::DivisionByZero;
        let quantum_error: QuantumError = vec_error.into();
        assert!(matches!(quantum_error, QuantumError::VectorError(_)));
    }

    #[test]
    fn test_coherence_error_conversion() {
        let coherence_error = CoherenceError::InvalidValue;
        let quantum_error: QuantumError = coherence_error.into();
        assert!(matches!(quantum_error, QuantumError::InvalidState));
    }
}
