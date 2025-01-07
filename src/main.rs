///////////////////////
// Bare metal rust,
// no standard library.
#![no_std]
#![no_main]
//////////////////////


// IMPORTS \\
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use scribble::{println, print};
// use scribble::vga_buffer::Color;
// END IMPORTS \\

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::instructions::interrupts;
    use scribble::vga_buffer::Color;

    // Disable interrupts during initialization
    interrupts::disable();

    // Initialize core systems
    scribble::init(boot_info);

    // Clear screen AFTER initialization
    scribble::vga_buffer::clear_screen();

    // Print boot messages with helper function
    scribble::vga_buffer::colored_print(
        Color::White,
        Color::Black,
        "Starting Scribble OS...\n"
    );

    scribble::vga_buffer::colored_print(
        Color::Yellow,
        Color::Black,
        "Initialization complete.\n"
    );

    // Initialize keyboard and cursor
    interrupts::without_interrupts(|| {
        let mut writer = scribble::vga_buffer::WRITER.lock();
        writer.clean_stray_cursors();
        writer.enable_cursor();
        writer.update_cursor();
    });

    // Write initial prompt
    print!("\n");
    scribble::vga_buffer::write_prompt();

    // Enable interrupts AFTER all initialization is complete
    interrupts::enable();

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
