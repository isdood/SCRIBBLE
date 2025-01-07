#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod allocator;
pub mod freezer;
pub mod gdt;
pub mod interrupts;
pub mod keyboard;
pub mod memory;
pub mod rtc;
pub mod serial;
pub mod splat;
pub mod stat;
pub mod vga_buffer;

use core::sync::atomic::AtomicBool;
use bootloader::BootInfo;

// Re-export format! macro from alloc
pub use alloc::format;
// Re-export String from alloc
pub use alloc::string::String;
// Re-export ToString trait
pub use alloc::string::ToString;

#[derive(Debug)]
pub enum InitError {
    PagingError,
    HeapError,
}

fn init_memory_management(boot_info: &'static BootInfo)
-> Result<(x86_64::structures::paging::OffsetPageTable<'static>, memory::BootInfoFrameAllocator), InitError> {
    let physical_memory_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    Ok((mapper, frame_allocator))
}

fn init_heap_memory(
    mapper: &mut x86_64::structures::paging::OffsetPageTable,
    frame_allocator: &mut memory::BootInfoFrameAllocator,
) -> Result<(), InitError> {
    allocator::init_heap(mapper, frame_allocator).map_err(|_| InitError::HeapError)
}

// Add this function that was referenced but missing
pub fn visualize_memory_map(_start_addr: VirtAddr, _size: usize) {
    // TODO: Implement memory map visualization
    splat::log(SplatLevel::BitsNBytes, "Memory map visualization not yet implemented");
}
