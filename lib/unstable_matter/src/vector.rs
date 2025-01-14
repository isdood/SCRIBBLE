// lib/unstable_matter/src/vector.rs
/// Vector-Space-Mesh Memory System - Vector Implementation
/// Last Updated: 2025-01-14 00:50:29 UTC
/// Author: isdood
/// Current User: isdood

use core::ops::{Add, Sub, Mul};
use core::default::Default;
use libm;
use crate::helium::Helium;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<T: PartialEq> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// Implement for general numeric types with required traits
impl<T> Vector3D<T>
where
T: PartialEq + Add<Output = T> + Mul<Output = T> + Copy + Default
{
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn squared_magnitude(&self) -> T {
        self.dot(self)
    }
}

// Specific implementation for f64 with quantum-aware operations
impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        libm::sqrt(self.squared_magnitude())
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            *self
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn angle_between(&self, other: &Self) -> f64 {
        let dot = self.dot(other);
        let mags = self.magnitude() * other.magnitude();
        if mags == 0.0 {
            0.0
        } else {
            libm::acos(dot / mags)
        }
    }

    // Quantum-aware operations for mesh system
    pub fn to_quantum(&self) -> Helium<Self> {
        Helium::new(*self)
    }
}

// Implementation for integer vectors
impl Vector3D<isize> {
    pub fn magnitude(&self) -> f64 {
        libm::sqrt(self.squared_magnitude() as f64)
    }

    pub fn to_float(&self) -> Vector3D<f64> {
        Vector3D {
            x: self.x as f64,
            y: self.y as f64,
            z: self.z as f64,
        }
    }
}

pub type FloatVector3D = Vector3D<f64>;
pub type IntVector3D = Vector3D<isize>;
pub type MeshVector = Vector3D<f64>;

impl<T: PartialEq + Add<Output = T>> Add for Vector3D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: PartialEq + Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: PartialEq + Mul<Output = T> + Copy> Mul<T> for Vector3D<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::PI;

    #[test]
    fn test_vector_operations() {
        let v1 = FloatVector3D::new(1.0, 0.0, 0.0);
        let v2 = FloatVector3D::new(0.0, 1.0, 0.0);

        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v1.cross(&v2), Vector3D::new(0.0, 0.0, 1.0));
        assert_eq!(v1.angle_between(&v2), PI / 2.0);
    }

    #[test]
    fn test_quantum_operations() {
        let v = FloatVector3D::new(1.0, 0.0, 0.0);
        let quantum_v = v.to_quantum();
        let (loaded_v, coherence) = quantum_v.quantum_load(core::sync::atomic::Ordering::SeqCst);
        assert_eq!(v, loaded_v);
        assert!(coherence <= 1.0);
    }

    #[test]
    fn test_int_vector_conversion() {
        let iv = IntVector3D::new(1, 0, 0);
        let fv = iv.to_float();
        assert_eq!(fv, FloatVector3D::new(1.0, 0.0, 0.0));
    }
}
