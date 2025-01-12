use unstable_matter::unstable_vectrix::{UnstableVectrix, VirtAddr, PhysAddr};
use spinUP::boot_params::BootParams;

pub struct BootInfoFrameAllocator {
    memory_map: UnstableVectrix<u8>,
    memory_map_entries: u32,
}

impl BootInfoFrameAllocator {
    pub fn init_from_boot_params(boot_params: &BootParams, phys_offset: u64) -> Self {
        let memory_map = unsafe {
            UnstableVectrix::from_phys(
                PhysAddr::new(boot_params.memory_map_addr as u64),
                                       (boot_params.memory_map_entries * core::mem::size_of::<u32>() as u32) as usize,
                                       phys_offset
            )
        };

        BootInfoFrameAllocator {
            memory_map,
            memory_map_entries: boot_params.memory_map_entries,
        }
    }
}

pub unsafe fn active_level_4_table(physical_memory_offset: u64) -> &'static mut UnstableVectrix<u64> {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys_addr = PhysAddr::new(level_4_table_frame.start_address().as_u64());
    let virt_addr = phys_addr.to_virt(physical_memory_offset);

    UnstableVectrix::from_virt(virt_addr, 512)
}
