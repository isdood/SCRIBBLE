#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }

#[link_section = ".text.start"]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        ".code16",
        // Setup
        "xor ax,ax",
        "mov ss,ax",
        "mov sp,0x7c00",
        "mov ds,ax",
        // DAP on stack
        "mov ax,0x10",  // Size and reserved
        "push ax",
        "mov ax,0x3f",  // Sectors (63)
    "push ax",
    "xor ax,ax",    // Offset
    "push ax",
    "mov ax,0x7e0", // Segment
    "push ax",
    "mov ax,1",     // LBA low
    "push ax",
    "xor ax,ax",    // LBA high
    "push ax",
    "push ax",
    "push ax",
    // Read
    "mov ah,0x42",
    "mov dl,0x80",
    "mov si,sp",
    "int 0x13",
    "jc 2f",
    // Jump
    "ljmp 0,0x7e00",
    "2:hlt",
    options(nomem, nostack),
    );
    loop {}
}

#[link_section = ".boot_sig"]
#[no_mangle]
pub static BOOT_SIG: [u8; 2] = [0x55, 0xaa];
