//! Error Handling for Crystal Lattice HPC Systems
//! ====================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:38:21 UTC
//! Version: 0.1.0
//! License: MIT

use std::fmt;
use std::error::Error;

/// Custom error type for mathematical operations in crystal lattice systems
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    // General Quantum State Errors
    CoherenceLoss(String),
    QuantumStateUnstable,
    PhaseError(f64),
    EnergyDepletion(f64),
    StabilityLoss(f64),
    ResonanceFailure(String),

    // Bounds and Domain Errors
    LatticeBoundsError(String),
    ComplexDomain(String),
    DomainError(String),
    LatticeOverflow(String),
    LatticeUnderflow(String),
    QuantumSingularity(String),

    // Operation-Specific Errors
    AdditionOverflow(String),
    SubtractionUnderflow(String),
    MultiplicationOverflow(String),
    DivisionByZero,
    RootConvergenceFailure(usize),
    LogarithmDomainError(f64),

    // Sequence and Series Errors
    SequenceLoss(usize),
    SeriesConvergenceFailure(usize),
    RecursionLimitExceeded(usize),
    IterationLimitExceeded(usize),

    // Geometric and Spatial Errors
    CircularityLoss(f64),
    OrthogonalityLoss(f64),
    SymmetryLoss(String),
    HarmonyLoss(f64),
    ContinuityLoss(String),

    // State Preservation Errors
    EntanglementLoss(String),
    CoherenceDecoherence(f64),
    PhaseDecoherence(f64),
    EnergyDissipation(f64),
    ResonanceDecay(f64),

    // Conversion and Type Errors
    TypeConversionError(String),
    PrecisionLoss(f64),
    ScaleError(String),
    DimensionalityError(String),
    UnitMismatch(String),

    // Fractal-Specific Errors
    JuliaStabilityLoss(String),
    MandelbrotStabilityLoss(String),
    FractalConvergenceFailure(String),
    FractalTypeMismatch,
    OrbitTrackingError(String),
    EscapeTimeError(String),
    BulbDetectionError(String),
    CardioidDetectionError(String),

    // Complex Number Errors
    ComplexOverflow(String),
    ComplexUnderflow(String),
    ComplexDivisionByZero,
    ComplexRootError(String),
    ComplexLogError(String),

    // Quantum Operation Errors
    QuantumOperationFailure(String),
    QuantumResonanceFailure(String),
    QuantumPhaseError(String),
    QuantumEnergyError(String),
    QuantumStabilityError(String),

    // Custom error with message
    Custom(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // General Quantum State Errors
            MathError::CoherenceLoss(msg) =>
            write!(f, "Quantum coherence lost: {}", msg),
            MathError::QuantumStateUnstable =>
            write!(f, "Quantum state stability threshold exceeded"),
            MathError::PhaseError(phase) =>
            write!(f, "Invalid phase value or phase alignment lost: {}", phase),
            MathError::EnergyDepletion(energy) =>
            write!(f, "System energy level below threshold: {}", energy),
            MathError::StabilityLoss(stability) =>
            write!(f, "System stability compromised: {}", stability),
            MathError::ResonanceFailure(msg) =>
            write!(f, "Quantum resonance synchronization failed: {}", msg),

            // Bounds and Domain Errors
            MathError::LatticeBoundsError(msg) =>
            write!(f, "Result exceeds crystal lattice bounds: {}", msg),
            MathError::ComplexDomain(msg) =>
            write!(f, "Operation resulted in complex domain: {}", msg),
            MathError::DomainError(msg) =>
            write!(f, "Input value outside valid domain: {}", msg),
            MathError::LatticeOverflow(msg) =>
            write!(f, "Crystal lattice maximum capacity exceeded: {}", msg),
            MathError::LatticeUnderflow(msg) =>
            write!(f, "Crystal lattice minimum threshold breached: {}", msg),
            MathError::QuantumSingularity(msg) =>
            write!(f, "Quantum singularity detected: {}", msg),

            // Operation-Specific Errors
            MathError::AdditionOverflow(msg) =>
            write!(f, "Addition operation exceeded maximum value: {}", msg),
            MathError::SubtractionUnderflow(msg) =>
            write!(f, "Subtraction operation resulted in negative overflow: {}", msg),
            MathError::MultiplicationOverflow(msg) =>
            write!(f, "Multiplication operation exceeded maximum value: {}", msg),
            MathError::DivisionByZero =>
            write!(f, "Division by zero attempted"),
            MathError::RootConvergenceFailure(iter) =>
            write!(f, "Root calculation failed to converge after {} iterations", iter),
            MathError::LogarithmDomainError(value) =>
            write!(f, "Logarithm input must be positive, got: {}", value),

            // Sequence and Series Errors
            MathError::SequenceLoss(iter) =>
            write!(f, "Sequence coherence lost at iteration {}", iter),
            MathError::SeriesConvergenceFailure(iter) =>
            write!(f, "Series failed to converge after {} iterations", iter),
            MathError::RecursionLimitExceeded(depth) =>
            write!(f, "Maximum recursion depth {} exceeded", depth),
            MathError::IterationLimitExceeded(limit) =>
            write!(f, "Maximum iteration count {} exceeded", limit),

            // Geometric and Spatial Errors
            MathError::CircularityLoss(value) =>
            write!(f, "Circular symmetry lost: deviation {}", value),
            MathError::OrthogonalityLoss(angle) =>
            write!(f, "Orthogonal alignment lost: angle {}", angle),
            MathError::SymmetryLoss(msg) =>
            write!(f, "Geometric symmetry violation: {}", msg),
            MathError::HarmonyLoss(value) =>
            write!(f, "Harmonic resonance lost: value {}", value),
            MathError::ContinuityLoss(msg) =>
            write!(f, "Function continuity broken: {}", msg),

            // State Preservation Errors
            MathError::EntanglementLoss(msg) =>
            write!(f, "Quantum entanglement state corrupted: {}", msg),
            MathError::CoherenceDecoherence(value) =>
            write!(f, "Quantum coherence decoherence: value {}", value),
            MathError::PhaseDecoherence(value) =>
            write!(f, "Phase alignment decoherence: value {}", value),
            MathError::EnergyDissipation(value) =>
            write!(f, "Energy state dissipation: value {}", value),
            MathError::ResonanceDecay(value) =>
            write!(f, "Quantum resonance decay: value {}", value),

            // Conversion and Type Errors
            MathError::TypeConversionError(msg) =>
            write!(f, "Type conversion failed: {}", msg),
            MathError::PrecisionLoss(value) =>
            write!(f, "Numerical precision lost: value {}", value),
            MathError::ScaleError(msg) =>
            write!(f, "Invalid scale factor: {}", msg),
            MathError::DimensionalityError(msg) =>
            write!(f, "Incompatible dimensionality: {}", msg),
            MathError::UnitMismatch(msg) =>
            write!(f, "Incompatible units: {}", msg),

            // Fractal-Specific Errors
            MathError::JuliaStabilityLoss(msg) =>
            write!(f, "Julia set stability lost: {}", msg),
            MathError::MandelbrotStabilityLoss(msg) =>
            write!(f, "Mandelbrot set stability lost: {}", msg),
            MathError::FractalConvergenceFailure(msg) =>
            write!(f, "Fractal iteration failed to converge: {}", msg),
            MathError::FractalTypeMismatch =>
            write!(f, "Fractal type mismatch between state and parameters"),
            MathError::OrbitTrackingError(msg) =>
            write!(f, "Orbit tracking failed: {}", msg),
            MathError::EscapeTimeError(msg) =>
            write!(f, "Escape time calculation error: {}", msg),
            MathError::BulbDetectionError(msg) =>
            write!(f, "Bulb detection failed: {}", msg),
            MathError::CardioidDetectionError(msg) =>
            write!(f, "Cardioid detection failed: {}", msg),

            // Complex Number Errors
            MathError::ComplexOverflow(msg) =>
            write!(f, "Complex number overflow: {}", msg),
            MathError::ComplexUnderflow(msg) =>
            write!(f, "Complex number underflow: {}", msg),
            MathError::ComplexDivisionByZero =>
            write!(f, "Complex division by zero attempted"),
            MathError::ComplexRootError(msg) =>
            write!(f, "Complex root calculation error: {}", msg),
            MathError::ComplexLogError(msg) =>
            write!(f, "Complex logarithm error: {}", msg),

            // Quantum Operation Errors
            MathError::QuantumOperationFailure(msg) =>
            write!(f, "Quantum operation failed: {}", msg),
            MathError::QuantumResonanceFailure(msg) =>
            write!(f, "Quantum resonance calculation failed: {}", msg),
            MathError::QuantumPhaseError(msg) =>
            write!(f, "Quantum phase error: {}", msg),
            MathError::QuantumEnergyError(msg) =>
            write!(f, "Quantum energy error: {}", msg),
            MathError::QuantumStabilityError(msg) =>
            write!(f, "Quantum stability error: {}", msg),

            // Custom error
            MathError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for MathError {}

/// Result type for crystal lattice operations
pub type MathResult<T> = Result<T, MathError>;

/// Error conversion traits
impl From<std::num::TryFromIntError> for MathError {
    fn from(_: std::num::TryFromIntError) -> Self {
        MathError::TypeConversionError("Integer conversion failed".to_string())
    }
}

impl From<std::num::ParseIntError> for MathError {
    fn from(_: std::num::ParseIntError) -> Self {
        MathError::TypeConversionError("Integer parsing failed".to_string())
    }
}

impl From<std::num::ParseFloatError> for MathError {
    fn from(_: std::num::ParseFloatError) -> Self {
        MathError::TypeConversionError("Float parsing failed".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", MathError::DivisionByZero),
                "Division by zero attempted"
        );
    }

    #[test]
    fn test_error_conversion() {
        let int_error: Result<i32, std::num::TryFromIntError> =
        i32::try_from(u64::MAX);
        let math_error: Result<i32, MathError> =
        int_error.map_err(MathError::from);
        assert!(matches!(math_error, Err(MathError::TypeConversionError(_))));
    }

    #[test]
    fn test_fractal_errors() {
        let error = MathError::FractalTypeMismatch;
        assert_eq!(
            format!("{}", error),
                "Fractal type mismatch between state and parameters"
        );
    }

    #[test]
    fn test_complex_errors() {
        let error = MathError::ComplexDivisionByZero;
        assert_eq!(
            format!("{}", error),
                "Complex division by zero attempted"
        );
    }
}
