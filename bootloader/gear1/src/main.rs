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

    // Set up IDT before disk operations
    "    call setup_idt",

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
"    jc error_handler",    // Jump to error handler on error

"    add di, 512",         // Next buffer position
"    dec si",              // Decrement counter
"    jnz next_sector",     // Continue if more sectors

// Simple far jump to gear2
"    mov dl, byte ptr [drive]", // Restore drive number
"    .byte 0xEA",          // Far jump opcode
"    .word 0x0000",        // Target offset
"    .word 0x07E0",        // Target segment

// IDT setup routine
"setup_idt:",
"    push ax",
"    push dx",

// Set up divide by zero handler (INT 0)
"    mov word ptr [0x0000], div_zero_handler",
"    mov word ptr [0x0002], cs",

// Set up double fault handler (INT 8)
"    mov word ptr [0x0020], double_fault_handler",
"    mov word ptr [0x0022], cs",

// Set up RTC handler (INT 70h)
"    mov word ptr [0x01C0], rtc_handler",
"    mov word ptr [0x01C2], cs",

// Initialize PIC
"    mov al, 0x11",        // Init command
"    out 0x20, al",        // Send to PIC1
"    out 0xA0, al",        // Send to PIC2

"    mov al, 0x20",        // PIC1 vector offset
"    out 0x21, al",
"    mov al, 0x28",        // PIC2 vector offset
"    out 0xA1, al",

"    mov al, 0x04",        // Tell PIC1 about PIC2
"    out 0x21, al",
"    mov al, 0x02",        // Tell PIC2 its cascade identity
"    out 0xA1, al",

"    mov al, 0x01",        // 8086 mode
"    out 0x21, al",
"    out 0xA1, al",

// Mask all interrupts except keyboard
"    mov al, 0xFD",        // Enable only IR1 (keyboard)
"    out 0x21, al",
"    mov al, 0xFF",        // Mask all interrupts on PIC2
"    out 0xA1, al",

"    pop dx",
"    pop ax",
"    ret",

// Exception handlers
"div_zero_handler:",
"    iret",

"double_fault_handler:",
"    iret",

"rtc_handler:",
"    push ax",
"    mov al, 0x20",        // EOI command
"    out 0x20, al",        // Send to PIC1
"    out 0xA0, al",        // Send to PIC2
"    pop ax",
"    iret",

"error_handler:",
"    hlt",
"    jmp error_handler",

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
