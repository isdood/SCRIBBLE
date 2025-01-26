//! Crystal-optimized vector implementation

use super::{CollectionAlignment, CollectionCore, CrystalCollection};
use crate::align::Alignment;
use std::ptr::{self, NonNull};
use std::slice;

/// A crystal-optimized vector type
pub struct CrystalVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
    core: CollectionCore,
}

// Safety: CrystalVec is Send if T is Send
unsafe impl<T: Send> Send for CrystalVec<T> {}
// Safety: CrystalVec is Sync if T is Sync
unsafe impl<T: Sync> Sync for CrystalVec<T> {}

impl<T> CrystalVec<T> {
    /// Creates a new empty vector
    pub fn new() -> Self {
        Self::with_alignment(CollectionAlignment::Default)
    }

    /// Creates a new empty vector with specified alignment
    pub fn with_alignment(alignment: CollectionAlignment) -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            core: CollectionCore::new(alignment),
        }
    }

    /// Creates a new vector with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let mut vec = Self::new();
        vec.reserve(capacity);
        vec
    }

    /// Pushes an item onto the end of the vector
    pub fn push(&mut self, item: T) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), item);
        }
        self.len += 1;
    }

    /// Pops an item from the end of the vector
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    /// Returns a slice of the vector's contents
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }

    /// Returns a mutable slice of the vector's contents
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }

    /// Reserves space for at least `additional` more elements
    pub fn reserve(&mut self, additional: usize) {
        let new_cap = self.len.checked_add(additional).expect("Capacity overflow");
        if new_cap > self.cap {
            self.grow_to(new_cap);
        }
    }

    /// Grows the vector's capacity
    fn grow(&mut self) {
        let new_cap = if self.cap == 0 {
            4
        } else {
            self.cap * 2
        };
        self.grow_to(new_cap);
    }

    /// Grows the vector to the specified capacity
    fn grow_to(&mut self, new_cap: usize) {
        let new_size = new_cap * std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();

        unsafe {
            let new_ptr = self.core.alloc(new_size, align);
            if self.len > 0 {
                ptr::copy_nonoverlapping(
                    self.ptr.as_ptr() as *const u8,
                    new_ptr,
                    self.len * std::mem::size_of::<T>(),
                );
                if self.cap > 0 {
                    self.core.dealloc(
                        self.ptr.as_ptr() as *mut u8,
                        self.cap * std::mem::size_of::<T>(),
                        align,
                    );
                }
            }
            self.ptr = NonNull::new_unchecked(new_ptr as *mut T);
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for CrystalVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len));
                self.core.dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    self.cap * std::mem::size_of::<T>(),
                    std::mem::align_of::<T>(),
                );
            }
        }
    }
}

impl<T> CrystalCollection for CrystalVec<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len
    }

    fn alignment(&self) -> Alignment {
        Alignment::Crystal16
    }

    fn clear(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len));
        }
        self.len = 0;
    }
}
