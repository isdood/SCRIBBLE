use x86_64::{
    structures::paging::{
        FrameAllocator, PageTable, PhysFrame, Size4KiB, OffsetPageTable
    },
    PhysAddr, VirtAddr,
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use crate::splat::{self, SplatLevel};
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::format;
use x86_64::structures::paging::mapper::MapToError;

// Memory Management Constants
pub const MAX_FRAMES: usize = 1024 * 1024;  // 4GB limit (with 4KB pages)
pub const PAGE_SIZE: usize = 4096;          // 4KB pages
const MEMORY_WARNING_THRESHOLD: f32 = 90.0;  // 90% usage warning
const MEMORY_CRITICAL_THRESHOLD: f32 = 95.0; // 95% usage critical
const ALLOCATION_LOG_INTERVAL: usize = 1000; // Log every 1000 allocations

// Atomic Memory Counters
static ALLOCATED_FRAMES: AtomicUsize = AtomicUsize::new(0);
static TOTAL_USABLE_FRAMES: AtomicUsize = AtomicUsize::new(0);
static FAILED_ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static ALLOCATION_HIGH_WATERMARK: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total_frames: usize,
    pub allocated_frames: usize,
    pub free_frames: usize,
    pub usage_percentage: f32,
    pub failed_allocations: usize,
    pub high_watermark: usize,
    timestamp: u64,  // UTC timestamp from crate::rtc
}

impl MemoryStats {
    pub fn new(total: usize, allocated: usize) -> Self {
        let free = total.saturating_sub(allocated);
        let usage = if total > 0 {
            (allocated as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        MemoryStats {
            total_frames: total,
            allocated_frames: allocated,
            free_frames: free,
            usage_percentage: usage,
            failed_allocations: FAILED_ALLOCATIONS.load(Ordering::Relaxed),
            high_watermark: ALLOCATION_HIGH_WATERMARK.load(Ordering::Relaxed),
            timestamp: crate::rtc::DateTime::now().to_string().parse().unwrap_or(0),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.usage_percentage < MEMORY_WARNING_THRESHOLD &&
        self.failed_allocations == 0
    }

    pub fn log_status(&self) {
        let level = if self.usage_percentage > MEMORY_CRITICAL_THRESHOLD {
            SplatLevel::Critical
        } else if self.usage_percentage > MEMORY_WARNING_THRESHOLD {
            SplatLevel::Warning
        } else {
            SplatLevel::BitsNBytes
        };

        splat::log(level, &format!("Memory Status: {}% used", self.usage_percentage));
    }
}

/// Initialize a new OffsetPageTable.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    splat::log(SplatLevel::BitsNBytes, "Initializing memory management system");
    let level_4_table = active_level_4_table(physical_memory_offset);

    splat::log(
        SplatLevel::BitsNBytes,
        &format!("L4 page table at physical address: {:#x}",
                 x86_64::registers::control::Cr3::read().0.start_address().as_u64())
    );

    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, flags) = Cr3::read();
    splat::log(
        SplatLevel::BitsNBytes,
        &format!("CR3 Flags: {:?}", flags)
    );

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        let total_frames = Self::count_usable_frames(memory_map);
        TOTAL_USABLE_FRAMES.store(total_frames, Ordering::SeqCst);

        splat::log(
            SplatLevel::BitsNBytes,
            &format!(
                "Frame Allocator Initialization:\n\
└─ Total Usable Memory: {} MB\n\
└─ Frame Count: {}\n\
└─ Page Size: {} KB",
total_frames * PAGE_SIZE / 1024 / 1024,
total_frames,
PAGE_SIZE / 1024
            )
        );

        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn count_usable_frames(memory_map: &MemoryMap) -> usize {
        memory_map
        .iter()
        .filter(|r| r.region_type == MemoryRegionType::Usable)
        .map(|r| {
            let start = r.range.start_addr();
            let end = r.range.end_addr();
            (end - start) as usize / PAGE_SIZE
        })
        .sum::<usize>()
        .min(MAX_FRAMES)
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.memory_map
        .iter()
        .filter(|r| r.region_type == MemoryRegionType::Usable)
        .flat_map(|r| (r.range.start_addr()..r.range.end_addr()).step_by(PAGE_SIZE))
        .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
        .take(MAX_FRAMES)
    }

    pub fn get_stats(&self) -> MemoryStats {
        let total = TOTAL_USABLE_FRAMES.load(Ordering::Relaxed);
        let allocated = ALLOCATED_FRAMES.load(Ordering::Relaxed);
        MemoryStats::new(total, allocated)
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        if let Some(frame) = frame {
            self.next = self.next.saturating_add(1);
            let allocated = ALLOCATED_FRAMES.fetch_add(1, Ordering::Relaxed) + 1;

            // Update high watermark if needed
            let current_high = ALLOCATION_HIGH_WATERMARK.load(Ordering::Relaxed);
            if allocated > current_high {
                ALLOCATION_HIGH_WATERMARK.store(allocated, Ordering::Relaxed);
            }

            // Log memory status at intervals
            if allocated % ALLOCATION_LOG_INTERVAL == 0 {
                self.get_stats().log_status();
            }

            Some(frame)
        } else {
            FAILED_ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            splat::log(
                SplatLevel::Critical,
                &format!(
                    "Memory allocation failed!\n\
└─ Attempted frame: {}\n\
└─ Total failures: {}",
self.next,
FAILED_ALLOCATIONS.load(Ordering::Relaxed)
                )
            );
            None
        }
    }
}

// Public interface
pub fn log_memory_status(allocator: &BootInfoFrameAllocator) {
    allocator.get_stats().log_status();
}

pub fn check_memory_health(allocator: &BootInfoFrameAllocator) -> bool {
    allocator.get_stats().is_healthy()
}

pub fn init_heap(heap_start: *mut u8, heap_size: usize) -> Result<(), MapToError<Size4KiB>> {
    use x86_64::structures::paging::{Page, PageTableFlags};

    splat::log(
        SplatLevel::BitsNBytes,
        &format!(
            "Initializing heap:\n\
└─ Start Address: {:#x}\n\
└─ Size: {} KB",
heap_start as u64,
heap_size / 1024
        )
    );

    // Get the page range for the heap
    let heap_start_addr = VirtAddr::new(heap_start as u64);
    let heap_end_addr = heap_start_addr + heap_size - 1u64;
    let heap_start_page = Page::containing_address(heap_start_addr);
    let heap_end_page = Page::containing_address(heap_end_addr);

    // Create a temporary frame allocator for heap initialization
    let mut temp_allocator = unsafe {
        BootInfoFrameAllocator::init(&bootloader::bootinfo::MemoryMap::new())
    };

    // Get a mutable mapper reference
    let mut mapper = unsafe {
        let phys_mem_offset = VirtAddr::new(bootloader::bootinfo::physical_memory_offset());
        OffsetPageTable::new(active_level_4_table(phys_mem_offset), phys_mem_offset)
    };

    // Map all pages in the heap range
    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        let frame = temp_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, &mut temp_allocator)?.flush();
        }
    }

    // Initialize the actual heap allocator
    unsafe {
        crate::allocator::ALLOCATOR.lock().init(heap_start, heap_size);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_stats() {
        let stats = MemoryStats::new(100, 50);
        assert_eq!(stats.usage_percentage, 50.0);
        assert_eq!(stats.free_frames, 50);
        assert!(stats.is_healthy());
    }
}
