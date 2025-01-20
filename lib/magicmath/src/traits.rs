//! Core Traits for Crystal Computing Operations
//! ======================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 23:56:38 UTC
//! Version: 0.1.0
//! License: MIT

use errors::MathResult;

/// Core trait for mesh-based values
pub trait MeshValue: Sized {
    fn to_f64(&self) -> MathResult<f64>;
    fn from(value: f64) -> Self;
    fn coherence(&self) -> MathResult<f64>;
    fn energy(&self) -> MathResult<f64>;
    fn magnitude(&self) -> MathResult<f64>;
    fn to_usize(&self) -> MathResult<usize>;
    fn check_harmony_state(&self) -> bool;
}

/// Crystal-aware addition operations
pub trait CrystalAdd: Sized {
    fn add(&self, other: &Self) -> MathResult<Self>;
    fn add_assign(&mut self, other: &Self) -> MathResult<()>;
}

/// Crystal-aware subtraction operations
pub trait CrystalSub: Sized {
    fn sub(&self, other: &Self) -> MathResult<Self>;
    fn sub_assign(&mut self, other: &Self) -> MathResult<()>;
}

/// Crystal-aware multiplication operations
pub trait CrystalMul: Sized {
    fn mul(&self, other: &Self) -> MathResult<Self>;
    fn mul_assign(&mut self, other: &Self) -> MathResult<()>;
}

/// Crystal-aware division operations
pub trait CrystalDiv: Sized {
    fn div(&self, other: &Self) -> MathResult<Self>;
    fn div_assign(&mut self, other: &Self) -> MathResult<()>;
}

/// Quantum state operations
pub trait Quantum {
    fn energy(&self) -> MathResult<f64>;
    fn phase(&self) -> MathResult<f64>;
}

/// Phase operations
pub trait Phase {
    fn phase_shift(&mut self, shift: f64) -> MathResult<()>;
}

/// Resonance operations
pub trait Resonance {
    fn frequency(&self) -> MathResult<f64>;
    fn amplitude(&self) -> MathResult<f64>;
}
