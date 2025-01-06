// in main.rs
#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use scribble::{println, vga_buffer}; // Add vga_buffer to the imports

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to Scribble!");

    scribble::init(boot_info);

    println!("Initialization complete.");

    loop {
        x86_64::instructions::hlt();
    }
}

fn print_prompt() {
    vga_buffer::write_prompt(); // Use the public interface function
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
