//! Crystal-optimized double-ended queue implementation

use super::{CollectionAlignment, CollectionCore, CrystalCollection};
use crate::align::Alignment;
use std::collections::VecDeque;

/// A crystal-optimized double-ended queue
pub struct CrystalDeque<T> {
    inner: VecDeque<T>,
    core: CollectionCore,
}

impl<T> CrystalDeque<T> {
    /// Creates a new empty deque
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
            core: CollectionCore::new(CollectionAlignment::Default),
        }
    }

    /// Creates a new deque with specified alignment
    pub fn with_alignment(alignment: CollectionAlignment) -> Self {
        Self {
            inner: VecDeque::new(),
            core: CollectionCore::new(alignment),
        }
    }

    /// Pushes a value to the front of the deque
    pub fn push_front(&mut self, value: T) {
        let alignment = self.core.alignment.bytes();
        if alignment > 16 {
            // Ensure data is properly aligned for SIMD operations
            unsafe {
                let ptr = &value as *const T as *const u8;
                if (ptr as usize) % alignment != 0 {
                    let mut aligned = Vec::with_capacity(std::mem::size_of::<T>());
                    aligned.set_len(std::mem::size_of::<T>());
                    std::ptr::copy_nonoverlapping(ptr, aligned.as_mut_ptr(), std::mem::size_of::<T>());
                }
            }
        }
        self.inner.push_front(value);
    }

    /// Pushes a value to the back of the deque
    pub fn push_back(&mut self, value: T) {
        let alignment = self.core.alignment.bytes();
        if alignment > 16 {
            // Ensure data is properly aligned for SIMD operations
            unsafe {
                let ptr = &value as *const T as *const u8;
                if (ptr as usize) % alignment != 0 {
                    let mut aligned = Vec::with_capacity(std::mem::size_of::<T>());
                    aligned.set_len(std::mem::size_of::<T>());
                    std::ptr::copy_nonoverlapping(ptr, aligned.as_mut_ptr(), std::mem::size_of::<T>());
                }
            }
        }
        self.inner.push_back(value);
    }

    /// Pops a value from the front of the deque
    pub fn pop_front(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    /// Pops a value from the back of the deque
    pub fn pop_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    }
}

impl<T> CrystalCollection for CrystalDeque<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn alignment(&self) -> Alignment {
        Alignment::from_bytes(self.core.alignment.bytes())
    }

    fn clear(&mut self) {
        self.inner.clear();
    }
}
