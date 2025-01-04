// src/main.rs
#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use scribble::{println, hlt_loop};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    scribble::init(boot_info);

    println!("\nWelcome to Scribble OS!");

    // Loop indefinitely
    hlt_loop();
}
