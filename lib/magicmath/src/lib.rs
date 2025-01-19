//! MagicMath: High-Performance Crystal Lattice Mathematical Operations
//! =======================================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 18:57:54 UTC
//! Version: 0.1.0
//! License: MIT
//!
//! A comprehensive library for harmony-aware mathematical operations
//! on crystal lattice systems, providing high-precision calculations
//! with harmony state preservation and stability monitoring.

pub mod core;
pub mod constants;
pub mod traits;
pub mod deref;
pub mod derefmut;
pub mod floor;
pub mod resonance;
pub mod mesh;
pub mod vector;
pub mod add; // Include the add module
pub mod sub; // Include the sub module
pub mod div; // Include the div module
pub mod mul; // Include the mul module

// Feature modules
pub mod fractal;
pub mod julia;
pub mod brot;
pub mod exp;

// Re-exports for convenient access
pub use crate::core::{
    HarmonyMath,
    HarmonyState,
    Operation,
    harmony_add,
    harmony_sub,
    harmony_mul,
    harmony_div,
    harmony_sqrt,
    harmony_ln,
    harmony_pi,
    harmony_phi,
    harmony_pythagoras,
    harmony_fibonacci,
};

// Re-export resonance types
pub use crate::resonance::{
    Resonance,
    Quantum,
    Phase,
};

// Re-export mesh types
pub use crate::mesh::MeshMath;

// Import the errors module
pub use errors::{
    CrystalError,
    MathError,
    MathResult,
    QuantumError,
    QuantumResult,
    VectorError,
    VectorResult,
    CoherenceError,
    CoherenceResult,
    CrystalResult,
};

pub use crate::traits::{
    MeshValue,
    ComplexQuantum,
    FractalValue,
    HarmonyState as HarmonyStateTrait,
    HarmonyOperation,
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
    HarmonyDeref,
    HarmonyDerefable,
};

pub use crate::derefmut::{
    HarmonyDerefMut,
    HarmonyDerefMutable,
};

// Re-export floor and sqrt functions
pub use crate::core::harmony_sqrt as sqrt;
pub use crate::floor::floor;

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
    pub const UPDATED: &str = "2025-01-19 18:57:54 UTC";

    /// Current user
    pub const CURRENT_USER: &str = "isdood";
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        HarmonyMath,
        HarmonyState,
        Operation,
        MeshValue,
        ComplexQuantum,
        FractalValue,
        HarmonyStateTrait,
        HarmonyOperation,
        Resonance,
        Quantum,
        Phase,
        MeshMath,
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
        HarmonyDeref,
        HarmonyDerefable,
        HarmonyDerefMut,
        HarmonyDerefMutable,
        harmony_add,
        harmony_sub,
        harmony_mul,
        harmony_div,
        harmony_sqrt,
        harmony_ln,
        harmony_pi,
        harmony_phi,
        harmony_pythagoras,
        harmony_fibonacci,
        floor,
        sqrt,
    };
    pub use errors::{
        CrystalError, MathError, MathResult, QuantumError, QuantumResult, VectorError, VectorResult,
        CoherenceError, CoherenceResult, CrystalResult,
    };
}

// Version compatibility check
#[doc(hidden)]
const _: () = {
    // Ensure we're using a compatible version of Rust
    #[rustversion::attr(not(since(1.70)), error("MagicMath requires Rust 1.70.0 or later"))]
    const CHECK_RUST_VERSION: () = ();
};

/// Create a new HarmonyMath instance with default configuration
#[inline]
pub fn new() -> HarmonyMath {
    HarmonyMath::new()
}
