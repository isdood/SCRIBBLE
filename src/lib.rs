#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod allocator;
pub mod freezer;
pub mod gdt;
pub mod interrupts;
pub mod keyboard;
pub mod memory;
pub mod pic8259;
pub mod rtc;
pub mod serial;
pub mod splat;
pub mod stat;
pub mod vga_buffer;
pub use crate::allocator::ALLOCATOR;

use bootloader::BootInfo;
use x86_64::{
    structures::paging::{
        OffsetPageTable, Size4KiB,
        mapper::MapToError,
    },
    VirtAddr,
};

// Re-export commonly used items
pub use alloc::format;
pub use crate::splat::SplatLevel;
pub use alloc::string::{String, ToString};
pub use x86_64::instructions::hlt;

#[derive(Debug)]
pub enum InitError {
    PagingError(MapToError<Size4KiB>),
    HeapError,
}

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

/// Memory management thresholds
#[allow(dead_code)]
const LOW_MEMORY_THRESHOLD: usize = HEAP_SIZE / 10;
#[allow(dead_code)]
const CRITICAL_MEMORY_THRESHOLD: usize = HEAP_SIZE / 20;
#[allow(dead_code)]
const FRAGMENTATION_THRESHOLD: f32 = 0.5;

/// Initialize the memory management system
pub fn init_memory_management(boot_info: &'static BootInfo)
-> Result<(OffsetPageTable<'static>, memory::BootInfoFrameAllocator), InitError> {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(physical_memory_offset) };
    let frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    Ok((mapper, frame_allocator))
}

pub fn init_heap_memory(
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut memory::BootInfoFrameAllocator,
) -> Result<(), InitError> {
    allocator::init_heap(mapper, frame_allocator)
    .map_err(|_| InitError::HeapError)
}

pub fn visualize_memory_map(_start_addr: VirtAddr, _size: usize) {
    splat::log(
        SplatLevel::BitsNBytes,
        "Memory map visualization not yet implemented"
    );
}

/// Get the current system timestamp as a formatted string
pub fn get_system_timestamp() -> String {
    format!("{}", rtc::DateTime::now().to_string())
}

/// Get the current system user
pub fn get_current_user() -> &'static str {
    "isdood"
}

/// Get the current kernel version
pub fn get_kernel_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Print system information
pub fn print_system_info() {
    splat::log(
        SplatLevel::Info,
        &format!(
            "Scribble Kernel\n\
Version: {}\n\
User: {}\n\
Time: {}",
get_kernel_version(),
                 get_current_user(),
                 get_system_timestamp()
        )
    );
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_memory_thresholds() {
        assert!(LOW_MEMORY_THRESHOLD < CRITICAL_MEMORY_THRESHOLD);
        assert!(FRAGMENTATION_THRESHOLD > 0.0 && FRAGMENTATION_THRESHOLD <= 1.0);
    }

    #[test_case]
    fn test_heap_constants() {
        assert!(HEAP_SIZE > 0);
        assert!(HEAP_START > 0);
    }
}
