#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

const GEAR2_START_SECTOR: u8 = 1;
const GEAR2_SECTOR_COUNT: u8 = 6;
const GEAR2_LOAD_ADDR: u16 = 0x7E00;

global_asm!(
    ".section .text",
    ".code16",
    ".global _start",
    "_start:",
    // Basic setup
    "    cli",                  // Disable interrupts
    "    xor ax, ax",
    "    mov ds, ax",
    "    mov es, ax",
    "    mov ss, ax",
    "    mov sp, 0x7C00",

    // Save video mode
    "    mov ah, 0x0F",
    "    int 0x10",
    "    push ax",             // Save current video mode

    // Set video mode (80x25 text mode)
    "    mov ah, 0x00",
    "    mov al, 0x03",        // Standard text mode
    "    int 0x10",

    // Print loading message
    "    mov si, loading_msg",
    "    call print_string",

    // Load Gear 2
    "    mov ah, 0x02",        // Read sectors function
    "    mov al, {sectors}",   // Number of sectors
    "    xor ch, ch",          // Cylinder 0
    "    mov cl, {start}",     // Starting sector
    "    xor dh, dh",          // Head 0
    "    mov dl, 0x80",        // First hard disk
    "    mov bx, {load_addr}", // Load address
    "    int 0x13",            // BIOS disk read
    "    jc error",            // If carry set, error occurred

    // Print A20 message
    "    mov si, a20_msg",
    "    call print_string",

    // Enable A20
    "    in al, 0x92",
    "    or al, 2",
    "    out 0x92, al",

    // Print GDT message
    "    mov si, gdt_msg",
    "    call print_string",

    // Load GDT
    "    lgdt [gdt_desc]",

    // Print PM message
    "    mov si, pm_msg",
    "    call print_string",

    // Switch to protected mode
    "    mov eax, cr0",
    "    or al, 1",
    "    mov cr0, eax",

    // Far jump to protected mode
    "    .byte 0xEA",          // Far jump instruction
    "    .long protected_mode", // 32-bit offset
    "    .word 0x08",          // Code segment selector

    // String printing function
    "print_string:",
    "    lodsb",
    "    test al, al",
    "    jz print_done",
    "    mov ah, 0x0E",
    "    mov bx, 0x07",
    "    int 0x10",
    "    jmp print_string",
    "print_done:",
    "    ret",

    ".align 4",
    ".code32",
    "protected_mode:",
    "    mov ax, 0x10",        // Data segment
    "    mov ds, ax",
    "    mov es, ax",
    "    mov fs, ax",
    "    mov gs, ax",
    "    mov ss, ax",

    // Write to video memory directly in protected mode
    "    mov edi, 0xB8000",
    "    mov eax, 0x0F410F42", // "AB" in white on black
    "    mov [edi], eax",

    // Short delay
    "    mov ecx, 0x100000",
    "delay_loop:",
    "    loop delay_loop",

    // Jump to Gear 2
    "    mov edx, 0x80",       // Boot drive
    "    mov eax, {load_addr}",
    "    jmp eax",             // Direct jump to Gear 2

    // Error handler
    ".code16",
    "error:",
    "    mov si, error_msg",
    "    call print_string",
    "halt:",
    "    hlt",
    "    jmp halt",

    // Messages
    "loading_msg: .asciz \"Loading Gear 2...\\r\\n\"",
    "a20_msg: .asciz \"Enabling A20...\\r\\n\"",
    "gdt_msg: .asciz \"Loading GDT...\\r\\n\"",
    "pm_msg: .asciz \"Entering Protected Mode...\\r\\n\"",
    "error_msg: .asciz \"Error loading Gear 2\\r\\n\"",

    // GDT
    ".align 8",
    "gdt:",
    "    .quad 0x0000000000000000", // Null descriptor
    "    .quad 0x00CF9A000000FFFF", // Code segment
    "    .quad 0x00CF92000000FFFF", // Data segment
    "gdt_desc:",
    "    .word gdt_desc - gdt - 1", // Limit
    "    .long gdt",                // Base

    // Pad to 510 bytes and add boot signature
    ".org 510",
    "    .word 0xAA55",

    sectors = const(GEAR2_SECTOR_COUNT),
            start = const(GEAR2_START_SECTOR + 1),
            load_addr = const(GEAR2_LOAD_ADDR),
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
