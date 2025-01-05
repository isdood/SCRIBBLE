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
    system_println!("Booting Scribble OS...");
    system_println!("Starting kernel initialization...");

    // Initialize kernel components
    scribble::init_kernel(boot_info);

    system_println!("Kernel initialization complete");
    system_println!("Initializing VGA...");

    // Initialize VGA after kernel is ready
    scribble::init_vga();

    system_println!("Welcome to Scribble OS");
    print!("> ");  // This will be green because it's a prompt

    // Enable interrupts
    x86_64::instructions::interrupts::enable();

    // Enter main loop
    scribble::hlt_loop();
}
