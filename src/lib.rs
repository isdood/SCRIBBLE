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

pub fn init_heap(boot_info: &'static BootInfo) {
    use x86_64::structures::paging::PageTable;
    use memory::BootInfoFrameAllocator;

    // Initialize a mapper
    let phys_mem_offset = VirtAddr::new(boot_info.memory_map.as_ptr() as u64);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    // Initialize the frame allocator
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Initialize the heap
    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");
}

pub fn init_kernel(boot_info: &'static BootInfo) {
    println!("Starting GDT initialization...");
    gdt::init();
    println!("GDT initialized");

    println!("Starting IDT initialization...");
    interrupts::init_idt();
    println!("IDT initialized");

    println!("Starting PIC initialization...");
    unsafe { interrupts::PICS.lock().initialize() };
    println!("PIC initialized");

    println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
    println!("Interrupts enabled");

    // Initialize heap allocation if needed
    if let Err(e) = init_heap(boot_info) {
        println!("Failed to initialize heap: {:?}", e);
    } else {
        println!("Heap initialized successfully");
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init_vga() {
    use vga_buffer::{Color, set_color, clear_screen, enable_cursor};

    enable_cursor();
    set_color(Color::White, Color::Black);
    clear_screen();
    println!("Welcome to Scribble OS");
    println!("Kernel initialized");
    println!("");
    print!("> ");
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


