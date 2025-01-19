//! Core Error Types for Crystal Computing Systems
//! ===========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 14:28:05 UTC
//! Version: 0.1.0
//! License: MIT

use scribe::Scribe;

/// Core error type for crystal computing operations
#[derive(Debug, Clone)]
pub enum CrystalError {
    /// Mathematical operation errors
    Math(MathError),
    /// Quantum state errors
    Quantum(QuantumError),
    /// Vector operation errors
    Vector(VectorError),
    /// Coherence errors
    Coherence(CoherenceError),
}

/// Error type for mathematical operations
#[derive(Debug, Clone)]
pub enum MathError {
    /// Division by zero error
    DivisionByZero,
    /// Value overflow error
    Overflow(String),
    /// Value underflow error
    Underflow(String),
    /// Invalid domain for operation
    InvalidDomain(String),
    /// Quantum state became unstable
    QuantumStateUnstable,
    /// Conversion error between types
    ConversionError(String),
    /// Invalid parameter value
    InvalidParameter(String),
    /// Logarithm domain error
    LogarithmDomainError(f64),
    /// Julia set stability loss
    JuliaStabilityLoss(String),
    /// Mandelbrot set stability loss
    MandelbrotStabilityLoss(String),
    /// Generic fractal stability loss
    FractalStabilityLoss(String),
    /// Fractal type mismatch
    FractalTypeMismatch,
    /// Complex number convergence failure
    ComplexConvergenceFailure(String),
    /// Resonance loss in quantum state
    ResonanceLoss(String),
    /// Iteration limit exceeded
    IterationLimitExceeded(usize),
    /// Generic quantum error
    QuantumError(String),
}

/// Error type for quantum operations
#[derive(Debug, Clone)]
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
    AlignmentFailure(String),
    /// Vector operation error
    VectorError(VectorError),
}

/// Error type for vector operations
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

impl Scribe for CrystalError {
    fn scribe(&self) -> String {
        match self {
            Self::Math(e) => e.scribe(),
            Self::Quantum(e) => e.scribe(),
            Self::Vector(e) => e.scribe(),
            Self::Coherence(e) => e.scribe(),
        }
    }
}

impl Scribe for MathError {
    fn scribe(&self) -> String {
        match self {
            Self::DivisionByZero => "Division by zero".to_string(),
            Self::Overflow(msg) => format!("Overflow error: {}", msg),
            Self::Underflow(msg) => format!("Underflow error: {}", msg),
            Self::InvalidDomain(msg) => format!("Invalid domain: {}", msg),
            Self::QuantumStateUnstable => "Quantum state became unstable".to_string(),
            Self::ConversionError(msg) => format!("Conversion error: {}", msg),
            Self::InvalidParameter(msg) => format!("Invalid parameter: {}", msg),
            Self::LogarithmDomainError(val) => format!("Logarithm domain error: {}", val),
            Self::JuliaStabilityLoss(msg) => format!("Julia set stability loss: {}", msg),
            Self::MandelbrotStabilityLoss(msg) => format!("Mandelbrot set stability loss: {}", msg),
            Self::FractalStabilityLoss(msg) => format!("Fractal stability loss: {}", msg),
            Self::FractalTypeMismatch => "Fractal type mismatch".to_string(),
            Self::ComplexConvergenceFailure(msg) => format!("Complex convergence failure: {}", msg),
            Self::ResonanceLoss(msg) => format!("Resonance loss: {}", msg),
            Self::IterationLimitExceeded(limit) => format!("Iteration limit exceeded: {}", limit),
            Self::QuantumError(msg) => format!("Quantum error: {}", msg),
        }
    }
}

impl Scribe for QuantumError {
    fn scribe(&self) -> String {
        match self {
            Self::InvalidState => "Invalid quantum state".to_string(),
            Self::BoundaryViolation => "Crystal boundary violation".to_string(),
            Self::CoherenceLoss => "Loss of quantum coherence".to_string(),
            Self::PhaseMisalignment => "Phase misalignment detected".to_string(),
            Self::ResonanceFailure => "Resonance failure".to_string(),
            Self::AlignmentFailure(msg) => format!("Alignment failure: {}", msg),
            Self::VectorError(e) => e.scribe(),
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

/// Result type aliases
pub type CrystalResult<T> = Result<T, CrystalError>;
pub type MathResult<T> = Result<T, MathError>;
pub type QuantumResult<T> = Result<T, QuantumError>;
pub type VectorResult<T> = Result<T, VectorError>;
pub type CoherenceResult<T> = Result<T, CoherenceError>;

// Error conversions
impl From<MathError> for CrystalError {
    fn from(error: MathError) -> Self {
        Self::Math(error)
    }
}

impl From<QuantumError> for CrystalError {
    fn from(error: QuantumError) -> Self {
        Self::Quantum(error)
    }
}

impl From<VectorError> for CrystalError {
    fn from(error: VectorError) -> Self {
        Self::Vector(error)
    }
}

impl From<CoherenceError> for CrystalError {
    fn from(error: CoherenceError) -> Self {
        Self::Coherence(error)
    }
}

impl From<VectorError> for QuantumError {
    fn from(error: VectorError) -> Self {
        Self::VectorError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_error_scribe() {
        let error = CrystalError::Math(MathError::DivisionByZero);
        assert_eq!(error.scribe(), "Division by zero");
    }

    #[test]
    fn test_math_error_scribe() {
        let error = MathError::InvalidDomain("test".to_string());
        assert_eq!(error.scribe(), "Invalid domain: test");
    }

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
        let math_error = MathError::DivisionByZero;
        let crystal_error: CrystalError = math_error.into();
        assert!(matches!(crystal_error, CrystalError::Math(_)));
    }
}
