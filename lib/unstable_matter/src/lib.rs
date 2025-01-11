#![no_std]

pub struct UnstableMatter<T> {
    addr: *mut T,
}

impl<T> UnstableMatter<T> {
    pub fn at(addr: usize) -> Self {
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

    pub fn as_ptr(&self) -> *mut T {
        self.addr
    }
}
