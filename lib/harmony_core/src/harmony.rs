//! Crystalline Harmony Module
//! ========================
//!
//! Core crystalline traits and types for the quantum harmony system.
//! Provides quantum-safe operations through crystalline lattice structures.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:34:08 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]

use crate::constants::*;

/// Core crystalline trait for types that can exist in quantum lattice space
pub trait Quantum: Clone + 'static {
    /// Gets the crystalline coherence value
    fn coherence(&self) -> f64;

    /// Checks if the crystalline lattice is stable
    fn is_stable(&self) -> bool {
        self.coherence() >= QUANTUM_STABILITY_THRESHOLD
    }

    /// Applies crystalline decoherence through lattice vibration
    fn decohere(&mut self);

    /// Restores crystalline coherence through lattice realignment
    fn recohere(&mut self);
}

/// Crystalline type for quantum lattice variance tracking
#[derive(Debug, Clone, Copy)]
pub struct QuantumPhantom<T: 'static> {
    /// Crystalline lattice configuration
    _lattice_config: [u8; 16],
    /// Type-specific quantum resonance
    _resonance: fn() -> Option<T>,
}

impl<T: 'static> QuantumPhantom<T> {
    /// Creates a new crystalline phantom with perfect symmetry
    pub const fn new() -> Self {
        Self {
            _lattice_config: [0; 16],
            _resonance: || None,
        }
    }

    /// Measures the crystalline lattice stability
    pub fn lattice_stability(&self) -> f64 {
        let mut stability = 0.0;
        for &config in self._lattice_config.iter() {
            stability += (config as f64) / 255.0;
        }
        stability / 16.0
    }
}

/// MeshValue trait for crystalline mathematical operations
pub trait MeshValue: Clone + 'static {
    /// Returns the crystalline zero state
    fn zero() -> Self;

    /// Adds two crystalline values maintaining lattice symmetry
    fn mesh_add(&self, other: &Self) -> Self;

    /// Subtracts two crystalline values preserving quantum state
    fn mesh_sub(&self, other: &Self) -> Self;

    /// Multiplies two crystalline values with coherence preservation
    fn mesh_mul(&self, other: &Self) -> Self;

    /// Divides two crystalline values maintaining stability
    fn mesh_div(&self, other: &Self) -> Self;

    /// Negates a crystalline value through lattice inversion
    fn mesh_neg(&self) -> Self;
}

/// MeshOps trait for crystalline vector operations in quantum space
pub trait MeshOps {
    /// Output type for mesh operations
    type Output;

    /// Adds two crystalline vectors preserving symmetry
    fn mesh_add(&self, rhs: &Self) -> Self::Output;

    /// Subtracts two crystalline vectors maintaining coherence
    fn mesh_sub(&self, rhs: &Self) -> Self::Output;

    /// Scales a crystalline vector by a quantum factor
    fn mesh_mul(&self, scalar: &f64) -> Self::Output;

    /// Divides a crystalline vector preserving quantum state
    fn mesh_div(&self, scalar: &f64) -> Self::Output;

    /// Inverts a crystalline vector through quantum reflection
    fn mesh_neg(&self) -> Self::Output;
}

// Crystalline implementations for primitive types
impl MeshValue for f64 {
    fn zero() -> Self { 0.0 }

    fn mesh_add(&self, other: &Self) -> Self {
        let result = self + other;
        if result.is_finite() { result } else { Self::zero() }
    }

    fn mesh_sub(&self, other: &Self) -> Self {
        let result = self - other;
        if result.is_finite() { result } else { Self::zero() }
    }

    fn mesh_mul(&self, other: &Self) -> Self {
        let result = self * other;
        if result.is_finite() { result } else { Self::zero() }
    }

    fn mesh_div(&self, other: &Self) -> Self {
        if *other != 0.0 {
            let result = self / other;
            if result.is_finite() { result } else { Self::zero() }
        } else {
            Self::zero()
        }
    }

    fn mesh_neg(&self) -> Self {
        let result = -self;
        if result.is_finite() { result } else { Self::zero() }
    }
}

impl MeshValue for i64 {
    fn zero() -> Self { 0 }

    fn mesh_add(&self, other: &Self) -> Self {
        self.saturating_add(*other)
    }

    fn mesh_sub(&self, other: &Self) -> Self {
        self.saturating_sub(*other)
    }

    fn mesh_mul(&self, other: &Self) -> Self {
        self.saturating_mul(*other)
    }

    fn mesh_div(&self, other: &Self) -> Self {
        if *other != 0 {
            self.checked_div(*other).unwrap_or_else(Self::zero)
        } else {
            Self::zero()
        }
    }

    fn mesh_neg(&self) -> Self {
        self.checked_neg().unwrap_or_else(Self::zero)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_phantom() {
        let phantom = QuantumPhantom::<i32>::new();
        assert!(phantom.lattice_stability() >= 0.0);
        assert!(phantom.lattice_stability() <= 1.0);
    }

    #[test]
    fn test_mesh_value_f64() {
        let a = 42.0;
        let b = 7.0;

        assert_eq!(a.mesh_add(&b), 49.0);
        assert_eq!(a.mesh_sub(&b), 35.0);
        assert_eq!(a.mesh_mul(&b), 294.0);
        assert_eq!(a.mesh_div(&b), 6.0);
        assert_eq!(a.mesh_neg(), -42.0);
    }

    #[test]
    fn test_mesh_value_i64() {
        let a = 42i64;
        let b = 7i64;

        assert_eq!(a.mesh_add(&b), 49);
        assert_eq!(a.mesh_sub(&b), 35);
        assert_eq!(a.mesh_mul(&b), 294);
        assert_eq!(a.mesh_div(&b), 6);
        assert_eq!(a.mesh_neg(), -42);
    }

    #[test]
    fn test_mesh_value_safety() {
        let max = i64::MAX;
        let min = i64::MIN;

        // Test overflow protection
        assert_eq!(max.mesh_add(&1), i64::MAX);
        assert_eq!(min.mesh_sub(&1), i64::MIN);

        // Test division by zero
        assert_eq!(42i64.mesh_div(&0), 0);
        assert_eq!(42.0f64.mesh_div(&0.0), 0.0);
    }
}
