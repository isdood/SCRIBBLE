#![no_std]

/// UnstableMatter Core Library
/// Last Updated: 2025-01-12 23:44:33 UTC
/// Author: isdood
/// Current User: isdood

// Module declarations
pub mod ufo;
pub mod vector_space;
pub mod align;
pub use align::{Alignment, AlignedRegion};

// Public exports
pub use ufo::{UFO, Protected, MemoryTrace};
pub use vector_space::{VectorSpace, MeshCell, CellState};

// Type aliases for common use
pub type Flying = ufo::Flying;
pub type Hovering = ufo::Hovering;
pub type Landed = ufo::Landed;

/// Memory address representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryAddress(usize);

impl MemoryAddress {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub const fn as_usize(&self) -> usize {
        self.0
    }

    pub const fn as_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }
}

// Core UnstableMatter definition
#[derive(Debug)]
pub struct UnstableMatter<T: 'static> {
    addr: MemoryAddress,
    _ufo: UFO<T>,
}

impl<T: 'static> UnstableMatter<T> {
    pub const fn const_at(addr: usize) -> Self {
        Self {
            addr: MemoryAddress::new(addr),
            _ufo: UFO::new(),
        }
    }

    pub fn at(addr: usize) -> Self {
        Self {
            addr: MemoryAddress::new(addr),
            _ufo: UFO::new(),
        }
    }

    pub unsafe fn read(&self) -> T {
        core::ptr::read_volatile(self.addr.as_ptr())
    }

    pub unsafe fn write(&mut self, value: T) {
        core::ptr::write_volatile(self.addr.as_ptr(), value)
    }

    pub const fn addr(&self) -> usize {
        self.addr.as_usize()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl Dimensions {
    pub const fn const_new(width: usize, height: usize, depth: usize) -> Self {
        Self { width, height, depth }
    }
}

#[derive(Debug)]
pub struct SpaceTime<T: 'static> {
    base: UnstableMatter<T>,
    size: usize,
    offset: usize,
    stride: usize,
    dimensions: Dimensions,
    _ufo: UFO<T>,
}

impl<T: 'static + Copy> SpaceTime<T> {
    pub const fn const_new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            base: UnstableMatter::const_at(base_addr),
            size,
            offset,
            stride: core::mem::size_of::<T>(),
            dimensions: Dimensions::const_new(size, 1, 1),
            _ufo: UFO::new(),
        }
    }

    pub fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self::const_new(base_addr, size, offset)
    }

    pub unsafe fn read_at(&self, index: usize) -> T {
        assert!(index < self.size);
        let addr = self.base.addr() + (index * self.stride) + self.offset;
        UnstableMatter::at(addr).read()
    }

    pub unsafe fn write_at(&mut self, index: usize, value: T) {
        assert!(index < self.size);
        let addr = self.base.addr() + (index * self.stride) + self.offset;
        UnstableMatter::at(addr).write(value)
    }

    pub const fn size(&self) -> usize {
        self.size
    }

    pub const fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub const fn stride(&self) -> usize {
        self.stride
    }

    pub const fn offset(&self) -> usize {
        self.offset
    }
}

unsafe impl<T: 'static> Send for SpaceTime<T> {}
unsafe impl<T: 'static> Sync for SpaceTime<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_address() {
        let addr = MemoryAddress::new(0x1000);
        assert_eq!(addr.as_usize(), 0x1000);
    }

    #[test]
    fn test_dimensions() {
        let dims = Dimensions::const_new(10, 20, 30);
        assert_eq!(dims.width, 10);
        assert_eq!(dims.height, 20);
        assert_eq!(dims.depth, 30);
    }

    #[test]
    fn test_space_time() {
        let space: SpaceTime<u32> = SpaceTime::new(0x1000, 100, 0);
        assert_eq!(space.size(), 100);
        assert_eq!(space.stride(), core::mem::size_of::<u32>());
    }
}
