#![no_std]
#![no_main]

use core::panic::PanicInfo;
use scribble::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting Scribble OS...");
    println!("Starting kernel initialization...");

    // Initialize kernel components
    scribble::init_kernel(boot_info);

    println!("Kernel initialization complete");

    // Show initial date/time
    scribble::show_datetime();

    println!("Welcome to Scribble OS!");

    scribble::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic: {}", info);
    scribble::hlt_loop();
}
