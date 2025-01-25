//! Crystal-Space Memory Allocator

use std::alloc::{GlobalAlloc, Layout};
use super::Alignment;

/// A specialized allocator for crystal-space vector computing
pub struct CrystalAllocator {
    alignment: Alignment,
}

impl CrystalAllocator {
    /// Creates a new crystal allocator with specified alignment
    #[inline]
    pub const fn new(alignment: Alignment) -> Self {
        Self { alignment }
    }

    /// Returns the current alignment requirement
    #[inline]
    pub const fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Creates an aligned layout for crystal-space allocation
    #[inline]
    pub fn crystal_layout(&self, size: usize) -> Layout {
        Layout::from_size_align(size, self.alignment.as_bytes())
            .expect("Invalid crystal-space layout")
    }
}

unsafe impl GlobalAlloc for CrystalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Ensure crystal-space alignment
        let aligned_layout = Layout::from_size_align_unchecked(
            layout.size(),
            layout.align().max(self.alignment.as_bytes())
        );

        std::alloc::System.alloc(aligned_layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let aligned_layout = Layout::from_size_align_unchecked(
            layout.size(),
            layout.align().max(self.alignment.as_bytes())
        );

        std::alloc::System.dealloc(ptr, aligned_layout)
    }
}
