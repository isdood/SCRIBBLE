// lib/magicmath/src/div.rs

//! Division Operations for Crystal Lattice HPC Systems
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:11:14 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        HARMONY_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_ATTENUATION_FACTOR,
        HARMONY_COHERENCE_THRESHOLD,
        SINGULARITY_THRESHOLD,
    },
    traits::MeshValue,
};

/// Harmony-aware division for crystal lattice values
/// Handles energy distribution and phase decoupling
pub fn harmony_div<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let coherence = calculate_distribution(a, b)?;
    if coherence < HARMONY_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_div(a, b)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Divide values with harmony coherence preservation
pub fn coherent_div<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let coherence = check_coherence(a, b)?;
    if !coherence.is_stable() {
        return Err(MathError::CoherenceLoss);
    }

    harmony_div(a, b)
}

/// Resonant division with harmonic attenuation
pub fn harmonic_div<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let harmonics = calculate_inverse_harmonics(a, b)?;
    let attenuated_result = apply_inverse_harmonics(a, b, harmonics)?;

    harmony_div(attenuated_result, T::unit())
}

/// Divide lattice values with phase attenuation
pub fn phase_div<T: MeshValue>(a: T, b: T, phase: f64) -> Result<T, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_attenuated = apply_phase_attenuation(a, b, phase)?;
    harmony_div(phase_attenuated.0, phase_attenuated.1)
}

// Internal helper functions

#[inline]
fn calculate_distribution<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_coherence = (a.coherence()? / b.coherence()?).sqrt();
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_div<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    check_singularity(b)?;
    if would_underflow(a, b)? {
        return Err(MathError::LatticeUnderflow);
    }
    Ok(a.raw_div(b))
}

#[inline]
fn check_singularity<T: MeshValue>(value: T) -> Result<(), MathError> {
    if value.magnitude()? < SINGULARITY_THRESHOLD {
        return Err(MathError::QuantumSingularity);
    }
    Ok(())
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
fn would_underflow<T: MeshValue>(a: T, b: T) -> Result<bool, MathError> {
    let quotient_energy = a.energy()? / b.energy()?;
    Ok(quotient_energy < MIN_LATTICE_SIZE as f64)
}

#[inline]
fn check_coherence<T: MeshValue>(a: T, b: T) -> Result<CoherenceState, MathError> {
    let coherence = a.coherence_state()?.divide(b.coherence_state()?);
    Ok(coherence)
}

#[inline]
fn calculate_inverse_harmonics<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_harmonics = (a.energy()? / b.energy()?).sqrt();
    Ok(base_harmonics / RESONANCE_FACTOR)
}

#[inline]
fn apply_inverse_harmonics<T: MeshValue>(a: T, b: T, harmonics: f64) -> Result<T, MathError> {
    let attenuated_a = a.attenuate(harmonics)?;
    let attenuated_b = b.attenuate(1.0 / harmonics)?;
    Ok(attenuated_a.raw_div(attenuated_b))
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_attenuation<T: MeshValue>(a: T, b: T, phase: f64) -> Result<(T, T), MathError> {
    let attenuated_phase = phase * PHASE_ATTENUATION_FACTOR;
    let phase_a = a.phase_attenuate(attenuated_phase)?;
    let phase_b = b.phase_attenuate(attenuated_phase)?;
    Ok((phase_a, phase_b))
}

/// Harmony coherence state for division
#[derive(Debug, Clone, Copy)]
pub struct CoherenceState {
    coherence: f64,
    phase: f64,
    energy: f64,
    stability: f64,
}

impl CoherenceState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= HARMONY_COHERENCE_THRESHOLD &&
        self.stability >= HARMONY_STABILITY_THRESHOLD
    }

    #[inline]
    pub fn divide(&self, other: CoherenceState) -> Self {
        Self {
            coherence: (self.coherence / other.coherence).sqrt(),
            phase: self.phase / other.phase,
            energy: self.energy / other.energy,
            stability: (self.stability / other.stability) * RESONANCE_FACTOR,
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
        fn raw_div(&self, other: Self) -> Self { self / other }
        fn attenuate(&self, factor: f64) -> Result<Self, MathError> { Ok(self / factor.sqrt()) }
        fn phase_attenuate(&self, phase: f64) -> Result<Self, MathError> {
            Ok(self / (phase * PHASE_ATTENUATION_FACTOR).cos())
        }
        fn coherence_state(&self) -> Result<CoherenceState, MathError> {
            Ok(CoherenceState {
                coherence: 1.0,
                phase: core::f64::consts::PI,
                energy: *self,
                stability: 1.0,
            })
        }
        fn unit() -> Self { 1.0 }
    }

    #[test]
    fn test_harmony_div() {
        assert_eq!(harmony_div(6.0, 2.0).unwrap(), 3.0);
        assert!(harmony_div(1.0, 0.1).is_err()); // Below stability threshold
    }

    #[test]
    fn test_coherent_div() {
        assert_eq!(coherent_div(6.0, 2.0).unwrap(), 3.0);
    }

    #[test]
    fn test_harmonic_div() {
        let result = harmonic_div(8.0, 2.0).unwrap();
        assert!(result < 4.0); // Due to harmonic attenuation
    }

    #[test]
    fn test_phase_div() {
        let result = phase_div(8.0, 2.0, 0.0).unwrap();
        assert_eq!(result, 4.0);

        assert!(phase_div(8.0, 2.0, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_coherence_state() {
        let state = CoherenceState {
            coherence: 1.0,
            phase: core::f64::consts::PI,
            energy: 1.0,
            stability: 1.0,
        };
        assert!(state.is_stable());

        let divided = state.divide(state);
        assert!(divided.stability > 0.0);
    }

    #[test]
    fn test_singularity() {
        assert!(harmony_div(1.0, 0.0).is_err());
        assert!(harmony_div(1.0, SINGULARITY_THRESHOLD / 2.0).is_err());
    }
}
