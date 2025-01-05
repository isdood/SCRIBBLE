// src/lib.rs
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod memory;
pub mod keyboard;
pub mod allocator;

use bootloader::BootInfo;
use x86_64::VirtAddr;

pub fn init_heap(_boot_info: &'static BootInfo) {
    println!("Starting heap initialization...");

    // Initialize memory management
    let phys_mem_offset = VirtAddr::new(_boot_info.memory_map.as_ptr() as u64);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&_boot_info.memory_map)
    };

    // Initialize the heap
    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    println!("Heap initialization complete");
}

pub fn init_kernel(boot_info: &'static BootInfo) {
    use x86_64::VirtAddr;

    println!("Starting GDT initialization...");
    gdt::init();
    println!("GDT initialized");

    println!("Starting IDT initialization...");
    interrupts::init_idt();
    println!("IDT initialized");

    // Initialize memory management before PIC
    println!("Starting memory management initialization...");
    let phys_mem_offset = VirtAddr::new(boot_info.memory_map.as_ptr() as u64);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    println!("Memory management initialized");

    // Initialize heap after memory management
    println!("Starting heap initialization...");
    match allocator::init_heap(&mut mapper, &mut frame_allocator) {
        Ok(_) => println!("Heap initialized successfully"),
        Err(e) => panic!("Heap initialization failed: {:?}", e),
    }

    println!("Starting PIC initialization...");
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    println!("PIC initialized");

    println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
    println!("Interrupts enabled");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn init_vga() {
    vga_buffer::enable_cursor();
    vga_buffer::clear_screen();
    print!("> ");
}
