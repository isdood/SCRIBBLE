//! Crystal-optimized collections
//!
//! Provides efficient collection types optimized for crystal-space operations.

pub mod vector;
pub mod map;
pub mod set;
pub mod deque;

pub use vector::CrystalVec;
pub use map::CrystalMap;
pub use set::CrystalSet;
pub use deque::CrystalDeque;

use crate::align::Alignment;
use std::alloc::{alloc, dealloc, Layout};

/// Trait for crystal-optimized collections
pub trait CrystalCollection {
    /// The type of items in the collection
    type Item;

    /// Returns the number of elements in the collection
    fn len(&self) -> usize;

    /// Returns true if the collection is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the alignment of the collection
    fn alignment(&self) -> Alignment;

    /// Clears all elements from the collection
    fn clear(&mut self);
}

/// Alignment strategy for collections
#[derive(Debug, Clone, Copy)]
pub enum CollectionAlignment {
    /// Default alignment based on item size
    Default,
    /// SIMD-optimized alignment
    Simd,
    /// Cache-line optimized alignment
    CacheLine,
    /// Custom alignment in bytes
    Custom(usize),
}

impl CollectionAlignment {
    /// Returns the actual alignment in bytes
    pub fn bytes(&self) -> usize {
        match self {
            Self::Default => std::mem::size_of::<usize>(),
            Self::Simd => 32, // AVX2 alignment
            Self::CacheLine => 64,
            Self::Custom(align) => *align,
        }
    }
}

/// Common functionality for crystal-optimized collections
pub(crate) struct CollectionCore {
    alignment: CollectionAlignment,
}

impl CollectionCore {
    /// Creates a new collection core with the specified alignment
    pub fn new(alignment: CollectionAlignment) -> Self {
        Self { alignment }
    }

    /// Allocates memory with the specified layout
    pub unsafe fn alloc(&self, size: usize, item_align: usize) -> *mut u8 {
        let align = self.alignment.bytes().max(item_align);
        let layout = Layout::from_size_align(size, align)
            .expect("Invalid layout");
        let ptr = alloc(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        ptr
    }

    /// Deallocates memory with the specified layout
    pub unsafe fn dealloc(&self, ptr: *mut u8, size: usize, item_align: usize) {
        let align = self.alignment.bytes().max(item_align);
        let layout = Layout::from_size_align(size, align)
            .expect("Invalid layout");
        dealloc(ptr, layout);
    }
}

// Re-export common traits
pub use std::iter::FromIterator;
pub use std::ops::{Index, IndexMut};
