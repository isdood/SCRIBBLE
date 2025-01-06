#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use scribble::println; // Ensure println is imported correctly

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the kernel
    scribble::init();

    // Print initial setup messages
    println!("Initializing kernel...");
    println!("Enabling interrupts...");

    // Enable interrupts
    x86_64::instructions::interrupts::enable();

    // Set input mode and print the prompt on a new line
    scribble::vga_buffer::WRITER.lock().new_line(); // Ensure we start on a new line for the prompt
    scribble::vga_buffer::WRITER.lock().set_input_mode(true);

    // Halt the CPU
    loop {
        x86_64::instructions::hlt();
    }
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    // Initialize the heap allocator here, for example using a fixed-size heap
    // You might need to adjust the heap size and location based on your memory layout
    let heap_start = ...;
    let heap_size = ...;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
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
