use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use scribble::{println, serial_println};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[PANIC] {}", info);
    scribble::hlt_loop();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("[DEBUG] Kernel starting");

    // Initialize basic hardware
    scribble::init_kernel(boot_info);
    serial_println!("[DEBUG] Basic hardware initialized");

    // Initialize heap
    scribble::init_heap(boot_info);
    serial_println!("[DEBUG] Heap initialized");

    // Initialize VGA
    scribble::init_vga();
    serial_println!("[DEBUG] VGA initialized");

    // Enter main loop
    scribble::hlt_loop();
}
