//! Trait Definitions for Crystal Lattice HPC Systems
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:27:10 UTC
//! Version: 0.1.0
//! License: MIT

use errors::MathError;

/// Trait for values that can be used in mesh computations
pub trait MeshValue {
    /// Convert the value to f64
    fn to_f64(&self) -> Result<f64, MathError>;

    /// Create from f64
    fn from(value: f64) -> Self;

    /// Get coherence value
    fn coherence(&self) -> Result<f64, MathError>;

    /// Get energy value
    fn energy(&self) -> Result<f64, MathError>;

    /// Get magnitude
    fn magnitude(&self) -> Result<f64, MathError>;

    /// Convert to usize
    fn to_usize(&self) -> Result<usize, MathError>;

    /// Basic arithmetic operations
    fn add(&self, other: &Self) -> Result<Self, MathError>
    where
    Self: Sized;

    fn sub(&self, other: &Self) -> Result<Self, MathError>
    where
    Self: Sized;

    fn mul(&self, other: &Self) -> Result<Self, MathError>
    where
    Self: Sized;

    fn div(&self, other: &Self) -> Result<Self, MathError>
    where
    Self: Sized;
}

// Implement MeshValue for f64
impl MeshValue for f64 {
    fn to_f64(&self) -> Result<f64, MathError> {
        Ok(*self)
    }

    fn from(value: f64) -> Self {
        value
    }

    fn coherence(&self) -> Result<f64, MathError> {
        Ok(1.0) // f64 values are always coherent
    }

    fn energy(&self) -> Result<f64, MathError> {
        Ok(self.abs())
    }

    fn magnitude(&self) -> Result<f64, MathError> {
        Ok(self.abs())
    }

    fn to_usize(&self) -> Result<usize, MathError> {
        if self.is_finite() && *self >= 0.0 {
            Ok(*self as usize)
        } else {
            Err(MathError::InvalidParameter("Cannot convert to usize".to_string()))
        }
    }

    fn add(&self, other: &Self) -> Result<Self, MathError> {
        Ok(self + other)
    }

    fn sub(&self, other: &Self) -> Result<Self, MathError> {
        Ok(self - other)
    }

    fn mul(&self, other: &Self) -> Result<Self, MathError> {
        Ok(self * other)
    }

    fn div(&self, other: &Self) -> Result<Self, MathError> {
        if *other == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(self / other)
        }
    }
}

/// Trait for quantum operations
pub trait Quantum {
    /// Get energy level
    fn energy(&self) -> Result<f64, MathError>;

    /// Get phase angle
    fn phase(&self) -> Result<f64, MathError>;
}

/// Trait for phase operations
pub trait Phase {
    /// Apply phase shift
    fn phase_shift(&mut self, shift: f64) -> Result<(), MathError>;
}

/// Trait for harmony state
pub trait HarmonyState {
    /// Get coherence value
    fn coherence(&self) -> Result<f64, MathError>;

    /// Get stability value
    fn stability(&self) -> Result<f64, MathError>;

    /// Get energy value
    fn energy(&self) -> Result<f64, MathError>;

    /// Get phase value
    fn phase(&self) -> Result<f64, MathError>;
}

/// Trait for harmony operations
pub trait HarmonyOperation {
    /// Get operation type
    fn operation_type(&self) -> Result<&'static str, MathError>;

    /// Get operation stability
    fn operation_stability(&self) -> Result<f64, MathError>;
}
