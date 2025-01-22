//! # ZigZag
//!
//! `zigzag` is a high-performance quantum computing and lattice symmetry library
//! that provides SIMD-optimized implementations of quantum gates and lattice transformations.

pub mod core;
pub mod quantum;
pub mod lattice;

pub use crate::quantum::QuantumState;
pub use crate::lattice::LatticeSymmetry;
