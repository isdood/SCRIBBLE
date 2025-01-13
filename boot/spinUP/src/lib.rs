// boot/spinUP/src/lib.rs
// Last Updated: 2025-01-13 05:41:39 UTC
// Author: Caleb J.D. Terkovics (isdood)
// Current User: isdood

#![no_std]

pub mod boot_params;
pub mod serial;
pub mod memory;  // New module for memory operations

// Re-export memory functions
pub use memory::*;
