//! Crystalline Vector Implementation
//! ============================
//!
//! Core quantum vector operations through crystalline
//! mesh structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 21:00:31 UTC
//! Version: 0.1.0
//! License: MIT

use crate::harmony::{MeshValue, MeshOps};
use libm;

/// A crystalline 3D vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D<T: MeshValue> {
    /// X component
    pub x: T,
    /// Y component
    pub y: T,
    /// Z component
    pub z: T,
}

/// A crystalline 4D vector for quantum operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector4D<T: MeshValue> {
    /// X component
    pub x: T,
    /// Y component
    pub y: T,
    /// Z component
    pub z: T,
    /// W component (quantum time)
    pub w: T,
}

impl<T: MeshValue> Vector3D<T> {
    /// Creates a new 3D vector
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Creates a zero vector
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Calculates the length squared
    pub fn length_squared(&self) -> f64 {
        let x = self.x.to_f64();
        let y = self.y.to_f64();
        let z = self.z.to_f64();
        x * x + y * y + z * z
    }

    /// Calculates the length
    pub fn length(&self) -> f64 {
        libm::sqrt(self.length_squared())
    }

    /// Normalizes the vector
    pub fn normalize(&self) -> Option<Self> {
        let len = self.length();
        if len == 0.0 {
            None
        } else {
            Some(Self {
                x: T::from_f64(self.x.to_f64() / len)?,
                 y: T::from_f64(self.y.to_f64() / len)?,
                 z: T::from_f64(self.z.to_f64() / len)?,
            })
        }
    }
}

impl<T: MeshValue> Vector4D<T> {
    /// Creates a new 4D vector
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a zero vector
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    /// Calculates the length squared in 4D space
    pub fn length_squared(&self) -> f64 {
        let x = self.x.to_f64();
        let y = self.y.to_f64();
        let z = self.z.to_f64();
        let w = self.w.to_f64();
        x * x + y * y + z * z + w * w
    }

    /// Calculates the length in 4D space
    pub fn length(&self) -> f64 {
        libm::sqrt(self.length_squared())
    }

    /// Normalizes the 4D vector
    pub fn normalize(&self) -> Option<Self> {
        let len = self.length();
        if len == 0.0 {
            None
        } else {
            Some(Self {
                x: T::from_f64(self.x.to_f64() / len)?,
                 y: T::from_f64(self.y.to_f64() / len)?,
                 z: T::from_f64(self.z.to_f64() / len)?,
                 w: T::from_f64(self.w.to_f64() / len)?,
            })
        }
    }
}

impl<T: MeshValue> MeshOps for Vector3D<T> {
    type Output = Self;

    fn mesh_add(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x.mesh_add(&rhs.x),
            y: self.y.mesh_add(&rhs.y),
            z: self.z.mesh_add(&rhs.z),
        }
    }

    fn mesh_sub(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x.mesh_sub(&rhs.x),
            y: self.y.mesh_sub(&rhs.y),
            z: self.z.mesh_sub(&rhs.z),
        }
    }

    fn mesh_mul(&self, scalar: &f64) -> Self::Output {
        Self {
            x: T::from_f64(self.x.to_f64() * scalar).unwrap_or(T::zero()),
            y: T::from_f64(self.y.to_f64() * scalar).unwrap_or(T::zero()),
            z: T::from_f64(self.z.to_f64() * scalar).unwrap_or(T::zero()),
        }
    }

    fn mesh_div(&self, scalar: &f64) -> Self::Output {
        if *scalar != 0.0 {
            self.mesh_mul(&(1.0 / scalar))
        } else {
            Self::zero()
        }
    }

    fn mesh_neg(&self) -> Self::Output {
        Self {
            x: self.x.mesh_neg(),
            y: self.y.mesh_neg(),
            z: self.z.mesh_neg(),
        }
    }
}

impl<T: MeshValue> MeshOps for Vector4D<T> {
    type Output = Self;

    fn mesh_add(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x.mesh_add(&rhs.x),
            y: self.y.mesh_add(&rhs.y),
            z: self.z.mesh_add(&rhs.z),
            w: self.w.mesh_add(&rhs.w),
        }
    }

    fn mesh_sub(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x.mesh_sub(&rhs.x),
            y: self.y.mesh_sub(&rhs.y),
            z: self.z.mesh_sub(&rhs.z),
            w: self.w.mesh_sub(&rhs.w),
        }
    }

    fn mesh_mul(&self, scalar: &f64) -> Self::Output {
        Self {
            x: T::from_f64(self.x.to_f64() * scalar).unwrap_or(T::zero()),
            y: T::from_f64(self.y.to_f64() * scalar).unwrap_or(T::zero()),
            z: T::from_f64(self.z.to_f64() * scalar).unwrap_or(T::zero()),
            w: T::from_f64(self.w.to_f64() * scalar).unwrap_or(T::zero()),
        }
    }

    fn mesh_div(&self, scalar: &f64) -> Self::Output {
        if *scalar != 0.0 {
            self.mesh_mul(&(1.0 / scalar))
        } else {
            Self::zero()
        }
    }

    fn mesh_neg(&self) -> Self::Output {
        Self {
            x: self.x.mesh_neg(),
            y: self.y.mesh_neg(),
            z: self.z.mesh_neg(),
            w: self.w.mesh_neg(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_basics() {
        let v = Vector3D::new(1.0f64, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);

        let zero = Vector3D::<f64>::zero();
        assert_eq!(zero.length(), 0.0);
    }

    #[test]
    fn test_vector4d_basics() {
        let v = Vector4D::new(1.0f64, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);

        let zero = Vector4D::<f64>::zero();
        assert_eq!(zero.length(), 0.0);
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3D::new(1.0f64, 2.0, 3.0);
        let v2 = Vector3D::new(4.0f64, 5.0, 6.0);

        let sum = v1.mesh_add(&v2);
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }

    #[test]
    fn test_vector_normalization() {
        let v = Vector3D::new(3.0f64, 4.0, 0.0);
        let normalized = v.normalize().unwrap();
        assert!((normalized.length() - 1.0).abs() < 1e-10);
    }
}
