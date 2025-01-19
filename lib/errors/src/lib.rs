//! Crystal Computing Error Types
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 14:28:05 UTC
//! Version: 0.1.0
//! License: MIT

pub mod core;

pub use core::{
    CrystalError,
    MathError,
    QuantumError,
    VectorError,
    CoherenceError,
    CrystalResult,
    MathResult,
    QuantumResult,
    VectorResult,
    CoherenceResult,
};
