//! Cube - 3D Grid Data Structure
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 20:13:24 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::{
    MeshValue,
    Vector3D,
};

use errors::QuantumError;
use core::fmt::{self, Display, Write};

use crate::align::{
    text::String,
    collections::Vec,
    mem::Box,
};

/// 3D grid structure for quantum states
#[derive(Debug)]
pub struct Cube<T> {
    /// Grid data storage
    data: Vec<Vec<Vec<Option<T>>>>,
    /// Grid size
    size: usize,
}

impl<T: Default + Clone> Cube<T> {
    /// Create a new cube with given size
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            let mut plane = Vec::with_capacity(size);
            for _ in 0..size {
                let line = vec![None; size];
                plane.push(line);
            }
            data.push(plane);
        }

        Self { data, size }
    }

    /// Get state at position
    pub fn get_state_at(&self, pos: &Vector3D) -> Result<T, QuantumError> {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let z = pos.z.floor() as usize;

        if x >= self.size || y >= self.size || z >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        self.data[x][y][z]
        .as_ref()
        .cloned()
        .ok_or(QuantumError::InvalidState)
    }

    /// Set state at position
    pub fn set_state_at(&mut self, pos: &Vector3D, value: T) -> Result<(), QuantumError> {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let z = pos.z.floor() as usize;

        if x >= self.size || y >= self.size || z >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        self.data[x][y][z] = Some(value);
        Ok(())
    }

    /// Get cube size
    pub fn size(&self) -> usize {
        self.size
    }

    /// Clear all states
    pub fn clear(&mut self) {
        for plane in &mut self.data {
            for line in plane {
                for state in line {
                    *state = None;
                }
            }
        }
    }
}

impl<T: MeshValue + Display> Display for Cube<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cube [{}x{}x{}]:", self.size, self.size, self.size)?;
        for (x, plane) in self.data.iter().enumerate() {
            writeln!(f, "Plane {}:", x)?;
            for line in plane {
                for state in line {
                    match state {
                        Some(val) => write!(f, "{} ", val)?,
                        None => write!(f, "- ")?,
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_creation() {
        let cube: Cube<f64> = Cube::new(3);
        assert_eq!(cube.size(), 3);
    }

    #[test]
    fn test_state_access() {
        let mut cube = Cube::new(2);
        let pos = Vector3D::new(0.0, 0.0, 0.0);

        // Initial state should be None
        assert!(cube.get_state_at(&pos).is_err());

        // Set and get state
        assert!(cube.set_state_at(&pos, 42.0).is_ok());
        assert_eq!(cube.get_state_at(&pos).unwrap(), 42.0);
    }

    #[test]
    fn test_boundary_check() {
        let mut cube = Cube::new(2);
        let pos = Vector3D::new(2.0, 0.0, 0.0);

        assert!(matches!(
            cube.set_state_at(&pos, 1.0),
                         Err(QuantumError::BoundaryViolation)
        ));
    }

    #[test]
    fn test_clear() {
        let mut cube = Cube::new(2);
        let pos = Vector3D::new(0.0, 0.0, 0.0);

        assert!(cube.set_state_at(&pos, 1.0).is_ok());
        cube.clear();
        assert!(cube.get_state_at(&pos).is_err());
    }
}
