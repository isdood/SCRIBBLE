//! Phantom - Quantum Phase Handler
//! =========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 20:47:12 UTC
//! Version: 0.1.1
//! License: MIT

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result,
};

use magicmath::{
    Vector3D,
    Vector4D,
    resonance::Resonance,
};

use errors::MathError;

/// A phantom state error type
#[derive(Debug)]
pub enum PhantomError {
    /// Invalid quantum state
    InvalidState,
    /// Phase coherence lost
    CoherenceLost,
    /// Field boundary violation
    BoundaryViolation,
}

/// Result type for phantom operations
pub type PhantomResult<T> = Result<T, PhantomError>;

/// A quantum phase state
#[derive(Debug)]
pub struct PhantomState {
    /// Position in 3D space
    position: Vector3D,
    /// Momentum in 4D space-time
    momentum: Vector4D,
    /// Quantum resonance
    resonance: Resonance,
    /// Phase coherence
    coherence: f64,
}

impl PhantomState {
    /// Create new phantom state
    pub fn new(position: Vector3D, momentum: Vector4D) -> Self {
        Self {
            position,
            momentum,
            resonance: Resonance::new(),
            coherence: 1.0,
        }
    }

    /// Get position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Get momentum
    pub fn momentum(&self) -> &Vector4D {
        &self.momentum
    }

    /// Get coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Set coherence level
    pub fn set_coherence(&mut self, value: f64) -> PhantomResult<()> {
        if value < 0.0 || value > 1.0 {
            return Err(PhantomError::InvalidState);
        }
        self.coherence = value;
        Ok(())
    }

    /// Apply phase shift
    pub fn apply_phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        self.resonance.phase_shift(shift)
    }

    /// Check if state is coherent
    pub fn is_coherent(&self) -> bool {
        self.coherence > 0.5
    }

    /// Calculate total energy
    pub fn energy(&self) -> Result<f64, MathError> {
        let pos_mag = self.position.magnitude()?;
        let mom_mag = self.momentum.magnitude()?;
        Ok(pos_mag * pos_mag + mom_mag * mom_mag)
    }
}

impl Display for PhantomState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Phantom State:")?;
        writeln!(f, "Position: {:?}", self.position)?;
        writeln!(f, "Momentum: {:?}", self.momentum)?;
        writeln!(f, "Coherence: {}", self.coherence)?;
        write!(f, "Resonance: {:?}", self.resonance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_state() -> PhantomState {
        PhantomState::new(
            Vector3D::new(1.0, 0.0, 0.0),
                          Vector4D::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[test]
    fn test_phantom_creation() {
        let pos = Vector3D::new(1.0, 0.0, 0.0);
        let mom = Vector4D::new(0.0, 0.0, 0.0, 1.0);
        let state = PhantomState::new(pos, mom);

        assert_eq!(state.coherence(), 1.0);
        assert!(state.is_coherent());
    }

    #[test]
    fn test_coherence_limits() {
        let mut state = create_test_state();

        assert!(state.set_coherence(0.5).is_ok());
        assert!(state.set_coherence(-0.1).is_err());
        assert!(state.set_coherence(1.1).is_err());
    }

    #[test]
    fn test_phase_shift() {
        let mut state = create_test_state();
        assert!(state.apply_phase_shift(0.5).is_ok());
    }

    #[test]
    fn test_state_energy() {
        let state = create_test_state();
        let energy = state.energy().unwrap();
        assert!(energy >= 0.0);
    }

    #[test]
    fn test_vector_debug() {
        let pos = Vector3D::new(1.0, 0.0, 0.0);
        let mom = Vector4D::new(0.0, 0.0, 0.0, 1.0);
        let state = PhantomState::new(pos.clone(), mom.clone());

        assert_eq!(format!("{:?}", state.position()), format!("{:?}", &pos));
        assert_eq!(format!("{:?}", state.momentum()), format!("{:?}", &mom));
    }
}
