//! Core Traits for Crystal Computing Systems
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:43:54 UTC
//! Version: 0.1.0
//! License: MIT

use errors::MathResult;
use crate::core::HarmonyState;

/// Core trait for values that can be used in mesh computations
pub trait MeshValue: Sized {
    /// Convert the value to f64
    fn to_f64(&self) -> MathResult<f64>;

    /// Create from f64
    fn from(value: f64) -> Self;

    /// Get coherence value
    fn coherence(&self) -> MathResult<f64>;

    /// Get energy value
    fn energy(&self) -> MathResult<f64>;

    /// Get magnitude
    fn magnitude(&self) -> MathResult<f64>;

    /// Convert to usize
    fn to_usize(&self) -> MathResult<usize>;

    /// Basic arithmetic operations
    fn add(&self, other: &Self) -> MathResult<Self>;
    fn sub(&self, other: &Self) -> MathResult<Self>;
    fn mul(&self, other: &Self) -> MathResult<Self>;
    fn div(&self, other: &Self) -> MathResult<Self>;
}

/// Trait for quantum operations
pub trait Quantum {
    /// Get energy level
    fn energy(&self) -> MathResult<f64>;

    /// Get phase angle
    fn phase(&self) -> MathResult<f64>;
}

/// Trait for phase operations
pub trait Phase {
    /// Apply phase shift
    fn phase_shift(&mut self, shift: f64) -> MathResult<()>;
}

/// Trait for resonance operations
pub trait Resonance {
    /// Get resonance frequency
    fn frequency(&self) -> MathResult<f64>;

    /// Get resonance amplitude
    fn amplitude(&self) -> MathResult<f64>;

    /// Apply resonance transformation
    fn transform(&mut self, frequency: f64, amplitude: f64) -> MathResult<()>;
}

/// Trait for harmony operations
pub trait Harmony {
    /// Get harmony state
    fn get_state(&self) -> &HarmonyState;

    /// Get mutable harmony state
    fn get_state_mut(&mut self) -> &mut HarmonyState;

    /// Check harmony stability
    fn is_stable(&self) -> bool;

    /// Apply harmony transformation
    fn transform(&mut self, other: &Self) -> MathResult<()>;
}

/// Trait for complex number operations
pub trait Complex: Sized {
    /// Get real part
    fn real(&self) -> f64;

    /// Get imaginary part
    fn imag(&self) -> f64;

    /// Get magnitude
    fn magnitude(&self) -> f64;

    /// Get phase angle
    fn angle(&self) -> f64;

    /// Basic complex operations
    fn add(&self, other: &Self) -> MathResult<Self>;
    fn sub(&self, other: &Self) -> MathResult<Self>;
    fn mul(&self, other: &Self) -> MathResult<Self>;
    fn div(&self, other: &Self) -> MathResult<Self>;
}

/// Trait for fractal operations
pub trait Fractal {
    /// Get iteration count
    fn iterations(&self) -> usize;

    /// Get escape value
    fn escape_value(&self) -> f64;

    /// Check if point is in set
    fn contains(&self, x: f64, y: f64) -> bool;

    /// Calculate fractal value at point
    fn calculate(&self, x: f64, y: f64) -> MathResult<f64>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector3D;

    #[test]
    fn test_mesh_value_f64() {
        let value = 42.0f64;
        assert_eq!(f64::from(value), value);
    }

    #[test]
    fn test_vector_mesh_value() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert!(v.magnitude().is_ok());
        assert!(v.to_f64().is_ok());
    }

    #[test]
    fn test_quantum_operations() {
        let v = Vector3D::new(1.0, 1.0, 0.0);
        assert!(v.energy().is_ok());
        assert!(v.phase().is_ok());
    }

    #[test]
    fn test_phase_operations() {
        let mut v = Vector3D::new(1.0, 0.0, 0.0);
        assert!(v.phase_shift(std::f64::consts::PI / 2.0).is_ok());
    }
}
