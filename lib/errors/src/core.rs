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
use scribe::native_string::String; // Import the correct String type

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
    /// Harmony state became unstable
    HarmonyStateUnstable,
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
    /// Resonance loss in harmony state
    ResonanceLoss(String),
    /// Iteration limit exceeded
    IterationLimitExceeded(usize),
    /// Generic harmony error
    HarmonyError(String),
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
            Self::DivisionByZero => {
                let mut result = String::new();
                result.push_str("Division by zero");
                result
            },
            Self::Overflow(msg) => {
                let mut result = String::new();
                result.push_str("Overflow error: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::Underflow(msg) => {
                let mut result = String::new();
                result.push_str("Underflow error: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::InvalidDomain(msg) => {
                let mut result = String::new();
                result.push_str("Invalid domain: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::HarmonyStateUnstable => {
                let mut result = String::new();
                result.push_str("Harmony state became unstable");
                result
            },
            Self::ConversionError(msg) => {
                let mut result = String::new();
                result.push_str("Conversion error: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::InvalidParameter(msg) => {
                let mut result = String::new();
                result.push_str("Invalid parameter: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::LogarithmDomainError(val) => {
                let mut result = String::new();
                result.push_str("Logarithm domain error: ");
                result.push_str(&val.to_string());
                result
            },
            Self::JuliaStabilityLoss(msg) => {
                let mut result = String::new();
                result.push_str("Julia set stability loss: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::MandelbrotStabilityLoss(msg) => {
                let mut result = String::new();
                result.push_str("Mandelbrot set stability loss: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::FractalStabilityLoss(msg) => {
                let mut result = String::new();
                result.push_str("Fractal stability loss: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::FractalTypeMismatch => {
                let mut result = String::new();
                result.push_str("Fractal type mismatch");
                result
            },
            Self::ComplexConvergenceFailure(msg) => {
                let mut result = String::new();
                result.push_str("Complex convergence failure: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::ResonanceLoss(msg) => {
                let mut result = String::new();
                result.push_str("Resonance loss: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::IterationLimitExceeded(limit) => {
                let mut result = String::new();
                result.push_str("Iteration limit exceeded: ");
                result.push_str(&limit.to_string());
                result
            },
            Self::HarmonyError(msg) => {
                let mut result = String::new();
                result.push_str("Harmony error: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
        }
    }
}

impl Scribe for QuantumError {
    fn scribe(&self) -> String {
        match self {
            Self::InvalidState => {
                let mut result = String::new();
                result.push_str("Invalid quantum state");
                result
            },
            Self::BoundaryViolation => {
                let mut result = String::new();
                result.push_str("Crystal boundary violation");
                result
            },
            Self::CoherenceLoss => {
                let mut result = String::new();
                result.push_str("Loss of quantum coherence");
                result
            },
            Self::PhaseMisalignment => {
                let mut result = String::new();
                result.push_str("Phase misalignment detected");
                result
            },
            Self::ResonanceFailure => {
                let mut result = String::new();
                result.push_str("Resonance failure");
                result
            },
            Self::AlignmentFailure(msg) => {
                let mut result = String::new();
                result.push_str("Alignment failure: ");
                result.push_str(msg.to_str()); // Convert to &str
                result
            },
            Self::VectorError(e) => e.scribe(),
        }
    }
}

impl Scribe for VectorError {
    fn scribe(&self) -> String {
        match self {
            Self::DivisionByZero => {
                let mut result = String::new();
                result.push_str("Division by zero");
                result
            },
            Self::InvalidDimension => {
                let mut result = String::new();
                result.push_str("Invalid vector dimension");
                result
            },
            Self::Overflow => {
                let mut result = String::new();
                result.push_str("Vector operation overflow");
                result
            },
            Self::NormalizationError => {
                let mut result = String::new();
                result.push_str("Vector normalization error");
                result
            },
        }
    }
}

impl Scribe for CoherenceError {
    fn scribe(&self) -> String {
        match self {
            Self::InvalidValue => {
                let mut result = String::new();
                result.push_str("Invalid coherence value");
                result
            },
            Self::PhaseAlignmentFailure => {
                let mut result = String::new();
                result.push_str("Phase alignment failure");
                result
            },
            Self::BoundaryViolation => {
                let mut result = String::new();
                result.push_str("Boundary violation");
                result
            },
            Self::ResonanceFailure => {
                let mut result = String::new();
                result.push_str("Resonance failure");
                result
            },
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
        assert_eq!(error.scribe().to_str(), "Division by zero");
    }

    #[test]
    fn test_math_error_scribe() {
        let error = MathError::InvalidDomain("test".to_string());
        assert_eq!(error.scribe().to_str(), "Invalid domain: test");
    }

    #[test]
    fn test_quantum_error_scribe() {
        let error = QuantumError::InvalidState;
        assert_eq!(error.scribe().to_str(), "Invalid quantum state");
    }

    #[test]
    fn test_vector_error_scribe() {
        let error = VectorError::DivisionByZero;
        assert_eq!(error.scribe().to_str(), "Division by zero");
    }

    #[test]
    fn test_coherence_error_scribe() {
        let error = CoherenceError::InvalidValue;
        assert_eq!(error.scribe().to_str(), "Invalid coherence value");
    }

    #[test]
    fn test_error_conversion() {
        let math_error = MathError::DivisionByZero;
        let crystal_error: CrystalError = math_error.into();
        assert!(matches!(crystal_error, CrystalError::Math(_)));
    }
}
