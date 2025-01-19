//! Native Subtraction operations for Crystal Lattice HPC Systems
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:20:00 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use crate::core::HarmonyState;
use crate::constants::{
    HARMONY_STABILITY_THRESHOLD,
    RESONANCE_FACTOR,
    PHASE_COUPLING_CONSTANT,
    HARMONY_COHERENCE_THRESHOLD
};

/// Native implementation of harmony-aware subtraction
#[derive(Debug, Clone)]
pub struct HarmonySub<T: MeshValue> {
    pub value: T,
    pub state: HarmonyState,
}

impl<T: MeshValue> HarmonySub<T> {
    /// Creates a new HarmonySub instance
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            state: HarmonyState::new(),
        }
    }

    /// Performs harmony-aware subtraction
    #[inline]
    pub fn sub(&self, rhs: &Self) -> Result<Self, String> {
        if self.state.coherence >= HARMONY_STABILITY_THRESHOLD &&
            rhs.state.coherence >= HARMONY_COHERENCE_THRESHOLD {
                let new_value = self.value.sub(&rhs.value)?;
                let new_phase = (self.state.phase - rhs.state.phase) * PHASE_COUPLING_CONSTANT;

                Ok(Self {
                    value: new_value,
                    state: HarmonyState {
                        coherence: self.state.coherence * RESONANCE_FACTOR,
                        phase: new_phase,
                        energy: (self.state.energy - rhs.state.energy).abs(), // Energy difference must be positive
                   stability: self.state.stability * RESONANCE_FACTOR,
                   iterations: self.state.iterations + 1,
                    },
                })
            } else {
                Err("Subtraction operation failed: harmony state unstable".to_string())
            }
    }

    /// Checks if the subtraction would maintain harmony
    #[inline]
    pub fn would_maintain_harmony(&self, rhs: &Self) -> bool {
        self.state.coherence >= HARMONY_STABILITY_THRESHOLD &&
        rhs.state.coherence >= HARMONY_COHERENCE_THRESHOLD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmony_sub() {
        let a = HarmonySub::new(5.0f64);
        let b = HarmonySub::new(3.0f64);
        let result = a.sub(&b).unwrap();
        assert_eq!(result.value, 2.0);
        assert!(result.state.coherence < 1.0);
        assert!(result.state.stability > 0.0);
    }

    #[test]
    fn test_harmony_sub_failure() {
        let mut a = HarmonySub::new(5.0f64);
        a.state.coherence = 0.0;
        let b = HarmonySub::new(3.0f64);
        assert!(a.sub(&b).is_err());
    }

    #[test]
    fn test_harmony_sub_phase_coupling() {
        let mut a = HarmonySub::new(5.0f64);
        let mut b = HarmonySub::new(3.0f64);
        a.state.phase = 0.5;
        b.state.phase = 0.3;
        let result = a.sub(&b).unwrap();
        assert!(result.state.phase < a.state.phase);
    }

    #[test]
    fn test_harmony_maintenance_check() {
        let a = HarmonySub::new(5.0f64);
        let b = HarmonySub::new(3.0f64);
        assert!(a.would_maintain_harmony(&b));

        let mut unstable = HarmonySub::new(1.0f64);
        unstable.state.coherence = 0.0;
        assert!(!a.would_maintain_harmony(&unstable));
    }
}
