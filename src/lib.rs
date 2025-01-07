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

use bootloader::BootInfo;
use x86_64::VirtAddr;

// Re-export commonly used items
pub use alloc::format;
pub use crate::splat::SplatLevel;
pub use alloc::string::{String, ToString};
pub use x86_64::instructions::hlt;

#[derive(Debug)]
pub enum InitError {
    PagingError,
    HeapError,
}

const HEAP_START: usize = 0x_4444_4444_0000;
const HEAP_SIZE: usize = 100 * 1024; // 100 KiB
/// Memory management thresholds
#[allow(dead_code)]
const LOW_MEMORY_THRESHOLD: usize = HEAP_SIZE / 10;
#[allow(dead_code)]
const CRITICAL_MEMORY_THRESHOLD: usize = HEAP_SIZE / 20;
#[allow(dead_code)]
const FRAGMENTATION_THRESHOLD: f32 = 0.5;

/// Initialize the memory management system
#[allow(dead_code)]
fn init_memory_management(boot_info: &'static BootInfo)
-> Result<(x86_64::structures::paging::OffsetPageTable<'static>, memory::BootInfoFrameAllocator), InitError> {
    // ... existing implementation ...
}

fn init_heap_memory(
    mapper: &mut x86_64::structures::paging::OffsetPageTable,
    frame_allocator: &mut memory::BootInfoFrameAllocator,
) -> Result<(), InitError> {
    allocator::init_heap(mapper, frame_allocator).map_err(|_| InitError::HeapError)
}

pub fn visualize_memory_map(_start_addr: VirtAddr, _size: usize) {
    use crate::splat::SplatLevel;
    splat::log(SplatLevel::BitsNBytes, "Memory map visualization not yet implemented");
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
