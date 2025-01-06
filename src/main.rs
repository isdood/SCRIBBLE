#![no_std]
#![no_main]

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use scribble::println;
use linked_list_allocator::LockedHeap;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    scribble::init(boot_info);

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    // Initialize heap allocation
    const HEAP_START: usize = 0x_4444_4444_0000;
    const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

    // Print a command prompt
    print_prompt();

    loop {
        // Handle keyboard input in the loop
    }
}

fn print_prompt() {
    use scribble::print;
    print!("> ");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
