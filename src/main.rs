#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use scribble::{println, hlt_loop};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting...");

    // Initialize core system components with boot_info
    scribble::init(boot_info);

    println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();

    println!("System ready!");

    hlt_loop()
}
