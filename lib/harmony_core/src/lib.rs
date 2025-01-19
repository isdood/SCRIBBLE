//! Harmony Core - Crystal Computing Core Operations
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 17:52:37 UTC
//! Version: 0.1.1
//! License: MIT

#![no_std]
#![feature(const_trait_impl)]

// External crates
extern crate magicmath;
extern crate scribe;

// Module declarations
pub mod vector;
pub mod harmony;
pub mod idk;
pub mod crystal;
pub mod errors;
pub mod zeronaut;
pub mod phantom;
pub mod align;
pub mod aether;
pub mod cube;
pub mod growth; // New module for crystal growth patterns
pub mod native; // New module for native types and macros

// Re-exports from local modules
pub use crystal::{CrystalLattice, CrystalNode};
pub use errors::{QuantumError, CoherenceError, QuantumResult, CoherenceResult};
pub use vector::{Vector3D, Vector4D};
pub use zeronaut::Zeronaut;
pub use phantom::Phantom;
pub use scribe::Scribe;
pub use harmony::{Quantum, Phase, Resonance};
pub use align::{Alignment, AlignmentState, AlignmentResult};
pub use aether::AetherField;
pub use growth::{GrowthPattern, GrowthState, CrystalGrowth}; // New exports
pub use native::{String, Box, vec, Vec}; // New exports

// Re-exports from magicmath
pub use magicmath::{
    QuantumMath,
    FractalParams,
    FractalState,
    JuliaParams,
    JuliaState,
    JuliaVariant,
    MandelbrotParams,
    MandelbrotState,
    MandelbrotVariant,
    generate_fractal,
    iterate_julia,
    iterate_mandelbrot,
};

// Constants for crystal computing operations
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

    /// Crystal growth rate factor
    pub const GROWTH_RATE_FACTOR: f64 = 0.5;

    /// Maximum fractal iteration depth for crystal growth
    pub const MAX_FRACTAL_DEPTH: usize = 64;

    /// Julia set parameters for optimal crystal growth
    pub const JULIA_GROWTH_REAL: f64 = -0.4;
    pub const JULIA_GROWTH_IMAG: f64 = 0.6;
}

/// Initialize a new quantum math context for crystal operations
pub fn init_quantum_math() -> QuantumMath {
    QuantumMath::new()
}

/// Create growth parameters for crystal expansion
pub fn create_growth_params() -> FractalParams {
    FractalParams::default()
    .with_max_iterations(constants::MAX_FRACTAL_DEPTH)
    .with_threshold(constants::CRYSTAL_RESONANCE_THRESHOLD)
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
    fn test_quantum_math_init() {
        let qmath = init_quantum_math();
        assert!(qmath.get_state().is_stable());
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
