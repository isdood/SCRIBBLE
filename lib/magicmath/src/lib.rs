//! Crystal Lattice High Performance Computing Library
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 01:32:46 UTC
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

// Vector Operations
pub mod vector3d;
pub mod vector4d;

// Resonance Operations
pub mod resonance;

// Re-exports for convenient access
pub use crate::core::HarmonyState;

pub use crate::traits::{
    MeshValue,
    CrystalAdd,
    CrystalSub,
    CrystalMul,
    CrystalDiv,
    Quantum,
    Phase,
    Resonance,
};

// Vector re-exports
pub use crate::vector3d::Vector3D;
pub use crate::vector4d::Vector4D;

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
    use errors::MathResult;

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
    fn test_crystal_operations() -> MathResult<()> {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        let _add_result = v1.add(&v2)?;
        let _sub_result = v1.sub(&v2)?;
        let _mul_result = v1.mul(&v2)?;
        let _div_result = v1.div(&v2)?;
        Ok(())
    }
}
