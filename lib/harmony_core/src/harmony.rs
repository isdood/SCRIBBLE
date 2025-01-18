//! Crystalline Harmony Implementation
//! ============================
//!
//! Core quantum harmony traits with crystalline resonance
//! tracking and mesh value operations.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:59:23 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::{Add, Sub, Mul, Div, Neg};
use crate::constants::QUANTUM_STABILITY_THRESHOLD;

/// Core quantum trait for crystalline coherence
pub trait Quantum {
    /// Gets the current quantum coherence value
    fn coherence(&self) -> f64;

    /// Checks if the quantum state is stable
    fn is_stable(&self) -> bool {
        self.coherence() >= QUANTUM_STABILITY_THRESHOLD
    }

    /// Applies quantum decoherence
    fn decohere(&mut self);

    /// Restores quantum coherence
    fn recohere(&mut self);
}

/// Trait for quantum mesh operations
pub trait MeshOps {
    /// Output type for mesh operations
    type Output;

    /// Performs mesh addition
    fn mesh_add(&self, rhs: &Self) -> Self::Output;

    /// Performs mesh subtraction
    fn mesh_sub(&self, rhs: &Self) -> Self::Output;

    /// Performs mesh multiplication by scalar
    fn mesh_mul(&self, scalar: &f64) -> Self::Output;

    /// Performs mesh division by scalar
    fn mesh_div(&self, scalar: &f64) -> Self::Output;

    /// Performs mesh negation
    fn mesh_neg(&self) -> Self::Output;
}

/// Trait for quantum mesh values
pub trait MeshValue: Clone + 'static + Sized {
    /// Creates a zero value
    fn zero() -> Self;

    /// Converts from f64
    fn from_f64(v: f64) -> Option<Self>;

    /// Converts to f64
    fn to_f64(&self) -> f64;

    /// Performs mesh addition
    fn mesh_add(&self, rhs: &Self) -> Self;

    /// Performs mesh subtraction
    fn mesh_sub(&self, rhs: &Self) -> Self;

    /// Performs mesh multiplication
    fn mesh_mul(&self, rhs: &Self) -> Self;

    /// Performs mesh division
    fn mesh_div(&self, rhs: &Self) -> Self;

    /// Performs mesh negation
    fn mesh_neg(&self) -> Self;
}

impl MeshValue for f64 {
    fn zero() -> Self { 0.0 }

    fn from_f64(v: f64) -> Option<Self> {
        Some(v)
    }

    fn to_f64(&self) -> f64 {
        *self
    }

    fn mesh_add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mesh_sub(&self, rhs: &Self) -> Self {
        self - rhs
    }

    fn mesh_mul(&self, rhs: &Self) -> Self {
        self * rhs
    }

    fn mesh_div(&self, rhs: &Self) -> Self {
        if *rhs != 0.0 { self / rhs } else { 0.0 }
    }

    fn mesh_neg(&self) -> Self {
        -self
    }
}

impl MeshValue for i64 {
    fn zero() -> Self { 0 }

    fn from_f64(v: f64) -> Option<Self> {
        Some(libm::round(v) as i64)
    }

    fn to_f64(&self) -> f64 {
        *self as f64
    }

    fn mesh_add(&self, rhs: &Self) -> Self {
        self.saturating_add(*rhs)
    }

    fn mesh_sub(&self, rhs: &Self) -> Self {
        self.saturating_sub(*rhs)
    }

    fn mesh_mul(&self, rhs: &Self) -> Self {
        self.saturating_mul(*rhs)
    }

    fn mesh_div(&self, rhs: &Self) -> Self {
        if *rhs != 0 { self.saturating_div(*rhs) } else { 0 }
    }

    fn mesh_neg(&self) -> Self {
        self.saturating_neg()
    }
}

impl MeshValue for u64 {
    fn zero() -> Self { 0 }

    fn from_f64(v: f64) -> Option<Self> {
        if v >= 0.0 {
            Some(libm::round(v) as u64)
        } else {
            None
        }
    }

    fn to_f64(&self) -> f64 {
        *self as f64
    }

    fn mesh_add(&self, rhs: &Self) -> Self {
        self.saturating_add(*rhs)
    }

    fn mesh_sub(&self, rhs: &Self) -> Self {
        self.saturating_sub(*rhs)
    }

    fn mesh_mul(&self, rhs: &Self) -> Self {
        self.saturating_mul(*rhs)
    }

    fn mesh_div(&self, rhs: &Self) -> Self {
        if *rhs != 0 { self.saturating_div(*rhs) } else { 0 }
    }

    fn mesh_neg(&self) -> Self {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_mesh_ops() {
        let a = 42.0f64;
        let b = 7.0f64;

        assert_eq!(a.mesh_add(&b), 49.0);
        assert_eq!(a.mesh_sub(&b), 35.0);
        assert_eq!(a.mesh_mul(&b), 294.0);
        assert_eq!(a.mesh_div(&b), 6.0);
        assert_eq!(a.mesh_neg(), -42.0);
    }

    #[test]
    fn test_i64_mesh_ops() {
        let a = 42i64;
        let b = 7i64;

        assert_eq!(a.mesh_add(&b), 49);
        assert_eq!(a.mesh_sub(&b), 35);
        assert_eq!(a.mesh_mul(&b), 294);
        assert_eq!(a.mesh_div(&b), 6);
        assert_eq!(a.mesh_neg(), -42);
    }

    #[test]
    fn test_u64_mesh_ops() {
        let a = 42u64;
        let b = 7u64;

        assert_eq!(a.mesh_add(&b), 49);
        assert_eq!(a.mesh_sub(&b), 35);
        assert_eq!(a.mesh_mul(&b), 294);
        assert_eq!(a.mesh_div(&b), 6);
        assert_eq!(a.mesh_neg(), 0);
    }
}
