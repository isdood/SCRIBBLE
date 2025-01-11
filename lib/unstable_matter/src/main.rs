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

#![no_std]

use core::ptr;

/// The `UnstableMatter` struct provides volatile read and write operations.
#[derive(Debug)]
pub struct UnstableMatter<T> {
    value: *mut T,
}

unsafe impl<T: Send> Send for UnstableMatter<T> {}
unsafe impl<T: Sync> Sync for UnstableMatter<T> {}

impl<T> UnstableMatter<T> {
    /// Creates a new `UnstableMatter` instance with the given value.
    pub fn new(mut value: T) -> UnstableMatter<T> {
        // In a no_std environment, we'll allocate directly on the stack
        // and then move the pointer to the heap
        UnstableMatter {
            value: &mut value as *mut T,
        }
    }

    /// Creates a new UnstableMatter instance pointing to a specific address
    ///
    /// # Safety
    /// This function is unsafe because it creates a raw pointer to an arbitrary address.
    /// The caller must ensure that the address is valid and properly aligned for type T.
    pub unsafe fn new_at_addr(addr: usize) -> UnstableMatter<T> {
        UnstableMatter {
            value: addr as *mut T,
        }
    }

    /// Performs a volatile read of the value.
    pub fn read(&self) -> T where T: Copy {
        unsafe { ptr::read_volatile(self.value) }
    }

    /// Performs a volatile write to the value.
    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.value, value) }
    }

    /// Gets the raw pointer to the value
    pub fn as_ptr(&self) -> *mut T {
        self.value
    }
}
