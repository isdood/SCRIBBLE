#![no_std]
#![no_main]
#![no_mangle]

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }

use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".start"]
#[export_name = "_start"]
pub unsafe extern "C" fn _real_start() -> ! {
    core::arch::asm!(
        // Init segments
        "xor ax, ax",
        "mov ds, ax",
        "mov ss, ax",
        "mov sp, 0x7c00",

        // Packet: size=16, sectors=63, dest=0x7e00, lba=1
        "push 0",
        "push 0",
        "push 0",
        "push 1",
        "push 0x07e0",
        "push 0",
        "push 63",
        "push 16",

        // Read disk
        "mov si, sp",
        "mov dl, 0x80",
        "mov ah, 0x42",
        "int 0x13",
        "jc 2f",

        // Jump to Gear2
        "jmp 0:0x7e00",

        "2: hlt",
        options(nomem, nostack)
    );
    loop {}
}

#[link_section = ".sig"]
pub static SIG: [u8; 2] = [0x55, 0xaa];
