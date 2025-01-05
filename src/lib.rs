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

pub fn init_heap(boot_info: &'static BootInfo) {
    use memory::{self, BootInfoFrameAllocator};

    // Get memory map from boot info
    let memory_map = &boot_info.memory_map;

    // Initialize the frame allocator
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(memory_map)
    };

    // Initialize page tables
    let mut mapper = unsafe {
        let phys_mem_offset = x86_64::VirtAddr::new(0xFFFF_8000_0000_0000);  // Common offset used by bootloader
        memory::init(phys_mem_offset)
    };

    // Initialize the heap
    crate::allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");
}

pub fn init_kernel(_boot_info: &'static BootInfo) {
    // Initialize GDT
    gdt::init();

    // Initialize IDT
    interrupts::init_idt();

    // Initialize PICS
    unsafe { interrupts::PICS.lock().initialize() };

    // Enable interrupts
    x86_64::instructions::interrupts::enable();
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
