//! Multiplication Operations for Crystal Lattice HPC Systems
//! =============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:09:55 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_AMPLIFICATION_FACTOR,
        QUANTUM_ENTANGLEMENT_THRESHOLD
    },
    traits::MeshValue,
};

/// Quantum-aware multiplication for crystal lattice values
/// Handles energy amplification and phase coupling
pub fn quantum_mul<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let coherence = calculate_amplification(a, b)?;
    if coherence < QUANTUM_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_mul(a, b)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Multiply values with quantum entanglement preservation
pub fn entangled_mul<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let entanglement = check_entanglement(a, b)?;
    if !entanglement.is_stable() {
        return Err(MathError::EntanglementLoss);
    }

    quantum_mul(a, b)
}

/// Resonant multiplication with harmonic amplification
pub fn harmonic_mul<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    let harmonics = calculate_harmonics(a, b)?;
    let amplified_result = apply_harmonics(a, b, harmonics)?;

    quantum_mul(amplified_result, T::unit())
}

/// Multiply lattice values with phase amplification
pub fn phase_mul<T: MeshValue>(a: T, b: T, phase: f64) -> Result<T, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_amplified = apply_phase_amplification(a, b, phase)?;
    quantum_mul(phase_amplified.0, phase_amplified.1)
}

// Internal helper functions

#[inline]
fn calculate_amplification<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_coherence = (a.coherence()? * b.coherence()?).sqrt();
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_mul<T: MeshValue>(a: T, b: T) -> Result<T, MathError> {
    if would_exceed_bounds(a, b)? {
        return Err(MathError::LatticeOverflow);
    }
    Ok(a.raw_mul(b))
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
fn would_exceed_bounds<T: MeshValue>(a: T, b: T) -> Result<bool, MathError> {
    let product_energy = a.energy()? * b.energy()?;
    Ok(product_energy > MAX_LATTICE_SIZE as f64)
}

#[inline]
fn check_entanglement<T: MeshValue>(a: T, b: T) -> Result<EntanglementState, MathError> {
    let entanglement = a.entanglement_state()?.combine(b.entanglement_state()?);
    Ok(entanglement)
}

#[inline]
fn calculate_harmonics<T: MeshValue>(a: T, b: T) -> Result<f64, MathError> {
    let base_harmonics = (a.energy()? * b.energy()?).sqrt();
    Ok(base_harmonics * RESONANCE_FACTOR)
}

#[inline]
fn apply_harmonics<T: MeshValue>(a: T, b: T, harmonics: f64) -> Result<T, MathError> {
    let amplified_a = a.amplify(harmonics)?;
    let amplified_b = b.amplify(harmonics)?;
    Ok(amplified_a.raw_mul(amplified_b))
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_amplification<T: MeshValue>(a: T, b: T, phase: f64) -> Result<(T, T), MathError> {
    let amplified_phase = phase * PHASE_AMPLIFICATION_FACTOR;
    let phase_a = a.phase_amplify(amplified_phase)?;
    let phase_b = b.phase_amplify(amplified_phase)?;
    Ok((phase_a, phase_b))
}

/// Quantum entanglement state for multiplication
#[derive(Debug, Clone, Copy)]
pub struct EntanglementState {
    coherence: f64,
    phase: f64,
    energy: f64,
    entanglement: f64,
}

impl EntanglementState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD &&
        self.entanglement >= QUANTUM_ENTANGLEMENT_THRESHOLD
    }

    #[inline]
    pub fn combine(&self, other: EntanglementState) -> Self {
        Self {
            coherence: (self.coherence * other.coherence).sqrt(),
            phase: self.phase * other.phase,
            energy: self.energy * other.energy,
            entanglement: (self.entanglement + other.entanglement) * RESONANCE_FACTOR,
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
        fn raw_mul(&self, other: Self) -> Self { self * other }
        fn amplify(&self, factor: f64) -> Result<Self, MathError> { Ok(self * factor.sqrt()) }
        fn phase_amplify(&self, phase: f64) -> Result<Self, MathError> {
            Ok(self * (phase * PHASE_AMPLIFICATION_FACTOR).cos())
        }
        fn entanglement_state(&self) -> Result<EntanglementState, MathError> {
            Ok(EntanglementState {
                coherence: 1.0,
                phase: 0.0,
                energy: *self,
                entanglement: 1.0,
            })
        }
        fn unit() -> Self { 1.0 }
    }

    #[test]
    fn test_quantum_mul() {
        assert_eq!(quantum_mul(2.0, 3.0).unwrap(), 6.0);
        assert!(quantum_mul(MAX_LATTICE_SIZE as f64, 2.0).is_err());
    }

    #[test]
    fn test_entangled_mul() {
        assert_eq!(entangled_mul(2.0, 3.0).unwrap(), 6.0);
    }

    #[test]
    fn test_harmonic_mul() {
        let result = harmonic_mul(2.0, 2.0).unwrap();
        assert!(result > 4.0); // Due to harmonic amplification
    }

    #[test]
    fn test_phase_mul() {
        let result = phase_mul(2.0, 2.0, 0.0).unwrap();
        assert_eq!(result, 4.0);

        assert!(phase_mul(2.0, 2.0, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_entanglement_state() {
        let state = EntanglementState {
            coherence: 1.0,
            phase: 0.0,
            energy: 1.0,
            entanglement: 1.0,
        };
        assert!(state.is_stable());

        let combined = state.combine(state);
        assert!(combined.entanglement > 1.0);
    }
}
