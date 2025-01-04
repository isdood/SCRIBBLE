// src/main.rs
#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use scribble::{print, println, hlt_loop};  // Added print macro import

// Define the entry point using bootloader's macro
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    scribble::init(boot_info);

    println!("\nWelcome to Scribble OS!");
    println!("Type something to test the keyboard...");
    print!("Ready for input > ");

    hlt_loop();
}
