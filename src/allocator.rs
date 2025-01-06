use x86_64::structures::paging::{mapper::MapToError, Page, PageTable, PhysFrame, Size4KiB, OffsetPageTable, PageTableFlags, Mapper, FrameAllocator};
use linked_list_allocator::LockedHeap;
use x86_64::VirtAddr;
use crate::println; // Add println macro

// Define the heap start and size
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

// Create a static instance of the LockedHeap
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = heap_start + HEAP_SIZE - 1u64;

    let heap_start_page = Page::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_end);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    unsafe {
        ALLOCATOR.lock().init(heap_start.as_u64() as usize, HEAP_SIZE);
    }

    Ok(())
}
