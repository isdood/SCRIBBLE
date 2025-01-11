#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        // Setup segments and stack
        "xor ax, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        "mov sp, 0x7c00",

        // Build disk packet on stack
        "push word ptr 0",      // Upper LBA bits
        "push word ptr 0",
        "push word ptr 0",      // Lower LBA bits
        "push word ptr 1",      // Starting at sector 1
        "push word ptr 0x07e0", // Buffer segment
        "push word ptr 0",      // Buffer offset
        "push word ptr 63",     // Sector count
        "push word ptr 16",     // Packet size

        // Read disk
        "mov si, sp",          // Packet pointer
        "mov ah, 0x42",        // Extended read
        "mov dl, 0x80",        // First hard drive
        "int 0x13",
        "jc 1f",              // On error, halt

        // Jump to Gear2
        "add sp, 16",          // Clean stack
        "push word ptr 0",     // CS = 0
        "push word ptr 0x7e00", // IP = 0x7e00
        "retf",                // Far jump

        // Error handler
        "1: cli",
        "hlt",
        options(nomem, nostack)
    );

    loop {}
}

#[link_section = ".signature"]
#[no_mangle]
pub static BOOT_SIGNATURE: [u8; 2] = [0x55, 0xaa];
