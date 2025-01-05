#![no_std]
#![no_main]

use core::panic::PanicInfo;
use scribble::{print, println};  // Keep only what we use
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    scribble::hlt_loop();
}

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // Initialize basic hardware first
    scribble::init_kernel(_boot_info);

    // Small delay to ensure hardware is ready
    for _ in 0..10000 {
        x86_64::instructions::nop();
    }

    // Now initialize VGA
    scribble::init_vga();

    // Enter main loop
    scribble::hlt_loop();
}
