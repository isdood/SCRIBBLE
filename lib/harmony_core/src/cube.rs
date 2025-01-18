//! Crystalline Cube Implementation
//! ==========================
//!
//! Core quantum cube operations through crystalline
//! structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 21:18:41 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::{QUANTUM_STABILITY_THRESHOLD, MAX_QUANTUM_SIZE},
    harmony::Quantum,
    vector::Vector3D,
    idk::ShardUninit
};

/// A crystalline cube structure for quantum operations
#[derive(Clone)]
pub struct CrystalCube<T: Clone + Default + 'static> {
    /// 3D array of data
    data: [[[ShardUninit<T>; MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE],
    /// Current dimensions
    dimensions: Vector3D<u64>,
    /// Quantum coherence
    coherence: f64,
}

impl<T: Clone + Default + 'static> CrystalCube<T> {
    /// Creates a new crystal cube with given dimensions
    pub fn new(x: u64, y: u64, z: u64) -> Option<Self> {
        if x as usize > MAX_QUANTUM_SIZE ||
            y as usize > MAX_QUANTUM_SIZE ||
            z as usize > MAX_QUANTUM_SIZE {
                return None;
            }

            let mut cube = Self {
                data: [[[ShardUninit::uninit(); MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE],
                dimensions: Vector3D::new(x, y, z),
                coherence: 1.0,
            };

        // Initialize with default values
        for i in 0..x as usize {
            for j in 0..y as usize {
                for k in 0..z as usize {
                    cube.data[i][j][k] = ShardUninit::new(T::default());
                }
            }
        }

        Some(cube)
    }

    /// Gets the current dimensions
    pub fn dimensions(&self) -> &Vector3D<u64> {
        &self.dimensions
    }

    /// Gets a reference to a value at specific coordinates
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&T> {
        if x >= self.dimensions.x as usize ||
            y >= self.dimensions.y as usize ||
            z >= self.dimensions.z as usize {
                return None;
            }
            unsafe { Some(self.data[x][y][z].assume_init_ref()) }
    }

    /// Gets a mutable reference to a value at specific coordinates
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> Option<&mut T> {
        if x >= self.dimensions.x as usize ||
            y >= self.dimensions.y as usize ||
            z >= self.dimensions.z as usize {
                return None;
            }
            unsafe { Some(self.data[x][y][z].assume_init_mut()) }
    }

    /// Sets a value at specific coordinates
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) -> Result<(), &'static str> {
        if x >= self.dimensions.x as usize ||
            y >= self.dimensions.y as usize ||
            z >= self.dimensions.z as usize {
                return Err("Coordinates out of bounds");
            }
            self.data[x][y][z] = ShardUninit::new(value);
        self.decohere();
        Ok(())
    }

    /// Clears the cube
    pub fn clear(&mut self) {
        for x in 0..self.dimensions.x as usize {
            for y in 0..self.dimensions.y as usize {
                for z in 0..self.dimensions.z as usize {
                    self.data[x][y][z] = ShardUninit::new(T::default());
                }
            }
        }
        self.recohere();
    }
}

impl<T: Clone + Default + 'static> Quantum for CrystalCube<T> {
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
    fn test_crystal_cube_creation() {
        let cube = CrystalCube::<u8>::new(2, 2, 2);
        assert!(cube.is_some());

        let cube = cube.unwrap();
        assert_eq!(cube.dimensions(), &Vector3D::new(2, 2, 2));
        assert!(cube.is_stable());
    }

    #[test]
    fn test_crystal_cube_bounds() {
        let cube = CrystalCube::<u8>::new(MAX_QUANTUM_SIZE as u64 + 1, 2, 2);
        assert!(cube.is_none());
    }

    #[test]
    fn test_crystal_cube_access() {
        let mut cube = CrystalCube::<u8>::new(2, 2, 2).unwrap();
        assert!(cube.get(0, 0, 0).is_some());
        assert!(cube.get(2, 2, 2).is_none());

        assert!(cube.set(0, 0, 0, 42).is_ok());
        assert!(cube.set(2, 2, 2, 42).is_err());

        assert_eq!(*cube.get(0, 0, 0).unwrap(), 42);
    }
}
