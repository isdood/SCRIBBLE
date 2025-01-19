//! Vector Mathematics and Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 21:27:24 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use errors::MathError;
use scribe::Scribe;
use scribe::native_string::String; // Import the correct String type from scribe
use crate::resonance::{Quantum, Phase};
use crate::add::Add; // Import Add trait
use crate::sub::Sub; // Import Sub trait
use crate::mul::Mul; // Import Mul trait
use crate::div::Div; // Import Div trait

/// Three-dimensional vector
#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Four-dimensional vector
#[derive(Debug, Clone, Copy)]
pub struct Vector4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector3D {
    /// Create a new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize vector
    pub fn normalize(&mut self) -> Result<(), MathError> {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Err(MathError::DivisionByZero);
        }
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        Ok(())
    }

    /// Calculate dot product
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate cross product
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Vector4D {
    /// Create a new 4D vector
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// Normalize vector
    pub fn normalize(&mut self) -> Result<(), MathError> {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Err(MathError::DivisionByZero);
        }
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        self.w /= mag;
        Ok(())
    }

    /// Calculate dot product
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl MeshValue for Vector3D {
    fn to_f64(&self) -> Result<f64, MathError> {
        Ok(self.magnitude())
    }

    fn from(value: f64) -> Self {
        Self::new(value, 0.0, 0.0)
    }

    fn coherence(&self) -> Result<f64, MathError> {
        if self.magnitude() <= 0.0 {
            return Err(MathError::InvalidParameter("Magnitude must be positive".to_string()));
        }
        Ok((self.x + self.y + self.z) / 3.0)
    }

    fn energy(&self) -> Result<f64, MathError> {
        Ok(self.magnitude())
    }

    fn magnitude(&self) -> Result<f64, MathError> {
        Ok(self.magnitude())
    }

    fn to_usize(&self) -> Result<usize, MathError> {
        Ok(self.magnitude() as usize)
    }
}

impl Quantum for Vector3D {
    fn energy(&self) -> Result<f64, MathError> {
        Ok(self.magnitude())
    }

    fn phase(&self) -> Result<f64, MathError> {
        if self.x == 0.0 {
            return Err(MathError::DivisionByZero);
        }
        Ok((self.y / self.x).atan())
    }
}

impl Phase for Vector3D {
    fn phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        let mag = self.magnitude();
        let current_phase = self.phase()?;
        let new_phase = current_phase + shift;
        self.x = mag * new_phase.cos();
        self.y = mag * new_phase.sin();
        Ok(())
    }
}

impl Scribe for Vector3D {
    fn scribe(&self) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(&self.x.scribe().to_str());
        result.push_str(", ");
        result.push_str(&self.y.scribe().to_str());
        result.push_str(", ");
        result.push_str(&self.z.scribe().to_str());
        result.push_str(")");
        result
    }
}

impl Scribe for Vector4D {
    fn scribe(&self) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(&self.x.scribe().to_str());
        result.push_str(", ");
        result.push_str(&self.y.scribe().to_str());
        result.push_str(", ");
        result.push_str(&self.z.scribe().to_str());
        result.push_str(", ");
        result.push_str(&self.w.scribe().to_str());
        result.push_str(")");
        result
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl Mul<f64> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
}

impl Div<f64> for Vector3D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Division by zero");
        }
        Self::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        )
    }
}

// Similar implementations for Vector4D
impl Add for Vector4D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Vector4D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul<f64> for Vector4D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs,
        )
    }
}

impl Div<f64> for Vector4D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Division by zero");
        }
        Self::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
            self.w / rhs,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::PI;

    #[test]
    fn test_vector3d_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector3d_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector3d_normalize() {
        let mut v = Vector3D::new(3.0, 4.0, 0.0);
        v.normalize().unwrap();
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_vector3d_dot() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_vector3d_cross() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross.z, 1.0);
    }

    #[test]
    fn test_vector3d_quantum() {
        let v = Vector3D::new(1.0, 1.0, 0.0);
        assert_eq!(v.phase().unwrap(), PI / 4.0);
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let sum = v1 + v2;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }
}
