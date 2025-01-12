// lib/unstable_matter/src/wrapper.rs

use crate::unstable_matter::UnstableMatter;
use crate::unstable_vectrix::UnstableVectrix;

pub enum Implementation {
    Old,
    New,
}

pub struct Wrapper<T> {
    implementation: Implementation,
    old: Option<UnstableMatter<T>>,
    new: Option<UnstableVectrix<T>>,
}

impl<T> Wrapper<T> {
    pub unsafe fn new(implementation: Implementation, addr: usize, size: usize, offset: usize) -> Self {
        match implementation {
            Implementation::Old => Self {
                implementation,
                old: Some(UnstableMatter::at(addr)), // Changed from new_at_addr to at
                new: None,
            },
            Implementation::New => Self {
                implementation,
                old: None,
                new: Some(UnstableVectrix::new(addr, size, offset)),
            },
        }
    }

    pub fn read(&self, idx: usize) -> T where T: Copy {
        match self.implementation {
            Implementation::Old => unsafe { self.old.as_ref().unwrap().read() },
            Implementation::New => self.new.as_ref().unwrap().read(idx),
        }
    }

    pub fn write(&mut self, idx: usize, value: T) { // Changed self to &mut self
        match self.implementation {
            Implementation::Old => unsafe { self.old.as_mut().unwrap().write(value) }, // Changed as_ref() to as_mut()
            Implementation::New => self.new.as_mut().unwrap().write(idx, value), // Changed as_ref() to as_mut()
        }
    }

    pub fn move_to(&mut self, new_addr: usize) {
        match self.implementation {
            Implementation::Old => {
                self.old = Some(unsafe { UnstableMatter::at(new_addr) }); // Changed from new_at_addr to at
            },
            Implementation::New => {
                self.new.as_mut().unwrap().move_to(new_addr);
            },
        }
    }
}
