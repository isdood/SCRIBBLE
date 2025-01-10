#![no_std]

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
    pub fn read(&self) -> T where T: Copy {
        unsafe { ptr::read_volatile(self.value) }
    }

    /// Performs a volatile write to the value.
    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.value, value) }
    }
}
