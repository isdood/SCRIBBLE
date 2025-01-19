//! Resonance Mathematics Implementation for Crystal Lattice Systems
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 18:56:20 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use errors::MathError;
use scribe::native_string::String;

/// Resonance mathematics operations and state management
#[derive(Debug, Clone)]
pub struct ResonanceMath {
    state: ResonanceState,
}

/// Resonance state for crystal lattice operations
#[derive(Debug, Clone, Copy)]
pub struct ResonanceState {
    pub harmony: f64,
    pub phase: f64,
    pub resonance: f64,
}

impl ResonanceState {
    /// Create new resonance state
    pub fn new() -> Self {
        Self {
            harmony: 1.0,
            phase: 0.0,
            resonance: 0.0,
        }
    }

    /// Check if resonance state is stable
    pub fn is_stable(&self) -> bool {
        self.harmony >= 0.5 && self.resonance >= 0.0
    }
}

impl Default for ResonanceState {
    fn default() -> Self {
        Self::new()
    }
}

impl ResonanceMath {
    /// Create new resonance math instance
    pub fn new() -> Self {
        Self {
            state: ResonanceState::new(),
        }
    }

    /// Get current resonance state
    pub fn get_state(&self) -> ResonanceState {
        self.state
    }

    /// Perform resonance operation
    pub fn operate<T: MeshValue>(&mut self, value: T) -> Result<T, MathError> {
        if !self.state.is_stable() {
            return Err(MathError::HarmonyStateUnstable(
                String::from("Crystal resonance state unstable during operation")
            ));
        }

        // Apply resonance transformation
        let result = value.to_f64()?;
        let transformed = result * self.state.harmony *
        (1.0 + self.state.resonance);

        Ok(T::from(transformed))
    }
}

impl Default for ResonanceMath {
    fn default() -> Self {
        Self::new()
    }
}
