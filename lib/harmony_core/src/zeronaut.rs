//! Zeronaut - Zero Point Energy Handler
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 20:36:33 UTC
//! Version: 0.1.1
//! License: MIT

use core::{
    fmt::{self, Display, Write},
    result::Result,
};

use magicmath::{
    MeshValue,
    Vector3D,
    resonance::Resonance,
};

use errors::MathError;

/// Zero point energy state
#[derive(Debug)]
pub struct ZeroState {
    /// Position in 3D space
    position: Vector3D,
    /// Energy level
    energy: f64,
    /// Quantum resonance
    resonance: Resonance,
}

impl ZeroState {
    /// Create new zero point state
    pub fn new(position: Vector3D) -> Self {
        Self {
            position,
            energy: 0.0,
            resonance: Resonance::new(),
        }
    }

    /// Get current position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Get current energy level
    pub fn energy(&self) -> f64 {
        self.energy
    }

    /// Get current resonance
    pub fn resonance(&self) -> &Resonance {
        &self.resonance
    }

    /// Set energy level
    pub fn set_energy(&mut self, energy: f64) -> Result<(), MathError> {
        if energy < 0.0 {
            return Err(MathError::NegativeEnergy);
        }
        self.energy = energy;
        Ok(())
    }

    /// Apply resonance shift
    pub fn apply_resonance(&mut self, shift: f64) -> Result<(), MathError> {
        self.resonance.phase_shift(shift)
    }

    /// Check if state is stable
    pub fn is_stable(&self) -> bool {
        self.energy >= 0.0
    }
}

impl Display for ZeroState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Zero Point State:")?;
        writeln!(f, "Position: {:?}", self.position)?;
        writeln!(f, "Energy: {}", self.energy)?;
        write!(f, "Resonance: {:?}", self.resonance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_state() {
        let pos = Vector3D::new(1.0, 0.0, 0.0);
        let state = ZeroState::new(pos.clone());

        // Use debug format for comparison since Vector3D doesn't implement PartialEq
        assert_eq!(format!("{:?}", state.position()), format!("{:?}", &pos));
        assert_eq!(state.energy(), 0.0);
        assert!(state.is_stable());
    }

    #[test]
    fn test_energy_limits() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let mut state = ZeroState::new(pos);

        assert!(state.set_energy(1.0).is_ok());
        assert!(state.set_energy(-1.0).is_err());
    }

    #[test]
    fn test_resonance() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let mut state = ZeroState::new(pos);

        assert!(state.apply_resonance(0.5).is_ok());
    }

    #[test]
    fn test_state_stability() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let mut state = ZeroState::new(pos);

        assert!(state.is_stable());
        assert!(state.set_energy(0.0).is_ok());
        assert!(state.is_stable());
    }
}
