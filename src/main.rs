// src/main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use scribble::{print, println};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic: {}", info);
    scribble::hlt_loop();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting Scribble OS...");

    match std::panic::catch_unwind(|| {
        println!("Starting kernel initialization...");
        scribble::init_kernel(boot_info);
        println!("Kernel initialization complete");
    }) {
        Ok(_) => {
            println!("Initializing VGA...");
            scribble::init_vga();
            println!("Welcome to Scribble OS!");
            print!("> ");
        }
        Err(e) => {
            println!("Kernel initialization failed!");
            if let Some(msg) = e.downcast_ref::<&str>() {
                println!("Error: {}", msg);
            }
        }
    }

    scribble::hlt_loop();
}
