#![no_std]
#![no_main]

use core::panic::PanicInfo;
use scribble::{println, init_kernel, init_vga};
use scribble::vga_buffer::{Color, set_color};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialize VGA with proper cursor position
    init_vga();

    // Set initial color to green
    set_color(Color::Green, Color::Black);

    // Print initial prompt
    print!("> ");

    // Initialize kernel
    init_kernel(boot_info);

    loop {
        // Your existing loop code...
    }
}
