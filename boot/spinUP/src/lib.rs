// spinUP/gear2/src/lib.rs
#![no_std]

pub mod boot_params;
pub mod serial;

// Re-export types that should be public
pub use crate::boot_params::*;
pub use crate::serial::*;
