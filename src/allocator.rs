// src/allocator.rs
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
use core::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(crate::HEAP_START as u64);
        let heap_end = heap_start + crate::HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(crate::HEAP_START as *mut u8, crate::HEAP_SIZE);
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct HeapStats {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
    pub largest_free_region: usize,
}

impl HeapStats {
    pub fn new() -> Self {
        let used_size = unsafe {
            let layout = Layout::from_size_align(1, 1).unwrap();
            let ptr = ALLOCATOR.alloc(layout);
            ALLOCATOR.dealloc(ptr, layout);
            // Estimate used size by checking allocator state
            0 // TODO: Implement proper size tracking
        };

        HeapStats {
            total_size: crate::HEAP_SIZE,
            used_size,
            free_size: crate::HEAP_SIZE.saturating_sub(used_size),
            largest_free_region: 0, // TODO: Implement
        }
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
