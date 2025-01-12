/// UnstableMatter: Low-Level Memory Access Wrapper
/// Last Updated: 2025-01-12 21:18:27 UTC
/// Author: Caleb J.D. Terkovics (isdood)
///
/// This module provides direct volatile memory access with safety guarantees.
/// It serves as the foundational layer for SpaceTime's memory operations.
///
/// Safety:
/// - All operations are marked unsafe as they involve raw pointer manipulation
/// - Send and Sync are implemented to allow safe concurrent access
/// - PhantomData ensures proper type tracking without runtime overhead

use core::marker::PhantomData;

#[derive(Debug)]
pub struct UnstableMatter<T> {
    addr: usize,
    _phantom: PhantomData<T>,
}

impl<T> UnstableMatter<T> {
    /// Creates a new UnstableMatter instance at compile time
    ///
    /// # Safety
    /// - Caller must ensure addr points to valid memory of type T
    /// - Addr must be properly aligned for T
    /// - Memory region must remain valid for the lifetime of the instance
    pub const fn const_at(addr: usize) -> Self {
        Self {
            addr,
            _phantom: PhantomData,
        }
    }

    /// Creates a new UnstableMatter instance at runtime
    ///
    /// # Safety
    /// Same safety requirements as const_at
    pub unsafe fn at(addr: usize) -> Self {
        Self::const_at(addr)
    }

    /// Performs a volatile read from the underlying memory
    ///
    /// # Safety
    /// - Memory must contain a valid value of type T
    /// - No other mutable references to this memory may exist
    pub unsafe fn read(&self) -> T
    where
    T: Copy,
    {
        core::ptr::read_volatile(self.addr as *const T)
    }

    /// Performs a volatile write to the underlying memory
    ///
    /// # Safety
    /// - Memory must be writable
    /// - No other references to this memory may exist
    pub unsafe fn write(&mut self, value: T) {
        core::ptr::write_volatile(self.addr as *mut T, value)
    }

    /// Returns the raw address being managed
    ///
    /// This method is const and can be used in constant contexts
    pub const fn addr(&self) -> usize {
        self.addr
    }
}

// Safety: UnstableMatter can be safely sent between threads
unsafe impl<T> Send for UnstableMatter<T> {}

// Safety: UnstableMatter can be safely shared between threads
unsafe impl<T> Sync for UnstableMatter<T> {}
