//! Native Addition operations for Crystal Lattice HPC Systems
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:28:00 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use crate::core::HarmonyState;
use errors::MathError;
use crate::constants::{
    HARMONY_STABILITY_THRESHOLD,
    RESONANCE_FACTOR,
    PHASE_COUPLING_CONSTANT
};

/// Native implementation of harmony-aware addition
#[derive(Debug, Clone)]
pub struct HarmonyAdd<T: MeshValue> {
    pub value: T,
    pub state: HarmonyState,
}

impl<T: MeshValue> HarmonyAdd<T> {
    /// Creates a new HarmonyAdd instance
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            state: HarmonyState::new(),
        }
    }

    /// Performs harmony-aware addition
    #[inline]
    pub fn add(&self, rhs: &Self) -> Result<Self, MathError> {
        if self.state.coherence >= HARMONY_STABILITY_THRESHOLD {
            let new_value = self.value.add(&rhs.value)?;
            let new_phase = (self.state.phase + rhs.state.phase) * PHASE_COUPLING_CONSTANT;

            Ok(Self {
                value: new_value,
                state: HarmonyState {
                    coherence: self.state.coherence * RESONANCE_FACTOR,
                    phase: new_phase,
                    energy: self.state.energy + rhs.state.energy,
                    stability: self.state.stability * RESONANCE_FACTOR,
                    iterations: self.state.iterations + 1,
                },
            })
        } else {
            Err(MathError::UnstableState("Addition operation failed: harmony state unstable".to_string()))
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

    /// Checks if the operation would maintain harmony
    #[inline]
    pub fn would_maintain_harmony(&self, rhs: &Self) -> bool {
        self.state.coherence >= HARMONY_STABILITY_THRESHOLD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmony_add_f64() {
        let a = HarmonyAdd::new(5.0f64);
        let b = HarmonyAdd::new(3.0f64);
        let result = a.add(&b).unwrap();
        assert_eq!(result.value.to_f64().unwrap(), 8.0);
        assert!(result.state.coherence < 1.0);
        assert!(result.state.stability > 0.0);
    }

    #[test]
    fn test_harmony_add_failure() {
        let mut a = HarmonyAdd::new(5.0f64);
        a.state.coherence = 0.0;
        let b = HarmonyAdd::new(3.0f64);
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn test_harmony_add_phase_coupling() {
        let mut a = HarmonyAdd::new(2.0f64);
        let mut b = HarmonyAdd::new(3.0f64);
        a.state.phase = 0.5;
        b.state.phase = 0.3;
        let result = a.add(&b).unwrap();
        assert!(result.state.phase < a.state.phase + b.state.phase);
    }

    #[test]
    fn test_harmony_maintenance_check() {
        let a = HarmonyAdd::new(5.0f64);
        let b = HarmonyAdd::new(3.0f64);
        assert!(a.would_maintain_harmony(&b));

        let mut unstable = HarmonyAdd::new(1.0f64);
        unstable.state.coherence = 0.0;
        assert!(!unstable.would_maintain_harmony(&a));
    }
}
