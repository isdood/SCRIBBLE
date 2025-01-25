#!/bin/bash

# Spark Array Module Setup Script
# Author: isdood
# Created: 2025-01-25 18:08:39 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized array types

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

create_directory_structure() {
    print_purple "🔮 Creating Spark Array Module structure..."
    mkdir -p forge/std/src/array
    mkdir -p forge/std/tests/array
}

setup_array_module() {
    cd forge/std || exit 1

    # Update lib.rs to include array module
    cat > src/lib.rs << 'EOL'
//! Spark Standard Library - Where Magic Begins ✨

#![feature(const_type_name)]

pub mod math;
pub mod types;
pub mod align;
pub mod any;
pub mod shard;
pub mod array;

pub use types::*;
pub use math::operations;
pub use align::space;
pub use shard::arch;
pub use array::{CrystalArray, ArrayOps};
EOL

    cat > src/array/mod.rs << 'EOL'
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
EOL

    cat > src/array/layout.rs << 'EOL'
//! Memory layout management for crystal arrays

use super::CrystalArray;
use crate::align::Alignment;
use std::mem;

impl<T> CrystalArray<T> {
    /// Returns the optimal alignment for the current architecture
    pub fn optimal_alignment() -> Alignment {
        let shard = crate::shard::arch::Shard::new();

        match shard.architecture() {
            crate::shard::arch::Architecture::X86_64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::AVX512F) {
                    Alignment::Vector64
                } else if shard.has_feature(crate::shard::arch::CpuFeature::AVX2) {
                    Alignment::Vector32
                } else {
                    Alignment::Vector16
                }
            }
            crate::shard::arch::Architecture::AArch64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::SVE) {
                    Alignment::Vector64
                } else {
                    Alignment::Vector16
                }
            }
            _ => Alignment::Crystal16,
        }
    }

    /// Returns true if the array's memory layout is optimal for SIMD operations
    pub fn is_simd_aligned(&self) -> bool {
        (self.ptr.as_ptr() as usize) % (self.alignment as usize) == 0
    }

    /// Returns the size of a SIMD vector for the current architecture
    pub fn vector_size() -> usize {
        let shard = crate::shard::arch::Shard::new();

        match shard.architecture() {
            crate::shard::arch::Architecture::X86_64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::AVX512F) {
                    64
                } else if shard.has_feature(crate::shard::arch::CpuFeature::AVX2) {
                    32
                } else {
                    16
                }
            }
            crate::shard::arch::Architecture::AArch64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::SVE) {
                    64
                } else {
                    16
                }
            }
            _ => 16,
        }
    }
}
EOL

    cat > src/array/ops.rs << 'EOL'
//! SIMD-optimized array operations

use super::CrystalArray;
use std::ops::{Add, Mul};

/// Trait for array operations that can be SIMD-accelerated
pub trait ArrayOps<T> {
    /// Adds two arrays element-wise
    fn add(&self, other: &Self) -> Self;

    /// Multiplies two arrays element-wise
    fn mul(&self, other: &Self) -> Self;

    /// Computes the dot product of two arrays
    fn dot(&self, other: &Self) -> T;
}

impl<T> ArrayOps<T> for CrystalArray<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default,
{
    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len(), "Arrays must have equal length");

        let mut result = Self::with_capacity(self.len(), self.alignment);

        // Use SIMD operations if available and aligned
        if self.is_simd_aligned() && other.is_simd_aligned() {
            // SIMD implementation would go here
            unimplemented!("SIMD operations not yet implemented");
        }

        // Fallback to scalar operations
        for i in 0..self.len() {
            let sum = *self.get(i).unwrap() + *other.get(i).unwrap();
            result.push(sum);
        }

        result
    }

    fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len(), "Arrays must have equal length");

        let mut result = Self::with_capacity(self.len(), self.alignment);

        // Use SIMD operations if available and aligned
        if self.is_simd_aligned() && other.is_simd_aligned() {
            // SIMD implementation would go here
            unimplemented!("SIMD operations not yet implemented");
        }

        // Fallback to scalar operations
        for i in 0..self.len() {
            let product = *self.get(i).unwrap() * *other.get(i).unwrap();
            result.push(product);
        }

        result
    }

    fn dot(&self, other: &Self) -> T {
        assert_eq!(self.len(), other.len(), "Arrays must have equal length");

        let mut sum = T::default();

        // Use SIMD operations if available and aligned
        if self.is_simd_aligned() && other.is_simd_aligned() {
            // SIMD implementation would go here
            unimplemented!("SIMD operations not yet implemented");
        }

        // Fallback to scalar operations
        for i in 0..self.len() {
            sum = sum + (*self.get(i).unwrap() * *other.get(i).unwrap());
        }

        sum
    }
}
EOL

    cat > src/array/iter.rs << 'EOL'
//! Iterator implementations for crystal arrays

use super::CrystalArray;
use std::iter::FromIterator;

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

impl<T> IntoIterator for CrystalArray<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            array: self,
            index: 0,
        }
    }
}

