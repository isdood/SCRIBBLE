#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[repr(C, packed)]
struct Dap {
    sz: u8,
    _pad: u8,
    cnt: u16,
    off: u16,
    seg: u16,
    lba: u32,
    _pad2: u32,
}

// Minimized StageInfo structure
#[repr(C, packed)]
pub struct StageInfo {
    boot_drive: u8,
    flags: u8,
    stage2_addr: u16,
}

#[no_mangle]
static mut STAGE_INFO: StageInfo = StageInfo {
    boot_drive: 0,
    flags: 0,
    stage2_addr: 0x07E0,
};

#[no_mangle]
#[link_section = ".boot.text"]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Minimal setup
        core::arch::asm!(
            "xor ax, ax",
            "mov ds, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",
            // Store boot drive directly
            "mov [{addr}], dl",
            addr = in(reg) &raw const STAGE_INFO.boot_drive,
        );

        // Quick A20 check/enable
        enable_a20();

        // Load gear2 (simplified DAP)
        let dap = Dap {
            sz: 16,
            _pad: 0,
            cnt: 32,    // Adjust if needed
            off: 0,
            seg: 0x07E0,
            lba: 1,
            _pad2: 0,
        };

        core::arch::asm!(
            "mov ah, 0x42",
            "mov si, {0:x}",
            "int 0x13",
            "jc 2f",
            // Fixed far jump syntax
            "push word ptr 0x07E0",
            "push word ptr 0",
            "retf",
            "2:",
            "mov ax, 0x0E45",
            "int 0x10",
            "hlt",
            in(reg) &dap,
                         options(noreturn),
        );
    }
}

#[inline(always)]
unsafe fn enable_a20() {
    // Simplified A20 enable - just try fast A20 gate
    core::arch::asm!(
        "in al, 0x92",
        "or al, 2",
        "out 0x92, al",
        options(nomem, nostack)
    );
}

#[panic_handler]
#[no_mangle]
fn panic(_: &PanicInfo) -> ! { loop {} }
