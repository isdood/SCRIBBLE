#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Define our assembly code using global_asm
core::arch::global_asm!(
    // Real mode code
    ".section .text",
    ".code16",
    ".globl _start",
    "_start:",
    "    cli",                // Disable interrupts
    "    xor ax, ax",        // Zero ax
    "    mov ds, ax",        // Set up data segment
    "    mov es, ax",        // Set up extra segment
    "    mov ss, ax",        // Set up stack segment
    "    mov sp, 0x7C00",    // Set up stack pointer
    "    cld",               // Clear direction flag

    // Print character
    "    mov ah, 0x0E",      // BIOS teletype output
    "    mov al, 'H'",       // Character to print
    "    mov bx, 0x0000",    // Page 0, black on black
    "    int 0x10",          // Call BIOS video interrupt

    // Infinite loop
    "1:",
    "    hlt",              // Halt the CPU
    "    jmp 1b",           // Jump back to local label 1 if interrupted
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
