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

pub fn init_kernel(boot_info: &'static BootInfo) {
    // Disable interrupts during initialization
    x86_64::instructions::interrupts::disable();

    println!("Initializing GDT...");
    gdt::init();

    println!("Initializing IDT...");
    interrupts::init_idt();

    println!("Initializing PIC...");
    unsafe {
        interrupts::PICS.lock().initialize();
    }

    println!("Initializing memory management...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    println!("Initializing heap...");
    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    println!("Initializing VGA...");
    vga_buffer::init();

    // Enable interrupts last
    println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
}

pub fn show_datetime() {
    let mut rtc = rtc::RTC_DEVICE.lock();
    let datetime = rtc.format_datetime();
    println!("Current Date and Time (UTC): {}", datetime);
    println!("Current User's Login: isdood");
}

// ... rest of your existing lib.rs code ...
