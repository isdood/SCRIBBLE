#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use scribble::println;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");

    scribble::init(boot_info);

    println!("Initialization complete!");
    print_prompt();

    loop {
        x86_64::instructions::hlt();  // Add this to reduce CPU usage
    }
}

fn print_prompt() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_prompt();
    });
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();  // Add this to reduce CPU usage
    }
}
