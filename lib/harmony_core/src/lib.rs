//! Harmony Core - Crystal Computing Core Operations
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 17:14:08 UTC
//! Version: 0.1.1
//! License: MIT

#![no_std]
#![feature(const_trait_impl)]

// External crates
extern crate magicmath;
extern crate errors;
extern crate scribe;

// Module declarations
pub mod align;
pub mod crystal;
pub mod cube;
pub mod growth;
pub mod harmony;
pub mod idk;
pub mod phantom;
pub mod zeronaut;
pub mod aether;

// Re-export common types from magicmath
pub use magicmath::{
    // Core traits
    traits::{
        MeshValue,
        CrystalAdd,
        CrystalSub,
        CrystalMul,
        CrystalDiv,
    },
    // Vectors
    vector3d::Vector3D,
    vector4d::Vector4D,
    // Math operations
    ops::{Field, Mesh, PhaseField, AetherField},
    // Resonance
    resonance::Resonance,
    // Math utilities
    utils::{floor, sqrt},
};

// Re-exports from errors
pub use errors::{
    MathError,
    QuantumError,
};

// Re-exports from core
pub use core::{
    fmt::{self, Write, Formatter, Result as FmtResult},
    result::Result,
};

// Re-exports from alloc
pub use alloc::{
    string::String,
    vec::Vec,
    boxed::Box,
};

// Constants module
pub mod constants {
    pub const MAX_QUANTUM_SIZE: usize = 256;
    pub const QUANTUM_STABILITY_THRESHOLD: f64 = 0.8;
    pub const CRYSTAL_RESONANCE_THRESHOLD: f64 = 0.7;
    pub const QUANTUM_GOLDEN_RATIO: f64 = 1.618033988749895;
    pub const MAX_PHASE_COHERENCE: f64 = 1.0;
    pub const MIN_PHASE_COHERENCE: f64 = 0.1;
    pub const AETHER_RESONANCE_FACTOR: f64 = 0.9;
    pub const ALIGNMENT_THRESHOLD: f64 = 0.95;
    pub const GROWTH_RATE_FACTOR: f64 = 0.5;
    pub const MAX_FRACTAL_DEPTH: usize = 64;
    pub const JULIA_GROWTH_REAL: f64 = -0.4;
    pub const JULIA_GROWTH_IMAG: f64 = 0.6;
}

/// Initialize fractal parameters for crystal growth
pub fn create_growth_params() -> Resonance {
    let mut params = Resonance::new();
    params.set_max_iterations(constants::MAX_FRACTAL_DEPTH);
    params.set_threshold(constants::CRYSTAL_RESONANCE_THRESHOLD);
    params
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

    #[test]
    fn test_growth_params() {
        let params = create_growth_params();
        assert_eq!(params.max_iterations(), constants::MAX_FRACTAL_DEPTH);
        assert_eq!(params.threshold(), constants::CRYSTAL_RESONANCE_THRESHOLD);
    }

    #[test]
    fn test_julia_constants() {
        assert!((-2.0..=2.0).contains(&constants::JULIA_GROWTH_REAL));
        assert!((-2.0..=2.0).contains(&constants::JULIA_GROWTH_IMAG));
    }
}
