//! Zeronaut - Zero-Point Energy Operations
//! =================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:59:58 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::{sqrt, floor};
use crate::{
    vector::Vector3D,
    crystal::CrystalNode,
    errors::QuantumError,
    constants::{
        QUANTUM_STABILITY_THRESHOLD,
        MAX_QUANTUM_SIZE,
        QUANTUM_GOLDEN_RATIO
    },
    align::{Alignment, AlignmentState},
    idk::ShardUninit,
    harmony::Quantum,
};

/// Core zero-point energy state
#[derive(Debug)]
pub struct ZeroCore {
    /// Energy level
    energy: f64,
    /// State storage
    state: ShardUninit<[f64; MAX_QUANTUM_SIZE]>,
}

/// Zero-point energy operator
#[derive(Debug)]
pub struct Zeronaut {
    /// Core zero-point data
    core: ZeroCore,
    /// Position in quantum space
    position: Vector3D,
    /// Current coherence value
    coherence: f64,
    /// Quantum alignment
    alignment: Alignment,
}

impl ZeroCore {
    /// Create a new zero-point core
    pub const fn new() -> Self {
        Self {
            energy: 0.0,
            state: ShardUninit::new(),
        }
    }

    /// Set energy level
    pub fn set_energy(&mut self, value: f64) -> Result<(), QuantumError> {
        if value < 0.0 || value > MAX_QUANTUM_SIZE as f64 {
            return Err(QuantumError::InvalidState);
        }
        self.energy = value;
        Ok(())
    }

    /// Get current energy level
    pub fn energy(&self) -> f64 {
        self.energy
    }

    /// Store state value
    pub unsafe fn store_state(&mut self, index: usize, value: f64) -> Result<(), QuantumError> {
        if index >= MAX_QUANTUM_SIZE {
            return Err(QuantumError::BoundaryViolation);
        }

        let state = self.state.get_mut()
        .ok_or(QuantumError::InvalidState)?;

        state[index] = value;
        Ok(())
    }

    /// Get stored state value
    pub unsafe fn get_state(&self, index: usize) -> Result<f64, QuantumError> {
        if index >= MAX_QUANTUM_SIZE {
            return Err(QuantumError::BoundaryViolation);
        }

        let state = self.state.get_ref()
        .ok_or(QuantumError::InvalidState)?;

        Ok(state[index])
    }
}

impl Zeronaut {
    /// Create a new zeronaut
    pub fn new(position: Vector3D) -> Self {
        Self {
            core: ZeroCore::new(),
            position: position.clone(),
            coherence: 1.0,
            alignment: Alignment::new(position),
        }
    }

    /// Get current position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Set position
    pub fn set_position(&mut self, pos: Vector3D) -> Result<(), QuantumError> {
        if pos.magnitude()? > MAX_QUANTUM_SIZE as f64 {
            return Err(QuantumError::BoundaryViolation);
        }
        self.position = pos;
        Ok(())
    }

    /// Calculate zero-point energy
    pub fn calculate_zero_point(&self) -> Result<f64, QuantumError> {
        if !self.is_stable() {
            return Err(QuantumError::CoherenceLoss);
        }

        let energy = self.core.energy();
        Ok(sqrt(energy * QUANTUM_GOLDEN_RATIO))
    }

    /// Measure quantum state
    pub fn measure_state(&mut self) -> Result<f64, QuantumError> {
        if !self.is_stable() {
            return Err(QuantumError::CoherenceLoss);
        }

        let state = self.alignment.align_with(&self.position)?;
        match state {
            AlignmentState::Perfect => Ok(1.0),
            AlignmentState::Partial(v) => Ok(v),
            _ => Err(QuantumError::InvalidState),
        }
    }

    /// Store quantum state
    pub fn store_state(&mut self, value: f64) -> Result<(), QuantumError> {
        if !self.is_stable() {
            return Err(QuantumError::CoherenceLoss);
        }

        let index = floor(self.core.energy()) as usize;
        unsafe {
            self.core.store_state(index, value)
        }
    }
}

impl Quantum for Zeronaut {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn recohere(&mut self) -> Result<(), QuantumError> {
        if self.core.energy() < QUANTUM_STABILITY_THRESHOLD {
            return Err(QuantumError::CoherenceLoss);
        }
        self.coherence = 1.0;
        Ok(())
    }

    fn decohere(&mut self) {
        self.coherence = 0.0;
    }

    fn phase_alignment(&self) -> f64 {
        self.alignment.get_state().into()
    }

    fn align_with(&mut self, target: &CrystalNode) -> Result<(), QuantumError> {
        let target_coherence = target.get_phase_coherence();
        if target_coherence < QUANTUM_STABILITY_THRESHOLD {
            return Err(QuantumError::PhaseMisalignment);
        }
        self.coherence = target_coherence;
        Ok(())
    }

    fn alignment_state(&self) -> AlignmentState {
        self.alignment.get_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeronaut_creation() {
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        let zeronaut = Zeronaut::new(pos);
        assert!(zeronaut.is_stable());
    }

    #[test]
    fn test_zero_point_calculation() {
        let zeronaut = Zeronaut::new(Vector3D::new(0.0, 0.0, 0.0));
        assert!(zeronaut.calculate_zero_point().is_ok());
    }

    #[test]
    fn test_state_measurement() {
        let mut zeronaut = Zeronaut::new(Vector3D::new(0.0, 0.0, 0.0));
        assert!(zeronaut.measure_state().is_ok());
    }

    #[test]
    fn test_coherence_operations() {
        let mut zeronaut = Zeronaut::new(Vector3D::new(0.0, 0.0, 0.0));
        assert!(zeronaut.is_stable());
        zeronaut.decohere();
        assert!(!zeronaut.is_stable());
    }
}
