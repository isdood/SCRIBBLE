///////////////////////
// Bare metal rust,
// no standard library.
#![no_std]
#![no_main]
//////////////////////


// IMPORTS \\
use bootloader::{entry_point, BootInfo};
use scribble::{
    println,
    debug_info,
    debug_warn,
    debug_error,
    stats
};
// use scribble::vga_buffer::Color;
// END IMPORTS \\

entry_point!(kernel_main);

// Add panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    debug_error!("PANIC: {}", info);
    loop {}
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::instructions::{interrupts, hlt};

    interrupts::disable();
    debug_info!("Starting kernel initialization");

    scribble::init(boot_info);
    debug_info!("Core systems initialized");

    // Test VGA buffer
    unsafe {
        let vga = 0xb8000 as *mut u16;
        *vga = 0x0F54;
        debug_info!("Direct VGA write complete");
    }

    interrupts::enable();
    debug_info!("Interrupts enabled");

    loop {
        hlt();

        // Print stats every 100 ticks
        let stats = stats::SYSTEM_STATS.lock();
        if stats.get_timer_ticks() % 100 == 0 {
            debug_info!("Stats - Timer: {}, Keyboard: {}",
                        stats.get_timer_ticks(),
                        stats.get_keyboard_interrupts()
            );
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
