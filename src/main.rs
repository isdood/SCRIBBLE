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
    use x86_64::instructions::{interrupts, hlt};
    use scribble::vga_buffer::{Color, WRITER};
    use core::fmt::Write;

    // Disable interrupts during initialization
    interrupts::disable();
    serial_println!("[DEBUG] Starting with interrupts disabled");

    // Initialize core systems
    scribble::init(boot_info);
    serial_println!("[DEBUG] Core init complete");

    // Test VGA buffer direct write
    unsafe {
        let vga = 0xb8000 as *mut u16;
        // White on black, character 'T'
        *vga = 0x0F54; // 0x0F is white on black, 0x54 is ASCII 'T'
        serial_println!("[DEBUG] Direct VGA write complete");
    }

    // Check PIC and interrupt status
    unsafe {
        let mut pics = PICS.lock();
        serial_println!("[DEBUG] PIC masks: primary={:08b}, secondary={:08b}",
                        pics.read_mask(pic8259::ChainedPics::PRIMARY),
                        pics.read_mask(pic8259::ChainedPics::SECONDARY));
    }

    // Enable interrupts and measure interrupt frequency
    interrupts::enable();
    serial_println!("[DEBUG] Interrupts enabled");

    let mut last_tick = 0;
    let mut tick_count = 0;
    let mut last_diagnostic = 0;

    loop {
        // Use hlt to reduce CPU usage
        hlt();

        // Count timer ticks for diagnostics
        if let Some(ticks) = get_timer_ticks() {
            if ticks != last_tick {
                tick_count += 1;
                last_tick = ticks;

                // Print diagnostics every 100 ticks
                if tick_count - last_diagnostic >= 100 {
                    serial_println!("[DEBUG] Timer ticks: {}, Keyboard interrupts received: {}",
                                    tick_count, get_keyboard_count());
                    last_diagnostic = tick_count;
                }
            }
        }
    }
}

// Add these helper functions to track interrupt activity
static mut TIMER_TICKS: u64 = 0;
static mut KEYBOARD_INTERRUPTS: u64 = 0;

pub fn get_timer_ticks() -> Option<u64> {
    if x86_64::instructions::interrupts::are_enabled() {
        Some(unsafe { TIMER_TICKS })
    } else {
        None
    }
}

pub fn get_keyboard_count() -> u64 {
    unsafe { KEYBOARD_INTERRUPTS }
}
