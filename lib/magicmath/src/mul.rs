//! Native Multiplication operations for Crystal Lattice HPC Systems
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:30:01 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use crate::core::HarmonyState;
use errors::MathError;
use crate::constants::{
    HARMONY_STABILITY_THRESHOLD,
    RESONANCE_FACTOR,
    PHASE_AMPLIFICATION_FACTOR,
    HARMONY_COHERENCE_THRESHOLD,
    HARMONY_ENERGY_THRESHOLD
};

/// Native implementation of harmony-aware multiplication
#[derive(Debug, Clone)]
pub struct HarmonyMul<T: MeshValue> {
    pub value: T,
    pub state: HarmonyState,
}

impl<T: MeshValue> HarmonyMul<T> {
    /// Creates a new HarmonyMul instance
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            state: HarmonyState::new(),
        }
    }

    /// Performs harmony-aware multiplication
    #[inline]
    pub fn mul(&self, rhs: &Self) -> Result<Self, MathError> {
        if self.state.coherence >= HARMONY_STABILITY_THRESHOLD &&
            rhs.state.coherence >= HARMONY_COHERENCE_THRESHOLD &&
            self.state.energy * rhs.state.energy >= HARMONY_ENERGY_THRESHOLD {

                let new_value = self.value.mul(&rhs.value)?;
                let new_phase = (self.state.phase * rhs.state.phase) * PHASE_AMPLIFICATION_FACTOR;

                Ok(Self {
                    value: new_value,
                    state: HarmonyState {
                        coherence: (self.state.coherence * rhs.state.coherence).sqrt() * RESONANCE_FACTOR,
                   phase: new_phase,
                   energy: self.state.energy * rhs.state.energy,
                   stability: (self.state.stability * rhs.state.stability).sqrt(),
                   iterations: self.state.iterations + 1,
                    },
                })
            } else {
                Err(MathError::UnstableState("Multiplication operation failed: harmony state unstable or energy too low".to_string()))
            }
    }

    /// Gets the value
    #[inline]
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// Gets the harmony state
    #[inline]
    pub fn get_state(&self) -> &HarmonyState {
        &self.state
    }

    /// Checks if multiplication would maintain harmony
    #[inline]
    pub fn would_maintain_harmony(&self, rhs: &Self) -> bool {
        self.state.coherence >= HARMONY_STABILITY_THRESHOLD &&
        rhs.state.coherence >= HARMONY_COHERENCE_THRESHOLD &&
        self.state.energy * rhs.state.energy >= HARMONY_ENERGY_THRESHOLD
    }

    /// Gets the projected energy level after multiplication
    #[inline]
    pub fn projected_energy(&self, rhs: &Self) -> f64 {
        self.state.energy * rhs.state.energy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmony_mul_f64() {
        let a = HarmonyMul::new(5.0f64);
        let b = HarmonyMul::new(3.0f64);
        let result = a.mul(&b).unwrap();
        assert_eq!(result.value.to_f64().unwrap(), 15.0);
        assert!(result.state.coherence < 1.0);
        assert!(result.state.stability > 0.0);
    }

    #[test]
    fn test_harmony_mul_failure() {
        let mut a = HarmonyMul::new(5.0f64);
        a.state.coherence = 0.0;
        let b = HarmonyMul::new(3.0f64);
        assert!(a.mul(&b).is_err());
    }

    #[test]
    fn test_harmony_mul_phase_amplification() {
        let mut a = HarmonyMul::new(2.0f64);
        let mut b = HarmonyMul::new(3.0f64);
        a.state.phase = 0.5;
        b.state.phase = 0.3;
        let result = a.mul(&b).unwrap();
        assert!(result.state.phase > a.state.phase * b.state.phase);
    }

    #[test]
    fn test_harmony_mul_energy_threshold() {
        let mut a = HarmonyMul::new(2.0f64);
        let mut b = HarmonyMul::new(3.0f64);
        a.state.energy = 0.0;
        assert!(a.mul(&b).is_err());
    }

    #[test]
    fn test_projected_energy() {
        let a = HarmonyMul::new(2.0f64);
        let b = HarmonyMul::new(3.0f64);
        assert_eq!(a.projected_energy(&b), a.state.energy * b.state.energy);
    }

    #[test]
    fn test_coherence_decay() {
        let a = HarmonyMul::new(2.0f64);
        let b = HarmonyMul::new(3.0f64);
        let result = a.mul(&b).unwrap();
        assert!(result.state.coherence < a.state.coherence);
    }
}
