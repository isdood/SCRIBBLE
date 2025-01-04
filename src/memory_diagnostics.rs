use crate::{print, println};
use x86_64::{
    structures::paging::{PageTable, PageTableFlags},
    VirtAddr,
};
use alloc::{string::String, vec::Vec, format};
use x86_64::structures::paging::page_table::PageTableEntry;

#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    total_memory: usize,
    used_memory: usize,
    free_memory: usize,
    allocated_pages: usize,
    free_pages: usize,
}

impl MemoryStats {
    pub fn new() -> Self {
        MemoryStats {
            total_memory: 0,
            used_memory: 0,
            free_memory: 0,
            allocated_pages: 0,
            free_pages: 0,
        }
    }

    pub fn update(&mut self, total: usize, used: usize, allocated_pages: usize) {
        self.total_memory = total;
        self.used_memory = used;
        self.free_memory = total.saturating_sub(used);
        self.allocated_pages = allocated_pages;
        self.free_pages = (total / 4096).saturating_sub(allocated_pages);
    }
}

impl fmt::Display for MemoryStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Memory Statistics:")?;
        writeln!(f, "  Total Memory: {} KB", self.total_memory / 1024)?;
        writeln!(f, "  Used Memory:  {} KB", self.used_memory / 1024)?;
        writeln!(f, "  Free Memory:  {} KB", self.free_memory / 1024)?;
        writeln!(f, "  Allocated Pages: {}", self.allocated_pages)?;
        writeln!(f, "  Free Pages: {}", self.free_pages)?;
        Ok(())
    }
}

pub struct MemoryMapper {
    stats: MemoryStats,
}

impl MemoryMapper {
    pub fn new() -> Self {
        MemoryMapper {
            stats: MemoryStats::new(),
        }
    }

    pub fn print_page_table(&self, page_table: &PageTable, level: usize) {
        println!("Page Table at Level {}:", level);
        for (i, entry) in page_table.iter().enumerate() {
            if !entry.is_unused() {
                println!("  Entry {}: {:?}", i, self.format_page_entry(entry));
            }
        }
    }

    pub fn visualize_memory_map(&self, start_addr: VirtAddr, size: usize) {
        println!("Memory Map Visualization:");
        println!("Virtual Address Range: {:#x} - {:#x}", start_addr.as_u64(),
                 start_addr.as_u64() + size as u64);

        const CHUNK_SIZE: usize = 4096;
        let chunks = size / CHUNK_SIZE;

        for i in 0..chunks {
            let addr = start_addr + (i * CHUNK_SIZE) as u64;
            if i % 16 == 0 {
                print!("\n{:#x}:", addr.as_u64());
            }
            print!(" {}", if self.is_page_present(addr) { "█" } else { "░" });
        }
        println!();
    }

    fn is_page_present(&self, addr: VirtAddr) -> bool {
        addr.as_u64() < 0x8000_0000_0000
    }

    fn format_page_entry(&self, entry: &PageTableEntry) -> String {
        let mut flags = Vec::new();

        if entry.flags().contains(PageTableFlags::PRESENT) {
            flags.push("PRESENT");
        }
        if entry.flags().contains(PageTableFlags::WRITABLE) {
            flags.push("WRITABLE");
        }
        if entry.flags().contains(PageTableFlags::USER_ACCESSIBLE) {
            flags.push("USER");
        }

        format!("Phys: {:#x}, Flags: {}", entry.addr().as_u64(), flags.join("|"))
    }

    pub fn update_stats(&mut self, boot_info: &bootloader::BootInfo) {
        let mut total_memory = 0;
        let mut used_memory = 0;
        let mut allocated_pages = 0;

        for region in boot_info.memory_map.iter() {
            total_memory += region.range.end_addr() - region.range.start_addr();
            if region.region_type != bootloader::bootinfo::MemoryRegionType::Usable {
                used_memory += region.range.end_addr() - region.range.start_addr();
                allocated_pages += (region.range.end_addr() - region.range.start_addr()) / 4096;
            }
        }

        self.stats.update(total_memory as usize, used_memory as usize, allocated_pages as usize);
    }

    pub fn print_stats(&self) {
        println!("{}", self.stats);
    }
}

use core::fmt;
