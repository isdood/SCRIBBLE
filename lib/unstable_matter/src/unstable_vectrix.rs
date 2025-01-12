// lib/unstable_matter/src/unstable_vectrix.rs

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
            base: UnstableMatter::at(base_addr),
            size,
            offset,
        }
    }

    pub fn read(&self, idx: usize) -> T where T: Copy {
        assert!(idx < self.size);
        unsafe { self.base.read() }
    }

    pub fn write(&mut self, idx: usize, value: T) {
        assert!(idx < self.size);
        unsafe { self.base.write(value) }
    }

    pub fn move_to(&mut self, new_addr: usize) {
        self.base = unsafe { UnstableMatter::at(new_addr) };
    }
}
