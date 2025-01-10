// src/unstable_matter.rs

//! This module defines the `UnstableMatter` struct, which provides volatile read and write operations
//! for handling memory-mapped I/O and other hardware-related operations.
//!
//! ## Purpose
//! The `UnstableMatter` struct is used to perform volatile memory operations, which are essential
//! for interacting with hardware where memory values can change independently of the program flow.
//! Examples include writing to video buffers, interacting with device registers, and more.
//!
//! ## Why It Is Inherently Unsafe
//! - Volatile operations bypass compiler optimizations, ensuring that every read and write operation
//!   happens exactly as specified. This is critical for hardware interactions but also means that
//!   the compiler cannot guarantee the safety of these operations.
//! - Using raw pointers (`*mut T`) means there's no automatic memory management, leading to potential
//!   issues like dangling pointers, data races, and undefined behavior if not handled correctly.
//! - The `unsafe` block is required because we are directly manipulating memory, which can lead to
//!   crashes or security vulnerabilities if used improperly.
//!
//! ## Recommendations for Stability and Robustness
//! - **Encapsulation**: Hide unsafe operations behind safe abstractions to minimize the usage of `unsafe` in your codebase.
//! - **Testing**: Thoroughly test all hardware interaction code to ensure it behaves correctly under different conditions.
//! - **Documentation**: Clearly document all unsafe code and the reasons for its use to help maintainers understand the context.
//! - **Concurrency**: Carefully handle concurrent access to volatile memory to avoid data races and ensure thread safety.

use alloc::boxed::Box;
use core::ptr;

/// The `UnstableMatter` struct provides volatile read and write operations.
pub struct UnstableMatter<T> {
    value: *mut T,
}

unsafe impl<T> Send for UnstableMatter<T> {}
unsafe impl<T> Sync for UnstableMatter<T> {}

impl<T> UnstableMatter<T> {
    /// Creates a new `UnstableMatter` instance with the given value.
    pub fn new(value: T) -> UnstableMatter<T> {
        let boxed = Box::new(value);
        UnstableMatter {
            value: Box::into_raw(boxed),
        }
    }

    /// Performs a volatile read of the value.
    ///
    /// # Safety
    /// This function is inherently unsafe because it performs a volatile read
    /// from a raw pointer, which can lead to undefined behavior if the pointer
    /// is invalid or if there are concurrent modifications.
    pub fn read(&self) -> T where T: Copy {
        unsafe { ptr::read_volatile(self.value) }
    }

    /// Performs a volatile write to the value.
    ///
    /// # Safety
    /// This function is inherently unsafe because it performs a volatile write
    /// to a raw pointer, which can lead to undefined behavior if the pointer
    /// is invalid or if there are concurrent modifications.
    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.value, value) }
    }
}
