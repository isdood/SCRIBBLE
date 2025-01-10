#[repr(C, packed)]
pub struct BootParams {
    pub kernel_load_addr: u64,
    pub kernel_size: u64,
}

#[no_mangle]
#[link_section = ".bootparams"]
pub static BOOT_PARAMS: BootParams = BootParams {
    kernel_load_addr: 0,
    kernel_size: 0,
};
