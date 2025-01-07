use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
        PageSize
    },
    VirtAddr,
};
use linked_list_allocator::LockedHeap;
use crate::stats::SYSTEM_STATS;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[derive(Debug)]
pub struct HeapStats {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
}

pub fn get_heap_stats() -> HeapStats {
    HeapStats {
        total_size: HEAP_SIZE,
        used_size: estimate_used_size(),
        free_size: HEAP_SIZE - estimate_used_size(),
    }
}

// This is a simple estimation - you might want to make it more accurate
fn estimate_used_size() -> usize {
    let stats = SYSTEM_STATS.lock();
    // Rough estimation based on timer ticks and keyboard interrupts
    // Each interrupt might use some memory
    let interrupt_memory = (stats.get_timer_ticks() + stats.get_keyboard_interrupts()) as usize * 64;
    interrupt_memory.min(HEAP_SIZE)
}

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    assert!(heap_start.is_aligned(Size4KiB::SIZE));

    let page_range = {
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START as *mut u8, HEAP_SIZE);
    }

    // Update system stats with initial heap info
    let heap_stats = get_heap_stats();
    crate::debug_info!(
        "Heap initialized: Total: {}KB, Used: {}KB, Free: {}KB",
        heap_stats.total_size / 1024,
        heap_stats.used_size / 1024,
        heap_stats.free_size / 1024
    );

    Ok(())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    let stats = get_heap_stats();
    panic!(
        "Allocation error: {:?}\nHeap stats: {:?}",
        layout, stats
    );
}
