//! Zig language interface

use super::{Shimmer, ShimmerResult, ShimmerError, ShimmerFn};

/// Zig-specific function attributes
#[repr(C)]
#[derive(Debug)]
pub struct ZigFnAttrs {
    pub is_export: bool,
    pub is_extern: bool,
}

impl Shimmer {
    /// Loads a Zig function
    pub fn zig_fn<T>(&self, name: &str, _attrs: ZigFnAttrs) -> ShimmerResult<T> {
        let _sym: ShimmerFn<T> = self.get_fn(name)?;
        // Zig-specific type checking and conversion would go here
        Err(ShimmerError::RuntimeError("Zig interface not yet implemented".into()))
    }
}
