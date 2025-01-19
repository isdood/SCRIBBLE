//! Aether - Quantum Field Operations
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 09:38:17 UTC
//! Last Updated: 2025-01-19 09:41:20 UTC
//! Version: 0.1.0
//! License: MIT

use meshmath::sqrt;
use crate::{
    constants::{
        QUANTUM_STABILITY_THRESHOLD,
        MAX_QUANTUM_SIZE,
        AETHER_RESONANCE_FACTOR
    },
    harmony::Quantum,
    errors::QuantumError,
    crystal::CrystalNode,
    vector::Vector3D
};

/// Aether field for quantum operations
#[derive(Debug)]
pub struct AetherField {
    /// Field strength
    strength: f64,
    /// Field coherence
    coherence: f64,
    /// Field position
    position: Vector3D<f64>,
}

impl AetherField {
    /// Create a new aether field
    pub fn new(position: Vector3D<f64>) -> Self {
        Self {
            strength: 1.0,
            coherence: 1.0,
            position,
        }
    }

    /// Get field strength at point
    pub fn strength_at(&self, point: &Vector3D<f64>) -> Result<f64, QuantumError> {
        let distance = self.position.dot(point)?;
        if distance > MAX_QUANTUM_SIZE as f64 {
            return Err(QuantumError::CrystalBoundaryViolation);
        }

        Ok(self.strength * sqrt(AETHER_RESONANCE_FACTOR / (distance + 1.0)))
    }

    /// Set field strength
    pub fn set_strength(&mut self, value: f64) -> Result<(), QuantumError> {
        if value <= 0.0 || value > MAX_QUANTUM_SIZE as f64 {
            return Err(QuantumError::InvalidState);
        }
        self.strength = value;
        Ok(())
    }
}

impl Quantum for AetherField {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn recohere(&mut self) -> Result<(), QuantumError> {
        if self.strength < QUANTUM_STABILITY_THRESHOLD {
            return Err(QuantumError::CoherenceLoss);
        }
        self.coherence = self.strength * AETHER_RESONANCE_FACTOR;
        Ok(())
    }

    fn decohere(&mut self) {
        self.coherence = 0.0;
    }

    fn phase_alignment(&self) -> f64 {
        self.coherence * AETHER_RESONANCE_FACTOR
    }

    fn align_with(&mut self, target: &CrystalNode) -> Result<(), QuantumError> {
        let target_phase = target.get_phase_coherence();
        if target_phase < QUANTUM_STABILITY_THRESHOLD {
            return Err(QuantumError::PhaseMisalignment);
        }
        self.coherence = target_phase * AETHER_RESONANCE_FACTOR;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aether_field_creation() {
        let position = Vector3D::new(0.0, 0.0, 0.0);
        let field = AetherField::new(position);
        assert!(field.coherence() >= QUANTUM_STABILITY_THRESHOLD);
    }

    #[test]
    fn test_field_strength_calculation() {
        let field = AetherField::new(Vector3D::new(0.0, 0.0, 0.0));
        let point = Vector3D::new(1.0, 1.0, 1.0);
        assert!(field.strength_at(&point).is_ok());
    }

    #[test]
    fn test_coherence_operations() {
        let mut field = AetherField::new(Vector3D::new(0.0, 0.0, 0.0));
        assert!(field.is_stable());
        field.decohere();
        assert!(!field.is_stable());
    }
}
