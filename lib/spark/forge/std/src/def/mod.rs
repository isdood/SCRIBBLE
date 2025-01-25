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
