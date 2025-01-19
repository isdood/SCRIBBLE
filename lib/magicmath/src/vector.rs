//! Vector Mathematics and Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:25:25 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use errors::MathError;
use scribe::Scribe;
use scribe::native_string::String;
use crate::resonance::{Quantum, Phase};
use crate::core::HarmonyState;
use crate::add::HarmonyAdd;
use crate::sub::HarmonySub;
use crate::mul::HarmonyMul;
use crate::div::HarmonyDiv;

/// Three-dimensional vector with harmony state
#[derive(Debug, Clone)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub state: HarmonyState,
}

/// Four-dimensional vector with harmony state
#[derive(Debug, Clone)]
pub struct Vector4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
    pub state: HarmonyState,
}

impl Vector3D {
    /// Create a new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            state: HarmonyState::new()
        }
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
        let div = HarmonyDiv::new(mag);
        self.x = HarmonyDiv::new(self.x).div(&div)?.value;
        self.y = HarmonyDiv::new(self.y).div(&div)?.value;
        self.z = HarmonyDiv::new(self.z).div(&div)?.value;
        Ok(())
    }

    /// Calculate dot product using harmony-aware multiplication
    pub fn dot(&self, other: &Self) -> Result<f64, MathError> {
        let x_mul = HarmonyMul::new(self.x).mul(&HarmonyMul::new(other.x))?.value;
        let y_mul = HarmonyMul::new(self.y).mul(&HarmonyMul::new(other.y))?.value;
        let z_mul = HarmonyMul::new(self.z).mul(&HarmonyMul::new(other.z))?.value;
        Ok(x_mul + y_mul + z_mul)
    }

    /// Calculate cross product using harmony-aware operations
    pub fn cross(&self, other: &Self) -> Result<Self, MathError> {
        let x = HarmonyMul::new(self.y).mul(&HarmonyMul::new(other.z))?.value -
        HarmonyMul::new(self.z).mul(&HarmonyMul::new(other.y))?.value;
        let y = HarmonyMul::new(self.z).mul(&HarmonyMul::new(other.x))?.value -
        HarmonyMul::new(self.x).mul(&HarmonyMul::new(other.z))?.value;
        let z = HarmonyMul::new(self.x).mul(&HarmonyMul::new(other.y))?.value -
        HarmonyMul::new(self.y).mul(&HarmonyMul::new(other.x))?.value;

        Ok(Self::new(x, y, z))
    }

    /// Add vectors using harmony-aware addition
    pub fn add(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self::new(
            HarmonyAdd::new(self.x).add(&HarmonyAdd::new(other.x))?.value,
                     HarmonyAdd::new(self.y).add(&HarmonyAdd::new(other.y))?.value,
                     HarmonyAdd::new(self.z).add(&HarmonyAdd::new(other.z))?.value,
        ))
    }

    /// Subtract vectors using harmony-aware subtraction
    pub fn sub(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self::new(
            HarmonySub::new(self.x).sub(&HarmonySub::new(other.x))?.value,
                     HarmonySub::new(self.y).sub(&HarmonySub::new(other.y))?.value,
                     HarmonySub::new(self.z).sub(&HarmonySub::new(other.z))?.value,
        ))
    }

    /// Multiply vector by scalar using harmony-aware multiplication
    pub fn mul(&self, scalar: f64) -> Result<Self, MathError> {
        Ok(Self::new(
            HarmonyMul::new(self.x).mul(&HarmonyMul::new(scalar))?.value,
                     HarmonyMul::new(self.y).mul(&HarmonyMul::new(scalar))?.value,
                     HarmonyMul::new(self.z).mul(&HarmonyMul::new(scalar))?.value,
        ))
    }

    /// Divide vector by scalar using harmony-aware division
    pub fn div(&self, scalar: f64) -> Result<Self, MathError> {
        let scalar_div = HarmonyDiv::new(scalar);
        Ok(Self::new(
            HarmonyDiv::new(self.x).div(&scalar_div)?.value,
                     HarmonyDiv::new(self.y).div(&scalar_div)?.value,
                     HarmonyDiv::new(self.z).div(&scalar_div)?.value,
        ))
    }
}

// Similar implementations for Vector4D...
// [Previous Vector4D implementations with harmony-aware operations]

impl MeshValue for Vector3D {
    // [Previous MeshValue implementation]
}

impl Quantum for Vector3D {
    // [Previous Quantum implementation]
}

impl Phase for Vector3D {
    // [Previous Phase implementation]
}

impl Scribe for Vector3D {
    // [Previous Scribe implementation]
}

impl Scribe for Vector4D {
    // [Previous Scribe implementation]
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
    fn test_vector3d_harmony_add() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let sum = v1.add(&v2).unwrap();
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }

    #[test]
    fn test_vector3d_harmony_mul() {
        let v = Vector3D::new(2.0, 3.0, 4.0);
        let result = v.mul(2.0).unwrap();
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 8.0);
    }

    #[test]
    fn test_vector3d_harmony_div() {
        let v = Vector3D::new(4.0, 6.0, 8.0);
        let result = v.div(2.0).unwrap();
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 4.0);
    }

    #[test]
    #[should_panic(expected = "DivisionByZero")]
    fn test_vector3d_harmony_div_by_zero() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let _result = v.div(0.0).unwrap();
    }
}
