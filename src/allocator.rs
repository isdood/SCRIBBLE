use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
        PageSize, FrameError
    },
    VirtAddr,
};
use linked_list_allocator::LockedHeap;
use crate::splat::{self, SplatLevel};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;
use alloc::format;

// Heap Configuration
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;  // 100 KB
pub const PAGE_SIZE: usize = 4096;        // 4 KB

// Memory Thresholds
const LOW_MEMORY_THRESHOLD: usize = HEAP_SIZE / 10;     // 10% of heap
const CRITICAL_MEMORY_THRESHOLD: usize = HEAP_SIZE / 20; // 5% of heap
const FRAGMENTATION_THRESHOLD: f64 = 0.3;               // 30% fragmentation warning

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// Memory Statistics
#[derive(Debug)]
struct MemoryMetrics {
    allocation_count: AtomicUsize,
    deallocation_count: AtomicUsize,
    total_allocated: AtomicUsize,
    peak_usage: AtomicUsize,
    failed_allocations: AtomicUsize,
}

lazy_static::lazy_static! {
    static ref MEMORY_METRICS: Mutex<MemoryMetrics> = Mutex::new(MemoryMetrics {
        allocation_count: AtomicUsize::new(0),
                                                                 deallocation_count: AtomicUsize::new(0),
                                                                 total_allocated: AtomicUsize::new(0),
                                                                 peak_usage: AtomicUsize::new(0),
                                                                 failed_allocations: AtomicUsize::new(0),
    });
}

#[derive(Debug, Clone, Copy)]
pub struct HeapStats {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
    pub allocation_count: usize,
    pub deallocation_count: usize,
    pub largest_contiguous: usize,
    pub fragmentation_ratio: f64,
    pub peak_usage: usize,
    pub failed_allocations: usize,
}

impl HeapStats {
    fn new() -> Self {
        let metrics = MEMORY_METRICS.lock();
        let (used, largest_free) = unsafe { ALLOCATOR.lock().size() };
        let free_size = HEAP_SIZE.saturating_sub(used);

        let fragmentation = if free_size > 0 {
            1.0 - (largest_free as f64 / free_size as f64)
        } else {
            0.0
        };

        HeapStats {
            total_size: HEAP_SIZE,
            used_size: used,
            free_size,
            allocation_count: metrics.allocation_count.load(Ordering::Relaxed),
            deallocation_count: metrics.deallocation_count.load(Ordering::Relaxed),
            largest_contiguous: largest_free,
            fragmentation_ratio: fragmentation,
            peak_usage: metrics.peak_usage.load(Ordering::Relaxed),
            failed_allocations: metrics.failed_allocations.load(Ordering::Relaxed),
        }
    }

    fn log_status(&self) {
        let level = self.determine_log_level();
        splat::log(
            level,
            &format!(
                "Heap Status:\n\
└─ Memory Usage: {}/{} KB ({:.1}%)\n\
└─ Largest Free Block: {} KB\n\
└─ Fragmentation: {:.1}%\n\
└─ Allocations: {} (Failed: {})\n\
└─ Peak Usage: {} KB",
self.used_size / 1024,
self.total_size / 1024,
(self.used_size as f64 / self.total_size as f64) * 100.0,
                     self.largest_contiguous / 1024,
                     self.fragmentation_ratio * 100.0,
                     self.allocation_count,
                     self.failed_allocations,
                     self.peak_usage / 1024
            )
        );
    }

    fn determine_log_level(&self) -> SplatLevel {
        if self.free_size < CRITICAL_MEMORY_THRESHOLD {
            SplatLevel::Critical
        } else if self.free_size < LOW_MEMORY_THRESHOLD ||
            self.fragmentation_ratio > FRAGMENTATION_THRESHOLD {
                SplatLevel::Warning
            } else {
                SplatLevel::BitsNBytes
            }
    }

    pub fn is_healthy(&self) -> bool {
        self.free_size > LOW_MEMORY_THRESHOLD &&
        self.fragmentation_ratio < FRAGMENTATION_THRESHOLD &&
        self.failed_allocations == 0
    }
}

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    splat::log(SplatLevel::BitsNBytes, "Initializing heap memory management");

    let heap_start = VirtAddr::new(HEAP_START as u64);
    if !heap_start.is_aligned(Size4KiB::SIZE) {
        splat::log(SplatLevel::Critical, "Heap start address is not page-aligned");
        return Err(MapToError::FrameAllocationFailed);
    }

    let pages = {
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // Map all heap pages
    for page in pages {
        let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    // Initialize the heap allocator
    unsafe {
        ALLOCATOR.lock().init(HEAP_START as *mut u8, HEAP_SIZE);
    }

    // Log initial heap state
    let stats = HeapStats::new();
    stats.log_status();

    Ok(())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    let stats = HeapStats::new();
    MEMORY_METRICS.lock().failed_allocations.fetch_add(1, Ordering::Relaxed);

    splat::log(
        SplatLevel::Critical,
        &format!(
            "ALLOCATION ERROR:\n\
└─ Requested: {} bytes (aligned to {})\n\
└─ Current Heap State:\n\
│  └─ Free: {} KB\n\
│  └─ Largest Block: {} KB\n\
│  └─ Fragmentation: {:.1}%",
layout.size(),
                 layout.align(),
                 stats.free_size / 1024,
                 stats.largest_contiguous / 1024,
                 stats.fragmentation_ratio * 100.0
        )
    );

    panic!("Memory allocation failed");
}

// Memory tracking functions
pub fn track_allocation(size: usize) {
    let mut metrics = MEMORY_METRICS.lock();
    metrics.allocation_count.fetch_add(1, Ordering::Relaxed);
    metrics.total_allocated.fetch_add(size, Ordering::Relaxed);

    let current_usage = metrics.total_allocated.load(Ordering::Relaxed);
    let peak = metrics.peak_usage.load(Ordering::Relaxed);
    if current_usage > peak {
        metrics.peak_usage.store(current_usage, Ordering::Relaxed);
    }
}

pub fn track_deallocation(size: usize) {
    let metrics = MEMORY_METRICS.lock();
    metrics.deallocation_count.fetch_add(1, Ordering::Relaxed);
    metrics.total_allocated.fetch_sub(size, Ordering::Relaxed);
}

pub fn get_heap_stats() -> HeapStats {
    HeapStats::new()
}

pub fn log_heap_status() {
    get_heap_stats().log_status();
}

pub fn check_heap_health() -> bool {
    get_heap_stats().is_healthy()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_stats() {
        let stats = HeapStats::new();
        assert!(stats.total_size == HEAP_SIZE);
        assert!(stats.fragmentation_ratio >= 0.0 && stats.fragmentation_ratio <= 1.0);
    }
}
