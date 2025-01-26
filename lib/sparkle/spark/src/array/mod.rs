//! Crystal-space optimized array implementations
//!
//! Provides SIMD-accelerated array operations with proper crystal alignment.

mod layout;
mod ops;
mod iter;

pub use layout::CrystalArray;
pub use ops::ArrayOps;
use crate::align::Alignment;
use crate::shard::arch::{Shard, CpuFeature};
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::marker::PhantomData;

/// A crystal-space aligned array with SIMD optimization support
#[derive(Debug)]
pub struct CrystalArray<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
    alignment: Alignment,
    _marker: PhantomData<T>,
}

impl<T> CrystalArray<T> {
    /// Creates a new empty array with the specified alignment
    pub fn new(alignment: Alignment) -> Self {
        Self::with_capacity(0, alignment)
    }

    /// Creates a new array with the given capacity and alignment
    pub fn with_capacity(capacity: usize, alignment: Alignment) -> Self {
        let layout = Layout::array::<T>(capacity.max(1))
            .unwrap()
            .align_to(alignment as usize)
            .unwrap();

        // Safety: layout is properly aligned and non-zero
        let ptr = unsafe {
            NonNull::new(alloc(layout) as *mut T)
                .expect("Failed to allocate memory")
        };

        CrystalArray {
            ptr,
            len: 0,
            capacity: capacity,
            alignment,
            _marker: PhantomData,
        }
    }

    /// Returns the length of the array
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the array is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the array
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the alignment of the array
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Pushes an element to the end of the array
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        // Safety: we just ensured there's enough capacity
        unsafe {
            std::ptr::write(self.ptr.as_ptr().add(self.len), value);
        }
        self.len += 1;
    }

    /// Pops an element from the end of the array
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // Safety: we just checked that len > 0
            Some(unsafe {
                std::ptr::read(self.ptr.as_ptr().add(self.len))
            })
        }
    }

    /// Returns a reference to the element at the given index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            // Safety: we just checked that index is in bounds
            Some(unsafe {
                &*self.ptr.as_ptr().add(index)
            })
        }
    }

    /// Returns a mutable reference to the element at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            // Safety: we just checked that index is in bounds
            Some(unsafe {
                &mut *self.ptr.as_ptr().add(index)
            })
        }
    }

    fn grow(&mut self) {
        let new_capacity = self.capacity.saturating_mul(2).max(1);
        let layout = Layout::array::<T>(new_capacity)
            .unwrap()
            .align_to(self.alignment as usize)
            .unwrap();

        // Safety: layout is properly aligned and non-zero
        let new_ptr = unsafe {
            NonNull::new(alloc(layout) as *mut T)
                .expect("Failed to allocate memory")
        };

        // Safety: both old and new pointers are properly aligned
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.ptr.as_ptr(),
                new_ptr.as_ptr(),
                self.len,
            );
        }

        let old_layout = Layout::array::<T>(self.capacity.max(1))
            .unwrap()
            .align_to(self.alignment as usize)
            .unwrap();

        // Safety: ptr and layout match the original allocation
        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, old_layout);
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T> Drop for CrystalArray<T> {
    fn drop(&mut self) {
        // Drop all elements
        while let Some(_) = self.pop() {}

        let layout = Layout::array::<T>(self.capacity.max(1))
            .unwrap()
            .align_to(self.alignment as usize)
            .unwrap();

        // Safety: ptr and layout match the original allocation
        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

// Implement Send and Sync if T is Send
unsafe impl<T: Send> Send for CrystalArray<T> {}
unsafe impl<T: Sync> Sync for CrystalArray<T> {}
