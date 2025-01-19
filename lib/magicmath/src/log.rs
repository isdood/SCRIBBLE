//! Natural Logarithm Operations for Crystal Lattice HPC Systems
//! ==============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:13:58 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_LOGARITHM_FACTOR,
        QUANTUM_CONTINUITY_THRESHOLD,
        CONVERGENCE_THRESHOLD,
        E
    },
    traits::MeshValue,
};

/// Quantum-aware natural logarithm for crystal lattice values
/// Handles energy compression and phase continuity
pub fn quantum_ln<T: MeshValue>(x: T) -> Result<T, MathError> {
    let coherence = calculate_compression(x)?;
    if coherence < QUANTUM_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_ln(x)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Calculate natural logarithm with quantum continuity preservation
pub fn continuous_ln<T: MeshValue>(x: T) -> Result<T, MathError> {
    let continuity = check_continuity(x)?;
    if !continuity.is_stable() {
        return Err(MathError::ContinuityLoss);
    }

    quantum_ln(x)
}

/// Resonant natural logarithm with harmonic compression
pub fn harmonic_ln<T: MeshValue>(x: T) -> Result<T, MathError> {
    let harmonics = calculate_compression_harmonics(x)?;
    let compressed_result = apply_compression_harmonics(x, harmonics)?;

    quantum_ln(compressed_result)
}

/// Calculate natural logarithm with phase compression
pub fn phase_ln<T: MeshValue>(x: T, phase: f64) -> Result<T, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_compressed = apply_phase_compression(x, phase)?;
    quantum_ln(phase_compressed)
}

// Internal helper functions

#[inline]
fn calculate_compression<T: MeshValue>(x: T) -> Result<f64, MathError> {
    let base_coherence = x.coherence()?.ln();
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_ln<T: MeshValue>(x: T) -> Result<T, MathError> {
    if x.is_negative() || x.is_zero() {
        return Err(MathError::DomainError);
    }

    taylor_ln(x)
}

#[inline]
fn taylor_ln<T: MeshValue>(x: T) -> Result<T, MathError> {
    let mut term = (x.raw_sub(T::unit())?).raw_div(x.raw_add(T::unit())?)?;
    let mut result = term.clone();
    let mut n = T::from(3);

    while term.magnitude()? > CONVERGENCE_THRESHOLD {
        term = term.raw_mul(
            (x.raw_sub(T::unit())?).raw_div(x.raw_add(T::unit())?)?
        )?.raw_mul(T::from((n - T::from(2)).to_f64())?)?
        .raw_div(T::from(n.to_f64())?)?;
        result = result.raw_add(term)?;
        n = n.raw_add(T::from(2))?;
    }

    Ok(result.raw_mul(T::from(2))?)
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
fn check_continuity<T: MeshValue>(x: T) -> Result<ContinuityState, MathError> {
    let continuity = x.continuity_state()?;
    Ok(continuity)
}

#[inline]
fn calculate_compression_harmonics<T: MeshValue>(x: T) -> Result<f64, MathError> {
    let base_harmonics = x.energy()?.ln();
    Ok(base_harmonics * RESONANCE_FACTOR)
}

#[inline]
fn apply_compression_harmonics<T: MeshValue>(x: T, harmonics: f64) -> Result<T, MathError> {
    x.compress(harmonics)
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_compression<T: MeshValue>(x: T, phase: f64) -> Result<T, MathError> {
    let compressed_phase = phase * PHASE_LOGARITHM_FACTOR;
    x.phase_compress(compressed_phase)
}

/// Quantum continuity state for natural logarithm
#[derive(Debug, Clone, Copy)]
pub struct ContinuityState {
    coherence: f64,
    phase: f64,
    energy: f64,
    continuity: f64,
}

impl ContinuityState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD &&
        self.continuity >= QUANTUM_CONTINUITY_THRESHOLD
    }

    #[inline]
    pub fn compress(&self) -> Self {
        Self {
            coherence: self.coherence.ln(),
            phase: self.phase.ln().abs(),
            energy: self.energy.ln(),
            continuity: self.continuity * RESONANCE_FACTOR,
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
        fn is_zero(&self) -> bool { *self == 0.0 }
        fn is_negative(&self) -> bool { *self < 0.0 }
        fn zero() -> Self { 0.0 }
        fn unit() -> Self { 1.0 }
        fn from(value: f64) -> Self { value }
        fn to_f64(&self) -> f64 { *self }
        fn compress(&self, factor: f64) -> Result<Self, MathError> {
            Ok(self * factor.ln())
        }
        fn phase_compress(&self, phase: f64) -> Result<Self, MathError> {
            Ok(self * (phase * PHASE_LOGARITHM_FACTOR).ln())
        }
        fn continuity_state(&self) -> Result<ContinuityState, MathError> {
            Ok(ContinuityState {
                coherence: 1.0,
                phase: 0.0,
                energy: *self,
                continuity: 1.0,
            })
        }
    }

    #[test]
    fn test_quantum_ln() {
        assert_eq!(quantum_ln(E).unwrap(), 1.0);
        assert!(quantum_ln(-1.0).is_err());
        assert!(quantum_ln(0.0).is_err());
    }

    #[test]
    fn test_continuous_ln() {
        assert_eq!(continuous_ln(E).unwrap(), 1.0);
    }

    #[test]
    fn test_harmonic_ln() {
        let result = harmonic_ln(E).unwrap();
        assert!(result < 1.0); // Due to harmonic compression
    }

    #[test]
    fn test_phase_ln() {
        let result = phase_ln(E, 0.0).unwrap();
        assert_eq!(result, 1.0);

        assert!(phase_ln(E, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_continuity_state() {
        let state = ContinuityState {
            coherence: 1.0,
            phase: E,
            energy: 1.0,
            continuity: 1.0,
        };
        assert!(state.is_stable());

        let compressed = state.compress();
        assert_eq!(compressed.phase, 1.0);
    }

    #[test]
    fn test_convergence() {
        let result = quantum_ln(2.0).unwrap();
        assert!((result.exp() - 2.0).abs() < CONVERGENCE_THRESHOLD);
    }
}
