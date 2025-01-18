//! Crystalline Zeronaut Implementation
//! ==============================
//!
//! Core quantum zero-point navigation through crystalline
//! structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 21:10:09 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::Deref;
use libm::floor;
use crate::{
    constants::{ZERO_POINT_ENERGY, QUANTUM_STABILITY_THRESHOLD},
    harmony::Quantum,
    vector::Vector3D,
    cube::CrystalCube
};

/// A quantum zero-point navigator
#[derive(Clone)]
pub struct Zeronaut<T: Clone + Default + 'static> {
    /// Contained data
    data: T,
    /// Current position
    position: Vector3D<f64>,
    /// Zero-point energy level
    energy: f64,
    /// Quantum coherence
    coherence: f64,
}

impl<T: Clone + Default + 'static> Zeronaut<T> {
    /// Creates a new zeronaut with positioned data
    pub fn new_positioned(data: T, x: f64, y: f64, z: f64) -> Self {
        Self {
            data,
            position: Vector3D::new(x, y, z),
            energy: ZERO_POINT_ENERGY,
            coherence: 1.0,
        }
    }

    /// Gets a reference to the contained data
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Gets a mutable reference to the contained data
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Gets the current position
    pub fn position(&self) -> &Vector3D<f64> {
        &self.position
    }

    /// Sets the position and updates quantum state
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position = Vector3D::new(x, y, z);
        self.decohere();
    }

    /// Gets the current zero-point energy
    pub fn energy(&self) -> f64 {
        self.energy
    }

    /// Attempts quantum tunneling to a new position
    pub fn tunnel(&mut self, target: &Vector3D<f64>) -> Result<(), &'static str> {
        let distance = {
            let dx = target.x - self.position.x;
            let dy = target.y - self.position.y;
            let dz = target.z - self.position.z;
            libm::sqrt(dx * dx + dy * dy + dz * dz)
        };

        // Check if tunneling is possible based on energy and distance
        if distance * self.energy <= ZERO_POINT_ENERGY {
            self.position = target.clone();
            self.decohere();
            Ok(())
        } else {
            Err("Insufficient energy for quantum tunneling")
        }
    }

    /// Attempts to enter a crystal cube
    pub fn enter_cube(&mut self, cube: &mut CrystalCube<T>) -> Result<(), &'static str> {
        let pos = self.position();
        let x = floor(pos.x) as usize;
        let y = floor(pos.y) as usize;
        let z = floor(pos.z) as usize;

        if let Some(cell) = cube.get_mut(x, y, z) {
            core::mem::swap(cell, &mut self.data);
            self.decohere();
            cube.decohere();
            Ok(())
        } else {
            Err("Invalid cube coordinates")
        }
    }
}

impl<T: Clone + Default + 'static> Quantum for Zeronaut<T> {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    fn decohere(&mut self) {
        self.coherence *= 0.9;
        if self.coherence < QUANTUM_STABILITY_THRESHOLD {
            self.coherence = QUANTUM_STABILITY_THRESHOLD;
        }
        self.energy *= 0.9;
        if self.energy < ZERO_POINT_ENERGY {
            self.energy = ZERO_POINT_ENERGY;
        }
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
        self.energy = ZERO_POINT_ENERGY;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeronaut_basics() {
        let zeronaut = Zeronaut::new_positioned(42u8, 1.0, 2.0, 3.0);
        assert_eq!(*zeronaut.data(), 42);
        assert_eq!(zeronaut.position().x, 1.0);
        assert_eq!(zeronaut.position().y, 2.0);
        assert_eq!(zeronaut.position().z, 3.0);
        assert!(zeronaut.is_stable());
    }

    #[test]
    fn test_zeronaut_movement() {
        let mut zeronaut = Zeronaut::new_positioned(42u8, 0.0, 0.0, 0.0);
        zeronaut.set_position(1.0, 1.0, 1.0);
        assert_eq!(zeronaut.position().x, 1.0);
        assert_eq!(zeronaut.position().y, 1.0);
        assert_eq!(zeronaut.position().z, 1.0);
    }

    #[test]
    fn test_zeronaut_tunneling() {
        let mut zeronaut = Zeronaut::new_positioned(42u8, 0.0, 0.0, 0.0);
        let target = Vector3D::new(0.1, 0.1, 0.1);
        assert!(zeronaut.tunnel(&target).is_ok());

        let far_target = Vector3D::new(100.0, 100.0, 100.0);
        assert!(zeronaut.tunnel(&far_target).is_err());
    }
}
