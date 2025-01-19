//! Alignment Operations for Crystal Computing
//! =====================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 13:35:00 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::{sqrt, QuantumMath};
use crate::{
    vector::{Vector3D, Vector4D},
    crystal::CrystalNode,
    errors::QuantumError,
    constants::{QUANTUM_STABILITY_THRESHOLD, ALIGNMENT_THRESHOLD},
    idk::ShardUninit,
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

/// Core alignment type for crystal operations
#[derive(Debug)]
pub struct AlignmentCore {
    /// Shard storage for aligned data
    shard: ShardUninit<[f64; 3]>,
    /// Current alignment state
    state: AlignmentState,
}

/// Alignment calculations for crystal operations
#[derive(Debug)]
pub struct Alignment {
    /// Core alignment data
    core: AlignmentCore,
    /// Reference vector for alignment
    reference: Vector3D,
    /// Current alignment state
    state: AlignmentState,
}

impl AlignmentCore {
    /// Create a new alignment core
    pub const fn new() -> Self {
        Self {
            shard: ShardUninit::new(),
            state: AlignmentState::Unknown,
        }
    }

    /// Get current alignment state
    pub fn state(&self) -> AlignmentState {
        self.state
    }

    /// Set alignment state
    pub fn set_state(&mut self, state: AlignmentState) {
        self.state = state;
    }
}

impl Alignment {
    /// Create new alignment calculator
    pub fn new(reference: Vector3D) -> Self {
        Self {
            core: AlignmentCore::new(),
            reference,
            state: AlignmentState::Unknown,
        }
    }

    /// Calculate alignment with target vector
    pub fn align_with(&mut self, target: &Vector3D) -> AlignmentResult<AlignmentState> {
        let mut qmath = QuantumMath::new();
        let dot = qmath.operate(magicmath::Operation::DotProduct, self.reference.dot(target)?)?;
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

        self.core.set_state(self.state);
        Ok(self.state)
    }

    /// Get current alignment state
    pub fn get_state(&self) -> AlignmentState {
        self.state
    }

    /// Check if alignment is stable
    pub fn is_stable(&self) -> bool {
        match self.state {
            AlignmentState::Perfect => true,
            AlignmentState::Partial(c) => c >= QUANTUM_STABILITY_THRESHOLD,
            _ => false
        }
    }

    /// Get reference vector
    pub fn reference(&self) -> &Vector3D {
        &self.reference
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use magicmath::QuantumMath;

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

    #[test]
    fn test_alignment_stability() {
        let mut alignment = Alignment::new(Vector3D::new(1.0, 0.0, 0.0));
        let target = Vector3D::new(0.9, 0.1, 0.0);
        alignment.align_with(&target).unwrap();
        assert!(alignment.is_stable());
    }
}
