#[repr(C, packed)]
pub struct BootParams {
    pub boot_drive: u8,
    pub partition_offset: u32,
    pub memory_map_addr: u32,
    pub memory_map_entries: u16,
    pub kernel_load_addr: u32,
    pub kernel_size: u32,
}

#[link_section = ".bootparams"]
pub static mut BOOT_PARAMS: BootParams = BootParams {
    boot_drive: 0,
    partition_offset: 0,
    memory_map_addr: 0,
    memory_map_entries: 0,
    kernel_load_addr: 0,
    kernel_size: 0,
};
