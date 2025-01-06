#![no_std]
#![no_main]

// Assuming this is part of the main entry point of your kernel (e.g., src/main.rs)

extern crate scribble;

use scribble::vga_buffer::WRITER;
use x86_64::instructions::interrupts;

fn main() {
    // Initialize the VGA buffer and other necessary components
    scribble::init();

    // Print initial setup messages
    println!("Initializing kernel...");
    println!("Enabling interrupts...");

    // Enable interrupts
    interrupts::enable();

    // Set input mode and print the prompt on a new line
    WRITER.lock().new_line(); // Ensure we start on a new line for the prompt
    WRITER.lock().set_input_mode(true);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic: {}", info);
    scribble::hlt_loop();
}
