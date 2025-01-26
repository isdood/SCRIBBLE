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
