#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use scribble::serial_println;

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[PANIC] {}", info);
    scribble::hlt_loop();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Initializing kernel...");

    // Initialize kernel with boot info
    scribble::init_kernel(boot_info);

    println!("Kernel initialized");

    // Initialize VGA (this will handle cursor and prompt)
    scribble::init_vga();

    println!("Welcome to Scribble OS");
    print!("> ");

    // Use hlt_loop
    scribble::hlt_loop();
}
