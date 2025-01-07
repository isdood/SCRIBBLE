use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, PageSize
    },
    VirtAddr,
};
use linked_list_allocator::LockedHeap;

// Adjust heap start to be page-aligned
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 1024 * 1024; // Increase to 1 MiB for more space

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    // Ensure heap start is page-aligned
    let heap_start = VirtAddr::new(HEAP_START as u64);
    assert!(heap_start.is_aligned(Size4KiB::SIZE));

    // Calculate page range
    let page_range = {
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // Map all pages in the range
    for page in page_range {
        let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::NO_EXECUTE; // Add NO_EXECUTE for safety
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?
            .flush();
        }
    }

    // Initialize the heap allocator
    unsafe {
        ALLOCATOR.lock().init(
            HEAP_START,
            HEAP_SIZE
        );
    }

    Ok(())
}

/// Handle allocation errors
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

// Add helper function to get heap usage statistics
pub fn get_heap_usage() -> Option<(usize, usize)> {
    let stats = unsafe { ALLOCATOR.lock().stats() };
    Some((stats.used, stats.total))
}
