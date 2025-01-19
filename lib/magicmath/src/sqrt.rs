//! Square Root Operations for Crystal Lattice HPC Systems
//! ===========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:12:30 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_BIFURCATION_FACTOR,
        QUANTUM_SYMMETRY_THRESHOLD,
        CONVERGENCE_THRESHOLD
    },
    traits::MeshValue,
};

/// Quantum-aware square root for crystal lattice values
/// Handles energy bifurcation and phase symmetry
pub fn quantum_sqrt<T: MeshValue>(x: T) -> Result<T, MathError> {
    let coherence = calculate_bifurcation(x)?;
    if coherence < QUANTUM_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_sqrt(x)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Calculate square root with quantum symmetry preservation
pub fn symmetric_sqrt<T: MeshValue>(x: T) -> Result<T, MathError> {
    let symmetry = check_symmetry(x)?;
    if !symmetry.is_stable() {
        return Err(MathError::SymmetryLoss);
    }

    quantum_sqrt(x)
}

/// Resonant square root with harmonic bifurcation
pub fn harmonic_sqrt<T: MeshValue>(x: T) -> Result<T, MathError> {
    let harmonics = calculate_bifurcation_harmonics(x)?;
    let bifurcated_result = apply_bifurcation_harmonics(x, harmonics)?;

    quantum_sqrt(bifurcated_result)
}

/// Calculate square root with phase bifurcation
pub fn phase_sqrt<T: MeshValue>(x: T, phase: f64) -> Result<T, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_bifurcated = apply_phase_bifurcation(x, phase)?;
    quantum_sqrt(phase_bifurcated)
}

// Internal helper functions

#[inline]
fn calculate_bifurcation<T: MeshValue>(x: T) -> Result<f64, MathError> {
    let base_coherence = x.coherence()?.sqrt();
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_sqrt<T: MeshValue>(x: T) -> Result<T, MathError> {
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
fn check_symmetry<T: MeshValue>(x: T) -> Result<SymmetryState, MathError> {
    let symmetry = x.symmetry_state()?;
    Ok(symmetry)
}

#[inline]
fn calculate_bifurcation_harmonics<T: MeshValue>(x: T) -> Result<f64, MathError> {
    let base_harmonics = x.energy()?.sqrt();
    Ok(base_harmonics * RESONANCE_FACTOR)
}

#[inline]
fn apply_bifurcation_harmonics<T: MeshValue>(x: T, harmonics: f64) -> Result<T, MathError> {
    x.bifurcate(harmonics)
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_bifurcation<T: MeshValue>(x: T, phase: f64) -> Result<T, MathError> {
    let bifurcated_phase = phase * PHASE_BIFURCATION_FACTOR;
    x.phase_bifurcate(bifurcated_phase)
}

/// Quantum symmetry state for square root
#[derive(Debug, Clone, Copy)]
pub struct SymmetryState {
    coherence: f64,
    phase: f64,
    energy: f64,
    symmetry: f64,
}

impl SymmetryState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD &&
        self.symmetry >= QUANTUM_SYMMETRY_THRESHOLD
    }

    #[inline]
    pub fn bifurcate(&self) -> Self {
        Self {
            coherence: self.coherence.sqrt(),
            phase: self.phase / 2.0,
            energy: self.energy.sqrt(),
            symmetry: self.symmetry * RESONANCE_FACTOR,
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
        fn raw_div(&self, other: Self) -> Result<Self, MathError> { Ok(self / other) }
        fn half(&self) -> Result<Self, MathError> { Ok(self / 2.0) }
        fn is_zero(&self) -> bool { *self == 0.0 }
        fn is_negative(&self) -> bool { *self < 0.0 }
        fn zero() -> Self { 0.0 }
        fn bifurcate(&self, factor: f64) -> Result<Self, MathError> {
            Ok(self * factor.sqrt())
        }
        fn phase_bifurcate(&self, phase: f64) -> Result<Self, MathError> {
            Ok(self * (phase * PHASE_BIFURCATION_FACTOR).cos())
        }
        fn symmetry_state(&self) -> Result<SymmetryState, MathError> {
            Ok(SymmetryState {
                coherence: 1.0,
                phase: 0.0,
                energy: *self,
                symmetry: 1.0,
            })
        }
    }

    #[test]
    fn test_quantum_sqrt() {
        assert_eq!(quantum_sqrt(4.0).unwrap(), 2.0);
        assert!(quantum_sqrt(-1.0).is_err());
    }

    #[test]
    fn test_symmetric_sqrt() {
        assert_eq!(symmetric_sqrt(4.0).unwrap(), 2.0);
    }

    #[test]
    fn test_harmonic_sqrt() {
        let result = harmonic_sqrt(4.0).unwrap();
        assert!(result > 2.0); // Due to harmonic bifurcation
    }

    #[test]
    fn test_phase_sqrt() {
        let result = phase_sqrt(4.0, 0.0).unwrap();
        assert_eq!(result, 2.0);

        assert!(phase_sqrt(4.0, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_symmetry_state() {
        let state = SymmetryState {
            coherence: 1.0,
            phase: core::f64::consts::PI,
            energy: 1.0,
            symmetry: 1.0,
        };
        assert!(state.is_stable());

        let bifurcated = state.bifurcate();
        assert_eq!(bifurcated.phase, core::f64::consts::PI / 2.0);
    }

    #[test]
    fn test_convergence() {
        let result = quantum_sqrt(2.0).unwrap();
        assert!((result * result - 2.0).abs() < CONVERGENCE_THRESHOLD);
    }
}
