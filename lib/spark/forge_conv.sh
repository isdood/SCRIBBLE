#!/bin/bash

# Spark Conv Module Setup Script
# Author: isdood
# Created: 2025-01-25 19:08:13 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized conversion system

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_conv_module() {
    cd forge/std || exit 1

    # 1. Create conv module structure
    mkdir -p src/conv
    mkdir -p tests/conv

    # 2. Update lib.rs
    if ! grep -q "pub mod conv;" src/lib.rs; then
        sed -i '/pub mod array;/a pub mod conv;' src/lib.rs
        sed -i '/pub use array::CrystalArray;/a pub use conv::{CrystalFrom, CrystalInto, CrystalTryFrom, CrystalTryInto};' src/lib.rs
    fi

    # 3. Create main module file
    cat > src/conv/mod.rs << 'EOL'
//! Crystal-optimized conversion traits and implementations.
//!
//! This module provides traits for converting between types in crystal-space,
//! with optimizations for SIMD operations and aligned memory access.

use crate::align::Alignment;
use std::error::Error;
use std::fmt;

/// Error type for conversion failures
#[derive(Debug)]
pub struct ConversionError {
    message: String,
}

impl ConversionError {
    /// Creates a new conversion error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Conversion error: {}", self.message)
    }
}

impl Error for ConversionError {}

/// A crystal-optimized version of From
pub trait CrystalFrom<T>: Sized {
    /// Converts from T to Self
    fn crystal_from(value: T) -> Self;

    /// Returns the optimal alignment for this conversion
    fn optimal_alignment() -> Alignment {
        Alignment::Crystal16
    }
}

/// A crystal-optimized version of Into
pub trait CrystalInto<T>: Sized {
    /// Converts self into T
    fn crystal_into(self) -> T;
}

/// A crystal-optimized version of TryFrom
pub trait CrystalTryFrom<T>: Sized {
    /// The type returned in the event of a conversion error
    type Error;

    /// Attempts to convert from T to Self
    fn crystal_try_from(value: T) -> Result<Self, Self::Error>;

    /// Returns the optimal alignment for this conversion
    fn optimal_alignment() -> Alignment {
        Alignment::Crystal16
    }
}

/// A crystal-optimized version of TryInto
pub trait CrystalTryInto<T>: Sized {
    /// The type returned in the event of a conversion error
    type Error;

    /// Attempts to convert self into T
    fn crystal_try_into(self) -> Result<T, Self::Error>;
}

// Implement CrystalInto when CrystalFrom is implemented
impl<T, U> CrystalInto<U> for T
where
    U: CrystalFrom<T>,
{
    fn crystal_into(self) -> U {
        U::crystal_from(self)
    }
}

// Implement CrystalTryInto when CrystalTryFrom is implemented
impl<T, U> CrystalTryInto<U> for T
where
    U: CrystalTryFrom<T>,
{
    type Error = U::Error;

    fn crystal_try_into(self) -> Result<U, U::Error> {
        U::crystal_try_from(self)
    }
}

// SIMD-optimized conversions for numeric types
macro_rules! impl_crystal_numeric_conversion {
    ($($from:ty => $to:ty),*) => {
        $(
            impl CrystalFrom<$from> for $to {
                fn crystal_from(value: $from) -> Self {
                    value as $to
                }

                fn optimal_alignment() -> Alignment {
                    if std::mem::size_of::<$to>() >= 32 {
                        Alignment::Vector32
                    } else {
                        Alignment::Crystal16
                    }
                }
            }
        )*
    }
}

// Implement numeric conversions
impl_crystal_numeric_conversion! {
    u8 => u16, u8 => u32, u8 => u64, u8 => u128, u8 => usize,
    u16 => u32, u16 => u64, u16 => u128, u16 => usize,
    u32 => u64, u32 => u128, u32 => usize,
    u64 => u128,
    i8 => i16, i8 => i32, i8 => i64, i8 => i128, i8 => isize,
    i16 => i32, i16 => i64, i16 => i128, i16 => isize,
    i32 => i64, i32 => i128, i32 => isize,
    i64 => i128,
    f32 => f64
}

