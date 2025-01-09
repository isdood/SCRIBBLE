#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    ".code16",
    ".section .text.boot",
    ".global _start",

    "_start:",
    // Skip BPB
    "    jmp short boot",
    "    nop",
    ".space 87, 0",

    "boot:",
    // Minimal setup - absolute bare minimum
    "    cli",                  // Disable interrupts
    "    xor ax, ax",          // Zero AX
    "    mov ss, ax",          // Stack segment
    "    mov ds, ax",          // Data segment
    "    mov es, ax",          // Extra segment
    "    mov sp, 0x7c00",      // Stack pointer
    "    mov byte ptr [drive], dl", // Save drive number

    // Load sectors one at a time to minimize potential issues
    "    mov si, 6",           // Counter for sectors to load
    "    mov di, 0x7e00",      // Target buffer

    "next_sector:",
    "    mov ax, 0x0201",      // AH=02 (read), AL=01 (one sector)
"    mov cx, si",          // Calculate sector number
"    mov dx, word ptr [drive]", // Drive and head 0
"    push si",             // Save counter
"    int 0x13",            // Read sector
"    pop si",              // Restore counter
"    jc next_sector",      // Retry on error

"    add di, 512",         // Next buffer position
"    dec si",              // Decrement counter
"    jnz next_sector",     // Continue if more sectors

// Simple far jump
"    mov dl, byte ptr [drive]", // Restore drive number
"    .byte 0xEA",          // Far jump opcode
"    .word 0x0000",        // Target offset
"    .word 0x07E0",        // Target segment

"drive: .byte 0",          // Drive storage
".byte 0",                 // Alignment padding

// Boot signature
".org 510",
".word 0xaa55",
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
