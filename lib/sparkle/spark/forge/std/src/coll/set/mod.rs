//! Crystal-optimized set implementation

use super::{CollectionAlignment, CollectionCore, CrystalCollection};
use crate::align::Alignment;
use std::collections::HashSet;

/// A crystal-optimized hash set
pub struct CrystalSet<T> {
    inner: HashSet<T>,
    core: CollectionCore,
}

impl<T> CrystalSet<T> {
    /// Creates a new empty set
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
            core: CollectionCore::new(CollectionAlignment::Default),
        }
    }

    /// Creates a new set with specified alignment
    pub fn with_alignment(alignment: CollectionAlignment) -> Self {
        Self {
            inner: HashSet::new(),
            core: CollectionCore::new(alignment),
        }
    }

    /// Inserts a value into the set
    pub fn insert(&mut self, value: T) -> bool
    where
        T: std::hash::Hash + Eq,
    {
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
        self.inner.insert(value)
    }

    /// Returns true if the set contains the value
    pub fn contains(&self, value: &T) -> bool
    where
        T: std::hash::Hash + Eq,
    {
        self.inner.contains(value)
    }
}

impl<T> CrystalCollection for CrystalSet<T> {
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
