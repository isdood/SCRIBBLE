//! Crystal Lattice High Performance Computing Library
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 22:42:17 UTC
//! Version: 0.1.0
//! License: MIT

// Core Module Exports
pub mod core;
pub mod traits;
pub mod constants;

// Mathematical Operations
pub mod add;
pub mod sub;
pub mod mul;
pub mod div;

// Quantum and Resonance
pub mod quantum;
pub mod resonance;

// Vector and Matrix Operations
pub mod vector;
pub mod matrix;

// Complex Number Operations
pub mod complex;

// Fractal Operations
pub mod fractal;

// Harmony Operations
pub mod harmony;

// Utility Modules
pub mod utils;

// Re-exports for convenient access
pub use crate::core::{
    HarmonyState,
    ResonanceState,
    QuantumState,
    ComplexState,
    FractalState,
};

pub use crate::traits::{
    MeshValue,
    Quantum,
    Phase,
    Resonance,
    Harmony,
    Complex,
    Fractal,
};

pub use crate::vector::{
    Vector3D,
    Vector4D,
};

pub use crate::matrix::{
    Matrix2D,
    Matrix3D,
    Matrix4D,
};

pub use crate::complex::{
    ComplexNumber,
    ComplexOperations,
};

pub use crate::fractal::{
    FractalPoint,
    FractalSet,
    JuliaSet,
    MandelbrotSet,
};

pub use crate::harmony::{
    HarmonyOperation,
    HarmonyTransform,
    HarmonyField,
};

pub use crate::quantum::{
    QuantumState,
    QuantumField,
    QuantumOperator,
};

pub use crate::resonance::{
    ResonanceField,
    ResonanceOperator,
    ResonanceTransform,
};

// Version Information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Initialize the library with default configuration
pub fn init() {
    // Initialize logging if needed
    #[cfg(feature = "logging")]
    crate::utils::logging::init_logger();
}

/// Get the library version
pub fn version() -> &'static str {
    VERSION
}

/// Get the library authors
pub fn authors() -> &'static str {
    AUTHORS
}

/// Get the library description
pub fn description() -> &'static str {
    DESCRIPTION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }

    #[test]
    fn test_authors() {
        assert!(!authors().is_empty());
    }

    #[test]
    fn test_description() {
        assert!(!description().is_empty());
    }

    #[test]
    fn test_init() {
        init();
        // Add assertions for successful initialization
    }
}
