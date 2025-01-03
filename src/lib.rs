#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod keyboard;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    println!("Initializing GDT...");
    gdt::init();

    println!("Initializing IDT...");
    interrupts::init_idt();

    println!("Initializing PIC...");
    unsafe {
        interrupts::PICS.lock().initialize();
    }

    println!("Initializing keyboard...");
    keyboard::init();

    println!("Initialization complete");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
