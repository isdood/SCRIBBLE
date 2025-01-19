//! Harmony Core - Crystal Computing Core Operations
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:14:08 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]
#![feature(const_trait_impl)]

extern crate alloc;

use alloc::vec::Vec;
use core::ops::{Add, Sub, Mul, Div, Neg};

pub mod vector;
pub mod idk;
pub mod crystal;
pub mod errors;
pub mod zeronaut;
pub mod phantom;
pub mod scribe;
pub mod align;
pub mod aether;

// Re-exports
pub use crystal::{CrystalLattice, CrystalNode, CrystalCube};
pub use errors::{QuantumError, CoherenceError, QuantumResult, CoherenceResult};
pub use vector::Vector3D;
pub use zeronaut::Zeronaut;
pub use phantom::Phantom;
pub use scribe::Scribe;

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
}

/// Trait for quantum operations
pub trait Quantum {
    /// Check if quantum state is stable
    fn is_stable(&self) -> bool;

    /// Get coherence level
    fn coherence(&self) -> f64;

    /// Attempt to recohere quantum state
    fn recohere(&mut self) -> QuantumResult<()>;

    /// Force decoherence of quantum state
    fn decohere(&mut self);
}

/// Crystal computing context
#[derive(Debug)]
pub struct CrystalContext {
    /// Crystal lattice size
    size: usize,
    /// Phase coherence level
    coherence: f64,
    /// Crystal resonance
    resonance: f64,
}

impl CrystalContext {
    /// Create new crystal computing context
    pub fn new(size: usize) -> Self {
        Self {
            size,
            coherence: constants::MAX_PHASE_COHERENCE,
            resonance: 1.0,
        }
    }

    /// Get current coherence level
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Get current resonance level
    pub fn resonance(&self) -> f64 {
        self.resonance
    }

    /// Check if context is stable
    pub fn is_stable(&self) -> bool {
        self.coherence >= constants::QUANTUM_STABILITY_THRESHOLD &&
        self.resonance >= constants::CRYSTAL_RESONANCE_THRESHOLD
    }
}

impl Default for CrystalContext {
    fn default() -> Self {
        Self::new(constants::MAX_QUANTUM_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_context() {
        let context = CrystalContext::default();
        assert!(context.is_stable());
        assert_eq!(context.coherence(), constants::MAX_PHASE_COHERENCE);
    }

    #[test]
    fn test_quantum_constants() {
        assert!(constants::QUANTUM_STABILITY_THRESHOLD > 0.0);
        assert!(constants::CRYSTAL_RESONANCE_THRESHOLD > 0.0);
        assert_eq!(constants::MAX_PHASE_COHERENCE, 1.0);
    }
}
