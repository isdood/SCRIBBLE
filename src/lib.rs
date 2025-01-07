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

    // Disable interrupts during initialization
    interrupts::disable();

    // Initialize GDT first
    gdt::init();
    debug_info!("GDT initialized");

    // Initialize memory management
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    debug_info!("Memory mapper initialized");

    // Initialize heap with error checking
    match allocator::init_heap(&mut mapper, &mut frame_allocator) {
        Ok(_) => debug_info!("Heap initialization successful: {} KB", allocator::HEAP_SIZE / 1024),
        Err(e) => panic!("Heap initialization failed: {:?}", e),
    }

    // Initialize interrupts after memory is set up
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    debug_info!("Interrupts initialized");

    // Enable interrupts
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
