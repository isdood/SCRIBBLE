//! Memory alignment optimizations for SIMD operations
//! Created: 2025-01-21 23:43:42 UTC
//! Author: isdood

use std::alloc::Layout;
use crate::superpurple::core::LatticeSymmetry;

/// SIMD alignment requirements
#[derive(Debug, Clone, Copy)]
pub struct SIMDAlignment {
    /// Required alignment in bytes
    pub alignment: usize,
    /// Preferred vector width
    pub vector_width: usize,
    /// Lattice symmetry type
    pub symmetry: LatticeSymmetry,
}

impl SIMDAlignment {
    /// Create new alignment requirements
    pub fn new(symmetry: LatticeSymmetry) -> Self {
        let (alignment, vector_width) = match symmetry {
            LatticeSymmetry::Cubic => (32, 8),      // AVX-512
            LatticeSymmetry::Tetragonal => (16, 4), // AVX2
            LatticeSymmetry::Hexagonal => (32, 6),  // Custom
            LatticeSymmetry::Custom(_) => (16, 4),  // Default
        };

        Self {
            alignment,
            vector_width,
            symmetry,
        }
    }

    /// Create memory layout with proper alignment
    pub fn create_layout(&self, size: usize) -> Option<Layout> {
        Layout::from_size_align(size, self.alignment).ok()
    }

    /// Check if pointer is properly aligned
    pub fn is_aligned(&self, ptr: *const u8) -> bool {
        (ptr as usize) % self.alignment == 0
    }

    /// Calculate padding needed for alignment
    pub fn padding_needed(&self, ptr: *const u8) -> usize {
        let addr = ptr as usize;
        (self.alignment - (addr % self.alignment)) % self.alignment
    }
}

/// Alignment utilities for SIMD operations
pub struct AlignmentUtils;

impl AlignmentUtils {
    /// Align pointer to required boundary
    pub fn align_pointer(ptr: *mut u8, alignment: usize) -> *mut u8 {
        let addr = ptr as usize;
        let offset = (alignment - (addr % alignment)) % alignment;
        unsafe { ptr.add(offset) }
    }

    /// Calculate aligned size
    pub fn align_size(size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }

    /// Check if size is aligned
    pub fn is_size_aligned(size: usize, alignment: usize) -> bool {
        size % alignment == 0
    }
}
