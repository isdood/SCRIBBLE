// src/lib.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::BootInfo;
use x86_64::structures::paging::{OffsetPageTable, PageTable, Size4KiB};
use x86_64::VirtAddr;

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
-> Result<(OffsetPageTable<'static>, memory::BootInfoFrameAllocator), InitError> {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset.unwrap_or(0) as u64);
    let mapper = unsafe { init(physical_memory_offset) };
    let frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    Ok((mapper, frame_allocator))
}

unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let phys = x86_64::registers::control::Cr3::read().0.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr
}
