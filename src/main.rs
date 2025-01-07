#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use scribble::{debug_info, debug_error, stats};
use x86_64::instructions::{interrupts, hlt};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    debug_error!("PANIC: {}", info);
    loop {
        hlt();
    }
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Disable interrupts during initialization
    interrupts::disable();

    // Initialize the system
    scribble::init(boot_info);
    debug_info!("System initialization complete");

    // Enable interrupts
    interrupts::enable();
    debug_info!("Interrupts enabled");

    // Main system loop with controlled memory access
    loop {
        hlt();

        if interrupts::are_enabled() {
            let stats = stats::SYSTEM_STATS.lock();
            if stats.get_timer_ticks() % 100 == 0 {
                // Release the lock quickly
                let ticks = stats.get_timer_ticks();
                let interrupts = stats.get_keyboard_interrupts();
                drop(stats); // Release lock before debug output

                debug_info!("Stats - Timer: {}, Keyboard: {}", ticks, interrupts);
            }
        }
    }
}
