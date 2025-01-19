//! Error Handling for Crystal Lattice HPC Systems
//! =====================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:23:39 UTC
//! Version: 0.1.0
//! License: MIT

use std::fmt;
use std::error::Error;

/// Custom error type for mathematical operations in crystal lattice systems
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    // General Quantum State Errors
    CoherenceLoss,
    QuantumStateUnstable,
    PhaseError,
    EnergyDepletion,
    StabilityLoss,
    ResonanceFailure,

    // Bounds and Domain Errors
    LatticeBoundsError,
    ComplexDomain,
    DomainError,
    LatticeOverflow,
    LatticeUnderflow,
    QuantumSingularity,

    // Operation-Specific Errors
    AdditionOverflow,
    SubtractionUnderflow,
    MultiplicationOverflow,
    DivisionByZero,
    RootConvergenceFailure,
    LogarithmDomainError,

    // Sequence and Series Errors
    SequenceLoss,
    SeriesConvergenceFailure,
    RecursionLimitExceeded,
    IterationLimitExceeded,

    // Geometric and Spatial Errors
    CircularityLoss,
    OrthogonalityLoss,
    SymmetryLoss,
    HarmonyLoss,
    ContinuityLoss,

    // State Preservation Errors
    EntanglementLoss,
    CoherenceDecoherence,
    PhaseDecoherence,
    EnergyDissipation,
    ResonanceDecay,

    // Conversion and Type Errors
    TypeConversionError,
    PrecisionLoss,
    ScaleError,
    DimensionalityError,
    UnitMismatch,

    // Custom error with message
    Custom(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // General Quantum State Errors
            MathError::CoherenceLoss =>
            write!(f, "Quantum coherence lost during operation"),
            MathError::QuantumStateUnstable =>
            write!(f, "Quantum state stability threshold exceeded"),
            MathError::PhaseError =>
            write!(f, "Invalid phase value or phase alignment lost"),
            MathError::EnergyDepletion =>
            write!(f, "System energy level below operational threshold"),
            MathError::StabilityLoss =>
            write!(f, "System stability compromised during operation"),
            MathError::ResonanceFailure =>
            write!(f, "Quantum resonance synchronization failed"),

            // Bounds and Domain Errors
            MathError::LatticeBoundsError =>
            write!(f, "Result exceeds crystal lattice bounds"),
            MathError::ComplexDomain =>
            write!(f, "Operation resulted in complex domain value"),
            MathError::DomainError =>
            write!(f, "Input value outside valid domain"),
            MathError::LatticeOverflow =>
            write!(f, "Crystal lattice maximum capacity exceeded"),
            MathError::LatticeUnderflow =>
            write!(f, "Crystal lattice minimum threshold breached"),
            MathError::QuantumSingularity =>
            write!(f, "Quantum singularity detected in calculation"),

            // Operation-Specific Errors
            MathError::AdditionOverflow =>
            write!(f, "Addition operation exceeded maximum value"),
            MathError::SubtractionUnderflow =>
            write!(f, "Subtraction operation resulted in negative overflow"),
            MathError::MultiplicationOverflow =>
            write!(f, "Multiplication operation exceeded maximum value"),
            MathError::DivisionByZero =>
            write!(f, "Division by zero attempted"),
            MathError::RootConvergenceFailure =>
            write!(f, "Root calculation failed to converge"),
            MathError::LogarithmDomainError =>
            write!(f, "Logarithm input must be positive"),

            // Sequence and Series Errors
            MathError::SequenceLoss =>
            write!(f, "Sequence coherence lost during calculation"),
            MathError::SeriesConvergenceFailure =>
            write!(f, "Series failed to converge within limit"),
            MathError::RecursionLimitExceeded =>
            write!(f, "Maximum recursion depth exceeded"),
            MathError::IterationLimitExceeded =>
            write!(f, "Maximum iteration count exceeded"),

            // Geometric and Spatial Errors
            MathError::CircularityLoss =>
            write!(f, "Circular symmetry lost during operation"),
            MathError::OrthogonalityLoss =>
            write!(f, "Orthogonal alignment lost during calculation"),
            MathError::SymmetryLoss =>
            write!(f, "Geometric symmetry violation detected"),
            MathError::HarmonyLoss =>
            write!(f, "Harmonic resonance lost during operation"),
            MathError::ContinuityLoss =>
            write!(f, "Function continuity broken during calculation"),

            // State Preservation Errors
            MathError::EntanglementLoss =>
            write!(f, "Quantum entanglement state corrupted"),
            MathError::CoherenceDecoherence =>
            write!(f, "Quantum coherence decoherence detected"),
            MathError::PhaseDecoherence =>
            write!(f, "Phase alignment decoherence detected"),
            MathError::EnergyDissipation =>
            write!(f, "Energy state dissipation exceeded threshold"),
            MathError::ResonanceDecay =>
            write!(f, "Quantum resonance decay detected"),

            // Conversion and Type Errors
            MathError::TypeConversionError =>
            write!(f, "Type conversion failed during operation"),
            MathError::PrecisionLoss =>
            write!(f, "Numerical precision lost during conversion"),
            MathError::ScaleError =>
            write!(f, "Invalid scale factor in operation"),
            MathError::DimensionalityError =>
            write!(f, "Incompatible dimensionality in operation"),
            MathError::UnitMismatch =>
            write!(f, "Incompatible units in operation"),

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
        MathError::TypeConversionError
    }
}

impl From<std::num::ParseIntError> for MathError {
    fn from(_: std::num::ParseIntError) -> Self {
        MathError::TypeConversionError
    }
}

impl From<std::num::ParseFloatError> for MathError {
    fn from(_: std::num::ParseFloatError) -> Self {
        MathError::TypeConversionError
    }
}

/// Helper functions for error handling
pub trait ErrorHandler {
    fn check_bounds<T: PartialOrd>(value: T, min: T, max: T) -> MathResult<()>;
    fn check_domain<T: PartialOrd + Default>(value: T) -> MathResult<()>;
    fn check_stability(coherence: f64, threshold: f64) -> MathResult<()>;
}

impl ErrorHandler for MathError {
    fn check_bounds<T: PartialOrd>(value: T, min: T, max: T) -> MathResult<()> {
        if value < min || value > max {
            Err(MathError::LatticeBoundsError)
        } else {
            Ok(())
        }
    }

    fn check_domain<T: PartialOrd + Default>(value: T) -> MathResult<()> {
        if value < T::default() {
            Err(MathError::DomainError)
        } else {
            Ok(())
        }
    }

    fn check_stability(coherence: f64, threshold: f64) -> MathResult<()> {
        if coherence < threshold {
            Err(MathError::QuantumStateUnstable)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", MathError::CoherenceLoss),
                "Quantum coherence lost during operation"
        );
    }

    #[test]
    fn test_error_bounds_check() {
        assert!(MathError::check_bounds(5, 0, 10).is_ok());
        assert!(MathError::check_bounds(11, 0, 10).is_err());
    }

    #[test]
    fn test_error_domain_check() {
        assert!(MathError::check_domain(5.0).is_ok());
        assert!(MathError::check_domain(-1.0).is_err());
    }

    #[test]
    fn test_error_stability_check() {
        assert!(MathError::check_stability(1.0, 0.5).is_ok());
        assert!(MathError::check_stability(0.4, 0.5).is_err());
    }

    #[test]
    fn test_custom_error() {
        let error = MathError::Custom("Test error".to_string());
        assert_eq!(format!("{}", error), "Test error");
    }
}
