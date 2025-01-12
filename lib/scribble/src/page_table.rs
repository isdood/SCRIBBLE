use x86_64::structures::paging::{PageTable, PageTableFlags, PhysFrame, Size4KiB};
use x86_64::{VirtAddr, PhysAddr};
use unstable_matter::unstable_vectrix::UnstableVectrix;

pub struct VectorTableEntry {
    vector: UnstableVectrix<u64>,
}

impl VectorTableEntry {
    pub unsafe fn new(addr: u64) -> Self {
        Self {
            vector: UnstableVectrix::new(addr as usize, 512, 0)
        }
    }
}

pub fn map_kernel_segment(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    if mapper.translate_page(page).is_ok() {
        return Err("Page already mapped");
    }

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
        .map_err(|_| "Failed to map page")?
        .flush();
    }
    Ok(())
}

// Keep the original function for compatibility
fn map_kernel_segment(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    if mapper.translate_page(page).is_ok() {
        return Err("Page already mapped");
    }

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
        .map_err(|_| "Failed to map page")?
        .flush();
    }

    Ok(())
}
