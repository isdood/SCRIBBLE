//! Scribe - Crystal Data Recording Operations
//! ====================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:57:36 UTC
//! Version: 0.1.0
//! License: MIT

use meshmath::sqrt;
use crate::{
    vector::Vector3D,
    crystal::CrystalCube,
    errors::QuantumError,
    constants::{
        QUANTUM_STABILITY_THRESHOLD,
        MAX_QUANTUM_SIZE
    },
    align::{Alignment, AlignmentState},
    idk::ShardUninit,
};

/// Core scribe state for data recording
#[derive(Debug)]
pub struct ScribeCore {
    /// Data storage
    data: ShardUninit<[u8; MAX_QUANTUM_SIZE]>,
    /// Current write position
    position: usize,
}

/// Crystal data recording operator
#[derive(Debug)]
pub struct Scribe {
    /// Core scribe data
    core: ScribeCore,
    /// Position in crystal space
    pos: Vector3D<f64>,
    /// Current coherence value
    coherence: f64,
    /// Quantum alignment
    alignment: Alignment,
}

impl ScribeCore {
    /// Create a new scribe core
    pub const fn new() -> Self {
        Self {
            data: ShardUninit::new(),
            position: 0,
        }
    }

    /// Check if position is valid
    pub fn is_valid_position(&self, pos: usize) -> bool {
        pos < MAX_QUANTUM_SIZE
    }

    /// Write data to storage
    pub unsafe fn write(&mut self, value: u8) -> Result<(), QuantumError> {
        if !self.is_valid_position(self.position) {
            return Err(QuantumError::BoundaryViolation);
        }

        let data = self.data.get_mut()
        .ok_or(QuantumError::InvalidState)?;

        data[self.position] = value;
        self.position += 1;
        Ok(())
    }

    /// Read data from storage
    pub unsafe fn read(&self, pos: usize) -> Result<u8, QuantumError> {
        if !self.is_valid_position(pos) {
            return Err(QuantumError::BoundaryViolation);
        }

        let data = self.data.get_ref()
        .ok_or(QuantumError::InvalidState)?;

        Ok(data[pos])
    }
}

impl Scribe {
    /// Create a new scribe
    pub fn new(position: Vector3D<f64>) -> Self {
        Self {
            core: ScribeCore::new(),
            pos: position.clone(),
            coherence: 1.0,
            alignment: Alignment::new(position),
        }
    }

    /// Get current position
    pub fn position(&self) -> &Vector3D<f64> {
        &self.pos
    }

    /// Set position
    pub fn set_position(&mut self, pos: &Vector3D<f64>) -> Result<(), QuantumError> {
        if pos.magnitude()? > MAX_QUANTUM_SIZE as f64 {
            return Err(QuantumError::BoundaryViolation);
        }
        self.pos = pos.clone();
        Ok(())
    }

    /// Record data to crystal cube
    pub fn record(&mut self, cube: &mut CrystalCube<u8>, value: u8) -> Result<(), QuantumError> {
        if !self.is_coherent() {
            return Err(QuantumError::CoherenceLoss);
        }

        let pos = self.position();
        cube.set_state_at(pos, value)?;

        unsafe {
            self.core.write(value)?;
        }

        Ok(())
    }

    /// Read data from crystal cube
    pub fn read(&self, cube: &CrystalCube<u8>) -> Result<u8, QuantumError> {
        if !self.is_coherent() {
            return Err(QuantumError::CoherenceLoss);
        }

        cube.get_state_at(self.position())
    }

    /// Check if scribe is coherent
    pub fn is_coherent(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    /// Get current alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        self.alignment.get_state()
    }

    /// Calculate quantum resonance
    pub fn calculate_resonance(&self) -> Result<f64, QuantumError> {
        if !self.is_coherent() {
            return Err(QuantumError::CoherenceLoss);
        }

        Ok(sqrt(self.coherence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scribe_creation() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let scribe = Scribe::new(pos);
        assert!(scribe.is_coherent());
    }

    #[test]
    fn test_position_setting() {
        let mut scribe = Scribe::new(Vector3D::new(0.0, 0.0, 0.0));
        let new_pos = Vector3D::new(1.0, 1.0, 1.0);
        assert!(scribe.set_position(&new_pos).is_ok());
    }

    #[test]
    fn test_resonance_calculation() {
        let scribe = Scribe::new(Vector3D::new(0.0, 0.0, 0.0));
        assert!(scribe.calculate_resonance().is_ok());
    }

    #[test]
    fn test_boundary_violation() {
        let mut scribe = Scribe::new(Vector3D::new(0.0, 0.0, 0.0));
        let invalid_pos = Vector3D::new(MAX_QUANTUM_SIZE as f64 + 1.0, 0.0, 0.0);
        assert!(scribe.set_position(&invalid_pos).is_err());
    }
}
