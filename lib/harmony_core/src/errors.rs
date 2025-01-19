//! Error Types for Crystal Computing Operations
//! ===================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:26:31 UTC
//! Version: 0.1.0
//! License: MIT

use core::fmt;
use core::error::Error;

/// Error types for quantum operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantumError {
    /// Crystal lattice boundary violation
    CrystalBoundaryViolation,
    /// Loss of quantum coherence
    CoherenceLoss,
    /// Phase misalignment in crystal structure
    PhaseMisalignment,
    /// Loss of crystal resonance
    ResonanceLoss,
    /// Insufficient quantum depth
    InsufficientDepth,
    /// Phase decoherence in crystal lattice
    PhaseDecoherence,
    /// No crystal lattice available
    NoCrystalLattice,
    /// Invalid quantum state
    InvalidState,
    /// Crystal structure failure
    CrystalFailure,
}

impl fmt::Display for QuantumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CrystalBoundaryViolation => write!(f, "Crystal lattice boundary violation"),
            Self::CoherenceLoss => write!(f, "Loss of quantum coherence"),
            Self::PhaseMisalignment => write!(f, "Phase misalignment in crystal structure"),
            Self::ResonanceLoss => write!(f, "Loss of crystal resonance"),
            Self::InsufficientDepth => write!(f, "Insufficient quantum depth"),
            Self::PhaseDecoherence => write!(f, "Phase decoherence in crystal lattice"),
            Self::NoCrystalLattice => write!(f, "No crystal lattice available"),
            Self::InvalidState => write!(f, "Invalid quantum state"),
            Self::CrystalFailure => write!(f, "Crystal structure failure"),
        }
    }
}

/// Error types for coherence operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoherenceError {
    /// Crystal decoherence
    CrystalDecoherence,
    /// Quantum instability
    QuantumInstability,
    /// Boundary violation
    BoundaryViolation,
    /// Phase alignment failure
    PhaseAlignmentFailure,
    /// Crystal structure failure
    StructureFailure,
}

impl fmt::Display for CoherenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CrystalDecoherence => write!(f, "Crystal decoherence detected"),
            Self::QuantumInstability => write!(f, "Quantum state instability"),
            Self::BoundaryViolation => write!(f, "Crystal boundary violation"),
            Self::PhaseAlignmentFailure => write!(f, "Phase alignment failure"),
            Self::StructureFailure => write!(f, "Crystal structure failure"),
        }
    }
}

impl Error for QuantumError {}
impl Error for CoherenceError {}

/// Result type for quantum operations
pub type QuantumResult<T> = Result<T, QuantumError>;

/// Result type for coherence operations
pub type CoherenceResult<T> = Result<T, CoherenceError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_error_display() {
        assert_eq!(
            QuantumError::CrystalBoundaryViolation.to_string(),
                   "Crystal lattice boundary violation"
        );
        assert_eq!(
            QuantumError::CoherenceLoss.to_string(),
                   "Loss of quantum coherence"
        );
    }

    #[test]
    fn test_coherence_error_display() {
        assert_eq!(
            CoherenceError::CrystalDecoherence.to_string(),
                   "Crystal decoherence detected"
        );
        assert_eq!(
            CoherenceError::QuantumInstability.to_string(),
                   "Quantum state instability"
        );
    }
}
