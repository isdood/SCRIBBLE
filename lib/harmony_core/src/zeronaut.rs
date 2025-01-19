//! Zeronaut - Crystal Lattice Navigator
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:15:06 UTC
//! Version: 0.1.0
//! License: MIT

use core::f64::consts::PI;
use alloc::string::String;
use meshmath::{sqrt, floor};

use crate::vector::Vector3D;
use crate::crystal::CrystalLattice;
use crate::idk::ShardUninit;
use crate::errors::CoherenceError;

/// Crystal state configuration
#[derive(Debug, Clone)]
pub struct CrystalState {
    /// Crystal resonance frequency
    resonance: f64,
    /// Crystal coupling strength
    coupling: f64,
    /// Phase alignment
    phase: f64,
}

impl Default for CrystalState {
    fn default() -> Self {
        Self {
            resonance: 1.0,
            coupling: 0.5,
            phase: 0.0,
        }
    }
}

/// Crystal lattice navigator
#[derive(Debug)]
pub struct Zeronaut<T: Clone + Default + 'static> {
    /// Position in crystal lattice
    pos: Vector3D<f64>,
    /// Quantum state data
    data: ShardUninit<T>,
    /// Crystal field strength
    field_strength: f64,
    /// Crystal state
    crystal_state: CrystalState,
    /// Associated crystal lattice
    lattice: Option<CrystalLattice>,
}

impl<T: Clone + Default + 'static> Zeronaut<T> {
    /// Create a new Zeronaut at the origin
    pub fn new() -> Self {
        Self {
            pos: Vector3D::zero(),
            data: ShardUninit::uninit(),
            field_strength: 1.0,
            crystal_state: CrystalState::default(),
            lattice: None,
        }
    }

    /// Get current position in crystal lattice
    pub fn position(&self) -> &Vector3D<f64> {
        &self.pos
    }

    /// Get crystal field strength
    pub fn field_strength(&self) -> f64 {
        self.field_strength
    }

    /// Get data reference with crystal verification
    pub fn data(&self) -> Result<&T, CoherenceError> {
        self.verify_crystal_coherence()?;
        unsafe {
            self.data.assume_init_ref()
            .map_err(|_| CoherenceError::QuantumInstability)
        }
    }

    /// Get mutable data reference with crystal verification
    pub fn data_mut(&mut self) -> Result<&mut T, CoherenceError> {
        self.verify_crystal_coherence()?;
        unsafe {
            self.data.assume_init_mut()
            .map_err(|_| CoherenceError::QuantumInstability)
        }
    }

    /// Set position in crystal lattice
    pub fn set_position(&mut self, pos: Vector3D<f64>) -> Result<(), CoherenceError> {
        // Verify crystal boundaries
        if let Some(ref lattice) = self.lattice {
            if !lattice.is_valid_position(&pos) {
                return Err(CoherenceError::BoundaryViolation);
            }
        }
        self.pos = pos;
        Ok(())
    }

    /// Set crystal field strength
    pub fn set_field_strength(&mut self, strength: f64) -> Result<(), CoherenceError> {
        if strength <= 0.0 {
            return Err(CoherenceError::StructureFailure);
        }
        self.field_strength = strength;
        Ok(())
    }

    /// Calculate distance to another position
    pub fn distance_to(&self, other: &Vector3D<f64>) -> f64 {
        let dx = self.pos.x - other.x;
        let dy = self.pos.y - other.y;
        let dz = self.pos.z - other.z;
        sqrt(dx * dx + dy * dy + dz * dz)
    }

    /// Get discrete crystal lattice coordinates
    pub fn lattice_position(&self) -> (usize, usize, usize) {
        let x = floor(self.pos.x) as usize;
        let y = floor(self.pos.y) as usize;
        let z = floor(self.pos.z) as usize;
        (x, y, z)
    }

    /// Project through crystal field
    pub fn project(&mut self, target: &Vector3D<f64>) -> Result<(), CoherenceError> {
        self.verify_crystal_coherence()?;

        let current = self.position().clone();
        let distance = self.distance_to(target);

        if distance < self.field_strength {
            self.set_position(target.clone())
        } else {
            // Calculate direction vector
            let dx = target.x - current.x;
            let dy = target.y - current.y;
            let dz = target.z - current.z;

            // Normalize and scale by field strength
            let mag = sqrt(dx * dx + dy * dy + dz * dz);
            let scale = self.field_strength * self.crystal_state.coupling / mag;

            // Update position within crystal constraints
            let new_pos = Vector3D::new(
                current.x + dx * scale,
                current.y + dy * scale,
                current.z + dz * scale,
            );

            self.set_position(new_pos)
        }
    }

    /// Verify crystal coherence state
    fn verify_crystal_coherence(&self) -> Result<(), CoherenceError> {
        if self.crystal_state.resonance < 0.1 {
            return Err(CoherenceError::CrystalDecoherence);
        }
        if self.crystal_state.phase > 2.0 * PI {
            return Err(CoherenceError::PhaseAlignmentFailure);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeronaut_creation() {
        let nav = Zeronaut::<u8>::new();
        assert_eq!(nav.field_strength(), 1.0);
        assert_eq!(nav.position(), &Vector3D::zero());
    }

    #[test]
    fn test_crystal_coherence() {
        let nav = Zeronaut::<u8>::new();
        assert!(nav.verify_crystal_coherence().is_ok());
    }

    #[test]
    fn test_position_update() {
        let mut nav = Zeronaut::<u8>::new();
        let new_pos = Vector3D::new(1.0, 1.0, 1.0);
        assert!(nav.set_position(new_pos.clone()).is_ok());
        assert_eq!(nav.position(), &new_pos);
    }
}
