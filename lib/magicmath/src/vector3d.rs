// lib/magicmath/src/vector3d.rs

//! Three-Dimensional Vector Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 17:54:42 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::{MeshValue, CrystalAdd, CrystalSub, CrystalMul, CrystalDiv, Quantum, Phase};
use crate::core::HarmonyState;
use crate::constants::HARMONY_STABILITY_THRESHOLD;
use errors::{MathError, MathResult};
use scribe::{Scribe, native_string::String};

/// Three-dimensional vector with harmony state tracking
#[derive(Debug, Clone, Copy, PartialEq)]  // Added Copy and PartialEq traits
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub state: HarmonyState,
}

impl Vector3D {
    /// Create a new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            state: HarmonyState::new(),
        }
    }

    /// Check harmony stability
    fn check_stability(&self) -> MathResult<()> {
        if !self.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable);
        }
        Ok(())
    }

    /// Calculate vector magnitude
    pub fn magnitude(&self) -> MathResult<f64> {
        Ok((self.x * self.x + self.y * self.y + self.z * self.z).sqrt())
    }

    /// Calculate dot product
    pub fn dot(&self, other: &Self) -> MathResult<f64> {
        Ok(self.x * other.x + self.y * other.y + self.z * other.z)
    }

    /// Calculate cross product
    pub fn cross(&self, other: &Self) -> MathResult<Self> {
        Ok(Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        ))
    }
}

impl MeshValue for Vector3D {
    fn to_f64(&self) -> MathResult<f64> {
        self.magnitude()
    }

    fn from(value: f64) -> Self {
        Self::new(value, 0.0, 0.0)
    }

    fn coherence(&self) -> MathResult<f64> {
        let mag = self.magnitude()?;
        if mag <= 0.0 {
            return Err(MathError::InvalidParameter(String::from("Magnitude must be positive")));
        }
        Ok((self.x + self.y + self.z) / 3.0)
    }

    fn energy(&self) -> MathResult<f64> {
        self.magnitude()
    }

    fn magnitude(&self) -> MathResult<f64> {
        self.magnitude()
    }

    fn to_usize(&self) -> MathResult<usize> {
        Ok(self.magnitude()? as usize)
    }

    fn check_harmony_state(&self) -> bool {
        self.state.coherence >= HARMONY_STABILITY_THRESHOLD
    }
}

impl CrystalAdd for Vector3D {
    fn add(&self, other: &Self) -> MathResult<Self> {
        self.check_stability()?;
        other.check_stability()?;

        Ok(Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        ))
    }

    fn add_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.add(other)?;
        Ok(())
    }
}

impl CrystalSub for Vector3D {
    fn sub(&self, other: &Self) -> MathResult<Self> {
        self.check_stability()?;
        other.check_stability()?;

        Ok(Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        ))
    }

    fn sub_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.sub(other)?;
        Ok(())
    }
}

impl CrystalMul for Vector3D {
    fn mul(&self, other: &Self) -> MathResult<Self> {
        self.check_stability()?;
        other.check_stability()?;

        Ok(Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
        ))
    }

    fn mul_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.mul(other)?;
        Ok(())
    }
}

impl CrystalDiv for Vector3D {
    fn div(&self, other: &Self) -> MathResult<Self> {
        self.check_stability()?;
        other.check_stability()?;

        if other.magnitude()? == 0.0 {
            return Err(MathError::DivisionByZero);
        }

        Ok(Self::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z,
        ))
    }

    fn div_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.div(other)?;
        Ok(())
    }
}

impl Quantum for Vector3D {
    fn energy(&self) -> MathResult<f64> {
        self.magnitude()
    }

    fn phase(&self) -> MathResult<f64> {
        if self.x == 0.0 {
            return Err(MathError::DivisionByZero);
        }
        Ok((self.y / self.x).atan())
    }
}

impl Phase for Vector3D {
    fn phase_shift(&mut self, shift: f64) -> MathResult<()> {
        self.check_stability()?;
        let mag = self.magnitude()?;
        let current_phase = self.phase()?;
        let new_phase = current_phase + shift;
        self.x = mag * new_phase.cos();
        self.y = mag * new_phase.sin();
        // z component remains unchanged during phase shift
        Ok(())
    }
}

impl Scribe for Vector3D {
    fn scribe(&self) -> String {
        String::from(format!("({}, {}, {})", self.x, self.y, self.z).as_str())
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
    fn test_vector3d_magnitude() -> MathResult<()> {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude()?, 5.0);
        Ok(())
    }

    #[test]
    fn test_vector3d_dot_product() -> MathResult<()> {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2)?, 32.0);
        Ok(())
    }

    #[test]
    fn test_vector3d_cross_product() -> MathResult<()> {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2)?;
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
        Ok(())
    }

    #[test]
    fn test_harmony_state() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert!(v.check_harmony_state());
    }
}
