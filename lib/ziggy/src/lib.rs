//! Ziggy Vector Operations
//! ======================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 16:52:44 UTC
//! Version: 0.1.0
//! License: MIT

use std::fmt::{self, Display, Formatter};
use scribe::Scribe;

/// 3D Vector implementation in Zig
#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Display for Vector3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Vector3D({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Scribe for Vector3D {
    fn scribe(&self) -> scribe::native_string::String {
        let mut result = scribe::native_string::String::new();
        result.push_str(&format!("Vector3D({}, {}, {})", self.x, self.y, self.z));
        result
    }
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Add two vectors
    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtract two vectors
    pub fn sub(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// Multiply vector by scalar
    pub fn scale(&self, factor: f64) -> Vector3D {
        Vector3D {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    /// Calculate dot product
    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate cross product
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize vector
    pub fn normalize(&self) -> Vector3D {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Vector3D::new(0.0, 0.0, 0.0);
        }
        self.scale(1.0 / mag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector_add() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = v1.add(&v2);
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vector_sub() {
        let v1 = Vector3D::new(4.0, 5.0, 6.0);
        let v2 = Vector3D::new(1.0, 2.0, 3.0);
        let result = v1.sub(&v2);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_vector_scale() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let result = v.scale(2.0);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_vector_cross() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector3D::new(3.0, 0.0, 0.0);
        let result = v.normalize();
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }
}
