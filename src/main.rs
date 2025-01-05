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
    println!("Initializing kernel...");  // Add this line

    // Initialize kernel first
    scribble::init_kernel(_boot_info);

    println!("Kernel initialized");  // Add this line

    // Initialize VGA (this will handle cursor and prompt)
    scribble::init_vga();

    println!("Welcome to Scribble OS");  // Add this line

    // Use hlt_loop (no need for extra prompt)
    scribble::hlt_loop();
}
