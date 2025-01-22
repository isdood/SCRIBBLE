//! Core traits for SIMD operations
//! Created: 2025-01-22 00:23:05
//! Author: isdood

use std::ops::{Add, Sub, Mul, Div};
use num_traits::{Float, Zero, One};

/// Trait for values that can be used in SIMD operations
pub trait SIMDValue:
    Copy + Clone + Default +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Float +
    Zero +
    One +
    Send +
    Sync +
    'static
{
    /// Returns the type ID for SIMD operations
    fn type_id() -> TypeId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeId {
    F32,
    F64,
}

impl SIMDValue for f32 {
    fn type_id() -> TypeId {
        TypeId::F32
    }
}

impl SIMDValue for f64 {
    fn type_id() -> TypeId {
        TypeId::F64
    }
}
