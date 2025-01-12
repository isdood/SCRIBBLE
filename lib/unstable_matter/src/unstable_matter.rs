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
