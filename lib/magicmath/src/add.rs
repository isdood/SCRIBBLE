//! Addition Operations for Crystal Lattice HPC Systems
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:07:30 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR
    },
    traits::MeshValue,
};

/// Quantum-aware addition for crystal lattice values
/// Maintains coherence through resonance stabilization
pub fn quantum_add<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let coherence = calculate_coherence(a, b)?;
    if coherence < QUANTUM_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_add(a, b)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Add values while preserving crystal structure alignment
pub fn aligned_add<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let alignment = check_alignment(a, b)?;
    if !alignment.is_stable() {
        return Err(MathError::AlignmentError);
    }

    quantum_add(a, b)
}

/// Resonant addition for enhanced quantum stability
pub fn resonant_add<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let resonance = calculate_resonance(a, b)?;
    let scaled_result = apply_resonance(a, b, resonance)?;

    quantum_add(scaled_result, T::zero())
}

/// Add lattice values with phase alignment
pub fn phase_add<T: MeshValue>(a: T, b: T, phase: f64) -> Result<T, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_adjusted = apply_phase_correction(a, b, phase)?;
    quantum_add(phase_adjusted.0, phase_adjusted.1)
}

// Internal helper functions

#[inline]
fn calculate_coherence<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_coherence = a.coherence()? * b.coherence()?;
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_add<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    if a.exceeds_bounds(b) {
        return Err(MathError::LatticeOverflow);
    }
    Ok(a.raw_add(b))
}

#[inline]
fn validate_lattice_bounds<T: MeshValue>(value: T) -> Result<(), MathError> {
    if value.magnitude()? > MAX_LATTICE_SIZE as f64 {
        return Err(MathError::LatticeOverflow);
    }
    Ok(())
}

#[inline]
fn check_alignment<T: MeshValue>(a: T, b: T) -> Result<AlignmentState, MathError> {
    let alignment = a.alignment_state()?.combine(b.alignment_state()?);
    Ok(alignment)
}

#[inline]
fn calculate_resonance<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_resonance = (a.energy()? * b.energy()?).sqrt();
    Ok(base_resonance * RESONANCE_FACTOR)
}

#[inline]
fn apply_resonance<T: MeshValue>(a: T, b: T, resonance: f64) -> Result<T, MathError> {
    let scaled = a.scale(resonance)?;
    Ok(scaled.raw_add(b.scale(resonance)?))
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_correction<T: MeshValue>(a: T, b: T, phase: f64) -> Result<(T, T), MathError> {
    let adjusted_a = a.phase_adjust(phase)?;
    let adjusted_b = b.phase_adjust(phase)?;
    Ok((adjusted_a, adjusted_b))
}

/// Quantum state for alignment calculations
#[derive(Debug, Clone, Copy)]
pub struct AlignmentState {
    coherence: f64,
    phase: f64,
    energy: f64,
}

impl AlignmentState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    #[inline]
    pub fn combine(&self, other: AlignmentState) -> Self {
        Self {
            coherence: self.coherence * other.coherence,
            phase: (self.phase + other.phase) / 2.0,
            energy: (self.energy + other.energy) * RESONANCE_FACTOR,
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
        fn raw_add(&self, other: Self) -> Self { self + other }
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
        fn exceeds_bounds(&self, other: Self) -> bool {
            self.abs() + other.abs() > MAX_LATTICE_SIZE as f64
        }
    }

    #[test]
    fn test_quantum_add() {
        assert_eq!(quantum_add(2.0, 3.0).unwrap(), 5.0);
        assert!(quantum_add(MAX_LATTICE_SIZE as f64, 1.0).is_err());
    }

    #[test]
    fn test_aligned_add() {
        assert_eq!(aligned_add(2.0, 3.0).unwrap(), 5.0);
    }

    #[test]
    fn test_resonant_add() {
        let result = resonant_add(2.0, 2.0).unwrap();
        assert!(result > 4.0); // Due to resonance factor
    }

    #[test]
    fn test_phase_add() {
        let result = phase_add(2.0, 2.0, 0.0).unwrap();
        assert_eq!(result, 4.0);

        assert!(phase_add(2.0, 2.0, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_alignment_state() {
        let state = AlignmentState {
            coherence: 1.0,
            phase: 0.0,
            energy: 1.0,
        };
        assert!(state.is_stable());
    }
}
