//! SIMD module for Superpurple operations
//! Created: 2025-01-21 23:47:17 UTC
//! Author: isdood

mod intrinsics;
mod operations;
mod optimizations;

pub use self::intrinsics::{CPUFeatures, avx512, avx2};
pub use self::operations::SIMDOps;
pub use self::optimizations::{SIMDOptimizer, SIMDStrategy, OptimizationMetrics};
