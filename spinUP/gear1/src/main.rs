#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[repr(C, align(4))]
struct Dap {
    sz: u8,
    _pad: u8,
    cnt: u16,
    off: u16,
    seg: u16,
    lba: u32,
    _pad2: u32,
}

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[repr(C, align(4))]
    struct DapWrapper(Dap);

    let dap = DapWrapper(Dap {
        sz: 16,
        _pad: 0,
        cnt: 32,
        off: 0,
        seg: 0x07E0,
        lba: 1,
        _pad2: 0,
    });

    unsafe {
        // First asm block for DAP setup
        core::arch::asm!(
            "mov si, {0:x}",
            in(reg) &dap.0,
        );

        // Second asm block with noreturn
        core::arch::asm!(
            // Initialize segments
            "xor ax, ax",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",

            // Store boot drive
            "mov byte ptr [0x7E00], dl",

            // Enable A20
            "in al, 0x92",
            "or al, 2",
            "out 0x92, al",

            // Load sectors
            "mov ah, 0x42",
            "int 0x13",
            "jc 2f",

            // Jump to gear2
            "push word ptr 0x07E0",
            "push word ptr 0",
            "retf",

            "2:",
            "mov al, 'E'",
            "mov ah, 0x0E",
            "int 0x10",
            "cli",
            "hlt",
            options(noreturn)
        );
    }
}

#[panic_handler]
#[no_mangle]
#[link_section = ".boot.text"]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
