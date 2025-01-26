//! Crystal-space optimized array implementations
//!
//! Provides SIMD-accelerated array operations with proper crystal alignment.

use crate::align::Alignment;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::fmt;

/// SIMD-optimized array operations trait
pub trait ArrayOps {
    /// Returns the optimal alignment for the current architecture
    fn optimal_alignment() -> Alignment;
}

#[derive(Clone)]
pub struct CrystalArray<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
    alignment: Alignment,
    _marker: PhantomData<T>,
}

impl<T> fmt::Debug for CrystalArray<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CrystalArray")
            .field("len", &self.len)
            .field("capacity", &self.capacity)
            .field("alignment", &self.alignment)
            .field("data", &self.as_slice())
            .finish()
    }
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
            .align_to(alignment.as_bytes())
            .unwrap();

        // Safety: layout is properly aligned and non-zero
        let ptr = unsafe {
            NonNull::new(alloc(layout) as *mut T)
                .expect("Failed to allocate memory")
        };

        CrystalArray {
            ptr,
            len: 0,
            capacity,
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

    /// Returns true if the array's memory layout is optimal for SIMD operations
    pub fn is_simd_aligned(&self) -> bool {
        (self.ptr.as_ptr() as usize) % self.alignment.as_bytes() == 0
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

    /// Returns an iterator over the array
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the array
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Returns a reference to the underlying slice
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }

    /// Returns a mutable reference to the underlying slice
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }

    /// Extends the array from a slice
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        for item in other {
            self.push(item.clone());
        }
    }

    fn grow(&mut self) {
        let new_capacity = self.capacity.saturating_mul(2).max(1);
        let layout = Layout::array::<T>(new_capacity)
            .unwrap()
            .align_to(self.alignment.as_bytes())
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
            .align_to(self.alignment.as_bytes())
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
        while let Some(_) = self.pop() {}

        let layout = Layout::array::<T>(self.capacity.max(1))
            .unwrap()
            .align_to(self.alignment.as_bytes())
            .unwrap();

        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

impl<T> ArrayOps for CrystalArray<T> {
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
            crate::shard::arch::Architecture::AArch64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::SVE) {
                    Alignment::Vector64
                } else {
                    Alignment::Crystal16
                }
            }
            _ => Alignment::Crystal16,
        }
    }
}

impl<T> FromIterator<T> for CrystalArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, _) = iter.size_hint();

        let mut array = Self::with_capacity(
            min,
            Self::optimal_alignment()
        );

        for item in iter {
            array.push(item);
        }

        array
    }
}

impl<T> AsRef<[T]> for CrystalArray<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for CrystalArray<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> std::ops::Deref for CrystalArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> std::ops::DerefMut for CrystalArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

unsafe impl<T: Send> Send for CrystalArray<T> {}
unsafe impl<T: Sync> Sync for CrystalArray<T> {}
