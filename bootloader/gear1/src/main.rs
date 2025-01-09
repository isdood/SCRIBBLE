#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

const GEAR2_START_SECTOR: u8 = 1;
const GEAR2_SECTOR_COUNT: u8 = 6;
const GEAR2_LOAD_ADDR: u16 = 0x7E00;

global_asm!(
    ".section .boot, \"ax\"",
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

    // Enable A20
    "    in al, 0x92",
    "    or al, 2",
    "    out 0x92, al",

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

    // Set up minimum GDT
    "    lgdt [gdtr]",

    // Switch to protected mode
    "    mov eax, cr0",
    "    or al, 1",
    "    mov cr0, eax",

    // Jump to 32-bit code
    "    jmp $+7",            // Flush prefetch queue
    ".byte 0xEA",             // Far jump opcode
    ".long pm_start",         // 32-bit offset
    ".word 0x08",             // Code segment selector

    ".code32",
    "pm_start:",
    // Set up segments for protected mode
    "    mov ax, 0x10",
    "    mov ds, ax",
    "    mov es, ax",
    "    mov fs, ax",
    "    mov gs, ax",
    "    mov ss, ax",
    "    mov esp, 0x7C00",

    // Jump to Gear 2
    "    mov edx, 0x80",      // Pass boot drive number
    "    jmp 0x7E00",         // Jump to Gear 2

    ".code16",
    "error:",
    "    mov si, msg",
    "print:",
    "    lodsb",
    "    test al, al",
    "    jz halt",
    "    mov ah, 0x0E",
    "    int 0x10",
    "    jmp print",
    "halt:",
    "    hlt",
    "    jmp halt",

    "msg: .asciz \"Boot error\"",

    // GDT
    ".align 8",
    "gdt:",
    "    .quad 0",                    // Null descriptor
    "    .quad 0x00CF9A000000FFFF",   // Code segment
    "    .quad 0x00CF92000000FFFF",   // Data segment
    "gdtr:",
    "    .word gdtr - gdt - 1",      // GDT limit
    "    .long gdt",                  // GDT base address

    // Boot signature
    ".org 510",
    ".word 0xaa55",

    sectors = const(GEAR2_SECTOR_COUNT),
            start = const(GEAR2_START_SECTOR + 1),
            load_addr = const(GEAR2_LOAD_ADDR),
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
