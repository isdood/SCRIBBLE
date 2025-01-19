//! Harmony - Core Crystal Computing Traits and Operations
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 09:38:17 UTC
//! Last Updated: 2025-01-19 20:59:16 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::traits::MathOps;
use crate::{
    errors::QuantumError,
    crystal::CrystalNode,
    align::{Alignment, AlignmentState},
    constants::{
        QUANTUM_STABILITY_THRESHOLD,
        MAX_PHASE_COHERENCE,
        MIN_PHASE_COHERENCE
    },
};

/// Core trait for quantum operations
pub trait Quantum {
    /// Check if quantum state is stable
    fn is_stable(&self) -> bool {
        self.coherence() >= QUANTUM_STABILITY_THRESHOLD
    }

    /// Get current coherence level
    fn coherence(&self) -> f64;

    /// Attempt to recohere quantum state
    fn recohere(&mut self) -> Result<(), QuantumError>;

    /// Force decoherence of quantum state
    fn decohere(&mut self);

    /// Get current phase alignment
    fn phase_alignment(&self) -> f64;

    /// Align phase with target node
    fn align_with(&mut self, target: &CrystalNode) -> Result<(), QuantumError>;

    /// Get current alignment state
    fn alignment_state(&self) -> AlignmentState;
}

/// Phase operations for crystal nodes
pub trait Phase {
    /// Get current phase value
    fn phase(&self) -> f64;

    /// Set phase value
    fn set_phase(&mut self, value: f64) -> Result<(), QuantumError>;

    /// Check if phase is coherent
    fn is_coherent(&self) -> bool {
        let phase = self.phase();
        phase >= MIN_PHASE_COHERENCE && phase <= MAX_PHASE_COHERENCE
    }
}

/// Resonance operations for crystal structures
pub trait Resonance {
    /// Get current resonance value
    fn resonance(&self) -> f64;

    /// Adjust resonance by factor
    fn adjust_resonance(&mut self, factor: f64) -> Result<(), QuantumError>;

    /// Check if resonance is stable
    fn is_resonant(&self) -> bool {
        self.resonance() >= QUANTUM_STABILITY_THRESHOLD
    }
}

/// Alignment operations for crystal structures
pub trait AlignmentOps {
    /// Get current alignment
    fn alignment(&self) -> &Alignment;

    /// Get mutable alignment
    fn alignment_mut(&mut self) -> &mut Alignment;

    /// Check if aligned
    fn is_aligned(&self) -> bool {
        matches!(self.alignment().state(),
                 AlignmentState::Perfect | AlignmentState::Partial(_))
    }
}

/// Arithmetic operations for quantum values
pub trait QuantumArithmetic: Sized + MathOps {
    /// Zero value
    fn zero() -> Self;

    /// Unit value
    fn one() -> Self;

    /// Check if value is zero
    fn is_zero(&self) -> bool;

    /// Check if value is valid
    fn is_valid(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestQuantum {
        coherence_value: f64,
        alignment: Alignment,
    }

    impl TestQuantum {
        fn new(coherence: f64) -> Self {
            use crate::vector::Vector3D;
            Self {
                coherence_value: coherence,
                alignment: Alignment::new(Vector3D::new(0.0, 0.0, 0.0)),
            }
        }
    }

    impl Quantum for TestQuantum {
        fn coherence(&self) -> f64 {
            self.coherence_value
        }

        fn recohere(&mut self) -> Result<(), QuantumError> {
            self.coherence_value = 1.0;
            Ok(())
        }

        fn decohere(&mut self) {
            self.coherence_value = 0.0;
        }

        fn phase_alignment(&self) -> f64 {
            1.0
        }

        fn align_with(&mut self, _: &CrystalNode) -> Result<(), QuantumError> {
            self.alignment.set_state(AlignmentState::Perfect);
            Ok(())
        }

        fn alignment_state(&self) -> AlignmentState {
            self.alignment.state()
        }
    }

    #[test]
    fn test_quantum_stability() {
        let stable = TestQuantum::new(0.9);
        let unstable = TestQuantum::new(0.5);

        assert!(stable.is_stable());
        assert!(!unstable.is_stable());
    }

    #[test]
    fn test_quantum_decoherence() {
        let mut quantum = TestQuantum::new(1.0);
        assert!(quantum.is_stable());
        quantum.decohere();
        assert!(!quantum.is_stable());
    }

    #[test]
    fn test_quantum_recoherence() {
        let mut quantum = TestQuantum::new(0.5);
        assert!(!quantum.is_stable());
        quantum.recohere().unwrap();
        assert!(quantum.is_stable());
    }

    #[test]
    fn test_phase_alignment() {
        let quantum = TestQuantum::new(1.0);
        assert_eq!(quantum.phase_alignment(), 1.0);
    }

    #[test]
    fn test_alignment_state() {
        let mut quantum = TestQuantum::new(0.9);
        let node = CrystalNode::new();
        quantum.align_with(&node).unwrap();
        assert_eq!(quantum.alignment_state(), AlignmentState::Perfect);
    }
}