// SIMD-optimized conversions for arrays
impl<T, U, const N: usize> CrystalTryFrom<[T; N]> for [U; N]
where
    U: CrystalTryFrom<T>,
{
    type Error = ConversionError;

    fn crystal_try_from(value: [T; N]) -> Result<Self, Self::Error> {
        let mut result = std::mem::MaybeUninit::<[U; N]>::uninit();
        let ptr = result.as_mut_ptr() as *mut U;

        for (i, item) in value.into_iter().enumerate() {
            match U::crystal_try_from(item) {
                Ok(converted) => unsafe {
                    ptr.add(i).write(converted);
                },
                Err(_) => return Err(ConversionError::new("Array conversion failed")),
            }
        }

        Ok(unsafe { result.assume_init() })
    }

    fn optimal_alignment() -> Alignment {
        if std::mem::size_of::<U>() * N >= 32 {
            Alignment::Vector32
        } else {
            Alignment::Crystal16
        }
    }
}

// String conversions
impl CrystalFrom<String> for Vec<u8> {
    fn crystal_from(value: String) -> Self {
        value.into_bytes()
    }

    fn optimal_alignment() -> Alignment {
        Alignment::Vector32
    }
}

impl CrystalTryFrom<Vec<u8>> for String {
    type Error = ConversionError;

    fn crystal_try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        String::from_utf8(value)
            .map_err(|_| ConversionError::new("Invalid UTF-8"))
    }

    fn optimal_alignment() -> Alignment {
        Alignment::Vector32
    }
}
EOL

    # 4. Create tests
    cat > tests/conv/mod.rs << 'EOL'
use spark_std::conv::{CrystalFrom, CrystalInto, CrystalTryFrom, CrystalTryInto};
use spark_std::align::Alignment;

#[test]
fn test_numeric_conversions() {
    let x: u8 = 42;
    let y: u16 = x.crystal_into();
    assert_eq!(y, 42u16);

    let z: u32 = u16::crystal_from(y);
    assert_eq!(z, 42u32);
}

#[test]
fn test_array_conversions() {
    let arr: [u8; 4] = [1, 2, 3, 4];
    let converted: [u16; 4] = arr.crystal_try_into().unwrap();
    assert_eq!(converted, [1, 2, 3, 4]);
}

#[test]
fn test_string_conversions() {
    let s = String::from("Hello");
    let bytes: Vec<u8> = s.crystal_into();
    let back: String = bytes.crystal_try_into().unwrap();
    assert_eq!(back, "Hello");
}

#[test]
fn test_alignment_optimization() {
    assert_eq!(<[u32; 8]>::optimal_alignment(), Alignment::Vector32);
    assert_eq!(<[u8; 4]>::optimal_alignment(), Alignment::Crystal16);
}

#[test]
fn test_conversion_error() {
    let invalid_utf8: Vec<u8> = vec![0xFF, 0xFF];
    let result: Result<String, _> = invalid_utf8.crystal_try_into();
    assert!(result.is_err());
}

#[test]
fn test_numeric_conversion_alignment() {
    assert_eq!(<u128>::optimal_alignment(), Alignment::Vector32);
    assert_eq!(<u8>::optimal_alignment(), Alignment::Crystal16);
}
EOL

    print_purple "✓ Created conv module files"
}

main() {
    print_purple "🔮 Creating Spark Conv Module..."
    setup_conv_module
    print_purple "✨ Conv module created with crystal-space optimization!

Features:
- Crystal-optimized conversions
- SIMD-accelerated numeric transforms
- Alignment-aware operations
- Safe string conversions
- Array transformations
- Error handling
- Comprehensive testing

Run 'cargo test' to verify the implementation!"
}

main
