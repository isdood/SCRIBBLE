//! Vector Mathematics and Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:35:50 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use errors::{MathError, MathResult};
use scribe::Scribe;
use scribe::native_string::String;
use crate::resonance::{Quantum, Phase};
use crate::core::HarmonyState;
use crate::constants::{HARMONY_STABILITY_THRESHOLD, HARMONY_COHERENCE_THRESHOLD};

/// Three-dimensional vector with harmony state
#[derive(Debug, Clone)]
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
}

impl MeshValue for Vector3D {
    fn to_f64(&self) -> MathResult<f64> {
        Ok(self.magnitude())
    }

    fn from(value: f64) -> Self {
        Self::new(value, 0.0, 0.0)
    }

    fn coherence(&self) -> MathResult<f64> {
        if self.magnitude() <= 0.0 {
            return Err(MathError::InvalidParameter("Magnitude must be positive".into()));
        }
        Ok((self.x + self.y + self.z) / 3.0)
    }

    fn energy(&self) -> MathResult<f64> {
        Ok(self.magnitude())
    }

    fn magnitude(&self) -> MathResult<f64> {
        Ok((self.x * self.x + self.y * self.y + self.z * self.z).sqrt())
    }

    fn to_usize(&self) -> MathResult<usize> {
        Ok(self.magnitude() as usize)
    }

    fn add(&self, other: &Self) -> MathResult<Self> {
        if self.state.coherence < HARMONY_STABILITY_THRESHOLD {
            return Err(MathError::HarmonyStateUnstable);
        }
        Ok(Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z
        ))
    }

    fn sub(&self, other: &Self) -> MathResult<Self> {
        if self.state.coherence < HARMONY_STABILITY_THRESHOLD {
            return Err(MathError::HarmonyStateUnstable);
        }
        Ok(Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z
        ))
    }

    fn mul(&self, other: &Self) -> MathResult<Self> {
        if self.state.coherence < HARMONY_STABILITY_THRESHOLD {
            return Err(MathError::HarmonyStateUnstable);
        }
        Ok(Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z
        ))
    }

    fn div(&self, other: &Self) -> MathResult<Self> {
        if other.magnitude() == 0.0 {
            return Err(MathError::DivisionByZero);
        }
        if self.state.coherence < HARMONY_STABILITY_THRESHOLD {
            return Err(MathError::HarmonyStateUnstable);
        }
        Ok(Self::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z
        ))
    }
}

impl Quantum for Vector3D {
    fn energy(&self) -> MathResult<f64> {
        Ok(self.magnitude())
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
        let mag = self.magnitude()?;
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
        result.push_str(&self.x.to_string());
        result.push_str(", ");
        result.push_str(&self.y.to_string());
        result.push_str(", ");
        result.push_str(&self.z.to_string());
        result.push_str(")");
        result
    }
}

// Similar implementations for Vector4D...
// [Previous Vector4D implementations]

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
        assert_eq!(v.magnitude().unwrap(), 5.0);
    }

    #[test]
    fn test_vector3d_add() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = v1.add(&v2).unwrap();
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vector3d_phase() {
        let v = Vector3D::new(1.0, 1.0, 0.0);
        assert_eq!(v.phase().unwrap(), PI / 4.0);
    }
}
