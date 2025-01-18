//! Crystalline Cube Implementation
//! ============================
//!
//! Core memory management through crystalline cubes and quantum-safe
//! allocation with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:47:53 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::{QUANTUM_STABILITY_THRESHOLD, CUBE_TIMESTAMP},
    harmony::{Quantum, MeshValue},
    vector::{Vector3D, Vector4D},
    CrystalArray,
};

/// A quantum-safe container for crystalline data
#[derive(Clone)]
pub struct CrystalCube<T: Clone + 'static> {
    /// The crystallized data
    data: T,
    /// Quantum coherence tracking
    coherence: f64,
    /// Crystalline timestamp
    timestamp: u64,
    /// Position in quantum space
    position: Vector3D<f64>,
}

/// A shared quantum-safe container with reference counting
#[derive(Clone)]
pub struct SharedCube<T: Clone + 'static> {
    /// Inner crystalline cube
    cube: CrystalCube<T>,
    /// Reference count
    refs: usize,
}

impl<T: Clone + 'static> CrystalCube<T> {
    /// Creates a new quantum cube with perfect crystalline coherence
    pub fn new(value: T) -> Self {
        Self {
            data: value,
            coherence: 1.0,
            timestamp: CUBE_TIMESTAMP as u64,
            position: Vector3D::zero(),
        }
    }

    /// Creates a new quantum cube at specific coordinates
    pub fn new_positioned(value: T, x: f64, y: f64, z: f64) -> Self {
        Self {
            data: value,
            coherence: 1.0,
            timestamp: CUBE_TIMESTAMP as u64,
            position: Vector3D::new(x, y, z),
        }
    }

    /// Gets a reference to the crystallized data
    pub fn get(&self) -> &T {
        &self.data
    }

    /// Gets a mutable reference to the crystallized data
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Updates the crystallized data while maintaining coherence
    pub fn set(&mut self, value: T) {
        self.data = value;
        self.decohere();
    }

    /// Gets the current quantum coherence value
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Gets the crystalline position in quantum space
    pub fn position(&self) -> &Vector3D<f64> {
        &self.position
    }

    /// Updates the crystalline position
    pub fn set_position(&mut self, pos: Vector3D<f64>) {
        self.position = pos;
        self.decohere();
    }
}

impl<T: Clone + 'static> SharedCube<T> {
    /// Creates a new shared quantum cube
    pub fn new(value: T) -> Self {
        Self {
            cube: CrystalCube::new(value),
            refs: 1,
        }
    }

    /// Increments the reference count
    pub fn inc_ref(&mut self) {
        self.refs = self.refs.saturating_add(1);
    }

    /// Decrements the reference count
    pub fn dec_ref(&mut self) -> bool {
        self.refs = self.refs.saturating_sub(1);
        self.refs == 0
    }

    /// Gets the current reference count
    pub fn ref_count(&self) -> usize {
        self.refs
    }

    /// Gets a reference to the inner crystalline cube
    pub fn inner(&self) -> &CrystalCube<T> {
        &self.cube
    }

    /// Gets a mutable reference to the inner crystalline cube
    pub fn inner_mut(&mut self) -> &mut CrystalCube<T> {
        &mut self.cube
    }
}

impl<T: Clone + 'static> Quantum for CrystalCube<T> {
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
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_cube_basics() {
        let mut cube = CrystalCube::new(42);
        assert_eq!(*cube.get(), 42);
        assert!(cube.is_stable());

        cube.set(43);
        assert_eq!(*cube.get(), 43);
        assert!(cube.coherence() < 1.0);
    }

    #[test]
    fn test_crystal_cube_position() {
        let mut cube = CrystalCube::new_positioned(42, 1.0, 2.0, 3.0);
        assert_eq!(cube.position().x, 1.0);
        assert_eq!(cube.position().y, 2.0);
        assert_eq!(cube.position().z, 3.0);

        cube.set_position(Vector3D::zero());
        assert_eq!(*cube.position(), Vector3D::zero());
    }

    #[test]
    fn test_shared_cube() {
        let mut shared = SharedCube::new(42);
        assert_eq!(shared.ref_count(), 1);

        shared.inc_ref();
        assert_eq!(shared.ref_count(), 2);

        assert!(!shared.dec_ref());
        assert_eq!(shared.ref_count(), 1);

        assert!(shared.dec_ref());
        assert_eq!(shared.ref_count(), 0);
    }
}
