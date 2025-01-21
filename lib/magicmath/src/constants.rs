//! Constants for Crystal Lattice HPC Systems
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:23:47 UTC
//! Version: 0.1.0
//! License: MIT

// System Constants
pub const MAX_QUANTUM_SIZE: usize = 1_000_000;  // Add this constant
pub const MAX_LATTICE_SIZE: usize = 1_000_000;
pub const MIN_LATTICE_SIZE: usize = 1;
pub const MAX_ITERATIONS: usize = 10_000;
pub const MIN_ITERATIONS: usize = 1;
pub const MAX_RECURSION_DEPTH: usize = 1_000;
pub const CONVERGENCE_EPSILON: f64 = 1e-10;

// Mathematical Constants
pub const PI: f64 = std::f64::consts::PI;
pub const TAU: f64 = 2.0 * PI;
pub const E: f64 = std::f64::consts::E;
pub const PHI: f64 = 1.618033988749895;
pub const SQRT_2: f64 = std::f64::consts::SQRT_2;
pub const SQRT_3: f64 = 1.732050807568877;
pub const SQRT_5: f64 = 2.236067977499790;
pub const LN_2: f64 = std::f64::consts::LN_2;
pub const LN_10: f64 = std::f64::consts::LN_10;

// Harmony State Constants
pub const HARMONY_STABILITY_THRESHOLD: f64 = 0.1;
pub const HARMONY_COHERENCE_THRESHOLD: f64 = 0.01;
pub const HARMONY_ENERGY_THRESHOLD: f64 = 0.001;
pub const HARMONY_PHASE_THRESHOLD: f64 = 0.0001;
pub const HARMONY_RESONANCE_THRESHOLD: f64 = 0.00001;
pub const HARMONY_ENTANGLEMENT_THRESHOLD: f64 = 0.000001;

// Phase Operation Constants
pub const PHASE_COUPLING_CONSTANT: f64 = 0.95;      // For addition
pub const PHASE_DECOUPLING_CONSTANT: f64 = 0.90;    // For subtraction
pub const PHASE_AMPLIFICATION_FACTOR: f64 = 1.05;   // For multiplication
pub const PHASE_ATTENUATION_FACTOR: f64 = 0.85;     // For division
pub const SINGULARITY_THRESHOLD: f64 = 1e-15;       // For division near zero

// Resonance Factors
pub const RESONANCE_FACTOR: f64 = 0.999;
pub const PHASE_RESONANCE_FACTOR: f64 = 0.9999;
pub const ENERGY_RESONANCE_FACTOR: f64 = 0.99999;
pub const COHERENCE_RESONANCE_FACTOR: f64 = 0.999999;
pub const STABILITY_RESONANCE_FACTOR: f64 = 0.9999999;

// Fractal Constants
pub const PHASE_FRACTAL_FACTOR: f64 = 0.1;
pub const PHASE_JULIA_FACTOR: f64 = 0.01;
pub const PHASE_MANDELBROT_FACTOR: f64 = 0.001;
pub const HARMONY_JULIA_THRESHOLD: f64 = 0.15;
pub const HARMONY_MANDELBROT_THRESHOLD: f64 = 0.2;
pub const HARMONY_FRACTAL_THRESHOLD: f64 = 0.25;
pub const MAX_ORBIT_POINTS: usize = 1_000;
pub const BULB_DETECTION_THRESHOLD: f64 = 0.0625;
pub const CARDIOID_DETECTION_THRESHOLD: f64 = 0.25;

// Complex Number Constants
pub const COMPLEX_OVERFLOW_THRESHOLD: f64 = 1e308;
pub const COMPLEX_UNDERFLOW_THRESHOLD: f64 = 1e-308;
pub const COMPLEX_ITERATION_LIMIT: usize = 1_000;
pub const COMPLEX_CONVERGENCE_THRESHOLD: f64 = 1e-15;
pub const COMPLEX_STABILITY_THRESHOLD: f64 = 1e-12;

// Optimization Constants
pub const CACHE_SIZE: usize = 1_024;
pub const THREAD_POOL_SIZE: usize = 8;
pub const CHUNK_SIZE: usize = 256;
pub const BUFFER_SIZE: usize = 4_096;
pub const MAX_PARALLEL_TASKS: usize = 64;

// Error Thresholds
pub const MAX_ERROR_RETRIES: usize = 3;
pub const ERROR_BACKOFF_MS: u64 = 100;
pub const ERROR_THRESHOLD: f64 = 1e-6;
pub const WARNING_THRESHOLD: f64 = 1e-3;
pub const CRITICAL_THRESHOLD: f64 = 1e-9;

// Performance Tuning
pub const OPTIMIZATION_LEVEL: u8 = 3;
pub const VECTORIZATION_THRESHOLD: usize = 32;
pub const UNROLL_FACTOR: usize = 4;
pub const PREFETCH_DISTANCE: usize = 8;
pub const BRANCH_PREDICTION_THRESHOLD: f64 = 0.75;

// System Metadata
pub const SYSTEM_VERSION: &str = "0.1.0";
pub const SYSTEM_NAME: &str = "MagicMath Crystal Lattice HPC";
pub const SYSTEM_AUTHOR: &str = "Caleb J.D. Terkovics <isdood>";
pub const SYSTEM_CREATED: &str = "2025-01-19";
pub const SYSTEM_UPDATED: &str = "2025-01-19 22:23:47 UTC";
pub const SYSTEM_LICENSE: &str = "MIT";

