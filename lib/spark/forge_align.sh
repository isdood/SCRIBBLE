#!/bin/bash

# Spark Alignment Module Setup Script
# Author: isdood
# Created: 2025-01-25 17:42:37 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-space vector computing allocation system

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

create_directory_structure() {
    print_purple "🔮 Creating Spark Alignment Module structure..."
    mkdir -p forge/std/src/align
    mkdir -p forge/std/tests/align
}

setup_align_module() {
    # Add align module to lib.rs
    if ! grep -q "pub mod align;" forge/std/src/lib.rs; then
        sed -i '/pub mod types;/a pub mod align;' forge/std/src/lib.rs
        sed -i '/pub use types/a pub use align::space;' forge/std/src/lib.rs
    fi

    # Create align module files
    cat > forge/std/src/align/mod.rs << 'EOL'
//! Crystal-Space Vector Computing Allocation System
//!
//! This module provides specialized allocation strategies for crystal-space
//! vector computing, optimizing memory layouts for parallel computation.

mod allocator;
mod crystal;
mod vector;

pub mod space {
    pub use super::allocator::CrystalAllocator;
    pub use super::crystal::{CrystalSpace, CrystalAxis};
    pub use super::vector::{VectorSpace, Alignment};
}

/// Memory alignment requirements for crystal-space computing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// 16-byte alignment for basic crystal operations
    Crystal16,
    /// 32-byte alignment for advanced vector operations
    Vector32,
    /// 64-byte alignment for parallel crystal-space operations
    Parallel64,
}

impl Alignment {
    #[inline]
    pub fn as_bytes(&self) -> usize {
        match self {
            Alignment::Crystal16 => 16,
            Alignment::Vector32 => 32,
            Alignment::Parallel64 => 64,
        }
    }
}
EOL

    cat > forge/std/src/align/allocator.rs << 'EOL'
//! Crystal-Space Memory Allocator

use std::alloc::{GlobalAlloc, Layout};
use super::Alignment;

/// A specialized allocator for crystal-space vector computing
pub struct CrystalAllocator {
    alignment: Alignment,
}

impl CrystalAllocator {
    /// Creates a new crystal allocator with specified alignment
    #[inline]
    pub const fn new(alignment: Alignment) -> Self {
        Self { alignment }
    }

    /// Returns the current alignment requirement
    #[inline]
    pub const fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Creates an aligned layout for crystal-space allocation
    #[inline]
    pub fn crystal_layout(&self, size: usize) -> Layout {
        Layout::from_size_align(size, self.alignment.as_bytes())
            .expect("Invalid crystal-space layout")
    }
}

unsafe impl GlobalAlloc for CrystalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Ensure crystal-space alignment
        let aligned_layout = Layout::from_size_align_unchecked(
            layout.size(),
            layout.align().max(self.alignment.as_bytes())
        );

        std::alloc::System.alloc(aligned_layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let aligned_layout = Layout::from_size_align_unchecked(
            layout.size(),
            layout.align().max(self.alignment.as_bytes())
        );

        std::alloc::System.dealloc(ptr, aligned_layout)
    }
}
EOL

    cat > forge/std/src/align/crystal.rs << 'EOL'
//! Crystal Space Definitions and Operations

use std::num::NonZeroUsize;
use super::Alignment;

/// Represents a dimension in crystal space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrystalAxis {
    size: NonZeroUsize,
    alignment: Alignment,
}

impl CrystalAxis {
    /// Creates a new crystal axis with specified size and alignment
    #[inline]
    pub fn new(size: NonZeroUsize, alignment: Alignment) -> Self {
        Self { size, alignment }
    }

    /// Returns the size of this axis
    #[inline]
    pub const fn size(&self) -> NonZeroUsize {
        self.size
    }

    /// Returns the alignment requirement for this axis
    #[inline]
    pub const fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Calculates the aligned size for this axis
    #[inline]
    pub fn aligned_size(&self) -> usize {
        let align = self.alignment.as_bytes();
        (self.size.get() + align - 1) & !(align - 1)
    }
}

/// Represents a multi-dimensional crystal space
#[derive(Debug)]
pub struct CrystalSpace {
    axes: Vec<CrystalAxis>,
    total_size: usize,
}

impl CrystalSpace {
    /// Creates a new crystal space with given axes
    pub fn new(axes: Vec<CrystalAxis>) -> Self {
        let total_size = axes.iter()
            .map(|axis| axis.aligned_size())
            .product();

        Self { axes, total_size }
    }

    /// Returns the total size of the crystal space
    #[inline]
    pub fn size(&self) -> usize {
        self.total_size
    }

