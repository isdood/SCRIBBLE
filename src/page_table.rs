// src/page_table.rs
use unstable_matter::unstable_vectrix::{UnstableVectrix, VirtAddr, PhysAddr};
use x86_64::structures::paging::{
    PageTable, PageTableFlags, PhysFrame, Size4KiB,
    FrameAllocator, Mapper, Page, RecursivePageTable,
};

pub struct VectorPageTable {
    table: UnstableVectrix<u64>,
    flags: PageTableFlags,
}

impl VectorPageTable {
    pub unsafe fn new(phys_offset: u64) -> Self {
        Self {
            table: UnstableVectrix::new(
                phys_offset as usize,
                512,  // Standard x86_64 page table size
                0
            ),
            flags: PageTableFlags::empty(),
        }
    }

    pub fn map_kernel_segment(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        mapper: &mut impl Mapper<Size4KiB>,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Result<(), &'static str> {
        // Check if the page is already mapped using vector operations
        let virt_addr = VirtAddr::new(page.start_address().as_u64());
        if self.is_mapped(virt_addr) {
            return Err("Page already mapped");
        }

        // Set up flags for kernel segment
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        self.flags = flags;

        // Attempt to map the page using vector operations
        unsafe {
            self.map_to_vector(
                virt_addr,
                PhysAddr::new(frame.start_address().as_u64()),
                               frame_allocator
            )?;
        }

        // Flush TLB
        unsafe {
            core::arch::asm!("invlpg [{}]", in(reg) page.start_address().as_u64());
        }

        Ok(())
    }

    unsafe fn map_to_vector(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Result<(), &'static str> {
        let table_idx = (virt.as_u64() >> 12) & 0x1FF;
        let entry = (phys.as_u64() & !0xFFF) | self.flags.bits() as u64;

        self.table.write(table_idx as usize, entry);
        Ok(())
    }

    fn is_mapped(&self, addr: VirtAddr) -> bool {
        let table_idx = (addr.as_u64() >> 12) & 0x1FF;
        let entry = self.table.read(table_idx as usize);
        entry & PageTableFlags::PRESENT.bits() as u64 != 0
    }
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
