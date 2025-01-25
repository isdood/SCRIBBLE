#!/bin/bash

# Spark Array Fix Script
# Author: isdood
# Created: 2025-01-25 18:17:22 UTC
# Repository: isdood/scribble
# Description: Fixes issues in the Crystal Array implementation

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

fix_array_module() {
    cd forge/std || exit 1

    # 1. Fix Alignment module first
    cat > src/align/mod.rs << 'EOL'
//! Crystal-space alignment primitives

pub mod space;

/// Memory alignment requirements for different types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Alignment {
    /// 16-byte crystal alignment
    Crystal16 = 16,
    /// 32-byte vector alignment
    Vector32 = 32,
    /// 64-byte vector alignment (SIMD)
    Vector64 = 64,
    /// 128-byte vector alignment
    Vector128 = 128,
    /// 256-byte parallel alignment
    Parallel256 = 256,
}

impl Alignment {
    /// Returns the alignment value in bytes
    pub fn as_bytes(self) -> usize {
        self as usize
    }

    /// Returns whether this alignment is suitable for SIMD operations
    pub fn is_simd_compatible(self) -> bool {
        matches!(self,
            Alignment::Vector32 |
            Alignment::Vector64 |
            Alignment::Vector128)
    }

    /// Returns whether this alignment is suitable for parallel operations
    pub fn is_parallel_compatible(self) -> bool {
        matches!(self, Alignment::Parallel256)
    }
}
EOL

    # 2. Add space module
    mkdir -p src/align/space
    cat > src/align/space/mod.rs << 'EOL'
//! Crystal-space axis and dimension handling

use super::Alignment;

/// A crystal-space axis with alignment requirements
#[derive(Debug, Clone)]
pub struct CrystalAxis {
    size: usize,
    alignment: Alignment,
}

impl CrystalAxis {
    /// Creates a new crystal axis with the given size and alignment
    pub fn new(size: usize, alignment: Alignment) -> Self {
        CrystalAxis {
            size,
            alignment,
        }
    }

    /// Returns the size of the axis
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the alignment requirement
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Returns whether this axis is SIMD-compatible
    pub fn is_simd_compatible(&self) -> bool {
        self.alignment.is_simd_compatible()
    }

    /// Returns whether this axis is parallel-compatible
    pub fn is_parallel_compatible(&self) -> bool {
        self.alignment.is_parallel_compatible()
    }
}
EOL

    # 3. Fix alignment tests
    mkdir -p tests/align
    cat > tests/align/mod.rs << 'EOL'
use spark_std::align::space::{CrystalAxis};
use spark_std::align::Alignment;

#[test]
fn test_crystal_space() {
    let axis = CrystalAxis::new(64, Alignment::Crystal16);
    assert_eq!(axis.size(), 64);
    assert_eq!(axis.alignment(), Alignment::Crystal16);
    assert!(!axis.is_simd_compatible());
    assert!(!axis.is_parallel_compatible());
}

#[test]
fn test_vector_space() {
    let axis = CrystalAxis::new(128, Alignment::Vector32);
    assert_eq!(axis.size(), 128);
    assert_eq!(axis.alignment(), Alignment::Vector32);
    assert!(axis.is_simd_compatible());
    assert!(!axis.is_parallel_compatible());
}

#[test]
fn test_crystal_allocator() {
    let sizes = [16, 32, 64, 128, 256];
    let axes: Vec<_> = sizes.iter()
        .copied()
        .map(|size| CrystalAxis::new(size, Alignment::Parallel256))
        .collect();

    for (i, axis) in axes.iter().enumerate() {
        assert_eq!(axis.size(), sizes[i]);
        assert_eq!(axis.alignment(), Alignment::Parallel256);
        assert!(!axis.is_simd_compatible());
        assert!(axis.is_parallel_compatible());
    }
}
EOL

    # 4. Fix array tests
    cat > tests/array/mod.rs << 'EOL'
use spark_std::array::CrystalArray;
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
fn test_optimal_alignment() {
    let shard = spark_std::shard::arch::Shard::new();
    let array: CrystalArray<f32> = CrystalArray::new(CrystalArray::<f32>::optimal_alignment());

    match shard.architecture() {
        spark_std::shard::arch::Architecture::X86_64 => {
            if shard.has_feature(spark_std::shard::arch::CpuFeature::AVX512F) {
                assert_eq!(array.alignment(), Alignment::Vector64);
            } else if shard.has_feature(spark_std::shard::arch::CpuFeature::AVX2) {
                assert_eq!(array.alignment(), Alignment::Vector32);
            } else {
                assert_eq!(array.alignment(), Alignment::Crystal16);
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

    for i in 0..100 {
        array.push(i);
        assert_eq!(array.get(i), Some(&i));
    }

    assert_eq!(array.len(), 100);
    assert!(array.capacity() >= 100);
}
EOL

    # 5. Update lib.rs
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
pub use align::Alignment;
pub use shard::arch;
pub use array::CrystalArray;
EOL

    print_purple "✓ Fixed array module implementation"
}

main() {
    print_purple "🔮 Fixing Spark Array Module..."
    fix_array_module
    print_purple "✨ Array module fixes applied!

Fixed Issues:
- Fixed duplicate discriminant values in Alignment enum
- Added is_simd_compatible and is_parallel_compatible methods
- Updated alignment tests with new methods
- Changed Parallel64 to Parallel256 for better distinction
- Added documentation for alignment variants
- Updated test cases for new alignment system

Run 'cargo test' to verify the fixes!"
}

main
