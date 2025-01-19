//! Harmony - Core Crystal Computing Traits and Operations
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 09:38:17 UTC
//! Last Updated: 2025-01-19 09:41:20 UTC
//! Version: 0.1.0
//! License: MIT

use crate::errors::QuantumError;
use crate::crystal::CrystalNode;
use crate::constants::{QUANTUM_STABILITY_THRESHOLD, MAX_PHASE_COHERENCE, MIN_PHASE_COHERENCE};

/// Quantum operations trait for crystal computing
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_stability() {
        struct TestQuantum {
            coherence_value: f64
        }

        impl Quantum for TestQuantum {
            fn coherence(&self) -> f64 { self.coherence_value }
            fn recohere(&mut self) -> Result<(), QuantumError> { Ok(()) }
            fn decohere(&mut self) { self.coherence_value = 0.0; }
            fn phase_alignment(&self) -> f64 { 1.0 }
            fn align_with(&mut self, _: &CrystalNode) -> Result<(), QuantumError> { Ok(()) }
        }

        let stable = TestQuantum { coherence_value: 0.9 };
        let unstable = TestQuantum { coherence_value: 0.5 };

        assert!(stable.is_stable());
        assert!(!unstable.is_stable());
    }
}
