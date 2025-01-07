#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

// Add required imports at the top
use bootloader::BootInfo;
use x86_64::VirtAddr;
use alloc::format;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::splat::SplatLevel;

// Module declarations
pub mod memory;  // Add this - it's referenced but wasn't declared
pub mod rtc;
pub mod freezer;
pub mod splat;
pub mod stat;
pub mod gdt;
pub mod keyboard;
pub mod serial;
pub mod allocator;
pub mod vga_buffer;
pub mod interrupts;

// System state tracking
static SYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub enum InitError {
    GDTFailed,
    MemoryInitFailed,
    HeapInitFailed,
    PICInitFailed,
    LockError(&'static str),
}

// Rest of the implementation remains the same...

fn init_memory_management(boot_info: &'static BootInfo)
-> Result<(x86_64::structures::paging::OffsetPageTable<'static>, memory::BootInfoFrameAllocator), InitError> {
    // Changed the return type to use the full path for OffsetPageTable
    // ... rest of the implementation remains the same
}

fn init_heap_memory(
    mut mapper: x86_64::structures::paging::OffsetPageTable,
    mut frame_allocator: memory::BootInfoFrameAllocator
) -> Result<(), InitError> {
    // Changed the parameter type to use the full path
    // ... rest of the implementation remains the same
}

// Add this function that was referenced but missing
pub fn visualize_memory_map(_start_addr: VirtAddr, _size: usize) {
    // TODO: Implement memory map visualization
    splat::log(SplatLevel::BitsNBytes, "Memory map visualization not yet implemented");
}
