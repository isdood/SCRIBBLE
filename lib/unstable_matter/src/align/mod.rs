/// Align: SpaceTime-aware Memory Management System
/// Last Updated: 2025-01-12 21:34:52 UTC
/// Author: isdood
///
/// This module provides memory management
/// functionality built on top of SpaceTime,
/// offering memory allocation, deallocation, and
/// region management aligned to a 3D mesh structure.

// lib/unstable_matter/src/align/mod.rs

use crate::{VectorSpace, MeshCell, CellState};

#[derive(Debug)]
pub struct AlignError {
    pub kind: AlignErrorKind,
}

#[derive(Debug)]
pub enum AlignErrorKind {
    OutOfMemory,
    InvalidAlignment,
    AddressInUse,
    InvalidSize,
    MeshUnavailable,
}

pub struct Align {
    vector_space: VectorSpace,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryLayout {
    size: usize,
    align: usize,
}

impl MemoryLayout {
    pub const fn new(size: usize, align: usize) -> Option<Self> {
        if !align.is_power_of_two() || align == 0 || size == 0 {
            return None;
        }
        Some(Self { size, align })
    }

    pub const fn size(&self) -> usize { self.size }
    pub const fn align(&self) -> usize { self.align }
}

impl Align {
    pub fn new(heap_start: usize, heap_size: usize) -> Self {
        let mut space = VectorSpace::new(heap_start, heap_size);
        unsafe { space.init_mesh() };

        Self {
            vector_space: space,
        }
    }

    pub fn allocate(&mut self, layout: MemoryLayout) -> Result<*mut u8, AlignError> {
        let size = layout.size();
        let align = layout.align();

        // Calculate required cells
        let cells_needed = (size + self.vector_space.config.cell_size - 1)
        / self.vector_space.config.cell_size;

        // Find contiguous free cells
        let start_cell = unsafe { self.find_contiguous_cells(cells_needed, align) }
        .ok_or(AlignError { kind: AlignErrorKind::OutOfMemory })?;

        // Mark cells as allocated
        unsafe {
            for i in 0..cells_needed {
                let cell_idx = start_cell + i;
                let mut cell = self.vector_space.mesh.read_at(cell_idx);
                cell.state = CellState::Allocated;
                self.vector_space.mesh.write_at(cell_idx, cell);
            }
        }

        // Calculate aligned address
        let base_addr = unsafe { self.vector_space.mesh.read_at(start_cell).addr };
        let aligned_addr = (base_addr + align - 1) & !(align - 1);

        Ok(aligned_addr as *mut u8)
    }

    pub fn deallocate(&mut self, ptr: *mut u8, layout: MemoryLayout) {
        let addr = ptr as usize;

        // Find the cell containing this address
        if let Some(cell_idx) = self.vector_space.addr_to_cell(addr) {
            let size = layout.size();
            let cells_to_free = (size + self.vector_space.config.cell_size - 1)
            / self.vector_space.config.cell_size;

            // Mark cells as free
            unsafe {
                for i in 0..cells_to_free {
                    let idx = cell_idx + i;
                    let mut cell = self.vector_space.mesh.read_at(idx);
                    cell.state = CellState::Free;
                    self.vector_space.mesh.write_at(idx, cell);
                }
            }
        }
    }

    unsafe fn find_contiguous_cells(&self, count: usize, align: usize) -> Option<usize> {
        let mut consecutive = 0;
        let mut start = 0;

        for i in 0..self.vector_space.mesh.size() {
            let cell = self.vector_space.mesh.read_at(i);

            if cell.state == CellState::Free {
                if consecutive == 0 {
                    // Check alignment of first cell
                    let aligned_addr = (cell.addr + align - 1) & !(align - 1);
                    if aligned_addr - cell.addr > self.vector_space.config.cell_size {
                        continue;
                    }
                    start = i;
                }
                consecutive += 1;
                if consecutive >= count {
                    return Some(start);
                }
            } else {
                consecutive = 0;
            }
        }
        None
    }
}

unsafe impl Send for Align {}
unsafe impl Sync for Align {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_aligned_allocation() {
        let mut align = Align::new(0x1000, 0x10000);

        // Test aligned allocation
        let layout = MemoryLayout::new(256, 256).unwrap();
        let ptr = align.allocate(layout).unwrap();
        assert_eq!(ptr as usize & (layout.align() - 1), 0);

        // Test deallocation
        align.deallocate(ptr, layout);

        // Verify cells are freed
        let cell_idx = align.vector_space.addr_to_cell(ptr as usize).unwrap();
        unsafe {
            let cell = align.vector_space.mesh.read_at(cell_idx);
            assert_eq!(cell.state, CellState::Free);
        }
    }
}
