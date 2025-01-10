#[repr(C)]
pub struct BootParams {
    pub kernel_load_addr: u32,
    pub kernel_size: u32,
    pub memory_map_addr: u32,
    pub memory_map_entries: u32,
}

#[no_mangle]
pub static BOOT_PARAMS: BootParams = BootParams {
    kernel_load_addr: 0x100000,  // Load kernel at 1MB
    kernel_size: 0,
    memory_map_addr: 0,
    memory_map_entries: 0,
};
