//! Crystometer - Crystal structure measurement and analysis toolset
//!
//! This crate provides bindings to the Zig core implementation along with
//! Rust-native utilities for crystal structure analysis.

use std::os::raw::{c_void, c_int};

#[repr(C)]
pub struct Crystal {
    _private: [u8; 0],
}

extern "C" {
    fn crystal_init() -> *mut Crystal;
    fn crystal_free(crystal: *mut Crystal);
    fn crystal_rotate(crystal: *mut Crystal, angle: f64) -> c_int;
    fn crystal_transform(crystal: *mut Crystal, matrix: *const f64) -> c_int;
}

/// Safe Rust wrapper for Crystal operations
pub struct CrystalWrapper {
    inner: *mut Crystal,
}

impl CrystalWrapper {
    pub fn new() -> Self {
        let inner = unsafe { crystal_init() };
        Self { inner }
    }

    pub fn rotate(&mut self, angle: f64) -> Result<(), &'static str> {
        let result = unsafe { crystal_rotate(self.inner, angle) };
        if result == 0 {
            Ok(())
        } else {
            Err("Rotation failed")
        }
    }

    pub fn transform(&mut self, matrix: &[f64]) -> Result<(), &'static str> {
        if matrix.len() != 9 {
            return Err("Transform matrix must be 3x3");
        }
        let result = unsafe { crystal_transform(self.inner, matrix.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err("Transform failed")
        }
    }
}

impl Drop for CrystalWrapper {
    fn drop(&mut self) {
        unsafe { crystal_free(self.inner) }
    }
}

unsafe impl Send for CrystalWrapper {}
unsafe impl Sync for CrystalWrapper {}
