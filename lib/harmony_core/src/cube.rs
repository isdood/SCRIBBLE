//! Crystal Cube Operations
//! ==================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 09:48:03 UTC
//! Last Updated: 2025-01-19 21:05:22 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::floor::floor;
use crate::{
    errors::QuantumError,
    vector::Vector3D,
    constants::MAX_QUANTUM_SIZE,
    align::{Alignment, AlignmentState},
    scribe::Scribe,
    native::Box,
};
use scribe::native_string::String;

/// A three-dimensional crystal cube structure
#[derive(Debug)]
pub struct CrystalCube<T> {
    alignment: Alignment,
    data: Box<[Box<[Box<[T]>]>]>,  // Use nested arrays managed by Box
    size: usize,
}

impl<T: Default + Clone + Scribe> CrystalCube<T> {
    /// Create a new crystal cube with given size
    pub fn new(size: usize) -> Self {
        let size = size.min(MAX_QUANTUM_SIZE);
        let data = (0..size)
        .map(|_| (0..size)
        .map(|_| (0..size)
        .map(|_| T::default())
        .collect::<Vec<_>>()
        .into_boxed_slice())
        .collect::<Vec<_>>()
        .into_boxed_slice())
        .collect::<Vec<_>>()
        .into_boxed_slice();

        let alignment = Alignment::new(Vector3D::new(0.0, 0.0, 0.0));

        Self { data, size, alignment }
    }

    /// Get state at given coordinates
    pub fn get_state(&self, x: usize, y: usize, z: usize) -> Result<T, QuantumError> {
        if x >= self.size || y >= self.size || z >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }
        Ok(self.data[x][y][z].clone())
    }

    /// Set state at given coordinates
    pub fn set_state(&mut self, x: usize, y: usize, z: usize, value: T) -> Result<(), QuantumError> {
        if x >= self.size || y >= self.size || z >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }
        self.data[x][y][z] = value;
        Ok(())
    }

    /// Get state at vector position
    pub fn get_state_at(&self, pos: &Vector3D) -> Result<T, QuantumError> {
        let x = floor(pos.x) as usize;
        let y = floor(pos.y) as usize;
        let z = floor(pos.z) as usize;
        self.get_state(x, y, z)
    }

    /// Set state at vector position
    pub fn set_state_at(&mut self, pos: &Vector3D, value: T) -> Result<(), QuantumError> {
        let x = floor(pos.x) as usize;
        let y = floor(pos.y) as usize;
        let z = floor(pos.z) as usize;
        self.set_state(x, y, z, value)
    }

    /// Get the size of the cube
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get current alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        self.alignment.state()
    }
}

impl<T: Default + Clone + Scribe> Scribe for CrystalCube<T> {
    fn scribe(&self) -> String {
        let mut result = String::new();
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    result.push_str(&format!(
                        "({},{},{}): {}\n",
                                             x, y, z,
                                             self.data[x][y][z].scribe()
                    ));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, Clone, Debug)]
    struct TestType {
        value: i32,
    }

    impl Scribe for TestType {
        fn scribe(&self) -> String {
            format!("{}", self.value)
        }
    }

    #[test]
    fn test_cube_creation() {
        let cube: CrystalCube<TestType> = CrystalCube::new(4);
        assert_eq!(cube.size(), 4);
    }

    #[test]
    fn test_state_access() {
        let mut cube = CrystalCube::new(4);
        let test_value = TestType { value: 42 };
        assert!(cube.set_state(0, 0, 0, test_value.clone()).is_ok());
        assert_eq!(cube.get_state(0, 0, 0).unwrap().value, 42);
    }

    #[test]
    fn test_boundary_violation() {
        let cube = CrystalCube::<TestType>::new(4);
        assert!(cube.get_state(4, 0, 0).is_err());
    }

    #[test]
    fn test_cube_scribe() {
        let mut cube = CrystalCube::new(2);
        let test_value = TestType { value: 42 };
        cube.set_state(0, 0, 0, test_value.clone()).unwrap();
        assert!(cube.scribe().contains("(0,0,0): 42"));
    }
}
