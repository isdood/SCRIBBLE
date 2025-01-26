//! Crystal-optimized interior mutability
//!
//! Similar to Cell but with crystal-space optimizations and better memory model.

use crate::align::Alignment;
use std::cell::UnsafeCell;
use std::fmt;
use std::marker::PhantomData;

/// Marker for thread-safe potion types
pub struct Stable;
/// Marker for thread-local potion types
pub struct Volatile;

/// Trait for potion safety markers
pub trait PotionKind {
    /// Returns true if this kind is thread-safe
    fn is_stable() -> bool;
}

impl PotionKind for Stable {
    fn is_stable() -> bool { true }
}

impl PotionKind for Volatile {
    fn is_stable() -> bool { false }
}

/// A crystal-space optimized container with interior mutability
pub struct Potion<T, K: PotionKind = Volatile> {
    value: UnsafeCell<T>,
    alignment: Alignment,
    _marker: PhantomData<K>,
}

/// Thread-safe variant of Potion
pub type StablePotion<T> = Potion<T, Stable>;
/// Thread-local variant of Potion
pub type VolatilePotion<T> = Potion<T, Volatile>;

impl<T, K: PotionKind> Potion<T, K> {
    /// Creates a new potion with optimal alignment
    pub fn new(value: T) -> Self {
        Self::with_alignment(value, Self::optimal_alignment())
    }

    /// Creates a new potion with specified alignment
    pub fn with_alignment(value: T, alignment: Alignment) -> Self {
        assert!(
            (std::mem::align_of::<T>() <= alignment.as_bytes()),
            "Alignment must be at least as large as type's alignment"
        );

        Self {
            value: UnsafeCell::new(value),
            alignment,
            _marker: PhantomData,
        }
    }

    /// Returns the potion's alignment
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Returns true if the potion is optimally aligned for SIMD
    pub fn is_simd_aligned(&self) -> bool {
        (self.value.get() as usize) % self.alignment.as_bytes() == 0
    }

    /// Returns optimal alignment for the current architecture
    fn optimal_alignment() -> Alignment {
        let shard = crate::shard::arch::Shard::new();
        match shard.architecture() {
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
        }
    }
}

impl<T: Copy, K: PotionKind> Potion<T, K> {
    /// Gets the current value
    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }

    /// Sets a new value
    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value; }
    }

    /// Updates the current value using a function
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.set(f(self.get()));
    }

    /// Replaces the current value, returning the old value
    pub fn replace(&self, value: T) -> T {
        let old = self.get();
        self.set(value);
        old
    }

    /// Swaps the values of two potions
    pub fn swap(&self, other: &Self) {
        let tmp = self.get();
        self.set(other.get());
        other.set(tmp);
    }
}

impl<T: Copy + Default, K: PotionKind> Default for Potion<T, K> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Copy + fmt::Debug, K: PotionKind> fmt::Debug for Potion<T, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Potion")
            .field("value", &self.get())
            .field("alignment", &self.alignment)
            .field("kind", &K::is_stable())
            .finish()
    }
}

// Safety: Stable potions are safe to share between threads
unsafe impl<T: Copy + Send> Send for Potion<T, Stable> {}
unsafe impl<T: Copy + Send + Sync> Sync for Potion<T, Stable> {}
