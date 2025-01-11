#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }

#[no_mangle]
#[link_section = ".start"]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        // Init segments
        "xor ax, ax",
        "mov ds, ax",
        "mov ss, ax",
        "mov sp, 0x7c00",

        // Packet: size=16, sectors=63, dest=0x7e00, lba=1
        "push word ptr 0",
        "push word ptr 0",
        "push word ptr 0",
        "push word ptr 1",
        "push word ptr 0x07e0",
        "push word ptr 0",
        "push word ptr 63",
        "push word ptr 16",

        // Read disk
        "mov si, sp",
        "mov dl, 0x80",
        "mov ah, 0x42",
        "int 0x13",
        "jc 2f",

        // Jump to Gear2
        "push word ptr 0",     // CS = 0
        "push word ptr 0x7e00", // IP = 0x7e00
        "retf",                // Far return

        "2: hlt",
        options(nomem, nostack)
    );
    loop {}
}

#[link_section = ".sig"]
#[no_mangle]
pub static BOOT_SIGNATURE: [u8; 2] = [0x55, 0xaa];
