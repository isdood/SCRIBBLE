#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Entry point for the bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Write "Hello World" to the VGA text buffer
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in b"Hello World".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07; // Light grey on black background
        }
    }

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
