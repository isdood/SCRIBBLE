//! Native Division operations for Crystal Lattice HPC Systems
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:30:59 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use crate::core::HarmonyState;
use errors::MathError;
use crate::constants::{
    HARMONY_STABILITY_THRESHOLD,
    RESONANCE_FACTOR,
    PHASE_ATTENUATION_FACTOR,
    HARMONY_COHERENCE_THRESHOLD,
    HARMONY_ENERGY_THRESHOLD,
    SINGULARITY_THRESHOLD
};

/// Native implementation of harmony-aware division
#[derive(Debug, Clone)]
pub struct HarmonyDiv<T: MeshValue> {
    pub value: T,
    pub state: HarmonyState,
}

impl<T: MeshValue> HarmonyDiv<T> {
    /// Creates a new HarmonyDiv instance
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            state: HarmonyState::new(),
        }
    }

    /// Performs harmony-aware division
    #[inline]
    pub fn div(&self, rhs: &Self) -> Result<Self, MathError> {
        // Check for division by zero using singularity threshold
        if rhs.value.to_f64()?.abs() < SINGULARITY_THRESHOLD {
            return Err(MathError::DivisionByZero);
        }

        if self.state.coherence >= HARMONY_STABILITY_THRESHOLD &&
            rhs.state.coherence >= HARMONY_COHERENCE_THRESHOLD &&
            self.state.energy >= HARMONY_ENERGY_THRESHOLD {

                let new_value = self.value.div(&rhs.value)?;
                let new_phase = (self.state.phase / rhs.state.phase) * PHASE_ATTENUATION_FACTOR;

                Ok(Self {
                    value: new_value,
                    state: HarmonyState {
                        coherence: (self.state.coherence / rhs.state.coherence).sqrt() * RESONANCE_FACTOR,
                   phase: new_phase,
                   energy: self.state.energy / rhs.state.energy,
                   stability: (self.state.stability / rhs.state.stability).sqrt(),
                   iterations: self.state.iterations + 1,
                    },
                })
            } else {
                Err(MathError::UnstableState("Division operation failed: harmony state unstable or energy too low".to_string()))
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

    /// Checks if division would maintain harmony
    #[inline]
    pub fn would_maintain_harmony(&self, rhs: &Self) -> Result<bool, MathError> {
        let rhs_magnitude = rhs.value.to_f64()?.abs();
        Ok(
            self.state.coherence >= HARMONY_STABILITY_THRESHOLD &&
            rhs.state.coherence >= HARMONY_COHERENCE_THRESHOLD &&
            self.state.energy >= HARMONY_ENERGY_THRESHOLD &&
            rhs_magnitude >= SINGULARITY_THRESHOLD
        )
    }

    /// Checks for potential singularities in the operation
    #[inline]
    pub fn check_singularity(&self, rhs: &Self) -> Result<bool, MathError> {
        Ok(rhs.value.to_f64()?.abs() < SINGULARITY_THRESHOLD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmony_div_f64() {
        let a = HarmonyDiv::new(6.0f64);
        let b = HarmonyDiv::new(2.0f64);
        let result = a.div(&b).unwrap();
        assert_eq!(result.value.to_f64().unwrap(), 3.0);
        assert!(result.state.coherence < 1.0);
        assert!(result.state.stability > 0.0);
    }

    #[test]
    fn test_harmony_div_by_zero() {
        let a = HarmonyDiv::new(5.0f64);
        let b = HarmonyDiv::new(0.0f64);
        assert!(matches!(a.div(&b), Err(MathError::DivisionByZero)));
    }

    #[test]
    fn test_harmony_div_near_zero() {
        let a = HarmonyDiv::new(5.0f64);
        let b = HarmonyDiv::new(SINGULARITY_THRESHOLD / 2.0);
        assert!(a.div(&b).is_err());
    }

    #[test]
    fn test_harmony_div_phase_attenuation() {
        let mut a = HarmonyDiv::new(6.0f64);
        let mut b = HarmonyDiv::new(2.0f64);
        a.state.phase = 0.6;
        b.state.phase = 0.2;
        let result = a.div(&b).unwrap();
        assert!(result.state.phase < a.state.phase);
    }

    #[test]
    fn test_singularity_check() {
        let a = HarmonyDiv::new(5.0f64);
        let b = HarmonyDiv::new(SINGULARITY_THRESHOLD / 2.0);
        assert!(a.check_singularity(&b).unwrap());

        let c = HarmonyDiv::new(1.0f64);
        assert!(!a.check_singularity(&c).unwrap());
    }

    #[test]
    fn test_coherence_decay() {
        let a = HarmonyDiv::new(6.0f64);
        let b = HarmonyDiv::new(2.0f64);
        let result = a.div(&b).unwrap();
        assert!(result.state.coherence < a.state.coherence);
    }
}
