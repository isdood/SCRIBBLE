#![no_std]
#![no_main]

use core::panic::PanicInfo;
use scribble::{print, println};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    scribble::hlt_loop();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Starting kernel initialization...");

    // Initialize kernel with boot info
    match std::panic::catch_unwind(|| {
        scribble::init_kernel(boot_info);
    }) {
        Ok(_) => println!("Kernel initialization completed successfully"),
        Err(e) => println!("Kernel initialization failed: {:?}", e),
    }

    println!("Initializing VGA...");
    scribble::init_vga();
    println!("VGA initialized");

    print!("> ");

    scribble::hlt_loop();
}
