#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

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

#[no_mangle]
#[link_section = ".boot.text"]
pub extern "C" fn _start() -> ! {
    let dap = Dap {
        sz: 16,
        _pad: 0,
        cnt: 32,
        off: 0,
        seg: 0x07E0,
        lba: 1,
        _pad2: 0,
    };

    unsafe {
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
            "mov si, {0:x}",
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
            in(reg) &dap,
                         options(noreturn)
        );
    }
}
