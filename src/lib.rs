// src/lib.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;
use crate::memory::{BootInfoFrameAllocator, init}; // Import the moved functions

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
pub mod unstable_matter;

pub use crate::allocator::ALLOCATOR;

#[derive(Debug)]
pub enum InitError {
    PagingError(x86_64::structures::paging::mapper::MapToError<Size4KiB>),
    HeapError,
}

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_memory_management(boot_info: &'static BootInfo)
-> Result<(OffsetPageTable<'static>, BootInfoFrameAllocator), InitError> {
    // Handle the Optional<u64> type using the helper function
    let physical_memory_offset = match optional_to_option(boot_info.physical_memory_offset) {
        Some(offset) => VirtAddr::new(offset),
        None => VirtAddr::new(0),
    };

    // Initialize the page table mapper
    let mapper = unsafe { init(physical_memory_offset) };

    // Initialize the frame allocator
    let frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    Ok((mapper, frame_allocator))
}

// Helper function to convert Optional<u64> to Option<u64>
fn optional_to_option(opt: Optional<u64>) -> Option<u64> {
    match opt {
        Optional::Some(val) => Some(val),
        Optional::None => None,
    }
}
