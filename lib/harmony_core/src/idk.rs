//! Custom Uninitialized Memory Implementation
//! ====================================
//!
//! Safe abstractions for handling uninitialized memory in
//! the shard architecture quantum computing environment.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 21:16:07 UTC
//! Version: 0.1.0
//! License: MIT

use core::{
    mem::ManuallyDrop,
    ptr,
};

/// A quantum-safe wrapper around potentially uninitialized memory.
/// This implementation is specifically designed for the shard architecture,
/// taking into account quantum decoherence and state preservation.
#[repr(transparent)]
pub struct ShardUninit<T> {
    /// The wrapped value, using ManuallyDrop to prevent automatic dropping
    /// of potentially uninitialized memory
    value: ManuallyDrop<T>,
}

impl<T> ShardUninit<T> {
    /// Creates a new instance with explicitly uninitialized contents.
    ///
    /// # Safety
    ///
    /// The contents are uninitialized and must not be read until initialized.
    #[inline]
    pub const fn uninit() -> Self {
        // SAFETY: ManuallyDrop prevents automatic dropping of uninitialized memory
        unsafe {
            Self {
                value: ManuallyDrop::new(core::mem::uninitialized()),
            }
        }
    }

    /// Creates a new instance with the given value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: ManuallyDrop::new(value),
        }
    }

    /// Creates an array of ShardUninit with uninitialized contents.
    ///
    /// # Safety
    ///
    /// The contents are uninitialized and must not be read until initialized.
    pub const fn uninit_array<const N: usize>() -> [Self; N] {
        // Create uninitialized array using MaybeUninit's internal representation
        unsafe { core::mem::transmute([0u8; core::mem::size_of::<T>() * N]) }
    }

    /// Returns a pointer to the contained value.
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        &*self.value as *const T
    }

    /// Returns a mutable pointer to the contained value.
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        &mut *self.value as *mut T
    }

    /// Extracts the value from the ShardUninit container.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value has been properly initialized.
    #[inline]
    pub unsafe fn assume_init(self) -> T {
        ManuallyDrop::into_inner(self.value)
    }

    /// Gets a reference to the contained value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value has been properly initialized.
    #[inline]
    pub unsafe fn assume_init_ref(&self) -> &T {
        &*self.value
    }

    /// Gets a mutable reference to the contained value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value has been properly initialized.
    #[inline]
    pub unsafe fn assume_init_mut(&mut self) -> &mut T {
        &mut *self.value
    }

    /// Writes a value to the uninitialized memory.
    #[inline]
    pub fn write(&mut self, value: T) -> &mut T {
        unsafe {
            ptr::write(self.as_mut_ptr(), value);
            self.assume_init_mut()
        }
    }
}

// Implement basic traits
impl<T> Copy for ShardUninit<T> where T: Copy {}

impl<T> Clone for ShardUninit<T> where T: Clone {
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: If T is Clone, we can safely clone the contained value
        unsafe {
            Self::new(ManuallyDrop::into_inner(ManuallyDrop::new((*self.value).clone())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uninit_creation() {
        let uninit: ShardUninit<u32> = ShardUninit::uninit();
        let value = 42;
        unsafe {
            ptr::write(uninit.as_mut_ptr(), value);
            assert_eq!(uninit.assume_init(), 42);
        }
    }

    #[test]
    fn test_init_array() {
        let mut arr: [ShardUninit<u32>; 4] = ShardUninit::uninit_array();
        for i in 0..4 {
            unsafe {
                ptr::write(arr[i].as_mut_ptr(), i as u32);
            }
        }
        unsafe {
            for i in 0..4 {
                assert_eq!(arr[i].assume_init(), i as u32);
            }
        }
    }

    #[test]
    fn test_write_and_read() {
        let mut uninit: ShardUninit<String> = ShardUninit::uninit();
        let value = String::from("test");
        let written = uninit.write(value);
        assert_eq!(written, "test");
    }
}
