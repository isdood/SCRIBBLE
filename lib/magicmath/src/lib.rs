//! Crystal Lattice High Performance Computing Library
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 02:10:19 UTC
//! Version: 0.1.0
//! License: MIT

// Core modules
pub mod core;
pub mod traits;
pub mod constants;

// Mathematical operations
pub mod add;
pub mod sub;
pub mod mul;
pub mod div;

// Vector operations
pub mod vector3d;
pub mod vector4d;

// Resonance operations
pub mod resonance;

// Public re-exports
pub use crate::core::HarmonyState;
pub use crate::traits::*;
pub use crate::vector3d::Vector3D;
pub use crate::vector4d::Vector4D;
pub use crate::constants::HARMONY_STABILITY_THRESHOLD;

// Constants for version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Initialize the library with default configuration
pub fn init() -> Result<(), errors::MathError> {
    // Verify system compatibility
    check_system_compatibility()?;

    // Initialize harmony state tracking
    init_harmony_tracking()?;

    Ok(())
}

/// Check if the system is compatible with the library
fn check_system_compatibility() -> Result<(), errors::MathError> {
    // Verify floating-point operations
    if !check_floating_point_support() {
        return Err(errors::MathError::HarmonyStateUnstable);
    }

    Ok(())
}

/// Initialize harmony state tracking system
fn init_harmony_tracking() -> Result<(), errors::MathError> {
    // Set up initial harmony state
    let _initial_state = HarmonyState::new();

    // Verify stability calculations
    if !_initial_state.is_stable() {
        return Err(errors::MathError::HarmonyStateUnstable);
    }

    Ok(())
}

/// Check if system supports required floating-point operations
#[inline]
fn check_floating_point_support() -> bool {
    // Basic floating-point operation test
    let test_value = 1.0_f64;
    (test_value / 3.0) * 3.0 > 0.99999
}

/// Get the library version
#[inline]
pub fn version() -> &'static str {
    VERSION
}

/// Get the library authors
#[inline]
pub fn authors() -> &'static str {
    AUTHORS
}

/// Get the library description
#[inline]
pub fn description() -> &'static str {
    DESCRIPTION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        assert!(init().is_ok());
    }

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
    fn test_system_compatibility() {
        assert!(check_system_compatibility().is_ok());
    }

    #[test]
    fn test_harmony_tracking() {
        assert!(init_harmony_tracking().is_ok());
    }

    #[test]
    fn test_floating_point_support() {
        assert!(check_floating_point_support());
    }
}
