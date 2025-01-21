//! Alignment Module
//! =================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 21:19:08 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::{
    MeshValue,
    Vector3D,
};

use errors::{MathError};

use core::{
    fmt::{self, Display, Formatter, Result as FmtResult},
    result::Result,
};

use magicmath::constants::{
    HARMONY_RESONANCE_THRESHOLD,
};

/// Alignment state for quantum nodes
#[derive(Debug, Clone)]
pub struct Alignment {
    /// Position in 3D space
    position: Vector3D,
    /// Alignment value
    alignment_value: f64,
}

impl Alignment {
    /// Create a new alignment
    pub fn new(position: Vector3D) -> Self {
        Self {
            position,
            alignment_value: 0.0,
        }
    }

    /// Get alignment value
    pub fn value(&self) -> f64 {
        self.alignment_value
    }

    /// Set alignment value
    pub fn set_value(&mut self, value: f64) -> Result<(), MathError> {
        if value < 0.0 || value > 1.0 {
            return Err(MathError::InvalidRange);
        }
        self.alignment_value = value;
        Ok(())
    }

    /// Get position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Check if alignment is stable
    pub fn is_stable(&self) -> bool {
        self.alignment_value >= HARMONY_RESONANCE_THRESHOLD
    }

    /// Get current state
    pub fn state(&self) -> AlignmentState {
        if self.is_stable() {
            AlignmentState::Stable
        } else {
            AlignmentState::Unstable
        }
    }
}

/// Alignment state types
#[derive(Debug, PartialEq)]
pub enum AlignmentState {
    /// Stable alignment
    Stable,
    /// Unstable alignment
    Unstable,
}

impl Display for AlignmentState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AlignmentState::Stable => write!(f, "Stable"),
            AlignmentState::Unstable => write!(f, "Unstable"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_creation() {
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        let alignment = Alignment::new(pos.clone());
        assert_eq!(alignment.value(), 0.0);
        assert_eq!(alignment.position(), &pos);
    }

    #[test]
    fn test_alignment_value() {
        let mut alignment = Alignment::new(Vector3D::new(0.0, 0.0, 0.0));
        assert!(alignment.set_value(0.5).is_ok());
        assert_eq!(alignment.value(), 0.5);
        assert!(alignment.set_value(-0.1).is_err());
        assert!(alignment.set_value(1.1).is_err());
    }

    #[test]
    fn test_alignment_stability() {
        let mut alignment = Alignment::new(Vector3D::new(0.0, 0.0, 0.0));
        alignment.set_value(0.9).unwrap();
        assert!(alignment.is_stable());
        alignment.set_value(0.1).unwrap();
        assert!(!alignment.is_stable());
    }
}
