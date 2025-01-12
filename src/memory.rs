// src/memory.rs
use unstable_matter::SpaceTime;
use spinUP::boot_params::BootParams;
use unstable_matter::VirtAddr;

pub struct MemoryRegion {
    pub start: usize,
    pub size: usize,
}

pub struct MemoryMap {
    pub regions: &'static [MemoryRegion],
}

pub struct OffsetPageTable<'a> {
    level_4_table: &'a mut PageTable,
}

pub struct BootInfoFrameAllocator {
    memory_regions: SpaceTime<MemoryRegion>,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_regions: SpaceTime::new(
                memory_map as *const _ as usize,
                memory_map.len(),
                                                 0
            ),
        }
    }
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = {
        let table_addr = physical_memory_offset.as_u64();
        let table = SpaceTime::new(table_addr as usize, 512, 0);
        &mut *table.as_mut_ptr()
    };
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}
