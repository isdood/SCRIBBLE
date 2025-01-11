use x86_64::{
    structures::paging::{
        PageTable, PageTableFlags, PhysFrame, Size4KiB,
        FrameAllocator, Mapper, Page, RecursivePageTable,
    },
    VirtAddr, PhysAddr,
};

fn map_kernel_segment(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    // Check if the page is already mapped
    if mapper.translate_page(page).is_ok() {
        // Either return error or unmap first
        return Err("Page already mapped");
    }

    // Set up flags for kernel segment
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    // Attempt to map the page
    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
        .map_err(|_| "Failed to map page")?
        .flush();
    }

    Ok(())
}
