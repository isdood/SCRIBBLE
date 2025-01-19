//! Harmony Core - Crystal Computing Core Operations
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:30:34 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]
#![feature(const_trait_impl)]

extern crate align;

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

// Re-exports from our own modules
pub use crystal::{CrystalLattice, CrystalNode, CrystalCube};
pub use errors::{QuantumError, CoherenceError, QuantumResult, CoherenceResult};
pub use vector::{Vector3D, Vector4D};
pub use zeronaut::Zeronaut;
pub use phantom::Phantom;
pub use scribe::Scribe;
pub use harmony::{MeshValue, MeshOps, Quantum, Resonance, Phase};
pub use align::{AlignmentError, AlignmentResult};

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

    /// Crystal alignment threshold
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
        assert!(constants::AETHER_RESONANCE_FACTOR > 0.0);
        assert!(constants::ALIGNMENT_THRESHOLD > 0.9);
    }
}
