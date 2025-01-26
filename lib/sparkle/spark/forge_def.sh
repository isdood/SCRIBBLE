#!/bin/bash

# Spark Def Module Setup Script
# Author: isdood
# Created: 2025-01-25 19:10:21 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized default value system

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_def_module() {
    cd forge/std || exit 1

    # 1. Create def module structure
    mkdir -p src/def
    mkdir -p tests/def

    # 2. Update lib.rs
    if ! grep -q "pub mod def;" src/lib.rs; then
        sed -i '/pub mod array;/a pub mod def;' src/lib.rs
        sed -i '/pub use array::CrystalArray;/a pub use def::{CrystalDefault, CrystalInit};' src/lib.rs
    fi

    # 3. Create main module file
    cat > src/def/mod.rs << 'EOL'
//! Crystal-optimized default value system
//!
//! This module provides traits and implementations for initializing values
//! with optimal alignment and SIMD-friendly patterns.

use crate::align::Alignment;
use std::mem::MaybeUninit;

/// A crystal-optimized version of Default
pub trait CrystalDefault: Sized {
    /// Creates a default value
    fn crystal_default() -> Self;

    /// Returns the optimal alignment for this type
    fn optimal_alignment() -> Alignment {
        Alignment::Crystal16
    }
}

/// A trait for zero-initialization
pub trait CrystalInit: Sized {
    /// Creates a zero-initialized value
    fn crystal_zeroed() -> Self;

    /// Creates an uninitialized value with proper alignment
    fn crystal_uninit() -> MaybeUninit<Self>;

    /// Returns the optimal alignment for initialization
    fn init_alignment() -> Alignment {
        Alignment::Crystal16
    }
}

// Implement for common numeric types
macro_rules! impl_crystal_numeric_default {
    ($($t:ty => $default:expr),*) => {
        $(
            impl CrystalDefault for $t {
                fn crystal_default() -> Self {
                    $default
                }

                fn optimal_alignment() -> Alignment {
                    if std::mem::size_of::<$t>() >= 32 {
                        Alignment::Vector32
                    } else {
                        Alignment::Crystal16
                    }
                }
            }

            impl CrystalInit for $t {
                fn crystal_zeroed() -> Self {
                    0 as $t
                }

                fn crystal_uninit() -> MaybeUninit<Self> {
                    MaybeUninit::uninit()
                }

                fn init_alignment() -> Alignment {
                    if std::mem::size_of::<$t>() >= 32 {
                        Alignment::Vector32
                    } else {
                        Alignment::Crystal16
                    }
                }
            }
        )*
    }
}

// Implement for arrays
impl<T: CrystalDefault, const N: usize> CrystalDefault for [T; N] {
    fn crystal_default() -> Self {
        let mut arr = std::mem::MaybeUninit::<[T; N]>::uninit();
        let ptr = arr.as_mut_ptr() as *mut T;

        for i in 0..N {
            unsafe {
                ptr.add(i).write(T::crystal_default());
            }
        }

        unsafe { arr.assume_init() }
    }

    fn optimal_alignment() -> Alignment {
        if std::mem::size_of::<T>() * N >= 32 {
            Alignment::Vector32
        } else {
            T::optimal_alignment()
        }
    }
}

impl<T: CrystalInit, const N: usize> CrystalInit for [T; N] {
    fn crystal_zeroed() -> Self {
        let mut arr = std::mem::MaybeUninit::<[T; N]>::uninit();
        let ptr = arr.as_mut_ptr() as *mut T;

        for i in 0..N {
            unsafe {
                ptr.add(i).write(T::crystal_zeroed());
            }
        }

        unsafe { arr.assume_init() }
    }

    fn crystal_uninit() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }

    fn init_alignment() -> Alignment {
        if std::mem::size_of::<T>() * N >= 32 {
            Alignment::Vector32
        } else {
            T::init_alignment()
        }
    }
}

