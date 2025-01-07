#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use scribble::{debug_info, debug_error, stats};
use x86_64::instructions::{interrupts, hlt};

/////////////////////////
entry_point!(kernel_main);
////////////////////////

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    debug_info!("Kernel boot started");

    // Disable interrupts during initialization
    interrupts::disable();
    debug_info!("Interrupts disabled for kernel initialization");

    // Initialize the system
    scribble::init(boot_info);
    debug_info!("System initialization completed successfully");

    // Enable interrupts
    debug_info!("Enabling interrupts for main loop");
    interrupts::enable();

    debug_info!("Entering main kernel loop");
    let mut last_tick_count = 0;
    loop {
        hlt(); // CPU sleep until next interrupt

        if interrupts::are_enabled() {
            let current_ticks = {
                let stats = stats::SYSTEM_STATS.lock();
                stats.get_timer_ticks()
            };

            // Log every 1000 ticks
            if current_ticks >= last_tick_count + 1000 {
                let (keyboard_ints, timer_ticks) = {
                    let stats = stats::SYSTEM_STATS.lock();
                    (stats.get_keyboard_interrupts(), stats.get_timer_ticks())
                };

                debug_info!("System Status - Uptime ticks: {}, Keyboard interrupts: {}",
                            timer_ticks, keyboard_ints);

                // If we're seeing unusually high interrupt counts, log a warning
                if timer_ticks - last_tick_count > 1100 { // Should be ~1000
                    debug_warn!("Timer ticks increasing faster than expected");
                }

                last_tick_count = current_ticks;
            }
        } else {
            debug_critical!("Interrupts disabled unexpectedly in main loop!");
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    debug_critical!("KERNEL PANIC: {}", info);
    debug::dump_debug_log();
    loop {
        hlt();
    }
}
