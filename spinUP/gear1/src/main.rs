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

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Setup segments and stack
        core::arch::asm!(
            "xor ax, ax",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",
        );

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

        // Store boot drive and enable A20
        core::arch::asm!(
            "mov [0x7E00], dl",  // Store boot drive at known location
            "in al, 0x92",       // Fast A20 gate
            "or al, 2",
            "out 0x92, al",
            "mov ah, 0x42",      // Load sectors
            "mov si, {0:x}",
            "int 0x13",
            "jc 2f",            // Error if carry set
            "push word ptr 0x07E0",
            "push word ptr 0",
            "retf",             // Jump to gear2
            "2:",
            "mov ax, 0x0E45",   // Print 'E' on error
            "int 0x10",
            "hlt",
            in(reg) &dap,
                         options(noreturn),
        );
    }
}

#[panic_handler]
#[no_mangle]
fn panic(_: &PanicInfo) -> ! { loop {} }
