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
pub mod rtc;

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

    // Disable interrupts during initialization
    x86_64::instructions::interrupts::disable();

    system_println!("Starting GDT initialization...");
    gdt::init();
    system_println!("GDT initialized");

    system_println!("Starting IDT initialization...");
    interrupts::init_idt();
    system_println!("IDT initialized");

    system_println!("Starting memory management initialization...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    system_println!("Physical memory offset: {:#x}", phys_mem_offset.as_u64());

    // Initialize the memory mapper
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    system_println!("Memory mapper initialized");

    // Initialize the frame allocator
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    println!("Frame allocator initialized");

    // Initialize heap
    println!("Starting heap initialization...");
    match allocator::init_heap(&mut mapper, &mut frame_allocator) {
        Ok(_) => println!("Heap initialized successfully"),
        Err(e) => {
            println!("Failed to initialize heap: {:?}", e);
            hlt_loop();
        }
    }

    println!("Starting PIC initialization...");
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    println!("PIC initialized");

    // Enable interrupts after everything is set up
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
    vga_buffer::clear_screen();
    vga_buffer::enable_cursor();
}

pub fn show_datetime() {
    let mut rtc = rtc::RTC_DEVICE.lock();
    let (year, month, day) = rtc.get_date();
    let (hours, minutes, seconds) = rtc.get_time();

    println!("Date: {}-{:02}-{:02}", year, month, day);
    println!("Time (UTC): {:02}:{:02}:{:02}", hours, minutes, seconds);
    println!("User: isdood");  // Hardcoded for now, could be made configurable
}
