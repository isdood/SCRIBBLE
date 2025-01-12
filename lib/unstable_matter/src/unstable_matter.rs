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

// lib/unstable_matter/src/unstable_matter.rs

/// A type that provides volatile access to memory at a specific address.
#[derive(Debug)]
pub struct UnstableMatter<T> {
    ptr: *mut T,
}

impl<T> UnstableMatter<T> {
    /// Creates a new UnstableMatter instance at the specified address.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The address is valid for the type T
    /// - The memory at the address is properly aligned for T
    /// - No other references to this memory exist while this UnstableMatter is in use
    pub unsafe fn at(addr: usize) -> Self {
        Self { ptr: addr as *mut T }
    }

    /// Performs a volatile read of the value.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The memory is valid for reading
    /// - No concurrent writes are happening to this memory location
    pub unsafe fn read(&self) -> T {
        core::ptr::read_volatile(self.ptr)
    }

    /// Performs a volatile write of the value.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The memory is valid for writing
    /// - No concurrent reads or writes are happening to this memory location
    pub unsafe fn write(&mut self, value: T) {
        core::ptr::write_volatile(self.ptr, value)
    }

    /// Returns the raw address as a usize.
    pub fn addr(&self) -> usize {
        self.ptr as usize
    }

    /// Inserts a memory fence to ensure ordering of memory operations.
    pub fn fence(&self) {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst)
    }

    /// Returns the raw pointer to the memory location.
    pub fn ptr(&self) -> *mut T {
        self.ptr
    }

    /// Returns a pointer offset by the specified number of elements.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The offset does not cause the pointer to wrap around the address space
    /// - The resulting pointer remains within the same allocated object
    pub fn ptr_add(&self, offset: usize) -> *mut T {
        unsafe { self.ptr.add(offset) }
    }
}

// Implement Send to allow transfer between threads
// This is safe because UnstableMatter only provides raw pointer access
// and requires unsafe blocks for all operations
unsafe impl<T> Send for UnstableMatter<T> {}
