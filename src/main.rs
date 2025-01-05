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
    // Initialize kernel first
    scribble::init_kernel(_boot_info);

    // Initialize VGA
    scribble::init_vga();

    // Print initial prompt
    print!("> ");

    // Use hlt_loop
    scribble::hlt_loop();
}
