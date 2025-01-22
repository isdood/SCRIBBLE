//! Memory module for Superpurple SIMD operations
//! Created: 2025-01-21 23:43:42 UTC
//! Author: isdood

mod pool;
mod alignment;
mod cache;

pub use self::pool::{CLSIMDMemoryPool, SymmetryAlignment};
pub use self::alignment::{SIMDAlignment, AlignmentUtils};
pub use self::cache::CacheManager;
