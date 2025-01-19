//! Golden Ratio Operations for Crystal Lattice HPC Systems
//! ============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:15:42 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_GOLDEN_FACTOR,
        QUANTUM_HARMONY_THRESHOLD,
        CONVERGENCE_THRESHOLD,
        PHI,
        INV_PHI
    },
    traits::MeshValue,
};

/// Quantum-aware golden ratio scaling for crystal lattice values
/// Handles energy harmonization and phase golden alignment
pub fn quantum_phi<T: MeshValue>(x: T) -> Result<T, MathError> {
    let coherence = calculate_harmonization(x)?;
    if coherence < QUANTUM_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_phi(x)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Calculate golden ratio with quantum harmony preservation
pub fn harmonic_phi<T: MeshValue>(x: T) -> Result<T, MathError> {
    let harmony = check_harmony(x)?;
    if !harmony.is_stable() {
        return Err(MathError::HarmonyLoss);
    }

    quantum_phi(x)
}

/// Fibonacci-based golden ratio calculation with recursive harmonics
pub fn fibonacci_phi<T: MeshValue>(x: T, iterations: usize) -> Result<T, MathError> {
    let harmonics = calculate_fibonacci_harmonics(x, iterations)?;
    let harmonized_result = apply_fibonacci_harmonics(x, harmonics)?;

    quantum_phi(harmonized_result)
}

/// Calculate golden ratio with phase harmonization
pub fn phase_phi<T: MeshValue>(x: T, phase: f64) -> Result<T, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_harmonized = apply_phase_harmonization(x, phase)?;
    quantum_phi(phase_harmonized)
}

// Internal helper functions

#[inline]
fn calculate_harmonization<T: MeshValue>(x: T) -> Result<f64, MathError> {
    let base_coherence = x.coherence()? * PHI;
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_phi<T: MeshValue>(x: T) -> Result<T, MathError> {
    let phi_scaled = x.raw_mul(T::from(PHI))?;
    let golden_correction = calculate_golden_correction(x)?;
    apply_golden_correction(phi_scaled, golden_correction)
}

#[inline]
fn calculate_golden_correction<T: MeshValue>(x: T) -> Result<f64, MathError> {
    let energy_ratio = x.energy()? / PHI;
    Ok((energy_ratio * RESONANCE_FACTOR).fract())
}

#[inline]
fn apply_golden_correction<T: MeshValue>(value: T, correction: f64) -> Result<T, MathError> {
    value.harmonize(correction)
}

#[inline]
fn validate_lattice_bounds<T: MeshValue>(value: T) -> Result<(), MathError> {
    let mag = value.magnitude()?;
    if mag < MIN_LATTICE_SIZE as f64 || mag > MAX_LATTICE_SIZE as f64 {
        return Err(MathError::LatticeBoundsError);
    }
    Ok(())
}

#[inline]
fn check_harmony<T: MeshValue>(x: T) -> Result<HarmonyState, MathError> {
    let harmony = x.harmony_state()?;
    Ok(harmony)
}

#[inline]
fn calculate_fibonacci_harmonics<T: MeshValue>(x: T, iterations: usize) -> Result<f64, MathError> {
    let mut fib_n1: f64 = 1.0;
    let mut fib_n2: f64 = 1.0;

    for _ in 0..iterations {
        let temp = fib_n1 + fib_n2;
        fib_n2 = fib_n1;
        fib_n1 = temp;
    }

    let ratio = fib_n1 / fib_n2;
    Ok(ratio * x.energy()?)
}

#[inline]
fn apply_fibonacci_harmonics<T: MeshValue>(x: T, harmonics: f64) -> Result<T, MathError> {
    x.fibonacci_harmonize(harmonics)
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_harmonization<T: MeshValue>(x: T, phase: f64) -> Result<T, MathError> {
    let harmonized_phase = phase * PHASE_GOLDEN_FACTOR;
    x.phase_harmonize(harmonized_phase)
}

/// Calculate golden ratio powers with quantum stability
pub fn phi_power<T: MeshValue>(x: T, n: i32) -> Result<T, MathError> {
    if n == 0 {
        return Ok(T::unit());
    }

    let base = if n > 0 { PHI } else { INV_PHI };
    let mut result = x;

    for _ in 0..n.abs() {
        result = result.raw_mul(T::from(base))?;
        result = stabilized_phi(result)?;
    }

    Ok(result)
}

/// Quantum harmony state for golden ratio calculations
#[derive(Debug, Clone, Copy)]
pub struct HarmonyState {
    coherence: f64,
    phase: f64,
    energy: f64,
    harmony: f64,
    golden_ratio: f64,
}

impl HarmonyState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD &&
        self.harmony >= QUANTUM_HARMONY_THRESHOLD &&
        (self.golden_ratio - PHI).abs() < CONVERGENCE_THRESHOLD
    }

    #[inline]
    pub fn harmonize(&self) -> Self {
        Self {
            coherence: self.coherence * PHI,
            phase: self.phase * PHI,
            energy: self.energy * PHI,
            harmony: self.harmony * RESONANCE_FACTOR,
            golden_ratio: PHI,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test implementation of MeshValue for f64
    impl MeshValue for f64 {
        fn coherence(&self) -> Result<f64, MathError> { Ok(1.0) }
        fn energy(&self) -> Result<f64, MathError> { Ok(*self) }
        fn magnitude(&self) -> Result<f64, MathError> { Ok(self.abs()) }
        fn raw_mul(&self, other: Self) -> Result<Self, MathError> { Ok(self * other) }
        fn from(value: f64) -> Self { value }
        fn unit() -> Self { 1.0 }
        fn harmonize(&self, correction: f64) -> Result<Self, MathError> {
            Ok(self * (1.0 + correction * (PHI - 1.0)))
        }
        fn fibonacci_harmonize(&self, harmonics: f64) -> Result<Self, MathError> {
            Ok(self * harmonics)
        }
        fn phase_harmonize(&self, phase: f64) -> Result<Self, MathError> {
            Ok(self * (phase * PHASE_GOLDEN_FACTOR).cos())
        }
        fn harmony_state(&self) -> Result<HarmonyState, MathError> {
            Ok(HarmonyState {
                coherence: 1.0,
                phase: 0.0,
                energy: *self,
                harmony: 1.0,
                golden_ratio: PHI,
            })
        }
    }

    #[test]
    fn test_quantum_phi() {
        let result = quantum_phi(1.0).unwrap();
        assert!((result - PHI).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_harmonic_phi() {
        let result = harmonic_phi(1.0).unwrap();
        assert!((result - PHI).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_fibonacci_phi() {
        let result = fibonacci_phi(1.0, 20).unwrap();
        assert!((result - PHI).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_phase_phi() {
        let result = phase_phi(1.0, 0.0).unwrap();
        assert!((result - PHI).abs() < CONVERGENCE_THRESHOLD);

        assert!(phase_phi(1.0, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_phi_power() {
        let result = phi_power(1.0, 2).unwrap();
        assert!((result - PHI * PHI).abs() < CONVERGENCE_THRESHOLD);

        let inv_result = phi_power(1.0, -1).unwrap();
        assert!((inv_result - INV_PHI).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_harmony_state() {
        let state = HarmonyState {
            coherence: 1.0,
            phase: 0.0,
            energy: 1.0,
            harmony: 1.0,
            golden_ratio: PHI,
        };
        assert!(state.is_stable());

        let harmonized = state.harmonize();
        assert_eq!(harmonized.golden_ratio, PHI);
    }
}
