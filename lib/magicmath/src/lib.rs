//! MagicMath: High-Performance Crystal Lattice Mathematical Operations
//! =======================================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 14:35:35 UTC
//! Version: 0.1.0
//! License: MIT

//! A comprehensive library for quantum-aware mathematical operations
//! on crystal lattice systems, providing high-precision calculations
//! with quantum state preservation and stability monitoring.

// Core modules
pub mod core;
pub mod constants;
pub mod traits;
pub mod deref;
pub mod derefmut;

// Feature modules
pub mod fractal;
pub mod julia;
pub mod brot;

// Re-exports for convenient access
pub use crate::core::{
    QuantumMath,
    QuantumState,
    Operation,
    quantum_add,
    quantum_sub,
    quantum_mul,
    quantum_div,
    quantum_sqrt,
    quantum_ln,
    quantum_pi,
    quantum_phi,
    quantum_pythagoras,
    quantum_fibonacci,
};

pub use errors::core::{
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

pub use crate::deref::{
    QuantumDeref,
    QuantumDerefable,
};

pub use crate::derefmut::{
    QuantumDerefMut,
    QuantumDerefMutable,
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
    pub const UPDATED: &str = "2025-01-19 14:35:35 UTC";

    /// Current user
    pub const CURRENT_USER: &str = "isdood";
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        QuantumMath,
        QuantumState,
        Operation,
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
        QuantumDeref,
        QuantumDerefable,
        QuantumDerefMut,
        QuantumDerefMutable,
        quantum_add,
        quantum_sub,
        quantum_mul,
        quantum_div,
        quantum_sqrt,
        quantum_ln,
        quantum_pi,
        quantum_phi,
        quantum_pythagoras,
        quantum_fibonacci,
    };
    pub use errors::core::{MathError, MathResult};
}

// Version compatibility check
#[doc(hidden)]
const _: () = {
    // Ensure we're using a compatible version of Rust
    #[rustversion::attr(not(since(1.70)), error("MagicMath requires Rust 1.70.0 or later"))]
    const CHECK_RUST_VERSION: () = ();
};

/// Create a new QuantumMath instance with default configuration
#[inline]
pub fn new() -> QuantumMath {
    QuantumMath::new()
}
