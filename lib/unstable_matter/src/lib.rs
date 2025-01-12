// lib/unstable_matter/src/lib.rs
// Last updated: 2025-01-12 04:54:31
// Author: isdood

#![no_std]

mod unstable_matter;
mod unstable_vectrix;
mod wrapper;

// Re-export the main types
pub use unstable_matter::UnstableMatter;
pub use unstable_vectrix::UnstableVectrix;
pub use wrapper::{Wrapper, Implementation};
