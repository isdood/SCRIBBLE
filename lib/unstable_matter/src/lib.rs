// In lib/unstable_matter/src/lib.rs
#![no_std]

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

    pub fn addr(&self) -> usize {
        self.addr as usize
    }
}

impl UnstableMatter<u8> {
    pub fn write_byte(&mut self, value: u8) {
        self.write(value)
    }
}
