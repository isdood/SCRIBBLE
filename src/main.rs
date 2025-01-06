#![no_std]
#![no_main]

use core::panic::PanicInfo;
use scribble::println;
use scribble::vga_buffer::WRITER;
use x86_64::instructions::interrupts;
use scribble::gdt::init; // Import the correct init function

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the kernel
    init();

    // Print initial setup messages
    println!("Initializing kernel...");
    println!("Enabling interrupts...");

    // Enable interrupts
    interrupts::enable();

    // Set input mode and print the prompt on a new line
    WRITER.lock().new_line(); // Ensure we start on a new line for the prompt
    WRITER.lock().set_input_mode(true);

    // Halt the CPU
    loop {
        x86_64::instructions::hlt();
    }
}

// Define the panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Print the panic information to the VGA buffer
    println!("{}", _info);
    // Halt the CPU
    loop {
        x86_64::instructions::hlt();
    }
}
