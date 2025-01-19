//! Harmony Core - Crystal Computing Core Operations
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:48:03 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]
#![feature(const_trait_impl)]

// Module declarations
pub mod vector;
pub mod harmony;
pub mod idk;
pub mod crystal;
pub mod errors;
pub mod zeronaut;
pub mod phantom;
pub mod scribe;
pub mod align;
pub mod aether;
pub mod cube;

// Re-exports
pub use crystal::{CrystalLattice, CrystalNode};
pub use errors::{QuantumError, CoherenceError, QuantumResult, CoherenceResult};
pub use vector::{Vector3D, Vector4D};
pub use zeronaut::Zeronaut;
pub use phantom::Phantom;
pub use scribe::Scribe;
pub use harmony::{Quantum, Phase, Resonance};
pub use align::{Alignment, AlignmentState, AlignmentResult};
pub use aether::AetherField;

/// Constants for crystal computing operations
pub mod constants {
    /// Maximum size of a quantum dimension
    pub const MAX_QUANTUM_SIZE: usize = 256;

    /// Quantum stability threshold
    pub const QUANTUM_STABILITY_THRESHOLD: f64 = 0.8;

    /// Crystal resonance threshold
    pub const CRYSTAL_RESONANCE_THRESHOLD: f64 = 0.7;

    /// Golden ratio for quantum operations
    pub const QUANTUM_GOLDEN_RATIO: f64 = 1.618033988749895;

    /// Maximum phase coherence level
    pub const MAX_PHASE_COHERENCE: f64 = 1.0;

    /// Minimum phase coherence level
    pub const MIN_PHASE_COHERENCE: f64 = 0.1;

    /// Aether resonance factor
    pub const AETHER_RESONANCE_FACTOR: f64 = 0.9;

    /// Alignment threshold for quantum operations
    pub const ALIGNMENT_THRESHOLD: f64 = 0.95;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(constants::QUANTUM_STABILITY_THRESHOLD > 0.0);
        assert!(constants::CRYSTAL_RESONANCE_THRESHOLD > 0.0);
        assert_eq!(constants::MAX_PHASE_COHERENCE, 1.0);
        assert!(constants::MIN_PHASE_COHERENCE > 0.0);
        assert!(constants::MIN_PHASE_COHERENCE < constants::MAX_PHASE_COHERENCE);
        assert!(constants::AETHER_RESONANCE_FACTOR > 0.0);
        assert!(constants::ALIGNMENT_THRESHOLD > 0.9);
    }
}
