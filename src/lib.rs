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
pub mod rtc;
pub mod serial;
pub mod splat;
pub mod stat;
pub mod vga_buffer;

use bootloader::BootInfo;
use x86_64::VirtAddr;

// Re-export commonly used items
pub use alloc::format;
pub use alloc::string::String;
pub use alloc::string::ToString;
pub use crate::splat::SplatLevel;

#[derive(Debug)]
pub enum InitError {
    PagingError,
    HeapError,
}

const HEAP_START: usize = 0x_4444_4444_0000;
const HEAP_SIZE: usize = 100 * 1024; // 100 KiB
const LOW_MEMORY_THRESHOLD: usize = HEAP_SIZE / 10;
const CRITICAL_MEMORY_THRESHOLD: usize = HEAP_SIZE / 20;
const FRAGMENTATION_THRESHOLD: f32 = 0.5;

fn init_memory_management(boot_info: &'static BootInfo)
-> Result<(x86_64::structures::paging::OffsetPageTable<'static>, memory::BootInfoFrameAllocator), InitError>
{
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(physical_memory_offset) };
    let frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    Ok((mapper, frame_allocator))
}

fn init_heap_memory(
    mapper: &mut x86_64::structures::paging::OffsetPageTable,
    frame_allocator: &mut memory::BootInfoFrameAllocator,
) -> Result<(), InitError> {
    allocator::init_heap(mapper, frame_allocator).map_err(|_| InitError::HeapError)
}

pub fn visualize_memory_map(start_addr: VirtAddr, size: usize) {
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
