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
    /// Creates a new `UnstableMatter` instance at a specific address
    ///
    /// # Safety
    /// This function is unsafe because it creates a raw pointer to an arbitrary address.
    /// The caller must ensure that the address is valid for reads/writes of type T.
    #[inline]
    pub unsafe fn at(addr: usize) -> Self {
        UnstableMatter {
            value: addr as *mut T,
        }
    }

    /// Performs a volatile read of the value.
    #[inline]
    pub fn read(&self) -> T
    where
    T: Copy
    {
        unsafe { ptr::read_volatile(self.value) }
    }

    /// Performs a volatile write to the value.
    #[inline]
    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.value, value) }
    }

    /// Gets the raw pointer to the value
    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.value
    }

    /// Gets the address as a usize
    #[inline]
    pub fn addr(&self) -> usize {
        self.value as usize
    }
}
