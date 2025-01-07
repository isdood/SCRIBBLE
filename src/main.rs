#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use scribble::{debug_info, debug_error, stats};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    debug_error!("PANIC: {}", info);
    loop {}
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::instructions::{interrupts, hlt};

    // First initialize the kernel before any debug output
    scribble::init(boot_info);

    interrupts::disable();
    debug_info!("Starting kernel initialization");
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

        let stats = stats::SYSTEM_STATS.lock();
        if stats.get_timer_ticks() % 100 == 0 {
            debug_info!("Stats - Timer: {}, Keyboard: {}",
                        stats.get_timer_ticks(),
                        stats.get_keyboard_interrupts()
            );
        }
    }
}
