/////////////////////////////////
// Bare metal rust, no standard
// library.

#![no_std]
#![no_main]

/////////////////////////////////

         //IMPORTS\\
/////////////////////////////////

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use scribble::{println, print};

////////////////////////////////

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to Scribble!");
    scribble::init(boot_info);
    println!("Initialization complete.");

    // Instead of directly calling restore_previous_cursor,
    // we can force a cursor update which will handle cleanup
    {
        let mut writer = scribble::vga_buffer::WRITER.lock();
        writer.update_cursor();
    }

    // Add an explicit newline before the prompt
    print!("\n");
    // Write the first prompt
    scribble::vga_buffer::write_prompt();

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
