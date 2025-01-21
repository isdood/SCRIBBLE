//! Alignment State Management
//! =======================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-21 00:24:28 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::Vector3D;
use core::fmt::{Display, Formatter, Result as FmtResult};

/// Represents an alignment state
#[derive(Debug, Clone, Copy)]
pub enum AlignmentState {
    Stable,
    Unstable,
}

/// Represents the alignment
#[derive(Debug, Clone)]
pub struct Alignment {
    position: Vector3D,
    coherence: f64,
}

impl Alignment {
    /// Create a new alignment with a specific position
    pub fn new(position: Vector3D) -> Self {
        Self {
            position,
            coherence: 0.0,
        }
    }

    /// Get the current position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Set the position
    pub fn set_position(&mut self, position: Vector3D) {
        self.position = position;
    }

    /// Get the current coherence value
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Set the coherence value
    pub fn set_coherence(&mut self, value: f64) {
        self.coherence = value.clamp(0.0, 1.0);
    }

    /// Get the current alignment state
    pub fn state(&self) -> AlignmentState {
        if self.coherence >= 0.5 {
            AlignmentState::Stable
        } else {
            AlignmentState::Unstable
        }
    }

    /// Check if the alignment is stable
    pub fn is_stable(&self) -> bool {
        matches!(self.state(), AlignmentState::Stable)
    }
}

impl Display for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Alignment(pos: {:?}, coherence: {})", self.position, self.coherence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_creation() {
        let pos = Vector3D::new(1.0, 0.0, 0.0);
        let alignment = Alignment::new(pos);
        assert_eq!(alignment.coherence(), 0.0);
        assert!(!alignment.is_stable());
    }

    #[test]
    fn test_alignment_value() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let mut alignment = Alignment::new(pos);
        alignment.set_coherence(0.7);
        assert_eq!(alignment.coherence(), 0.7);
    }

    #[test]
    fn test_alignment_stability() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let mut alignment = Alignment::new(pos);

        // Initially unstable
        assert!(!alignment.is_stable());
        assert!(matches!(alignment.state(), AlignmentState::Unstable));

        // Set to mid-range - should be stable
        alignment.set_coherence(0.5);
        assert!(alignment.is_stable());
        assert!(matches!(alignment.state(), AlignmentState::Stable));

        // Set to maximum - should be stable
        alignment.set_coherence(1.0);
        assert!(alignment.is_stable());
        assert!(matches!(alignment.state(), AlignmentState::Stable));

        // Set to minimum - should be unstable
        alignment.set_coherence(0.0);
        assert!(!alignment.is_stable());
        assert!(matches!(alignment.state(), AlignmentState::Unstable));
    }
}
