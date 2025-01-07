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

    // Main system loop with sleep
    let mut last_stats_print = 0;
    loop {
        hlt(); // CPU sleep until next interrupt

        if interrupts::are_enabled() {
            let ticks = {
                let stats = stats::SYSTEM_STATS.lock();
                stats.get_timer_ticks()
            };

            // Only print stats every 5000 ticks (approximately every 5 seconds)
            if ticks >= last_stats_print + 5000 {
                let (keyboard_ints, timer_ticks) = {
                    let stats = stats::SYSTEM_STATS.lock();
                    (stats.get_keyboard_interrupts(), stats.get_timer_ticks())
                };

                debug_info!("System Stats - Timer: {}, Keyboard: {}",
                            timer_ticks, keyboard_ints);
                last_stats_print = ticks;
            }
        }
    }
}
