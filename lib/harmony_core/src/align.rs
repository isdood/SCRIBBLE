//! Alignment Operations for Crystal Computing
//! =====================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:35:20 UTC
//! Version: 0.1.0
//! License: MIT

use meshmath::sqrt;
use crate::{
    vector::{Vector3D, Vector4D},
    crystal::CrystalCube,
    phantom::Phantom,
    errors::QuantumError,
    constants::{QUANTUM_STABILITY_THRESHOLD, ALIGNMENT_THRESHOLD},
};

/// Alignment state for quantum phase coherence
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignmentState {
    /// Perfect alignment achieved
    Perfect,
    /// Partial alignment with given coherence factor
    Partial(f64),
    /// Misaligned state
    Misaligned,
    /// Unknown alignment state
    Unknown,
}

/// Result type for alignment operations
pub type AlignmentResult<T> = Result<T, QuantumError>;

/// Alignment calculations for crystal operations
#[derive(Debug)]
pub struct Alignment {
    /// Current alignment state
    state: AlignmentState,
    /// Reference vector for alignment
    reference: Vector3D<f64>,
    /// Phantom state for quantum alignment
    phantom: Phantom,
}

impl Alignment {
    /// Create new alignment calculator
    pub fn new(reference: Vector3D<f64>) -> Self {
        Self {
            state: AlignmentState::Unknown,
            reference,
            phantom: Phantom::new(),
        }
    }

    /// Calculate alignment with target vector
    pub fn align_with(&mut self, target: &Vector3D<f64>) -> AlignmentResult<AlignmentState> {
        let dot = self.reference.dot(target)?;
        let mag_ref = self.reference.magnitude()?;
        let mag_target = target.magnitude()?;

        let coherence = dot / (mag_ref * mag_target);

        self.state = if coherence >= ALIGNMENT_THRESHOLD {
            AlignmentState::Perfect
        } else if coherence >= QUANTUM_STABILITY_THRESHOLD {
            AlignmentState::Partial(coherence)
        } else {
            AlignmentState::Misaligned
        };

        Ok(self.state)
    }

    /// Calculate quantum alignment with crystal cube
    pub fn align_quantum(&mut self, cube: &CrystalCube<f64>) -> AlignmentResult<AlignmentState> {
        // First ensure phantom is in valid state
        self.phantom.stabilize()?;

        // Get quantum state vector
        let state = Vector4D::new(
            self.reference.x,
            self.reference.y,
            self.reference.z,
            sqrt(cube.get_state(0, 0, 0)?)
        );

        // Calculate quantum alignment factor
        let alignment = state.magnitude()?;

        self.state = if alignment >= ALIGNMENT_THRESHOLD {
            AlignmentState::Perfect
        } else if alignment >= QUANTUM_STABILITY_THRESHOLD {
            AlignmentState::Partial(alignment)
        } else {
            AlignmentState::Misaligned
        };

        Ok(self.state)
    }

    /// Get current alignment state
    pub fn get_state(&self) -> AlignmentState {
        self.state
    }

    /// Check if alignment is stable
    pub fn is_stable(&self) -> bool {
        matches!(self.state,
                 AlignmentState::Perfect |
                 AlignmentState::Partial(c) if c >= QUANTUM_STABILITY_THRESHOLD
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_creation() {
        let reference = Vector3D::new(1.0, 0.0, 0.0);
        let alignment = Alignment::new(reference);
        assert_eq!(alignment.get_state(), AlignmentState::Unknown);
    }

    #[test]
    fn test_perfect_alignment() {
        let mut alignment = Alignment::new(Vector3D::new(1.0, 0.0, 0.0));
        let target = Vector3D::new(1.0, 0.0, 0.0);
        let state = alignment.align_with(&target).unwrap();
        assert_eq!(state, AlignmentState::Perfect);
    }

    #[test]
    fn test_misaligned_state() {
        let mut alignment = Alignment::new(Vector3D::new(1.0, 0.0, 0.0));
        let target = Vector3D::new(0.0, 1.0, 0.0);
        let state = alignment.align_with(&target).unwrap();
        assert_eq!(state, AlignmentState::Misaligned);
    }
}
