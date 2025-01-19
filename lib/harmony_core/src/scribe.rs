//! Error Types for Crystal Computing
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 13:58:19 UTC
//! Version: 0.1.1
//! License: MIT

use scribe::Scribe;
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

impl Scribe for QuantumError {
    fn scribe(&self) -> String {
        match self {
            Self::InvalidState => "Invalid quantum state".to_string(),
            Self::BoundaryViolation => "Crystal boundary violation".to_string(),
            Self::CoherenceLoss => "Loss of quantum coherence".to_string(),
            Self::PhaseMisalignment => "Phase misalignment detected".to_string(),
            Self::ResonanceFailure => "Resonance failure".to_string(),
            Self::AlignmentFailure(state) => format!("Alignment failure: {:?}", state),
            Self::VectorError(e) => format!("Vector error: {:?}", e),
        }
    }
}

impl Scribe for VectorError {
    fn scribe(&self) -> String {
        match self {
            Self::DivisionByZero => "Division by zero".to_string(),
            Self::InvalidDimension => "Invalid vector dimension".to_string(),
            Self::Overflow => "Vector operation overflow".to_string(),
            Self::NormalizationError => "Vector normalization error".to_string(),
        }
    }
}

impl Scribe for CoherenceError {
    fn scribe(&self) -> String {
        match self {
            Self::InvalidValue => "Invalid coherence value".to_string(),
            Self::PhaseAlignmentFailure => "Phase alignment failure".to_string(),
            Self::BoundaryViolation => "Boundary violation".to_string(),
            Self::ResonanceFailure => "Resonance failure".to_string(),
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
    fn test_quantum_error_scribe() {
        let error = QuantumError::InvalidState;
        assert_eq!(error.scribe(), "Invalid quantum state");
    }

    #[test]
    fn test_vector_error_scribe() {
        let error = VectorError::DivisionByZero;
        assert_eq!(error.scribe(), "Division by zero");
    }

    #[test]
    fn test_coherence_error_scribe() {
        let error = CoherenceError::InvalidValue;
        assert_eq!(error.scribe(), "Invalid coherence value");
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
