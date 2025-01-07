// src/memory.rs

use x86_64::structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB};
use x86_64::VirtAddr;
use bootloader::boot_info::{MemoryRegions, MemoryRegionKind};

// Define BootInfoFrameAllocator here (unchanged from previous example)

// This function initializes the offset page table
// It is marked as unsafe because it accesses raw pointers and hardware-specific resources
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    // Get a mutable reference to the active level 4 page table
    let level_4_table = active_level_4_table(physical_memory_offset);
    // Create a new OffsetPageTable with the level 4 table and offset
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

// This function returns a mutable reference to the active level 4 page table
// It is marked as unsafe because it dereferences raw pointers
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    // Read the physical address of the level 4 page table from the CR3 register
    let phys = x86_64::registers::control::Cr3::read().0.start_address();
    // Calculate the virtual address of the level 4 page table using the physical memory offset
    let virt = physical_memory_offset + phys.as_u64();
    // Get a raw pointer to the level 4 page table
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    // Dereference the raw pointer to get a mutable reference to the page table
    &mut *page_table_ptr
}
