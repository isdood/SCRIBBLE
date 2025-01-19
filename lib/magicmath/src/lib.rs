//! Crystal Lattice High Performance Computing Library
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 23:54:38 UTC
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

// Vector and Resonance Operations
pub mod vector;
pub mod resonance;

// Re-exports for convenient access
pub use crate::core::HarmonyState;

pub use crate::traits::{
    MeshValue,
    Quantum,
    Phase,
    Resonance,
};

pub use crate::vector::{
    Vector3D,
    Vector4D,
};

// Version Information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Initialize the library with default configuration
pub fn init() {}

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
}
