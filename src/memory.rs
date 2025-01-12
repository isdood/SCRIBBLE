// src/memory.rs
use unstable_matter::unstable_vectrix::{UnstableVectrix, VirtAddr, PhysAddr};
use crate::page_table::VectorPageTable;

pub struct MemoryManager {
    page_table: VectorPageTable,
    heap_start: VirtAddr,
    heap_size: usize,
}

impl MemoryManager {
    pub unsafe fn new(phys_offset: u64, heap_start: u64, heap_size: usize) -> Self {
        Self {
            page_table: VectorPageTable::new(phys_offset),
            heap_start: VirtAddr::new(heap_start),
            heap_size,
        }
    }

    pub fn init_heap(&mut self) -> Result<(), &'static str> {
        // Initialize heap with vector operations
        let heap_vec = unsafe {
            UnstableVectrix::new(
                self.heap_start.as_u64() as usize,
                                 self.heap_size,
                                 0
            )
        };

        // Zero out the heap area
        for i in 0..self.heap_size {
            heap_vec.write(i, 0);
        }

        Ok(())
    }
}
