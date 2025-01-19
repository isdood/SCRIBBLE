//! MagicMath: High-Performance Crystal Lattice Mathematical Operations
//! =======================================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:41:02 UTC
//! Version: 0.1.0
//! License: MIT

//! A comprehensive library for quantum-aware mathematical operations
//! on crystal lattice systems, providing high-precision calculations
//! with quantum state preservation and stability monitoring.

// Core modules
pub mod core;
pub mod errors;
pub mod constants;
pub mod traits;

// Feature modules
pub mod fractal;
pub mod julia;
pub mod brot;

// Re-exports for convenient access
pub use crate::core::{
    QuantumMath,
    QuantumState,
    Operation,
};

pub use crate::errors::{
    MathError,
    MathResult,
};

pub use crate::traits::{
    MeshValue,
    ComplexQuantum,
    FractalValue,
    QuantumState as QuantumStateTrait,
    QuantumOperation,
    Resonance,
};

pub use crate::fractal::{
    FractalParams,
    FractalState,
    FractalType,
    generate_fractal,
};

pub use crate::julia::{
    JuliaParams,
    JuliaState,
    JuliaVariant,
    iterate_julia,
};

pub use crate::brot::{
    MandelbrotParams,
    MandelbrotState,
    MandelbrotVariant,
    iterate_mandelbrot,
};

/// Library configuration and version information
pub mod config {
    /// Current library version
    pub const VERSION: &str = "0.1.0";

    /// Minimum supported Rust version
    pub const MSRV: &str = "1.70.0";

    /// Documentation URL
    pub const DOCS_URL: &str = "https://docs.rs/magicmath";

    /// Repository URL
    pub const REPO_URL: &str = "https://github.com/isdood/magicmath";

    /// Creation timestamp
    pub const CREATED: &str = "2025-01-19";

    /// Last update timestamp
    pub const UPDATED: &str = "2025-01-19 10:41:02 UTC";

    /// Current user
    pub const CURRENT_USER: &str = "isdood";
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        QuantumMath,
        QuantumState,
        Operation,
        MathError,
        MathResult,
        MeshValue,
        ComplexQuantum,
        FractalValue,
        QuantumStateTrait,
        QuantumOperation,
        Resonance,
        FractalParams,
        FractalState,
        FractalType,
        generate_fractal,
        JuliaParams,
        JuliaState,
        JuliaVariant,
        iterate_julia,
        MandelbrotParams,
        MandelbrotState,
        MandelbrotVariant,
        iterate_mandelbrot,
    };

    pub use crate::core::{
        add::quantum_add,
        sub::quantum_sub,
        mul::quantum_mul,
        div::quantum_div,
        sqrt::quantum_sqrt,
        log::quantum_ln,
        pi::quantum_pi,
        gold::quantum_phi,
        py::quantum_pythagoras,
        fibb::quantum_fibonacci,
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
    use crate::constants::*;

    #[test]
    fn test_version() {
        assert_eq!(config::VERSION, "0.1.0");
    }

    #[test]
    fn test_quantum_math() {
        let mut qmath = new();
        assert!(qmath.get_state().is_stable());
    }

    #[test]
    fn test_fractal_generation() {
        let params = FractalParams::default();
        let state = FractalState::Julia(JuliaState::new(0.0, 0.0));
        let result = generate_fractal(state, &params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_julia_set() {
        let params = JuliaParams::default();
        let state = JuliaState::new(0.0, 0.0);
        let result = iterate_julia(state, &params, JuliaVariant::Standard);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mandelbrot_set() {
        let params = MandelbrotParams::default();
        let state = MandelbrotState::new(0.0, 0.0);
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Standard);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quantum_operations() {
        let mut qmath = new();
        let test_value = 2.0f64;

        let operations = [
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
            Operation::Julia,
            Operation::Mandelbrot,
            Operation::Fractal,
        ];

        for op in &operations {
            assert!(qmath.operate(*op, test_value).is_ok());
        }
    }

    #[test]
    fn test_error_handling() {
        let mut qmath = new();

        // Test division by zero
        let result = qmath.operate(Operation::Divide, 0.0);
        assert!(matches!(result, Err(MathError::DivisionByZero)));

        // Test negative logarithm
        let result = qmath.operate(Operation::Logarithm, -1.0);
        assert!(matches!(result, Err(MathError::LogarithmDomainError(_))));
    }

    #[test]
    fn test_fractal_errors() {
        let params = FractalParams::default();
        let wrong_state = FractalState::Mandelbrot(MandelbrotState::new(0.0, 0.0));
        let result = generate_fractal(wrong_state, &params);
        assert!(matches!(result, Err(MathError::FractalTypeMismatch)));
    }
}
