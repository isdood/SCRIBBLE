// lib/magicmath/src/traits.rs

//! Traits for Crystal Lattice HPC Systems
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 14:43:57 UTC
//! Version: 0.1.0
//! License: MIT

use errors::core::MathError;

/// Core trait for values that can be used in crystal lattice calculations
pub trait MeshValue: Sized {
    /// Get coherence value of the type
    fn coherence(&self) -> Result<f64, MathError>;

    /// Get energy value of the type
    fn energy(&self) -> Result<f64, MathError>;

    /// Get magnitude of the value
    fn magnitude(&self) -> Result<f64, MathError>;

    /// Convert value to usize
    fn to_usize(&self) -> Result<usize, MathError>;

    /// Convert value to f64
    fn to_f64(&self) -> Result<f64, MathError>;

    /// Create value from f64
    fn from(value: f64) -> Self;
}

/// Trait for complex quantum values
pub trait ComplexQuantum: MeshValue {
    /// Get real part of complex number
    fn real(&self) -> Result<f64, MathError>;

    /// Get imaginary part of complex number
    fn imag(&self) -> Result<f64, MathError>;

    /// Get phase angle of complex number
    fn phase(&self) -> Result<f64, MathError>;

    /// Create new complex quantum value
    fn new_complex(real: f64, imag: f64) -> Self;

    /// Get complex conjugate
    fn conjugate(&self) -> Result<Self, MathError>;
}

/// Trait for fractal iteration values
pub trait FractalValue: ComplexQuantum {
    /// Check if point is in set
    fn in_set(&self) -> Result<bool, MathError>;

    /// Get escape time
    fn escape_time(&self) -> Result<Option<usize>, MathError>;

    /// Get orbit of point
    fn orbit(&self) -> Result<Vec<(f64, f64)>, MathError>;

    /// Check stability of point
    fn is_stable(&self) -> Result<bool, MathError>;

    /// Get current iteration count
    fn iterations(&self) -> Result<usize, MathError>;
}

/// Trait for harmony state tracking
pub trait HarmonyState {
    /// Get current coherence
    fn get_coherence(&self) -> Result<f64, MathError>;

    /// Get current phase
    fn get_phase(&self) -> Result<f64, MathError>;

    /// Get current energy
    fn get_energy(&self) -> Result<f64, MathError>;

    /// Get current stability
    fn get_stability(&self) -> Result<f64, MathError>;

    /// Update harmony state
    fn update_state(&mut self, coherence: f64, phase: f64, energy: f64) -> Result<(), MathError>;

    /// Check if state is stable
    fn is_stable(&self) -> Result<bool, MathError>;
}

/// Trait for harmony operations
pub trait HarmonyOperation {
    /// Get operation coherence factor
    fn coherence_factor(&self) -> f64;

    /// Get operation phase shift
    fn phase_shift(&self) -> f64;

    /// Get operation energy factor
    fn energy_factor(&self) -> f64;

    /// Get operation stability impact
    fn stability_impact(&self) -> f64;
}

/// Trait for resonance calculations
pub trait Resonance {
    /// Calculate quantum resonance
    fn quantum_resonance(&self) -> Result<f64, MathError>;

    /// Calculate golden ratio resonance
    fn golden_resonance(&self) -> Result<f64, MathError>;

    /// Calculate Fibonacci resonance
    fn fibonacci_resonance(&self) -> Result<f64, MathError>;

    /// Calculate harmonic resonance
    fn harmonic_resonance(&self) -> Result<f64, MathError>;
}
