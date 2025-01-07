// At the top of src/allocator.rs
use core::sync::atomic::{AtomicUsize, Ordering};
use core::fmt;
use linked_list_allocator::LockedHeap;
use x86_64::structures::paging::{
    mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
};
use x86_64::VirtAddr;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

lazy_static::lazy_static! {
    static ref MEMORY_METRICS: spin::Mutex<MemoryMetrics> = spin::Mutex::new(MemoryMetrics::new());
}

#[derive(Debug, Clone)]
pub struct HeapStats {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
    pub fragmentation_ratio: f32,
    pub largest_free_region: usize,
}

struct MemoryMetrics {
    failed_allocations: AtomicUsize,
}

impl MemoryMetrics {
    const fn new() -> Self {
        MemoryMetrics {
            failed_allocations: AtomicUsize::new(0),
        }
    }
}

impl HeapStats {
    pub fn new() -> Self {
        let used_size = ALLOCATOR.lock().used_size();
        let free_size = crate::HEAP_SIZE.saturating_sub(used_size);

        HeapStats {
            total_size: crate::HEAP_SIZE,
            used_size,
            free_size,
            fragmentation_ratio: 0.0, // You'll need to implement this
            largest_free_region: free_size,
        }
    }
}