// Module-specific Constants
pub mod harmony {
    pub const MIN_COHERENCE: f64 = 0.0;
    pub const MAX_COHERENCE: f64 = 1.0;
    pub const MIN_ENERGY: f64 = 0.0;
    pub const MAX_ENERGY: f64 = 1e6;
    pub const MIN_PHASE: f64 = -std::f64::consts::PI;
    pub const MAX_PHASE: f64 = std::f64::consts::PI;
}

pub mod fractal {
    pub const MIN_ESCAPE_RADIUS: f64 = 2.0;
    pub const MAX_ESCAPE_RADIUS: f64 = 1e6;
    pub const MIN_ITERATIONS: usize = 1;
    pub const MAX_ITERATIONS: usize = 1_000_000;
    pub const DEFAULT_HEIGHT: usize = 1_000;
    pub const DEFAULT_WIDTH: usize = 1_000;
}

pub mod complex {
    pub const I: (f64, f64) = (0.0, 1.0);
    pub const ONE: (f64, f64) = (1.0, 0.0);
    pub const ZERO: (f64, f64) = (0.0, 0.0);
    pub const INF: (f64, f64) = (f64::INFINITY, f64::INFINITY);
}

pub mod mesh {
    // Resolution bounds
    pub const RESOLUTION_MIN: usize = 10;
    pub const RESOLUTION_MAX: usize = 10_000;
    pub const PRECISION_DEFAULT: f64 = 1e-6;

    // Mesh stability thresholds
    pub const STABILITY_THRESHOLD: f64 = 0.25;
    pub const COHERENCE_THRESHOLD: f64 = 0.15;
    pub const PRECISION_THRESHOLD: f64 = 1e-12;

    // Mesh optimization parameters
    pub const CACHE_LINE_SIZE: usize = 64;
    pub const VECTOR_WIDTH: usize = 4;
    pub const PREFETCH_DISTANCE: usize = 8;

    // Default mesh dimensions
    pub const DEFAULT_WIDTH: usize = 100;
    pub const DEFAULT_HEIGHT: usize = 100;
    pub const DEFAULT_DEPTH: usize = 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathematical_constants() {
        assert!((PI - std::f64::consts::PI).abs() < 1e-10);
        assert!((PHI * PHI - PHI - 1.0).abs() < 1e-10);
        assert!((SQRT_5 * SQRT_5 - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_harmony_thresholds() {
        assert!(HARMONY_STABILITY_THRESHOLD > HARMONY_COHERENCE_THRESHOLD);
        assert!(HARMONY_COHERENCE_THRESHOLD > HARMONY_ENERGY_THRESHOLD);
        assert!(HARMONY_ENERGY_THRESHOLD > HARMONY_PHASE_THRESHOLD);
    }

    #[test]
    fn test_phase_operation_constants() {
        assert!(PHASE_AMPLIFICATION_FACTOR > 1.0);
        assert!(PHASE_ATTENUATION_FACTOR < 1.0);
        assert!(PHASE_COUPLING_CONSTANT < 1.0);
        assert!(PHASE_DECOUPLING_CONSTANT < PHASE_COUPLING_CONSTANT);
    }

    #[test]
    fn test_resonance_factors() {
        assert!(RESONANCE_FACTOR < 1.0);
        assert!(PHASE_RESONANCE_FACTOR < 1.0);
        assert!(ENERGY_RESONANCE_FACTOR < 1.0);
        assert!(COHERENCE_RESONANCE_FACTOR < 1.0);
    }

    #[test]
    fn test_fractal_constants() {
        assert!(HARMONY_MANDELBROT_THRESHOLD > HARMONY_JULIA_THRESHOLD);
        assert!(HARMONY_FRACTAL_THRESHOLD > HARMONY_MANDELBROT_THRESHOLD);
        assert!(MAX_ORBIT_POINTS > 0);
    }

    #[test]
    fn test_complex_constants() {
        assert!(COMPLEX_OVERFLOW_THRESHOLD > 0.0);
        assert!(COMPLEX_UNDERFLOW_THRESHOLD > 0.0);
        assert!(COMPLEX_ITERATION_LIMIT > 0);
    }

    #[test]
    fn test_optimization_constants() {
        assert!(CACHE_SIZE.is_power_of_two());
        assert!(CHUNK_SIZE.is_power_of_two());
        assert!(BUFFER_SIZE.is_power_of_two());
        assert!(THREAD_POOL_SIZE > 0);
    }

    #[test]
    fn test_module_constants() {
        assert!(harmony::MAX_COHERENCE > harmony::MIN_COHERENCE);
        assert!(harmony::MAX_ENERGY > harmony::MIN_ENERGY);
        assert!(harmony::MAX_PHASE > harmony::MIN_PHASE);
    }

    #[test]
    fn test_mesh_constants() {
        assert!(mesh::RESOLUTION_MAX > mesh::RESOLUTION_MIN);
        assert!(mesh::PRECISION_DEFAULT > 0.0);
        assert!(mesh::STABILITY_THRESHOLD > mesh::COHERENCE_THRESHOLD);
        assert!(mesh::DEFAULT_WIDTH > 0);
        assert!(mesh::DEFAULT_HEIGHT > 0);
        assert!(mesh::DEFAULT_DEPTH > 0);
    }
}
