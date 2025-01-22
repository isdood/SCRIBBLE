//! Superpurple: SIMD-optimized quantum crystal lattice mathematics
//! Created: 2025-01-22 00:23:05
//! Author: isdood

use std::error::Error;
use std::fmt;

pub mod core;
pub mod quantum;
pub mod memory;
pub mod lattice;
pub mod simd;

// Re-export commonly used types and traits
pub use self::core::{
    CLSIMDVec3,
    LatticeSymmetry,
    SIMDValue,
    VectorOps,
};

// ... (other re-exports)

#[derive(Debug)]
pub struct InitError {
    message: String,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Initialization error: {}", self.message)
    }
}

impl Error for InitError {}

/// Initialize Superpurple with optimal settings
pub fn init() -> Result<(), Box<dyn Error>> {
    let features = CPUFeatures::detect();
    let optimizer = SIMDOptimizer::new();

    GLOBAL_STATE.set(GlobalState {
        features,
        optimizer,
        initialized: true,
    }).map_err(|_| {
        Box::new(InitError {
            message: "Failed to initialize global state".to_string(),
        })
    })?;

    Ok(())
}

// ... (rest of the module)
