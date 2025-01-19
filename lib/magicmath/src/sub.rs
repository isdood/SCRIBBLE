// lib/magicmath/src/sub.rs

//! Subtraction Operations for Crystal Lattice HPC Systems
//! ============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:08:36 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        HARMONY_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_COUPLING_CONSTANT
    },
    traits::MeshValue,
};

/// Harmony-aware subtraction for crystal lattice values
/// Maintains coherence through resonance stabilization
pub fn harmony_sub<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let coherence = calculate_decoherence(a, b)?;
    if coherence < HARMONY_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_sub(a, b)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Subtract values while preserving crystal structure alignment
pub fn aligned_sub<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let alignment = check_alignment(a, b)?;
    if !alignment.is_stable() {
        return Err(MathError::AlignmentError);
    }

    harmony_sub(a, b)
}

/// Anti-resonant subtraction for enhanced harmony stability
pub fn antiresonant_sub<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let antiresonance = calculate_antiresonance(a, b)?;
    let scaled_result = apply_antiresonance(a, b, antiresonance)?;

    harmony_sub(scaled_result, T::zero())
}

/// Subtract lattice values with phase decoupling
pub fn phase_sub<T: MeshValue>(a: T, b: T, phase: f64) -> Result<T, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_adjusted = apply_phase_decoupling(a, b, phase)?;
    harmony_sub(phase_adjusted.0, phase_adjusted.1)
}

// Internal helper functions

#[inline]
fn calculate_decoherence<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_coherence = a.coherence()? / b.coherence()?;
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_sub<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    if a.below_bounds(b) {
        return Err(MathError::LatticeUnderflow);
    }
    Ok(a.raw_sub(b))
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
fn check_alignment<T: MeshValue>(a: T, b: T) -> Result<AlignmentState, MathError> {
    let alignment = a.alignment_state()?.decouple(b.alignment_state()?);
    Ok(alignment)
}

#[inline]
fn calculate_antiresonance<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_antiresonance = (a.energy()? / b.energy()?).sqrt();
    Ok(base_antiresonance / RESONANCE_FACTOR)
}

#[inline]
fn apply_antiresonance<T: MeshValue>(a: T, b: T, antiresonance: f64) -> Result<T, MathError> {
    let scaled_a = a.scale(antiresonance)?;
    let scaled_b = b.scale(1.0 / antiresonance)?;
    Ok(scaled_a.raw_sub(scaled_b))
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_decoupling<T: MeshValue>(a: T, b: T, phase: f64) -> Result<(T, T), MathError> {
    let decoupled_a = a.phase_adjust(phase)?;
    let decoupled_b = b.phase_adjust(phase + PHASE_COUPLING_CONSTANT)?;
    Ok((decoupled_a, decoupled_b))
}

/// Harmony state for alignment calculations in subtraction
#[derive(Debug, Clone, Copy)]
pub struct AlignmentState {
    coherence: f64,
    phase: f64,
    energy: f64,
}

impl AlignmentState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= HARMONY_STABILITY_THRESHOLD
    }

    #[inline]
    pub fn decouple(&self, other: AlignmentState) -> Self {
        Self {
            coherence: self.coherence / other.coherence,
            phase: self.phase - other.phase,
            energy: (self.energy - other.energy).abs() / RESONANCE_FACTOR,
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
        fn raw_sub(&self, other: Self) -> Self { self - other }
        fn scale(&self, factor: f64) -> Result<Self, MathError> { Ok(self * factor) }
        fn phase_adjust(&self, phase: f64) -> Result<Self, MathError> {
            Ok(self * phase.cos())
        }
        fn alignment_state(&self) -> Result<AlignmentState, MathError> {
            Ok(AlignmentState {
                coherence: 1.0,
                phase: 0.0,
                energy: *self,
            })
        }
        fn below_bounds(&self, other: Self) -> bool {
            self.abs() - other.abs() < MIN_LATTICE_SIZE as f64
        }
    }

    #[test]
    fn test_harmony_sub() {
        assert_eq!(harmony_sub(5.0, 3.0).unwrap(), 2.0);
        assert!(harmony_sub(1.0, 2.0).is_err()); // Below minimum
    }

    #[test]
    fn test_aligned_sub() {
        assert_eq!(aligned_sub(5.0, 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_antiresonant_sub() {
        let result = antiresonant_sub(4.0, 2.0).unwrap();
        assert!(result < 2.0); // Due to antiresonance factor
    }

    #[test]
    fn test_phase_sub() {
        let result = phase_sub(4.0, 2.0, 0.0).unwrap();
        assert_eq!(result, 2.0);

        assert!(phase_sub(4.0, 2.0, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_alignment_state() {
        let state = AlignmentState {
            coherence: 1.0,
            phase: 0.0,
            energy: 1.0,
        };
        assert!(state.is_stable());

        let other = AlignmentState {
            coherence: 0.5,
            phase: core::f64::consts::PI,
            energy: 0.5,
        };

        let decoupled = state.decouple(other);
        assert!(decoupled.coherence > 1.0);
    }
}
