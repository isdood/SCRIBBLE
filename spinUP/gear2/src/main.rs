#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u16;
    unsafe {
        *vga = 0x0F4F;       // White 'O' on black
        *(vga.add(1)) = 0x0F4B; // White 'K' on black
        *(vga.add(2)) = 0x0F21; // White '!' on black
    }

    loop {
        core::arch::asm!("hlt");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::arch::asm!("hlt");
    }
}
