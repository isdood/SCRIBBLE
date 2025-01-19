//! Mesh Value Traits Module
//! =====================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 08:55:47 UTC
//! Last Updated: 2025-01-19 08:55:47 UTC
//! Version: 0.1.0
//! License: MIT

use super::errors::MathResult;

/// Trait for mesh-compatible numeric types
pub trait MeshValue: Copy + Clone + core::fmt::Debug {
    fn mesh_add(self, other: Self) -> MathResult<Self>;
    fn mesh_sub(self, other: Self) -> MathResult<Self>;
    fn mesh_mul(self, other: Self) -> MathResult<Self>;
    fn mesh_div(self, other: Self) -> MathResult<Self>;
    fn mesh_neg(self) -> MathResult<Self>;
    fn mesh_magnitude(self) -> MathResult<f64>;
    fn mesh_normalize(self) -> MathResult<Self>;
    fn mesh_zero() -> Self;
    fn mesh_one() -> Self;
    fn as_f64(self) -> MathResult<f64>;
    fn from_f64(value: f64) -> MathResult<Self>;
}