    /// Returns a slice of the crystal axes
    #[inline]
    pub fn axes(&self) -> &[CrystalAxis] {
        &self.axes
    }
}
EOL

    cat > forge/std/src/align/vector.rs << 'EOL'
//! Vector Space Operations for Crystal Computing

use std::num::NonZeroUsize;
use super::{Alignment, crystal::CrystalAxis};

/// A vector space for crystal computing
#[derive(Debug)]
pub struct VectorSpace {
    dimensions: Vec<NonZeroUsize>,
    alignment: Alignment,
}

impl VectorSpace {
    /// Creates a new vector space with given dimensions and alignment
    pub fn new(dimensions: Vec<NonZeroUsize>, alignment: Alignment) -> Self {
        Self { dimensions, alignment }
    }

    /// Converts this vector space to crystal axes
    pub fn to_crystal_axes(&self) -> Vec<CrystalAxis> {
        self.dimensions
            .iter()
            .map(|&size| CrystalAxis::new(size, self.alignment))
            .collect()
    }

    /// Returns the total size of the vector space
    pub fn size(&self) -> usize {
        self.dimensions
            .iter()
            .map(|d| d.get())
            .product()
    }

    /// Returns the alignment requirement
    #[inline]
    pub const fn alignment(&self) -> Alignment {
        self.alignment
    }
}
EOL

    # Create tests
    cat > forge/std/src/align/mod.rs << 'EOL'
//! Crystal-Space Vector Computing Allocation System
//!
//! This module provides specialized allocation strategies for crystal-space
//! vector computing, optimizing memory layouts for parallel computation.

mod allocator;
mod crystal;
mod vector;

pub mod space {
    pub use super::allocator::CrystalAllocator;
    pub use super::crystal::{CrystalSpace, CrystalAxis};
    pub use super::vector::VectorSpace;
    pub use super::Alignment;  // Import Alignment from parent module
}

/// Memory alignment requirements for crystal-space computing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// 16-byte alignment for basic crystal operations
    Crystal16,
    /// 32-byte alignment for advanced vector operations
    Vector32,
    /// 64-byte alignment for parallel crystal-space operations
    Parallel64,
}

impl Alignment {
    #[inline]
    pub fn as_bytes(&self) -> usize {
        match self {
            Alignment::Crystal16 => 16,
            Alignment::Vector32 => 32,
            Alignment::Parallel64 => 64,
        }
    }
}
EOL

    cat > forge/std/tests/align/mod.rs << 'EOL'
use spark_std::align::space::{
    CrystalAllocator,
    CrystalSpace,
    CrystalAxis,
    VectorSpace,
};
use spark_std::align::Alignment;
use std::num::NonZeroUsize;

#[test]
fn test_crystal_allocator() {
    let allocator = CrystalAllocator::new(Alignment::Crystal16);
    assert_eq!(allocator.alignment(), Alignment::Crystal16);

    let layout = allocator.crystal_layout(100);
    assert_eq!(layout.align(), 16);
}

#[test]
fn test_vector_space() {
    let dims: Vec<NonZeroUsize> = vec![1, 2, 3]
        .into_iter()
        .map(|n| NonZeroUsize::new(n).unwrap())
        .collect();

    let space = VectorSpace::new(dims, Alignment::Vector32);
    assert_eq!(space.size(), 6); // 1 * 2 * 3
    assert_eq!(space.alignment(), Alignment::Vector32);
}

#[test]
fn test_crystal_space() {
    let axes = vec![1, 2, 3]
        .into_iter()
        .map(|n| NonZeroUsize::new(n).unwrap())
        .map(|size| CrystalAxis::new(size, Alignment::Parallel64))
        .collect();

    let space = CrystalSpace::new(axes);
    assert!(space.size() >= 6); // Must be at least 1 * 2 * 3, but aligned
}
EOL

    # Update main test file to include align module tests
    if ! grep -q "mod align;" forge/std/tests/primitive_tests.rs; then
        echo -e "\nmod align;" >> forge/std/tests/primitive_tests.rs
    fi

    print_purple "✓ Created alignment module files"
}

main() {
    print_purple "🔮 Creating Spark Alignment Module..."
    create_directory_structure
    setup_align_module
    print_purple "✨ Alignment module created with crystal-space allocation support!

Features:
- Crystal-space memory allocation
- Vector space operations
- Parallel computing alignment
- Multi-dimensional crystal spaces

Alignment Types:
- Crystal16 (16-byte)
- Vector32 (32-byte)
- Parallel64 (64-byte)

Run 'cd forge/std && cargo test' to verify the crystal alignments!"
}

main
