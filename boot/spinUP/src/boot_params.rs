// boot/spinUP/src/boot_params.rs
// Last Updated: 2025-01-13 05:30:37 UTC
// Author: Caleb J.D. Terkovics (isdood)
// Current User: isdood

use unstable_matter::{
    space_config::SpaceMetadata,
    vector_space::VectorSpace,
};

#[repr(C)]
pub struct BootParams {
    pub kernel_load_addr: u32,
    pub kernel_size: u32,
    pub space_metadata: *const SpaceMetadata,
    pub vector_space: *const VectorSpace,
}
