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

    // Now we can properly manage the cursor
    {
        let mut writer = scribble::vga_buffer::WRITER.lock();
        writer.restore_previous_cursor();  // Clean up any existing cursor
        writer.enable_cursor();           // Properly initialize cursor
    }

    print!("\n");
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
