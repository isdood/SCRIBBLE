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

#[no_mangle]
#[link_section = ".boot.text"]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Setup segments
        core::arch::asm!(
            "xor ax, ax",
            "mov ds, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",
        );

        // Store boot drive
        let drive: u8;
        core::arch::asm!("mov {}, dl", out(reg_byte) drive);

        // Load gear2
        let dap = Dap {
            sz: 16,
            _pad: 0,
            cnt: 32,
            off: 0,
            seg: 0x07E0,
            lba: 1,
            _pad2: 0,
        };

        core::arch::asm!(
            "mov ah, 0x42",
            "mov si, {0:x}",
            "int 0x13",
            "jc 2f",         // Jump to error handler if carry set
            "cmp ah, 0",     // Check status
            "jne 2f",        // Jump if not zero (error)
        "push word ptr 0x07E0",
        "push word ptr 0",
        "mov dl, {1}",
        "retf",
        "2:",           // Error handler (using numeric label)
        "mov ax, 0x0E45",  // Print 'E' on error
        "int 0x10",
        "hlt",
        in(reg) &dap,
                         in(reg_byte) drive,
                         options(noreturn),
        );
    }
}

#[panic_handler]
#[no_mangle]
#[link_section = ".boot.text.panic"]
fn panic(_: &PanicInfo) -> ! { loop {} }
