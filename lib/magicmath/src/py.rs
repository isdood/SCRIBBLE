//! Pythagorean Theorem Operations for Crystal Lattice HPC Systems
//! ================================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:17:29 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_ORTHOGONAL_FACTOR,
        QUANTUM_ORTHOGONALITY_THRESHOLD,
        CONVERGENCE_THRESHOLD,
        DIMENSIONAL_COUPLING_CONSTANT
    },
    traits::MeshValue,
};

/// Quantum-aware Pythagorean theorem for crystal lattice values
/// Handles dimensional coupling and orthogonal alignment
pub fn quantum_pythagoras<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let coherence = calculate_orthogonality(a, b)?;
    if coherence < QUANTUM_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_pythagoras(a, b)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Calculate hypotenuse with quantum orthogonality preservation
pub fn orthogonal_hypotenuse<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let orthogonality = check_orthogonality(a, b)?;
    if !orthogonality.is_stable() {
        return Err(MathError::OrthogonalityLoss);
    }

    quantum_pythagoras(a, b)
}

/// Calculate leg with harmonic dimensional reduction
pub fn harmonic_leg<T: MeshValue>(hypotenuse: T, other_leg: T) -> Result<T, MathError> {
    let harmonics = calculate_dimensional_harmonics(hypotenuse, other_leg)?;
    let reduced_result = apply_dimensional_reduction(hypotenuse, other_leg, harmonics)?;

    quantum_pythagoras(reduced_result, T::unit())
}

/// Calculate Pythagorean triple verification with phase coupling
pub fn phase_triple<T: MeshValue>(a: T, b: T, c: T, phase: f64) -> Result<bool, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_coupled = apply_phase_coupling(a, b, c, phase)?;
    verify_pythagorean_triple(phase_coupled.0, phase_coupled.1, phase_coupled.2)
}

// Internal helper functions

#[inline]
fn calculate_orthogonality<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_coherence = (a.coherence()? * b.coherence()?).sqrt();
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_pythagoras<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let squared_a = a.raw_mul(a)?;
    let squared_b = b.raw_mul(b)?;
    let sum = squared_a.raw_add(squared_b)?;
    quantum_sqrt(sum)
}

#[inline]
fn quantum_sqrt<T: MeshValue>(x: T) -> Result<T, MathError> {
    if x.is_negative() {
        return Err(MathError::ComplexDomain);
    }

    newton_sqrt(x)
}

#[inline]
fn newton_sqrt<T: MeshValue>(x: T) -> Result<T, MathError> {
    let mut guess = x.half()?;
    let mut prev_guess = T::zero();

    while relative_error(guess, prev_guess)? > CONVERGENCE_THRESHOLD {
        prev_guess = guess;
        guess = average(guess, x.raw_div(guess)?)?;
    }

    Ok(guess)
}

#[inline]
fn relative_error<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    if b.is_zero() {
        return Ok(f64::INFINITY);
    }
    Ok((a.magnitude()? - b.magnitude()?) / b.magnitude()?)
}

#[inline]
fn average<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    Ok(a.raw_add(b)?.half()?)
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
fn check_orthogonality<T: MeshValue>(a: T, b: T) -> Result<OrthogonalityState, MathError> {
    let orthogonality = a.orthogonality_state()?.combine(b.orthogonality_state()?);
    Ok(orthogonality)
}

#[inline]
fn calculate_dimensional_harmonics<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let energy_ratio = a.energy()? / b.energy()?;
    Ok(energy_ratio.sqrt() * RESONANCE_FACTOR)
}

