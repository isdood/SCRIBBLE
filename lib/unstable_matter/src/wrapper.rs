// lib/unstable_matter/src/wrapper.rs

use crate::unstable_vectrix::UnstableVectrix;

pub struct Wrapper<T> {
    vectrix: UnstableVectrix<T>,
}

impl<T> Wrapper<T> {
    pub unsafe fn new(addr: usize, size: usize, _offset: usize) -> Self {
        Self {
            vectrix: UnstableVectrix::new(addr, size, _offset),
        }
    }

    pub fn read(&self, idx: usize) -> T where T: Copy {
        self.vectrix.read(idx)
    }

    pub fn write(&mut self, idx: usize, value: T) {
        self.vectrix.write(idx, value)
    }

    pub fn move_to(&mut self, new_addr: usize) {
        self.vectrix.move_to(new_addr)
    }
}
