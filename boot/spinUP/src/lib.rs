// boot/spinUP/src/lib.rs
// Last Updated: 2025-01-13 05:30:37 UTC
// Author: Caleb J.D. Terkovics (isdood)
// Current User: isdood

#![no_std]

pub mod boot_params;
pub mod serial;

// Re-export types that should be public
pub use crate::boot_params::BootParams;
pub use crate::serial::*;
