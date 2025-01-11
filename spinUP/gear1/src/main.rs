#![no_std]  // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

// The panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Setup stack
    core::arch::asm!(
        "mov sp, 0x7c00",
        options(nomem, nostack)
    );

    // Reset segments
    core::arch::asm!(
        "xor ax, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        options(nomem, nostack)
    );

    // Load Gear2
    let drive: u8 = 0x80;  // Boot drive

    // Set up disk address packet on stack
    core::arch::asm!(
        // Create disk address packet
        "push word ptr 0",     // Upper 16 bits of LBA (48-bit)
    "push word ptr 0",     // Upper 32 bits of LBA (48-bit)
    "push word ptr 0",     // Lower 32 bits of LBA - high word
    "push word ptr 1",     // Lower 32 bits of LBA - low word (sector 1)
    "push word ptr 0x07e0", // Segment of buffer
    "push word ptr 0",     // Offset of buffer
    "push word ptr 63",    // Number of sectors (63 more sectors)
    "push word ptr 16",    // Size of packet (16 bytes)
    "mov si, sp",         // SI points to packet

    // Call INT 13h
    "mov ah, 0x42",      // Extended read
    "mov dl, {drive}",   // Drive number
    "int 0x13",          // Call BIOS
    "jc 2f",            // If error, jump to error handler

    // Clean up stack
    "add sp, 16",       // Remove packet from stack

    // Jump to Gear2
    "push word ptr 0",   // CS = 0
    "push word ptr 0x7e00", // IP = 0x7e00
    "retf",              // Far return to Gear2

    // Error handler
    "2:",
    "mov al, 'E'",      // Error character
    "mov ah, 0x0e",     // TTY output
    "int 0x10",         // Print it
    "cli",              // Disable interrupts
    "hlt",              // Halt
    drive = in(reg_byte) drive,
                     options(nomem)
    );

    loop {}
}

// Ensure the binary has a 512-byte size and ends with the boot signature
#[used]
#[no_mangle]
#[link_section = ".boot_signature"]
static BOOT_SIGNATURE: [u8; 2] = [0x55, 0xaa];