#[inline]
fn apply_dimensional_reduction<T: MeshValue>(hyp: T, leg: T, harmonics: f64) -> Result<T, MathError> {
    let squared_hyp = hyp.raw_mul(hyp)?;
    let squared_leg = leg.raw_mul(leg)?;
    let diff = squared_hyp.raw_sub(squared_leg)?;
    quantum_sqrt(diff.scale(harmonics)?)
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_coupling<T: MeshValue>(a: T, b: T, c: T, phase: f64)
-> Result<(T, T, T), MathError>
{
    let coupled_phase = phase * PHASE_ORTHOGONAL_FACTOR;
    let coupled_a = a.phase_couple(coupled_phase)?;
    let coupled_b = b.phase_couple(coupled_phase + DIMENSIONAL_COUPLING_CONSTANT)?;
    let coupled_c = c.phase_couple(coupled_phase + 2.0 * DIMENSIONAL_COUPLING_CONSTANT)?;
    Ok((coupled_a, coupled_b, coupled_c))
}

#[inline]
fn verify_pythagorean_triple<T: MeshValue>(a: T, b: T, c: T) -> Result<bool, MathError> {
    let squared_a = a.raw_mul(a)?;
    let squared_b = b.raw_mul(b)?;
    let squared_c = c.raw_mul(c)?;
    let sum = squared_a.raw_add(squared_b)?;

    Ok((sum.raw_sub(squared_c)?.magnitude()? < CONVERGENCE_THRESHOLD))
}

/// Quantum orthogonality state for Pythagorean calculations
#[derive(Debug, Clone, Copy)]
pub struct OrthogonalityState {
    coherence: f64,
    phase: f64,
    energy: f64,
    orthogonality: f64,
}

impl OrthogonalityState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD &&
        self.orthogonality >= QUANTUM_ORTHOGONALITY_THRESHOLD
    }

    #[inline]
    pub fn combine(&self, other: OrthogonalityState) -> Self {
        Self {
            coherence: (self.coherence * other.coherence).sqrt(),
            phase: self.phase + other.phase,
            energy: (self.energy * other.energy).sqrt(),
            orthogonality: (self.orthogonality + other.orthogonality) * RESONANCE_FACTOR,
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
        fn raw_add(&self, other: Self) -> Result<Self, MathError> { Ok(self + other) }
        fn raw_sub(&self, other: Self) -> Result<Self, MathError> { Ok(self - other) }
        fn raw_mul(&self, other: Self) -> Result<Self, MathError> { Ok(self * other) }
        fn raw_div(&self, other: Self) -> Result<Self, MathError> { Ok(self / other) }
        fn half(&self) -> Result<Self, MathError> { Ok(self / 2.0) }
        fn is_zero(&self) -> bool { *self == 0.0 }
        fn is_negative(&self) -> bool { *self < 0.0 }
        fn zero() -> Self { 0.0 }
        fn unit() -> Self { 1.0 }
        fn scale(&self, factor: f64) -> Result<Self, MathError> { Ok(self * factor) }
        fn phase_couple(&self, phase: f64) -> Result<Self, MathError> {
            Ok(self * (phase * PHASE_ORTHOGONAL_FACTOR).cos())
        }
        fn orthogonality_state(&self) -> Result<OrthogonalityState, MathError> {
            Ok(OrthogonalityState {
                coherence: 1.0,
                phase: 0.0,
                energy: *self,
                orthogonality: 1.0,
            })
        }
    }

    #[test]
    fn test_quantum_pythagoras() {
        let result = quantum_pythagoras(3.0, 4.0).unwrap();
        assert!((result - 5.0).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_orthogonal_hypotenuse() {
        let result = orthogonal_hypotenuse(3.0, 4.0).unwrap();
        assert!((result - 5.0).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_harmonic_leg() {
        let result = harmonic_leg(5.0, 4.0).unwrap();
        assert!((result - 3.0).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_phase_triple() {
        let is_triple = phase_triple(3.0, 4.0, 5.0, 0.0).unwrap();
        assert!(is_triple);

        let not_triple = phase_triple(3.0, 4.0, 6.0, 0.0).unwrap();
        assert!(!not_triple);

        assert!(phase_triple(3.0, 4.0, 5.0, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_orthogonality_state() {
        let state = OrthogonalityState {
            coherence: 1.0,
            phase: 0.0,
            energy: 1.0,
            orthogonality: 1.0,
        };
        assert!(state.is_stable());

        let combined = state.combine(state);
        assert!(combined.orthogonality > 1.0);
    }
}
