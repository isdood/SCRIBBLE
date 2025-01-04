#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use scribble::{init, println, hlt_loop};  // Add init to the imports

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting Scribble OS...");

    // Initialize the system
    init(boot_info);  // Use the imported init function

    println!("Boot sequence complete!");
    println!("Welcome to Scribble OS!");
    println!("Type something to test the keyboard...");

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    hlt_loop();
}
