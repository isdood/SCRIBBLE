#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
use scribble::println;
use scribble::vga_buffer::{self, Color, WRITER};

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize VGA with proper cursor position
    vga_buffer::init();

    // Set initial color to green
    vga_buffer::set_color(Color::Green, Color::Black);

    // Print initial prompt
    print!("> ");

    // Initialize hardware
    scribble::init();

    // Main loop
    loop {
        // Your existing loop code...
    }
}
