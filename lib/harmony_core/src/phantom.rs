//! Phantom - Quantum Phase Handler
//! =========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 20:41:14 UTC
//! Version: 0.1.1
//! License: MIT

use core::{
    fmt::{self, Display, Formatter, Result as FmtResult},
    result::Result,
};

use magicmath::{
    Vector3D,
    Vector4D,
    resonance::Resonance,
    field::PhaseField,
};

use errors::MathError;

/// A phantom field error
#[derive(Debug)]
pub enum PhaseError {
    /// Invalid quantum state
    InvalidState,
    /// Phase coherence lost
    CoherenceLost,
    /// Field boundary violation
    BoundaryViolation,
}

/// Result type for phantom operations
pub type PhaseResult<T> = Result<T, PhaseError>;

/// A phantom quantum field
#[derive(Debug)]
pub struct PhantomField {
    /// Position in 3D space
    position: Vector3D,
    /// Momentum in 4D space-time
    momentum: Vector4D,
    /// Phase field
    field: PhaseField,
    /// Quantum resonance
    resonance: Resonance,
    /// Energy level
    energy: f64,
}

impl PhantomField {
    /// Create new phantom field
    pub fn new(position: Vector3D, momentum: Vector4D) -> Self {
        Self {
            position,
            momentum,
            field: PhaseField::new(),
            resonance: Resonance::new(),
            energy: 0.0,
        }
    }

    /// Get field position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Get field momentum
    pub fn momentum(&self) -> &Vector4D {
        &self.momentum
    }

    /// Get field energy
    pub fn energy(&self) -> Result<f64, MathError> {
        Ok(self.energy)
    }

    /// Set field energy
    pub fn set_energy(&mut self, value: f64) -> PhaseResult<()> {
        if value < 0.0 {
            return Err(PhaseError::InvalidState);
        }
        self.energy = value;
        Ok(())
    }

    /// Apply phase shift
    pub fn apply_phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        self.field.phase_shift(shift)
    }

    /// Check if field is stable
    pub fn is_stable(&self) -> bool {
        self.energy >= 0.0
    }
}

impl Display for PhantomField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Phantom Field:")?;
        writeln!(f, "Position: {:?}", self.position)?;
        writeln!(f, "Momentum: {:?}", self.momentum)?;
        writeln!(f, "Phase: {:?}", self.field)?;
        writeln!(f, "Resonance: {:?}", self.resonance)?;
        write!(f, "Energy: {}", self.energy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phantom_field() {
        let pos = Vector3D::new(1.0, 0.0, 0.0);
        let mom = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        let field = PhantomField::new(pos, mom);

        assert!(field.is_stable());
        assert_eq!(field.energy().unwrap(), 0.0);
    }

    #[test]
    fn test_energy_limits() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let mom = Vector4D::new(0.0, 0.0, 0.0, 0.0);
        let mut field = PhantomField::new(pos, mom);

        assert!(field.set_energy(1.0).is_ok());
        assert!(field.set_energy(-1.0).is_err());
    }

    #[test]
    fn test_phase_shift() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let mom = Vector4D::new(0.0, 0.0, 0.0, 0.0);
        let mut field = PhantomField::new(pos, mom);

        assert!(field.apply_phase_shift(0.5).is_ok());
    }

    #[test]
    fn test_field_comparison() {
        let pos = Vector3D::new(1.0, 0.0, 0.0);
        let mom = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        let field = PhantomField::new(pos.clone(), mom.clone());

        // Compare using Debug format since Vector types don't implement PartialEq
        assert_eq!(format!("{:?}", field.position()), format!("{:?}", &pos));
        assert_eq!(format!("{:?}", field.momentum()), format!("{:?}", &mom));
    }
}
