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

pub struct UnstableMatter<T> {
    ptr: *mut T,
}

impl<T> UnstableMatter<T> {
    pub unsafe fn at(addr: usize) -> Self {
        Self { ptr: addr as *mut T }
    }

    pub unsafe fn read(&self) -> T {
        core::ptr::read_volatile(self.ptr)
    }

    pub unsafe fn write(&mut self, value: T) {
        core::ptr::write_volatile(self.ptr, value)
    }

    // Add fence operations for hardware operations
    pub fn fence(&self) {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    }
}
