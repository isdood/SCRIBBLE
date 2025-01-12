// lib/unstable_matter/src/unstable_vectrix.rs

use crate::unstable_matter::UnstableMatter;

#[derive(Debug)]
pub struct UnstableVectrix<T> {
    base: UnstableMatter<T>,
    size: usize,
    offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub u64);

impl<T> UnstableVectrix<T> {
    pub unsafe fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            base: UnstableMatter::at(base_addr),
            size,
            offset,
        }
    }

    pub fn from_virt(addr: VirtAddr, size: usize) -> Self {
        unsafe { Self::new(addr.0 as usize, size, 0) }
    }

    pub fn from_phys(addr: PhysAddr, size: usize, phys_offset: u64) -> Self {
        unsafe { Self::new((addr.0 + phys_offset) as usize, size, 0) }
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

    pub fn virt_addr(&self) -> VirtAddr {
        VirtAddr(self.base.addr() as u64)
    }

    pub fn phys_addr(&self, phys_offset: u64) -> PhysAddr {
        PhysAddr(self.base.addr() as u64 - phys_offset)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl VirtAddr {
    pub const fn new(addr: u64) -> Self {
        VirtAddr(addr)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }
}

impl PhysAddr {
    pub const fn new(addr: u64) -> Self {
        PhysAddr(addr)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn to_virt(self, phys_offset: u64) -> VirtAddr {
        VirtAddr(self.0 + phys_offset)
    }
}

// Required for safe usage across threads
unsafe impl<T> Send for UnstableVectrix<T> {}