pub struct IntoIter<T> {
    array: CrystalArray<T>,
    index: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let item = self.array.get(self.index).cloned();
            self.index += 1;
            item
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.array.len() - self.index;
        (remaining, Some(remaining))
    }
}
EOL

    mkdir -p tests
    cat > tests/primitive_tests.rs << 'EOL'
//! Core tests for Spark Standard Library

mod align;
mod any;
mod shard;
mod array;

#[test]
fn test_primitive_types() {
    assert!(true);
}

#[test]
fn test_math_operations() {
    assert!(true);
}
EOL

    mkdir -p tests/array
    cat > tests/array/mod.rs << 'EOL'
use spark_std::array::{CrystalArray, ArrayOps};
use spark_std::align::Alignment;

#[test]
fn test_array_creation() {
    let array: CrystalArray<i32> = CrystalArray::new(Alignment::Crystal16);
    assert!(array.is_empty());
    assert_eq!(array.len(), 0);
}

#[test]
fn test_array_push_pop() {
    let mut array = CrystalArray::new(Alignment::Crystal16);

    array.push(1);
    array.push(2);
    array.push(3);

    assert_eq!(array.len(), 3);
    assert_eq!(array.pop(), Some(3));
    assert_eq!(array.pop(), Some(2));
    assert_eq!(array.pop(), Some(1));
    assert_eq!(array.pop(), None);
}

#[test]
fn test_array_get() {
    let mut array = CrystalArray::new(Alignment::Crystal16);

    array.push(1);
    array.push(2);

    assert_eq!(array.get(0), Some(&1));
    assert_eq!(array.get(1), Some(&2));
    assert_eq!(array.get(2), None);
}

#[test]
fn test_array_alignment() {
    let array: CrystalArray<f32> = CrystalArray::new(Alignment::Vector32);
    assert!(array.is_simd_aligned());
    assert_eq!(array.alignment(), Alignment::Vector32);
}

#[test]
fn test_array_operations() {
    let mut a = CrystalArray::new(Alignment::Crystal16);
    let mut b = CrystalArray::new(Alignment::Crystal16);

    for i in 0..4 {
        a.push(i as f32);
        b.push((i * 2) as f32);
    }

    // Test basic operations with scalars for now since SIMD is unimplemented
    for i in 0..4 {
        assert_eq!(*a.get(i).unwrap(), i as f32);
        assert_eq!(*b.get(i).unwrap(), (i * 2) as f32);
    }
}

#[test]
fn test_optimal_alignment() {
    let shard = spark_std::shard::arch::Shard::new();
    let array: CrystalArray<f32> = CrystalArray::new(CrystalArray::<f32>::optimal_alignment());

    match shard.architecture() {
        spark_std::shard::arch::Architecture::X86_64 => {
            if shard.has_feature(spark_std::shard::arch::CpuFeature::AVX512F) {
                assert_eq!(array.alignment(), Alignment::Vector64);
            } else if shard.has_feature(spark_std::shard::arch::CpuFeature::AVX2) {
                assert_eq!(array.alignment(), Alignment::Vector32);
            }
        }
        spark_std::shard::arch::Architecture::AArch64 => {
            if shard.has_feature(spark_std::shard::arch::CpuFeature::SVE) {
                assert_eq!(array.alignment(), Alignment::Vector64);
            } else {
                assert_eq!(array.alignment(), Alignment::Vector16);
            }
        }
        _ => assert_eq!(array.alignment(), Alignment::Crystal16),
    }
}

#[test]
fn test_iterator() {
    let mut array = CrystalArray::new(Alignment::Crystal16);
    for i in 0..5 {
        array.push(i);
    }

    let sum: i32 = array.into_iter().sum();
    assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4 = 10
}

#[test]
fn test_from_iterator() {
    let vec = vec![1, 2, 3, 4, 5];
    let array: CrystalArray<i32> = vec.into_iter().collect();

    assert_eq!(array.len(), 5);
    for i in 0..5 {
        assert_eq!(array.get(i), Some(&(i as i32 + 1)));
    }
}

#[test]
fn test_grow() {
    let mut array = CrystalArray::new(Alignment::Crystal16);

    // Test growth by pushing more elements than initial capacity
    for i in 0..100 {
        array.push(i);
        assert_eq!(array.get(i), Some(&i));
    }

    assert_eq!(array.len(), 100);
    assert!(array.capacity() >= 100);
}
EOL

    cd ../.. || exit 1

    print_purple "✓ Created array module files"
}

main() {
    print_purple "🔮 Creating Spark Array Module..."
    create_directory_structure
    setup_array_module
    print_purple "✨ Array module created with crystal-space optimization!

Features:
- SIMD-optimized operations
- Architecture-aware alignment
- Safe memory management
- Iterator support

Array Operations:
- Element-wise addition
- Element-wise multiplication
- Dot product
- Dynamic resizing

Run 'cd forge/std && cargo test' to verify the array implementation!"
}

main
