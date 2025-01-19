//! Vector Types for Crystal Computing
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:33:25 UTC
//! Version: 0.1.0
//! License: MIT

use meshmath::sqrt;
use crate::errors::QuantumError;

/// Three-dimensional vector type for crystal lattice navigation
#[derive(Debug, Clone, PartialEq)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Four-dimensional vector type for quantum field operations
#[derive(Debug, Clone, PartialEq)]
pub struct Vector4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector3D<T>
where
T: Clone + Default + Into<f64>,
{
    /// Create a new 3D vector
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Create a zero vector
    pub fn zero() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }

    /// Calculate dot product using meshmath
    pub fn dot(&self, other: &Self) -> Result<f64, QuantumError> {
        let x = self.x.clone().into() * other.x.clone().into();
        let y = self.y.clone().into() * other.y.clone().into();
        let z = self.z.clone().into() * other.z.clone().into();

        Ok(x + y + z)
    }

    /// Calculate cross product using meshmath
    pub fn cross(&self, other: &Self) -> Result<Vector3D<f64>, QuantumError> {
        let x = self.y.clone().into() * other.z.clone().into()
        - self.z.clone().into() * other.y.clone().into();

        let y = self.z.clone().into() * other.x.clone().into()
        - self.x.clone().into() * other.z.clone().into();

        let z = self.x.clone().into() * other.y.clone().into()
        - self.y.clone().into() * other.x.clone().into();

        Ok(Vector3D::new(x, y, z))
    }

    /// Calculate magnitude using meshmath
    pub fn magnitude(&self) -> Result<f64, QuantumError> {
        let x = self.x.clone().into();
        let y = self.y.clone().into();
        let z = self.z.clone().into();
        Ok(sqrt(x * x + y * y + z * z))
    }
}

impl<T> Vector4D<T>
where
T: Clone + Default + Into<f64>,
{
    /// Create a new 4D vector
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Create a zero vector
    pub fn zero() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default(),
        }
    }

    /// Calculate dot product using meshmath
    pub fn dot(&self, other: &Self) -> Result<f64, QuantumError> {
        let x = self.x.clone().into() * other.x.clone().into();
        let y = self.y.clone().into() * other.y.clone().into();
        let z = self.z.clone().into() * other.z.clone().into();
        let w = self.w.clone().into() * other.w.clone().into();

        Ok(x + y + z + w)
    }

    /// Calculate magnitude using meshmath
    pub fn magnitude(&self) -> Result<f64, QuantumError> {
        let x = self.x.clone().into();
        let y = self.y.clone().into();
        let z = self.z.clone().into();
        let w = self.w.clone().into();
        Ok(sqrt(x * x + y * y + z * z + w * w))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector4d_creation() {
        let v = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);
    }

    #[test]
    fn test_vector_operations() {
        let v1: Vector3D<f64> = Vector3D::new(1.0, 0.0, 0.0);
        let v2: Vector3D<f64> = Vector3D::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2).unwrap();
        assert_eq!(cross, Vector3D::new(0.0, 0.0, 1.0));
    }
}
