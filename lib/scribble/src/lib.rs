#![no_std]

/// Scribble Core Library
/// Last Updated: 2025-01-13 00:03:42 UTC
/// Author: isdood
/// Current User: isdood

// External crate imports
extern crate unstable_matter;

// Public exports from unstable_matter
pub use unstable_matter::vector_space::{Vector3D, VectorSpace, MeshCell, CellState};
pub use unstable_matter::ufo::{UFO, Protected, MemoryTrace};

// Module declarations
pub mod align;

// Public exports from local modules
pub use align::{Alignment, AlignedRegion};

// Type aliases for common use
pub type Flying = unstable_matter::ufo::Flying;
pub type Hovering = unstable_matter::ufo::Hovering;
pub type Landed = unstable_matter::ufo::Landed;

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

// Core Scribble memory management
#[derive(Debug)]
pub struct ScribbleMemory<T: 'static> {
    addr: MemoryAddress,
    _ufo: UFO<T>,
}

impl<T: 'static> ScribbleMemory<T> {
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
    pub const fn new(width: usize, height: usize, depth: usize) -> Self {
        Self { width, height, depth }
    }
}

#[derive(Debug)]
pub struct MemorySpace<T: 'static> {
    base: ScribbleMemory<T>,
    size: usize,
    offset: usize,
    stride: usize,
    dimensions: Dimensions,
    _ufo: UFO<T>,
}

impl<T: 'static + Copy> MemorySpace<T> {
    pub const fn const_new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            base: ScribbleMemory::const_at(base_addr),
            size,
            offset,
            stride: core::mem::size_of::<T>(),
            dimensions: Dimensions::new(size, 1, 1),
            _ufo: UFO::new(),
        }
    }

    pub fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self::const_new(base_addr, size, offset)
    }

    pub unsafe fn read_at(&self, index: usize) -> T {
        assert!(index < self.size);
        let addr = self.base.addr() + (index * self.stride) + self.offset;
        ScribbleMemory::at(addr).read()
    }

    pub unsafe fn write_at(&mut self, index: usize, value: T) {
        assert!(index < self.size);
        let addr = self.base.addr() + (index * self.stride) + self.offset;
        ScribbleMemory::at(addr).write(value)
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

unsafe impl<T: 'static> Send for MemorySpace<T> {}
unsafe impl<T: 'static> Sync for MemorySpace<T> {}

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
        let dims = Dimensions::new(10, 20, 30);
        assert_eq!(dims.width, 10);
        assert_eq!(dims.height, 20);
        assert_eq!(dims.depth, 30);
    }

    #[test]
    fn test_memory_space() {
        let space: MemorySpace<u32> = MemorySpace::new(0x1000, 100, 0);
        assert_eq!(space.size(), 100);
        assert_eq!(space.stride(), core::mem::size_of::<u32>());
    }
}
