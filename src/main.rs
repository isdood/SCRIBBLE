#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use scribble::println;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // Disable interrupts during early boot
    x86_64::instructions::interrupts::disable();

    println!("Booting...");

    // Initialize core system components
    scribble::init();

    println!("Enabling interrupts...");
    // Re-enable interrupts
    x86_64::instructions::interrupts::enable();

    println!("System ready!");

    scribble::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
