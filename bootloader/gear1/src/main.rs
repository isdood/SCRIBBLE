#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

const GEAR2_START_SECTOR: u8 = 1;
const GEAR2_SECTOR_COUNT: u8 = 6;
const GEAR2_LOAD_ADDR: u16 = 0x7E00;

global_asm!(
    // Start in 16-bit real mode
    ".section .text",
    ".code16",
    ".global _start",
    "_start:",
    // BIOS Parameter Block to help with booting
    "    jmp boot",
    "    nop",
    ".space 87, 0",          // Space for BPB

    "boot:",
    // Set up segments
    "    cli",               // Disable interrupts
    "    xor ax, ax",
    "    mov ds, ax",
    "    mov es, ax",
    "    mov ss, ax",
    "    mov sp, 0x7C00",    // Set up stack
    "    sti",               // Enable interrupts for BIOS calls

    // Print boot message
    "    mov si, boot_msg",
    "    call print",

    // Reset disk system
    "    xor ah, ah",
    "    int 0x13",
    "    jc error",

    // Load Gear 2
    "    mov ah, 0x02",      // Read sectors
    "    mov al, {sectors}", // Number of sectors
    "    xor ch, ch",        // Cylinder 0
    "    mov cl, {start}",   // Start sector
    "    xor dh, dh",        // Head 0
    "    mov dl, 0x80",      // First hard disk
    "    mov bx, {load_addr}", // Load address
    "    int 0x13",
    "    jc error",

    // Enable A20 line
    "    in al, 0x92",
    "    or al, 2",
    "    out 0x92, al",

    // Load GDT
    "    cli",               // Disable interrupts for mode switch
    "    lgdt [gdt_desc]",

    // Switch to protected mode
    "    mov eax, cr0",
    "    or al, 1",
    "    mov cr0, eax",

    // Far jump to flush pipeline and load CS
    "    .byte 0xEA",
    "    .long protected_mode",
    "    .word 0x08",

    // Print function (SI = string pointer)
    "print:",
    "    push ax",
    "    mov ah, 0x0E",      // BIOS teletype
    "print_loop:",
    "    lodsb",             // Load byte from SI
    "    test al, al",       // Check for null
    "    jz print_done",
    "    int 0x10",          // Print character
    "    jmp print_loop",
    "print_done:",
    "    pop ax",
    "    ret",

    // Error handler
    "error:",
    "    mov si, error_msg",
    "    call print",
    "halt:",
    "    cli",
    "    hlt",
    "    jmp halt",

    // 32-bit protected mode code
    ".code32",
    ".align 4",
    "protected_mode:",
    // Set up segment registers
    "    mov ax, 0x10",      // Data segment
    "    mov ds, ax",
    "    mov es, ax",
    "    mov fs, ax",
    "    mov gs, ax",
    "    mov ss, ax",
    "    mov esp, 0xD000",   // New stack

    // Print 'P' to show we're in protected mode
    "    mov edi, 0xB8000",  // VGA text buffer
    "    mov al, 'P'",
    "    mov ah, 0x0F",      // White on black
    "    mov [edi], ax",

    // Jump to Gear 2
    "    mov eax, {load_addr}",
    "    call eax",

    // GDT
    ".align 8",
    "gdt:",
    "    .quad 0x0000000000000000", // Null descriptor
    "    .quad 0x00CF9A000000FFFF", // Code segment (0x08)
"    .quad 0x00CF92000000FFFF", // Data segment (0x10)
"gdt_desc:",
"    .word gdt_desc - gdt - 1", // Size
"    .long gdt",                // Offset

// Strings
"boot_msg: .ascii \"G1 \"",    // Remove the 0
"    .byte 0",                 // Add null terminator as separate byte
"error_msg: .ascii \"ERR\"",   // Remove the 0
"    .byte 0",                 // Add null terminator as separate byte

sectors = const(GEAR2_SECTOR_COUNT),
            start = const(GEAR2_START_SECTOR + 1),
            load_addr = const(GEAR2_LOAD_ADDR),
);

sectors = const(GEAR2_SECTOR_COUNT),
            start = const(GEAR2_START_SECTOR + 1),
            load_addr = const(GEAR2_LOAD_ADDR),
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
