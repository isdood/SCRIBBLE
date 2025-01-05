use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use scribble::{println, serial_println};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[PANIC] {}", info);
    scribble::hlt_loop();
}

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    serial_println!("[DEBUG] Kernel starting");

    // Initialize kernel
    scribble::init_kernel(_boot_info);
    serial_println!("[DEBUG] Kernel initialized");

    // Initialize VGA
    scribble::init_vga();
    serial_println!("[DEBUG] VGA initialized");

    // Halt
    scribble::hlt_loop();
}
