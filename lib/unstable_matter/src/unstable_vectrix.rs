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

use crate::UnstableMatter;

pub struct UnstableVectrix<T> {
    base: UnstableMatter<T>,
    size: usize,
    offset: usize,
}

impl<T> UnstableVectrix<T> {
    pub unsafe fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            base: UnstableMatter::at(base_addr), // Changed from new_at_addr to at
            size,
            offset,
        }
    }

    pub fn read(&self, idx: usize) -> T where T: Copy {
        assert!(idx < self.size);
        unsafe { self.base.read() }
    }

    pub fn write(&mut self, idx: usize, value: T) { // Changed to &mut self
        assert!(idx < self.size);
        unsafe { self.base.write(value) }
    }

    pub fn move_to(&mut self, new_addr: usize) {
        self.base = unsafe { UnstableMatter::at(new_addr) }; // Changed from new_at_addr to at
    }
}
