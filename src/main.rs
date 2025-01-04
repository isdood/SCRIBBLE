#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use scribble::{println, hlt_loop};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting Scribble OS...");

    scribble::init(boot_info);

    println!("Boot sequence complete!");
    println!("Welcome to Scribble OS!");
    println!("Type something to test the keyboard...");

    hlt_loop();
}

// Remove the duplicate panic handler since it's already defined in lib.rs
