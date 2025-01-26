#!/bin/bash

# Spark Clone Fix Script (Part 4)
# Author: isdood
# Created: 2025-01-25 18:51:33 UTC
# Repository: isdood/scribble
# Description: Fixes clone module dead code warnings

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

fix_clone_module() {
    cd forge/std || exit 1

    # Update clone module implementation to use strategy field
    cat > src/clone/mod.rs << 'EOL'
//! Crystal-optimized cloning system
//!
//! Provides efficient cloning strategies optimized for crystal-space operations.

use crate::align::Alignment;
use std::any::TypeId;
use std::marker::PhantomData;

mod error;
pub use error::CloneError;

/// Strategy for crystal-optimized cloning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloneStrategy {
    /// Copy data directly (for Copy types)
    Memcopy,
    /// Deep clone with crystal alignment
    DeepClone,
    /// Reference counting with crystal alignment
    RefCounted,
    /// Copy-on-write with crystal alignment
    CopyOnWrite,
}

/// Marker for types that cannot be cloned
#[derive(Debug)]
pub struct NoClone(PhantomData<*const ()>);

/// Trait for crystal-optimized cloning
pub trait CrystalClone: Sized {
    /// Returns the optimal clone strategy for this type
    fn clone_strategy() -> CloneStrategy;

    /// Creates a crystal-optimized clone
    fn crystal_clone(&self) -> Result<Self, CloneError>;

    /// Creates a crystal-optimized clone with a specific strategy
    fn crystal_clone_with(&self, strategy: CloneStrategy) -> Result<Self, CloneError>;
}

/// Helper for implementing crystal-optimized cloning
#[derive(Debug)]
pub struct CloneHelper {
    strategy: CloneStrategy,
    alignment: Alignment,
}

impl CloneHelper {
    /// Creates a new clone helper with optimal settings
    pub fn new() -> Self {
        Self::with_strategy(CloneStrategy::DeepClone)
    }

    /// Creates a new clone helper with a specific strategy
    pub fn with_strategy(strategy: CloneStrategy) -> Self {
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

        Self { strategy, alignment }
    }

    /// Returns the current clone strategy
    pub fn strategy(&self) -> CloneStrategy {
        self.strategy
    }

    /// Sets a new clone strategy
    pub fn set_strategy(&mut self, strategy: CloneStrategy) {
        self.strategy = strategy;
    }

    /// Returns true if the type should use memcopy
    pub fn should_memcopy<T: 'static>(&self) -> bool {
        if self.strategy == CloneStrategy::Memcopy {
            matches!(TypeId::of::<T>(),
                id if id == TypeId::of::<u8>() ||
                    id == TypeId::of::<u16>() ||
                    id == TypeId::of::<u32>() ||
                    id == TypeId::of::<u64>() ||
                    id == TypeId::of::<i8>() ||
                    id == TypeId::of::<i16>() ||
                    id == TypeId::of::<i32>() ||
                    id == TypeId::of::<i64>() ||
                    id == TypeId::of::<f32>() ||
                    id == TypeId::of::<f64>() ||
                    id == TypeId::of::<bool>() ||
                    id == TypeId::of::<char>()
            )
        } else {
            false
        }
    }

    /// Performs an optimized clone operation
    pub unsafe fn clone_bytes<T>(&self, src: *const T, count: usize) -> *mut T {
        let size = std::mem::size_of::<T>() * count;
        let align = std::mem::align_of::<T>().max(self.alignment.as_bytes());

        let layout = std::alloc::Layout::from_size_align(size, align)
            .expect("Invalid layout");

        let dst = std::alloc::alloc(layout) as *mut T;
        if dst.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        match self.strategy {
            CloneStrategy::Memcopy => {
                std::ptr::copy_nonoverlapping(src, dst, count);
            }
            _ => {
                // Use SIMD operations when available for non-memcopy strategies
                if self.alignment.as_bytes() >= 32 && size >= 32 {
                    self.clone_bytes_simd(src as *const u8, dst as *mut u8, size);
                } else {
                    std::ptr::copy_nonoverlapping(src, dst, count);
                }
            }
        }

        dst
    }

    /// Uses SIMD operations for cloning when available
    #[cfg(target_feature = "avx2")]
    unsafe fn clone_bytes_simd(&self, src: *const u8, dst: *mut u8, size: usize) {
        use std::arch::x86_64::{__m256i, _mm256_load_si256, _mm256_store_si256};

        let chunks = size / 32;
        let src_chunks = src as *const __m256i;
        let dst_chunks = dst as *mut __m256i;

        for i in 0..chunks {
            _mm256_store_si256(
                dst_chunks.add(i),
                _mm256_load_si256(src_chunks.add(i))
            );
        }

        // Copy remaining bytes
        let remainder = size % 32;
        if remainder > 0 {
            std::ptr::copy_nonoverlapping(
                src.add(size - remainder),
                dst.add(size - remainder),
                remainder
            );
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    unsafe fn clone_bytes_simd(&self, src: *const u8, dst: *mut u8, size: usize) {
        // Fallback to regular copy
        std::ptr::copy_nonoverlapping(src, dst, size);
    }

    /// Returns the current alignment
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
}

// Remove the blanket impl for Copy types and replace with specific impls
macro_rules! impl_crystal_clone_primitive {
    ($($t:ty),*) => {
        $(
            impl CrystalClone for $t {
                fn clone_strategy() -> CloneStrategy {
                    CloneStrategy::Memcopy
                }

                fn crystal_clone(&self) -> Result<Self, CloneError> {
                    Ok(*self)
                }

                fn crystal_clone_with(&self, strategy: CloneStrategy) -> Result<Self, CloneError> {
                    match strategy {
                        CloneStrategy::Memcopy => self.crystal_clone(),
                        _ => Err(CloneError::InvalidStrategy),
                    }
                }
            }
        )*
    }
}

impl_crystal_clone_primitive!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    bool, char
);

// Implement for Box without conflicting with primitive types
impl<T: CrystalClone + ?Sized> CrystalClone for Box<T> {
    fn clone_strategy() -> CloneStrategy {
        T::clone_strategy()
    }

    fn crystal_clone(&self) -> Result<Self, CloneError> {
        Ok(Box::new((**self).crystal_clone()?))
    }

    fn crystal_clone_with(&self, strategy: CloneStrategy) -> Result<Self, CloneError> {
        Ok(Box::new((**self).crystal_clone_with(strategy)?))
    }
}

// Implement for Vec without conflicting with primitive types
impl<T: CrystalClone> CrystalClone for Vec<T> {
    fn clone_strategy() -> CloneStrategy {
        T::clone_strategy()
    }

    fn crystal_clone(&self) -> Result<Self, CloneError> {
        let mut result = Vec::with_capacity(self.len());
        for item in self {
            result.push(item.crystal_clone()?);
        }
        Ok(result)
    }

    fn crystal_clone_with(&self, strategy: CloneStrategy) -> Result<Self, CloneError> {
        let mut result = Vec::with_capacity(self.len());
        for item in self {
            result.push(item.crystal_clone_with(strategy)?);
        }
        Ok(result)
    }
}
EOL

    print_purple "✓ Fixed clone module implementation"
}

main() {
    print_purple "🔮 Fixing Spark Clone Module..."
    fix_clone_module
    print_purple "✨ Clone module fixes applied!

Fixed Issues:
- Added strategy field usage
- Added strategy accessor methods
- Improved clone_bytes implementation
- Added strategy-based cloning
- Fixed dead code warnings
- Improved error handling
- Added better documentation

Run 'cargo test' to verify the fixes!"
}

main
