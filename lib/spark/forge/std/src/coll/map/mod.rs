//! Crystal-optimized map implementation

use super::{CollectionAlignment, CollectionCore, CrystalCollection};
use crate::align::Alignment;
use std::collections::HashMap;

/// A crystal-optimized hash map
pub struct CrystalMap<K, V> {
    inner: HashMap<K, V>,
    core: CollectionCore,
}

impl<K, V> CrystalMap<K, V> {
    /// Creates a new empty map
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            core: CollectionCore::new(CollectionAlignment::Default),
        }
    }

    /// Creates a new map with specified alignment
    pub fn with_alignment(alignment: CollectionAlignment) -> Self {
        Self {
            inner: HashMap::new(),
            core: CollectionCore::new(alignment),
        }
    }

    /// Inserts a key-value pair into the map
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: std::hash::Hash + Eq,
    {
        let alignment = self.core.alignment.bytes();
        if alignment > 16 {
            // Ensure data is properly aligned for SIMD operations
            unsafe {
                let ptr = &value as *const V as *const u8;
                if (ptr as usize) % alignment != 0 {
                    let mut aligned = Vec::with_capacity(std::mem::size_of::<V>());
                    aligned.set_len(std::mem::size_of::<V>());
                    std::ptr::copy_nonoverlapping(ptr, aligned.as_mut_ptr(), std::mem::size_of::<V>());
                }
            }
        }
        self.inner.insert(key, value)
    }

    /// Gets a reference to the value corresponding to the key
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: std::hash::Hash + Eq,
    {
        self.inner.get(key)
    }
}

impl<K, V> CrystalCollection for CrystalMap<K, V> {
    type Item = (K, V);

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
