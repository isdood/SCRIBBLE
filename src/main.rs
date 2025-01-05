#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
use scribble::{print, println};  // Add print macro import
use scribble::vga_buffer::{self, Color};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // Initialize kernel first
    scribble::init_kernel(_boot_info);

    // Initialize VGA and enable cursor
    scribble::init_vga();

    // Print initial prompt
    print!("> ");

    // Use hlt_loop instead of busy loop
    scribble::hlt_loop();
}
