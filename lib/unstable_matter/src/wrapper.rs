// src/wrapper.rs

mod unstable_matter;
mod unstable_vectrix;

use unstable_matter::UnstableMatter;
use unstable_vectrix::UnstableVectrix;

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
                old: Some(UnstableMatter::new_at_addr(addr)),
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
            Implementation::Old => self.old.as_ref().unwrap().read(),
            Implementation::New => self.new.as_ref().unwrap().read(idx),
        }
    }

    pub fn write(&self, idx: usize, value: T) {
        match self.implementation {
            Implementation::Old => self.old.as_ref().unwrap().write(value),
            Implementation::New => self.new.as_ref().unwrap().write(idx, value),
        }
    }

    pub fn move_to(&mut self, new_addr: usize) {
        match self.implementation {
            Implementation::Old => {
                self.old = Some(unsafe { UnstableMatter::new_at_addr(new_addr) });
            },
            Implementation::New => {
                self.new.as_mut().unwrap().move_to(new_addr);
            },
        }
    }
}
