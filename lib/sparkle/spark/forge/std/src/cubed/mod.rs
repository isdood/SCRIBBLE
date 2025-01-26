//! Crystal-optimized heap allocation
//!
//! Similar to Box but with crystal-space optimizations and alignment guarantees.

use crate::align::Alignment;
use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

mod error;
pub use error::CubeError;

/// A crystal-space optimized heap allocation
pub struct Cubed<T: ?Sized> {
    ptr: NonNull<T>,
    layout: Layout,
    alignment: Alignment,
}

/// Allocation strategy for Cubed types
#[derive(Debug, Clone, Copy)]
pub struct CubedAlloc {
    alignment: Alignment,
}

impl CubedAlloc {
    /// Creates a new allocator with the specified alignment
    pub fn new(alignment: Alignment) -> Self {
        Self { alignment }
    }

    /// Returns optimal alignment for the current architecture
    pub fn optimal() -> Self {
        let shard = crate::shard::arch::Shard::new();
        let alignment = match shard.architecture() {
            crate::shard::arch::Architecture::X86_64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::AVX512F) {
                    Alignment::Vector64
                } else if shard.has_feature(crate::shard::arch::CpuFeature::AVX2) {
                    Alignment::Vector32
                } else {
                    Alignment::Crystal16
                }
            }
            _ => Alignment::Crystal16,
        };
        Self::new(alignment)
    }

    /// Allocates memory with the specified layout
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let aligned_layout = layout
            .align_to(self.alignment.as_bytes())
            .expect("Failed to align layout");
        System.alloc(aligned_layout)
    }

    /// Deallocates memory with the specified layout
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let aligned_layout = layout
            .align_to(self.alignment.as_bytes())
            .expect("Failed to align layout");
        System.dealloc(ptr, aligned_layout)
    }
}

impl<T> Cubed<T> {
    /// Creates a new crystal-space optimized heap allocation
    pub fn new(value: T) -> Self {
        Self::with_allocator(value, CubedAlloc::optimal())
    }

    /// Creates a new heap allocation with the specified allocator
    pub fn with_allocator(value: T, alloc: CubedAlloc) -> Self {
        let layout = Layout::new::<T>();

        // Safety: layout is properly aligned and non-zero
        let ptr = unsafe {
            let ptr = alloc.alloc(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            std::ptr::write(ptr.cast::<T>(), value);
            NonNull::new_unchecked(ptr.cast::<T>())
        };

        Self {
            ptr,
            layout,
            alignment: alloc.alignment,
        }
    }

    /// Returns the alignment of this allocation
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Returns true if this allocation is optimally aligned for SIMD
    pub fn is_simd_aligned(&self) -> bool {
        (self.ptr.as_ptr() as usize) % self.alignment.as_bytes() == 0
    }

    /// Converts into the contained value
    pub fn into_inner(self) -> T {
        let value = unsafe { std::ptr::read(self.ptr.as_ptr()) };
        std::mem::forget(self);
        value
    }
}

impl<T: ?Sized> Cubed<T> {
    /// Returns a reference to the underlying value
    pub fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }

    /// Returns a mutable reference to the underlying value
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> Drop for Cubed<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.ptr.as_ptr());
            let alloc = CubedAlloc::new(self.alignment);
            alloc.dealloc(self.ptr.cast().as_ptr(), self.layout);
        }
    }
}

impl<T: ?Sized> Deref for Cubed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> DerefMut for Cubed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Cubed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cubed")
            .field("value", &&**self)
            .field("alignment", &self.alignment)
            .finish()
    }
}

impl<T> From<T> for Cubed<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

unsafe impl<T: ?Sized + Send> Send for Cubed<T> {}
unsafe impl<T: ?Sized + Sync> Sync for Cubed<T> {}
