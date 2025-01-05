#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn show_datetime() {
    let mut rtc = rtc::RTC_DEVICE.lock();
    let (year, month, day) = rtc.get_date();
    let (hours, minutes, seconds) = rtc.get_time();
    crate::println!(
        "Current Date and Time (UTC): {}-{:02}-{:02} {:02}:{:02}:{:02}",
                    year, month, day, hours, minutes, seconds
    );
    crate::println!("Current User's Login: isdood");
    crate::print!("> ");
}

// ... rest of lib.rs remains the same ...

pub fn init_kernel(boot_info: &'static BootInfo) {
    // Disable interrupts during initialization
    x86_64::instructions::interrupts::disable();

    crate::println!("Initializing GDT...");
    gdt::init();

    crate::println!("Initializing IDT...");
    interrupts::init_idt();

    crate::println!("Initializing PIC...");
    unsafe {
        interrupts::PICS.lock().initialize();
    }

    crate::println!("Initializing memory management...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    crate::println!("Initializing heap...");
    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    crate::println!("Initializing VGA...");
    vga_buffer::init();

    crate::println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
}

#[inline]
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
