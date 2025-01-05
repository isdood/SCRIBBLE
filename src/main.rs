#![no_std]
#![no_main]

use core::panic::PanicInfo;
use scribble::{print, println};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    scribble::hlt_loop();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Initializing kernel...");

    // Initialize kernel with boot info
    scribble::init_kernel(boot_info);

    println!("Kernel initialized");

    // Initialize VGA (this will handle cursor and prompt)
    scribble::vga_buffer::init();
    scribble::vga_buffer::clear_screen();

    println!("Welcome to Scribble OS");
    print!("> ");

    scribble::hlt_loop();
}
