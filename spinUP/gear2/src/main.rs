#![no_std]
#![no_main]
#![feature(asm_const)]

// Tell the compiler we're targeting i686 initially
#[cfg(target_arch = "x86_64")]
global_asm!(".code32");

#[no_mangle]
#[link_section = ".text"]
pub unsafe extern "C" fn _start() -> ! {
    // We start in 32-bit protected mode
    let vga = 0xb8000 as *mut u16;

    // Write "OK!" to show we're running
    *vga = 0x0F4F;       // White 'O' on black
    *(vga.add(1)) = 0x0F4B; // White 'K' on black
    *(vga.add(2)) = 0x0F21; // White '!' on black

    loop {
        core::arch::asm!("hlt");
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
