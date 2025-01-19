//! MagicMath: High-Performance Crystal Lattice Mathematical Operations
//! =======================================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:26:21 UTC
//! Version: 0.1.0
//! License: MIT

//! A comprehensive library for quantum-aware mathematical operations
//! on crystal lattice systems, providing high-precision calculations
//! with quantum state preservation and stability monitoring.

pub mod core;
pub mod errors;
pub mod constants;
pub mod traits;

// Re-exports for convenient access
pub use crate::core::{
    QuantumMath,
    QuantumState,
    Operation,
};

pub use crate::errors::{
    MathError,
    MathResult,
    ErrorHandler,
};

/// Library configuration and constants
pub mod config {
    /// Current library version
    pub const VERSION: &str = "0.1.0";

    /// Minimum supported Rust version
    pub const MSRV: &str = "1.70.0";

    /// Documentation URL
    pub const DOCS_URL: &str = "https://docs.rs/magicmath";

    /// Repository URL
    pub const REPO_URL: &str = "https://github.com/isdood/magicmath";
}

/// Primary trait for types that can be used in quantum calculations
pub trait QuantumValue: traits::MeshValue {
    /// Get the quantum state of the value
    fn quantum_state(&self) -> Result<QuantumState, MathError>;

    /// Check if the value is in a stable quantum state
    fn is_stable(&self) -> Result<bool, MathError>;
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        QuantumMath,
        QuantumState,
        Operation,
        QuantumValue,
        MathError,
        MathResult,
        ErrorHandler,
    };

    pub use crate::traits::MeshValue;

    pub use crate::core::{
        add::*,
        sub::*,
        mul::*,
        div::*,
        sqrt::*,
        log::*,
        pi::*,
        gold::*,
        py::*,
        fibb::*,
    };
}

// Version compatibility check
#[doc(hidden)]
#[cfg(not(feature = "unstable"))]
const _: () = {
    // Ensure we're using a compatible version of Rust
    #[cfg(not(rust_compiler_version = "1.70.0"))]
    compile_error!("MagicMath requires Rust 1.70.0 or later");
};

/// Create a new QuantumMath instance with default configuration
pub fn new() -> QuantumMath {
    QuantumMath::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(config::VERSION, "0.1.0");
    }

    #[test]
    fn test_new_quantum_math() {
        let qmath = new();
        assert!(qmath.get_state().is_stable());
    }

    // Example implementation for f64
    impl QuantumValue for f64 {
        fn quantum_state(&self) -> Result<QuantumState, MathError> {
            Ok(QuantumState::new())
        }

        fn is_stable(&self) -> Result<bool, MathError> {
            Ok(true)
        }
    }

    #[test]
    fn test_quantum_value_f64() {
        let value: f64 = 42.0;
        assert!(value.is_stable().unwrap());
        assert!(value.quantum_state().unwrap().is_stable());
    }

    // Test all operation types
    #[test]
    fn test_operations() {
        let mut qmath = new();
        let test_value = 2.0;

        for operation in &[
            Operation::Add,
            Operation::Subtract,
            Operation::Multiply,
            Operation::Divide,
            Operation::SquareRoot,
            Operation::Logarithm,
            Operation::Pi,
            Operation::Golden,
            Operation::Pythagorean,
            Operation::Fibonacci,
        ] {
            assert!(qmath.operate(*operation, test_value).is_ok());
        }
    }

    // Test error handling
    #[test]
    fn test_error_handling() {
        let mut qmath = new();

        // Test division by zero
        let result = qmath.operate(Operation::Divide, 0.0);
        assert!(matches!(result, Err(MathError::DivisionByZero)));

        // Test negative logarithm
        let result = qmath.operate(Operation::Logarithm, -1.0);
        assert!(matches!(result, Err(MathError::LogarithmDomainError)));
    }

    // Test quantum state preservation
    #[test]
    fn test_quantum_state_preservation() {
        let mut qmath = new();
        let initial_state = qmath.get_state();

        // Perform several operations
        for _ in 0..5 {
            qmath.operate(Operation::Add, 1.0).unwrap();
        }

        let final_state = qmath.get_state();
        assert!(final_state.is_stable());
        assert!(final_state.iterations > initial_state.iterations);
    }
}

// Library metadata
#[doc(hidden)]
pub mod metadata {
    #![allow(dead_code)]

    pub const AUTHOR: &str = "Caleb J.D. Terkovics <isdood>";
    pub const CREATED: &str = "2025-01-19";
    pub const UPDATED: &str = "2025-01-19 10:26:21 UTC";
    pub const LICENSE: &str = "MIT";
    pub const DESCRIPTION: &str = "High-Performance Crystal Lattice Mathematical Operations";
}
