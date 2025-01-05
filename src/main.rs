// src/main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use scribble::{print, println};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic: {}", info);
    scribble::hlt_loop();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use scribble::{print, println};

    println!("Booting Scribble OS...");
    println!("Starting kernel initialization...");

    // Initialize kernel components with error checking
    // Disable interrupts during initialization
    x86_64::instructions::interrupts::disable();

    // Initialize kernel components
    scribble::init_kernel(boot_info);

    println!("Kernel initialization complete");
    println!("Initializing VGA...");

    // Initialize VGA after kernel is ready
    scribble::init_vga();

    println!("Welcome to Scribble OS!");
    print!("> ");

    // Enable interrupts
    x86_64::instructions::interrupts::enable();

    // Enter main loop
    scribble::hlt_loop();
}
