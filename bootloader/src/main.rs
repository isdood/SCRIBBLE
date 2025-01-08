#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::ptr::write_volatile(0xb8000 as *mut u8, b'X');
        core::ptr::write_volatile(0xb8001 as *mut u8, 0x0F);
    }
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
