//! Vector Operations for Crystal Computing
//! ================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:58:37 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::{Add, Sub, Mul, Div};
use meshmath::sqrt;
use crate::{
    errors::VectorError,
    align::{Alignment, AlignmentState},
    harmony::QuantumArithmetic,
    idk::ShardUninit,
};

/// Three-dimensional vector
#[derive(Debug, Clone, Copy)]
pub struct Vector3D<T> {
    /// X component
    pub x: T,
    /// Y component
    pub y: T,
    /// Z component
    pub z: T,
    /// Core alignment
    alignment: ShardUninit<Alignment>,
}

/// Four-dimensional vector
#[derive(Debug, Clone, Copy)]
pub struct Vector4D<T> {
    /// X component
    pub x: T,
    /// Y component
    pub y: T,
    /// Z component
    pub z: T,
    /// W component
    pub w: T,
    /// Core alignment
    alignment: ShardUninit<Alignment>,
}

impl<T: Copy + QuantumArithmetic> Vector3D<T> {
    /// Create a new 3D vector
    pub fn new(x: T, y: T, z: T) -> Self {
        let pos = Vector3D::new_raw(x, y, z);
        Self {
            x,
            y,
            z,
            alignment: ShardUninit::new(),
        }
    }

    /// Create a new raw 3D vector without alignment
    pub const fn new_raw(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            alignment: ShardUninit::new(),
        }
    }

    /// Calculate dot product
    pub fn dot(&self, other: &Self) -> Result<T, VectorError> {
        if !self.is_valid() || !other.is_valid() {
            return Err(VectorError::InvalidDimension);
        }
        Ok(self.x * other.x + self.y * other.y + self.z * other.z)
    }

    /// Calculate cross product
    pub fn cross(&self, other: &Self) -> Result<Self, VectorError> {
        if !self.is_valid() || !other.is_valid() {
            return Err(VectorError::InvalidDimension);
        }
        Ok(Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        ))
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> Result<T, VectorError> {
        if !self.is_valid() {
            return Err(VectorError::InvalidDimension);
        }
        let sum = self.x * self.x + self.y * self.y + self.z * self.z;
        Ok(sqrt(sum))
    }

    /// Normalize vector
    pub fn normalize(&mut self) -> Result<(), VectorError> {
        let mag = self.magnitude()?;
        if mag.is_zero() {
            return Err(VectorError::DivisionByZero);
        }
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
        Ok(())
    }

    /// Check if vector is valid
    pub fn is_valid(&self) -> bool {
        self.x.is_valid() && self.y.is_valid() && self.z.is_valid()
    }

    /// Get current alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        unsafe {
            self.alignment.get_ref()
            .map_or(AlignmentState::Unknown, |a| a.get_state())
        }
    }
}

impl<T: Copy + QuantumArithmetic> Vector4D<T> {
    /// Create a new 4D vector
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x,
            y,
            z,
            w,
            alignment: ShardUninit::new(),
        }
    }

    /// Calculate dot product
    pub fn dot(&self, other: &Self) -> Result<T, VectorError> {
        if !self.is_valid() || !other.is_valid() {
            return Err(VectorError::InvalidDimension);
        }
        Ok(self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w)
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> Result<T, VectorError> {
        if !self.is_valid() {
            return Err(VectorError::InvalidDimension);
        }
        let sum = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        Ok(sqrt(sum))
    }

    /// Normalize vector
    pub fn normalize(&mut self) -> Result<(), VectorError> {
        let mag = self.magnitude()?;
        if mag.is_zero() {
            return Err(VectorError::DivisionByZero);
        }
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
        self.w = self.w / mag;
        Ok(())
    }

    /// Check if vector is valid
    pub fn is_valid(&self) -> bool {
        self.x.is_valid() && self.y.is_valid() &&
        self.z.is_valid() && self.w.is_valid()
    }

    /// Get current alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        unsafe {
            self.alignment.get_ref()
            .map_or(AlignmentState::Unknown, |a| a.get_state())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl QuantumArithmetic for f64 {
        fn zero() -> Self { 0.0 }
        fn one() -> Self { 1.0 }
        fn is_zero(&self) -> bool { *self == 0.0 }
        fn is_valid(&self) -> bool { self.is_finite() }
    }

    #[test]
    fn test_vector3d_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert!(v.is_valid());
    }

    #[test]
    fn test_vector3d_dot_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(1.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2).unwrap(), 1.0);
    }

    #[test]
    fn test_vector3d_cross_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let v3 = v1.cross(&v2).unwrap();
        assert_eq!(v3.z, 1.0);
    }

    #[test]
    fn test_vector4d_creation() {
        let v = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        assert!(v.is_valid());
    }

    #[test]
    fn test_vector4d_magnitude() {
        let v = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(v.magnitude().unwrap(), 1.0);
    }
}