// Implement for numeric types
impl_crystal_numeric_default! {
    u8 => 0,
    u16 => 0,
    u32 => 0,
    u64 => 0,
    u128 => 0,
    usize => 0,
    i8 => 0,
    i16 => 0,
    i32 => 0,
    i64 => 0,
    i128 => 0,
    isize => 0,
    f32 => 0.0,
    f64 => 0.0
}

// Implement for bool
impl CrystalDefault for bool {
    fn crystal_default() -> Self {
        false
    }
}

impl CrystalInit for bool {
    fn crystal_zeroed() -> Self {
        false
    }

    fn crystal_uninit() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }
}

// Implement for common containers
impl<T: CrystalDefault> CrystalDefault for Vec<T> {
    fn crystal_default() -> Self {
        Vec::new()
    }

    fn optimal_alignment() -> Alignment {
        T::optimal_alignment()
    }
}

impl<T> CrystalInit for Vec<T> {
    fn crystal_zeroed() -> Self {
        Vec::new()
    }

    fn crystal_uninit() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }

    fn init_alignment() -> Alignment {
        Alignment::Vector32
    }
}

impl CrystalDefault for String {
    fn crystal_default() -> Self {
        String::new()
    }

    fn optimal_alignment() -> Alignment {
        Alignment::Vector32
    }
}

impl CrystalInit for String {
    fn crystal_zeroed() -> Self {
        String::new()
    }

    fn crystal_uninit() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }

    fn init_alignment() -> Alignment {
        Alignment::Vector32
    }
}
EOL

    # 4. Create tests
    cat > tests/def/mod.rs << 'EOL'
use spark_std::def::{CrystalDefault, CrystalInit};
use spark_std::align::Alignment;

#[test]
fn test_numeric_defaults() {
    assert_eq!(u32::crystal_default(), 0);
    assert_eq!(i64::crystal_default(), 0);
    assert_eq!(f32::crystal_default(), 0.0);
}

#[test]
fn test_array_defaults() {
    let arr: [i32; 4] = CrystalDefault::crystal_default();
    assert_eq!(arr, [0, 0, 0, 0]);
}

#[test]
fn test_container_defaults() {
    let vec: Vec<i32> = CrystalDefault::crystal_default();
    assert!(vec.is_empty());

    let string: String = CrystalDefault::crystal_default();
    assert!(string.is_empty());
}

#[test]
fn test_alignment_optimization() {
    assert_eq!(<[u32; 8]>::optimal_alignment(), Alignment::Vector32);
    assert_eq!(<u8>::optimal_alignment(), Alignment::Crystal16);
}

#[test]
fn test_zero_initialization() {
    let zero_i32 = i32::crystal_zeroed();
    assert_eq!(zero_i32, 0);

    let zero_arr: [i32; 4] = CrystalInit::crystal_zeroed();
    assert_eq!(zero_arr, [0, 0, 0, 0]);
}

#[test]
fn test_uninitialized() {
    let mut uninit = i32::crystal_uninit();
    unsafe {
        *uninit.as_mut_ptr() = 42;
        assert_eq!(uninit.assume_init(), 42);
    }
}

#[test]
fn test_bool_default() {
    assert_eq!(bool::crystal_default(), false);
    assert_eq!(bool::crystal_zeroed(), false);
}

#[test]
fn test_string_zero() {
    let zero_string = String::crystal_zeroed();
    assert!(zero_string.is_empty());
}
EOL

    print_purple "✓ Created def module files"
}

main() {
    print_purple "🔮 Creating Spark Def Module..."
    setup_def_module
    print_purple "✨ Def module created with crystal-space optimization!

Features:
- Crystal-optimized defaults
- SIMD-friendly initialization
- Zero-initialization support
- Uninitialized memory handling
- Alignment-aware operations
- Container optimizations
- Array specializations

Run 'cargo test' to verify the implementation!"
}

main
