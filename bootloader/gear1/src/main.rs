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

    // Set video mode (80x25 text mode)
    "    mov ax, 0x0003",      // Standard text mode 3
    "    int 0x10",

    // Print 'G1' to screen to show we're in Gear 1
    "    mov ah, 0x0E",
    "    mov al, 'G'",
    "    int 0x10",
    "    mov al, '1'",
    "    int 0x10",

    // Load Gear 2
    "    mov ah, 0x02",        // Read sectors function
    "    mov al, {sectors}",   // Number of sectors
    "    xor ch, ch",          // Cylinder 0
    "    mov cl, {start}",     // Starting sector
    "    xor dh, dh",          // Head 0
    "    mov dl, 0x80",        // First hard disk
    "    mov bx, {load_addr}", // Load address
    "    int 0x13",            // BIOS disk read
    "    jc boot_error",       // If carry set, error occurred

    // Enable A20
    "    in al, 0x92",
    "    or al, 2",
    "    out 0x92, al",

    // Set up GDT
    "    mov ax, cs",          // Get current code segment
    "    mov ds, ax",          // Set DS to access GDT
    "    lgdt [boot_gdt_desc]",// Load GDT

    // Prepare for protected mode
    "    mov eax, cr0",
    "    or al, 1",            // Set PE bit
    "    mov cr0, eax",        // Enter protected mode

    // Clear prefetch queue
    "    jmp 1f",
    "1:",

    // Far jump to protected mode using absolute addressing
    "    .byte 0xEA",          // Far jump opcode
    "    .long boot_protected", // 32-bit offset
    "    .word 0x08",          // Code segment selector

    // Error handler
    "boot_error:",
    "    mov ah, 0x0E",
    "    mov al, 'E'",         // Print 'E' for error
    "    int 0x10",
    "    cli",
    "    hlt",

    ".align 4",
    ".code32",                 // 32-bit protected mode code
    "boot_protected:",
    // Set up protected mode segments
    "    mov ax, 0x10",        // Data segment selector
    "    mov ds, ax",
    "    mov es, ax",
    "    mov fs, ax",
    "    mov gs, ax",
    "    mov ss, ax",
    "    mov esp, 0xD000",     // Set up new stack

    // Write 'G1' directly to video memory
    "    mov edi, 0xB8000",    // VGA text buffer
    "    mov al, 'G'",
    "    mov ah, 0x0F",        // White on black
    "    mov [edi], ax",
    "    mov al, '1'",
    "    mov [edi + 2], ax",

    // Small delay
    "    mov ecx, 0x1000000",
    "boot_delay:",
    "    loop boot_delay",

    // Jump to Gear 2
    "    mov edx, 0x80",       // Boot drive number
    "    mov eax, {load_addr}",
    "    call eax",            // Call Gear 2

    "boot_halt:",
    "    hlt",
    "    jmp boot_halt",

    // GDT
    ".align 8",
    "boot_gdt:",
    "    .quad 0x0000000000000000", // Null descriptor
    "    .quad 0x00CF9A000000FFFF", // Code segment (0x08)
"    .quad 0x00CF92000000FFFF", // Data segment (0x10)

"boot_gdt_desc:",
"    .word boot_gdt_desc - boot_gdt - 1", // GDT size - 1
"    .long boot_gdt",                     // GDT base address

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
