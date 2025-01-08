#![no_std]
#![no_main]

use core::panic::PanicInfo;

core::arch::global_asm!(
    ".section .text",
    ".code16",
    ".global _start",
    "_start:",
    "    cli",                // Disable interrupts
    "    xor ax, ax",        // Zero ax
    "    mov ds, ax",        // Set up data segment
    "    mov es, ax",        // Set up extra segment
    "    mov ss, ax",        // Set up stack segment

    // Add segment validation
    "    cmp ax, 0",         // Verify segments are zero
    "    jne error",         // Jump to error if not

    "    mov sp, 0x7000",    // Set up stack pointer below bootloader
    "    mov bp, sp",        // Set up base pointer
    "    cld",               // Clear direction flag

    // Clear registers for security
    "    xor bx, bx",
    "    xor cx, cx",
    "    xor dx, dx",
    "    xor si, si",
    "    xor di, di",

    // Print character
    "    mov ah, 0x0E",      // BIOS teletype output
    "    mov al, 'H'",       // Character to print
    "    mov bx, 0x0000",    // Page 0, black on black
    "    int 0x10",          // Call BIOS video interrupt

    // Infinite loop with interrupt check
    "1:",
    "    cli",              // Ensure interrupts remain disabled
    "    hlt",              // Halt CPU
    "    jmp 1b",           // Jump back if interrupted

    // Error handler
    "error:",
    "    mov al, 'E'",      // Print error character
    "    mov ah, 0x0E",
    "    int 0x10",
    "    cli",              // Disable interrupts
    "    hlt",              // Halt on error
    "    jmp error",        // Infinite loop if interrupted

    // Fill to 510 bytes and add boot signature
    ".fill 510-(.-_start), 1, 0",
                        ".byte 0x55, 0xAA"
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
