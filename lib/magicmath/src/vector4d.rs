//! Four-Dimensional Vector Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 02:07:58 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::{MeshValue, CrystalAdd, CrystalSub, CrystalMul, CrystalDiv, Quantum, Phase};
use crate::core::HarmonyState;
use crate::constants::HARMONY_STABILITY_THRESHOLD;
use errors::{MathError, MathResult};
use scribe::{Scribe, native_string::String};

/// Four-dimensional vector with harmony state tracking
#[derive(Debug, Clone)]
pub struct Vector4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
    pub state: HarmonyState,
}

impl Vector4D {
    /// Create a new 4D vector
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x,
            y,
            z,
            w,
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
        Ok((self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt())
    }

    /// Calculate dot product
    pub fn dot(&self, other: &Self) -> MathResult<f64> {
        Ok(self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w)
    }
}

impl MeshValue for Vector4D {
    fn to_f64(&self) -> MathResult<f64> {
        self.magnitude()
    }

    fn from(value: f64) -> Self {
        Self::new(value, 0.0, 0.0, 0.0)
    }

    fn coherence(&self) -> MathResult<f64> {
        let mag = self.magnitude()?;
        if mag <= 0.0 {
            return Err(MathError::InvalidParameter(String::from("Magnitude must be positive")));
        }
        Ok((self.x + self.y + self.z + self.w) / 4.0)
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

impl CrystalAdd for Vector4D {
    fn add(&self, other: &Self) -> MathResult<Self> {
        self.check_stability()?;
        other.check_stability()?;

        Ok(Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        ))
    }

    fn add_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.add(other)?;
        Ok(())
    }
}

impl CrystalSub for Vector4D {
    fn sub(&self, other: &Self) -> MathResult<Self> {
        self.check_stability()?;
        other.check_stability()?;

        Ok(Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        ))
    }

    fn sub_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.sub(other)?;
        Ok(())
    }
}

impl CrystalMul for Vector4D {
    fn mul(&self, other: &Self) -> MathResult<Self> {
        self.check_stability()?;
        other.check_stability()?;

        Ok(Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        ))
    }

    fn mul_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.mul(other)?;
        Ok(())
    }
}

impl CrystalDiv for Vector4D {
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
            self.w / other.w,
        ))
    }

    fn div_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.div(other)?;
        Ok(())
    }
}

impl Quantum for Vector4D {
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

impl Phase for Vector4D {
    fn phase_shift(&mut self, shift: f64) -> MathResult<()> {
        self.check_stability()?;
        let mag = self.magnitude()?;
        let current_phase = self.phase()?;
        let new_phase = current_phase + shift;
        self.x = mag * new_phase.cos();
        self.y = mag * new_phase.sin();
        // z and w components remain unchanged during phase shift
        Ok(())
    }
}

impl Scribe for Vector4D {
    fn scribe(&self) -> String {
        String::from(format!("({}, {}, {}, {})", self.x, self.y, self.z, self.w).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector4d_creation() {
        let v = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);
    }

    #[test]
    fn test_vector4d_magnitude() -> MathResult<()> {
        let v = Vector4D::new(2.0, 0.0, 0.0, 0.0);
        assert_eq!(v.magnitude()?, 2.0);
        Ok(())
    }

    #[test]
    fn test_vector4d_dot_product() -> MathResult<()> {
        let v1 = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4D::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(v1.dot(&v2)?, 70.0);
        Ok(())
    }

    #[test]
    fn test_harmony_state() {
        let v = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        assert!(v.check_harmony_state());
    }
}
