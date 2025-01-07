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
    use x86_64::instructions::interrupts;

    // Temporarily disable interrupts during critical initialization
    interrupts::disable();

    // Initialize first
    scribble::init(boot_info);

    // Explicit screen clear
    scribble::vga_buffer::clear_screen();

    // Basic VGA test with different colors to verify buffer is working
    scribble::vga_buffer::set_color(Color::White, Color::Black);
    println!("Starting Scribble OS...");

    scribble::vga_buffer::set_color(Color::Yellow, Color::Black);
    println!("Initialization complete.");

    // Re-enable interrupts
    interrupts::enable();

    // Initialize cursor after basic setup is confirmed working
    interrupts::without_interrupts(|| {
        let mut writer = scribble::vga_buffer::WRITER.lock();
        writer.clean_stray_cursors();
        writer.enable_cursor();
        writer.update_cursor();
    });

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
