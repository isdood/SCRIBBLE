#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use spinUP::boot_params::BootParams;

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

// Remove this line since we're using it as a dependency
// pub mod unstable_matter;

pub use crate::allocator::ALLOCATOR;

#[derive(Debug)]
pub enum InitError {
    PagingError(x86_64::structures::paging::mapper::MapToError<Size4KiB>),
    HeapError,
}

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

// Update the function to use BootParams from spinUP instead of bootloader
pub fn init_memory_management(boot_params: &'static BootParams)
-> Result<(OffsetPageTable<'static>, BootInfoFrameAllocator), InitError> {
    let physical_memory_offset = VirtAddr::new(boot_params.memory_map_addr as u64);

    // Initialize the page table mapper
    let mapper = unsafe { memory::init(physical_memory_offset) };

    // Initialize the frame allocator with the boot parameters
    let frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init_from_boot_params(boot_params)
    };

    Ok((mapper, frame_allocator))
}
