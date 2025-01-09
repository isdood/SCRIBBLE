#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Constants for Gear 2 loading
const GEAR2_START_SECTOR: u8 = 1;  // Sector where Gear 2 starts
const GEAR2_SECTOR_COUNT: u8 = 6;  // Number of sectors to load (based on your Gear 2 size)
const GEAR2_LOAD_ADDR: u16 = 0x7E00;  // Where to load Gear 2

global_asm!(
    ".section .boot, \"ax\"",
    ".code16",  // Start in 16-bit real mode
    ".global _start",
    "_start:",
    // Set up segments
    "    xor ax, ax",
    "    mov ds, ax",
    "    mov es, ax",
    "    mov ss, ax",
    // Set up stack
    "    mov sp, 0x7C00",

    // Load Gear 2
    // Reset disk system
    "    xor ah, ah",
    "    int 0x13",

    // Read sectors
    "    mov ah, 0x02",         // Read sectors function
    "    mov al, {sectors}",    // Number of sectors to read
    "    mov ch, 0",           // Cylinder 0
    "    mov cl, {start}",     // Starting sector (1-based)
"    mov dh, 0",           // Head 0
"    mov dl, 0x80",        // First hard disk
"    mov bx, {load_addr}", // Load address
"    int 0x13",

// Check for errors
"    jc error",

// Jump to Gear 2
"    jmp 0:{load_addr}",

"error:",
"    mov si, offset error_msg",
"print_error:",
"    lodsb",
"    or al, al",
"    jz halt",
"    mov ah, 0x0E",
"    mov bx, 0x07",
"    int 0x10",
"    jmp print_error",

"halt:",
"    hlt",
"    jmp halt",

"error_msg: .asciz \"Error loading Gear 2\"",

sectors = const(GEAR2_SECTOR_COUNT),
            start = const(GEAR2_START_SECTOR + 1),  // CL is 1-based
            load_addr = const(GEAR2_LOAD_ADDR),
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
