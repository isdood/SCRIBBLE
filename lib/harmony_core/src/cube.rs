//! Crystal Cube Operations
//! ==================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 09:48:03 UTC
//! Last Updated: 2025-01-19 21:22:00 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::floor::floor;
use crate::{
    errors::QuantumError,
    vector::Vector3D,
    constants::MAX_QUANTUM_SIZE,
    align::{Alignment, AlignmentState},
    scribe::Scribe,
    native::{Box, Vec},
};
use scribe::native_string::String;
use scribe::string::WriteStr;

/// A three-dimensional crystal cube structure
#[derive(Debug)]
pub struct Cube<T> {
    alignment: Alignment,
    data: Box<[Box<[Box<[T]>]>]>,  // Use nested arrays managed by Box
    size: usize,
}

/// Storage for boxed cube values
#[derive(Debug)]
pub struct CubeBox<T> {
    value: T,
}

impl<T> CubeBox<T> {
    /// Create a new cube box with given value
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Get the inner value
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Convert into raw pointer
    pub fn into_raw(self) -> *mut T {
        Box::into_raw(Box::new(self.value))
    }

    /// Create from raw pointer
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            value: *Box::from_raw(ptr)
        }
    }
}

impl<T: Default + Clone + Scribe> Cube<T> {
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

impl<T: Default + Clone + Scribe> Scribe for Cube<T> {
    fn scribe(&self) -> String {
        let mut result = String::new();
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    result.write_str("(").unwrap();
                    result.write_str(&x.scribe()).unwrap();
                    result.write_str(",").unwrap();
                    result.write_str(&y.scribe()).unwrap();
                    result.write_str(",").unwrap();
                    result.write_str(&z.scribe()).unwrap();
                    result.write_str("): ").unwrap();
                    result.write_str(&self.data[x][y][z].scribe()).unwrap();
                    result.write_str("\n").unwrap();
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default)]
    struct TestValue {
        value: i32,
    }

    impl Scribe for TestValue {
        fn scribe(&self) -> String {
            let mut result = String::new();
            result.write_str(&self.value.scribe()).unwrap();
            result
        }
    }

    #[test]
    fn test_cube_creation() {
        let cube: Cube<TestValue> = Cube::new(4);
        assert_eq!(cube.size(), 4);
    }

    #[test]
    fn test_state_access() {
        let mut cube = Cube::new(4);
        let test_value = TestValue { value: 42 };
        assert!(cube.set_state(0, 0, 0, test_value.clone()).is_ok());
        assert_eq!(cube.get_state(0, 0, 0).unwrap().value, 42);
    }

    #[test]
    fn test_boundary_violation() {
        let cube = Cube::<TestValue>::new(4);
        assert!(cube.get_state(4, 0, 0).is_err());
    }

    #[test]
    fn test_cube_scribe() {
        let mut cube = Cube::new(2);
        let test_value = TestValue { value: 42 };
        cube.set_state(0, 0, 0, test_value.clone()).unwrap();
        assert!(cube.scribe().contains("(0,0,0): 42"));
    }

    #[test]
    fn test_cube_box() {
        let value = TestValue { value: 42 };
        let cube_box = CubeBox::new(value);
        assert_eq!(cube_box.into_inner().value, 42);
    }
}
