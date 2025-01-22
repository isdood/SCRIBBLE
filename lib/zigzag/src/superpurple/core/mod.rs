//! Core module for Superpurple SIMD operations
//! Created: 2025-01-21 23:38:32 UTC
//! Author: isdood

mod vector;
mod symmetry;
mod traits;

pub use self::vector::CLSIMDVec3;
pub use self::symmetry::LatticeSymmetry;
pub use self::traits::{SIMDValue, VectorOps};

// Re-export commonly used types
pub type Vec3f = CLSIMDVec3<f32>;
pub type Vec3d = CLSIMDVec3<f64>;
