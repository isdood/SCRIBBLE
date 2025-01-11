#![no_std]

// In lib/unstable_matter/src/lib.rs or similar:
pub struct UnstableMatter<T> {
    addr: *mut T,
}

impl<T> UnstableMatter<T> {
    pub unsafe fn at(addr: usize) -> Self {
        Self {
            addr: addr as *mut T,
        }
    }

    pub fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(self.addr) }
    }

    pub fn write(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(self.addr, value) }
    }
}
