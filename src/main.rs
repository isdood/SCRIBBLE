#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use scribble::{debug_info, debug_error, stats};
use x86_64::instructions::interrupts;

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    debug_error!("PANIC: {}", info);
    loop {
        x86_64::instructions::hlt();  // Add hlt instruction in panic loop
    }
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialize with interrupts disabled
    interrupts::disable();

    // Initialize core systems
    scribble::init(boot_info);
    debug_info!("Core systems initialized");

    // Test VGA buffer
    unsafe {
        let vga = 0xb8000 as *mut u16;
        *vga = 0x0F54;
        debug_info!("Direct VGA write complete");
    }

    // Enable interrupts after all initialization is complete
    interrupts::enable_and_hlt();  // Use enable_and_hlt instead of separate enable/hlt
    debug_info!("Interrupts enabled");

    // Main system loop
    loop {
        // Use hlt_loop instead of raw hlt to properly handle interrupts
        x86_64::instructions::hlt();

        // Only check stats when interrupts are enabled
        if interrupts::are_enabled() {
            let stats = stats::SYSTEM_STATS.lock();
            if stats.get_timer_ticks() % 100 == 0 {
                debug_info!("Stats - Timer: {}, Keyboard: {}",
                            stats.get_timer_ticks(),
                            stats.get_keyboard_interrupts()
                );
            }
        }
    }
}
