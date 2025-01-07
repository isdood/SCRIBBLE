#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

// Add required imports
use bootloader::BootInfo;
use x86_64::VirtAddr;
use alloc::format;
use crate::interrupts::{init_idt, PICS};

// First declare all modules
pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;
pub mod keyboard;
pub mod splat;
pub mod stats;

// Re-export stats
pub use stats::SYSTEM_STATS;
// Re-export splat macros (don't re-export them, they're already available from the splat module)
pub use splat::{SplatLevel, log as splat_log};

pub fn init(boot_info: &'static BootInfo) {
    use x86_64::instructions::interrupts;

    splat::log(SplatLevel::Info, "Starting system initialization");

    // Disable interrupts during initialization
    interrupts::disable();
    splat::log(SplatLevel::Info, "Interrupts disabled for initialization");

    // Initialize GDT first
    splat::log(SplatLevel::Info, "Initializing GDT...");
    gdt::init();
    splat::log(SplatLevel::Info, "GDT initialized successfully");

    // Initialize memory management
    splat::log(SplatLevel::Info, "Setting up memory management...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    splat::log(SplatLevel::Info, &format!("Physical memory offset: {:#x}", boot_info.physical_memory_offset));

    let mut mapper = unsafe {
        splat::log(SplatLevel::Info, "Creating page mapper...");
        memory::init(phys_mem_offset)
    };

    let mut frame_allocator = unsafe {
        splat::log(SplatLevel::Info, "Initializing frame allocator...");
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    splat::log(SplatLevel::Info, "Memory management initialized");

    // Initialize heap
    splat::log(SplatLevel::Info, &format!("Initializing heap (size: {} KB)...", allocator::HEAP_SIZE / 1024));
    match allocator::init_heap(&mut mapper, &mut frame_allocator) {
        Ok(_) => splat::log(SplatLevel::Info, "Heap initialization successful"),
        Err(e) => {
            splat::log(SplatLevel::Critical, &format!("Heap initialization failed: {:?}", e));
            panic!("Heap initialization failed: {:?}", e);
        }
    }

    // Initialize interrupts
    splat::log(SplatLevel::Info, "Loading IDT...");
    init_idt();

    splat::log(SplatLevel::Info, "Initializing PIC...");
    unsafe {
        match PICS.try_lock() {
            Some(mut pics) => {
                pics.initialize();
                splat::log(SplatLevel::Info, "PIC initialized successfully");
            },
            None => {
                splat::log(SplatLevel::Critical, "Failed to acquire PIC lock during initialization");
                panic!("Failed to acquire PIC lock during initialization");
            }
        }
    }

    // Enable interrupts
    splat::log(SplatLevel::Info, "Enabling interrupts...");
    interrupts::enable();
    splat::log(SplatLevel::Info, "System initialization complete");
}

// Keep the existing macro definitions for print/println/serial functions
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

// Test configuration
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
