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
use crate::interrupts::{init_idt, PICS};
use crate::{debug_info, debug_warn, debug_critical};

// First declare all modules
pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;
pub mod keyboard;
pub mod debug;
pub mod stats;

// Then do any re-exports
pub use stats::SYSTEM_STATS;

// Define macros here
#[macro_export]
macro_rules! debug_info {
    ($($arg:tt)*) => {{
        let message = {
            use alloc::format;
            format!($($arg)*)
        };
        $crate::debug::log($crate::debug::DebugLevel::Info, &message)
    }};
}

#[macro_export]
macro_rules! debug_warn {
    ($($arg:tt)*) => {{
        let message = {
            use alloc::format;
            format!($($arg)*)
        };
        $crate::debug::log($crate::debug::DebugLevel::Warning, &message)
    }};
}

#[macro_export]
macro_rules! debug_error {
    ($($arg:tt)*) => {{
        let message = {
            use alloc::format;
            format!($($arg)*)
        };
        $crate::debug::log($crate::debug::DebugLevel::Error, &message)
    }};
}

pub fn init(boot_info: &'static BootInfo) {
    use x86_64::instructions::interrupts;

    debug_info!("Starting system initialization");

    // Disable interrupts during initialization
    interrupts::disable();
    debug_info!("Interrupts disabled for initialization");

    // Initialize GDT first
    debug_info!("Initializing GDT...");
    gdt::init();
    debug_info!("GDT initialized successfully");

    // Initialize memory management
    debug_info!("Setting up memory management...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    debug_info!("Physical memory offset: {:#x}", boot_info.physical_memory_offset);

    let mut mapper = unsafe {
        debug_info!("Creating page mapper...");
        memory::init(phys_mem_offset)
    };

    let mut frame_allocator = unsafe {
        debug_info!("Initializing frame allocator...");
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    debug_info!("Memory management initialized");

    // Initialize heap
    debug_info!("Initializing heap (size: {} KB)...", allocator::HEAP_SIZE / 1024);
    match allocator::init_heap(&mut mapper, &mut frame_allocator) {
        Ok(_) => debug_info!("Heap initialization successful"),
        Err(e) => {
            debug_critical!("Heap initialization failed: {:?}", e);
            panic!("Heap initialization failed: {:?}", e);
        }
    }

    // Initialize interrupts
    debug_info!("Loading IDT...");
    init_idt();

    debug_info!("Initializing PIC...");
    unsafe {
        match PICS.try_lock() {
            Some(mut pics) => {
                pics.initialize();
                debug_info!("PIC initialized successfully");
            },
            None => {
                debug_critical!("Failed to acquire PIC lock during initialization");
                panic!("Failed to acquire PIC lock during initialization");
            }
        }
    }

    // Enable interrupts
    debug_info!("Enabling interrupts...");
    interrupts::enable();
    debug_info!("System initialization complete");
}


#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::serial::SERIAL1.lock(), $($arg)*);
    });
}

#[macro_export]
macro_rules! debug_println {
    () => ($crate::debug_print!("\n"));
    ($($arg:tt)*) => ($crate::debug_print!("{}\n", format_args!($($arg)*)));
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

// Test configuration...
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
// END Macros \\

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
