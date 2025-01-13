#![no_std]

/// UnstableMatter Core Library
/// Last Updated: 2025-01-13 00:18:26 UTC
/// Author: Caleb J.D. Terkovics (isdood)
/// Current User: isdood

// Module declarations
pub mod ufo;
pub mod vector_space;

// Re-export core types
pub use vector_space::{
    VectorSpace,
    Vector3D,
    MeshCell,
    CellState,
    SpaceConfig,
    SpaceMetadata,
};

// Public exports
pub use ufo::{
    UFO,
    TrackedUFO,
    Protected,
    MemoryTrace,
    Flying,
    Hovering,
    Landed,
};

pub use crate::vector_space::SpaceTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryAddress(usize);

impl MemoryAddress {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub const fn as_usize(&self) -> usize {
        self.0
    }
}

// Fluid memory space representation
#[derive(Debug)]
pub struct FluidMemory<T: 'static> {
    base: MemoryAddress,
    _marker: core::marker::PhantomData<T>,
}

impl<T: 'static> FluidMemory<T> {
    pub const fn new(addr: usize) -> Self {
        Self {
            base: MemoryAddress::new(addr),
            _marker: core::marker::PhantomData,
        }
    }

    pub const fn base_addr(&self) -> usize {
        self.base.as_usize()
    }

    pub unsafe fn read(&self, offset: usize) -> T {
        core::ptr::read_volatile((self.base_addr() + offset) as *const T)
    }

    pub unsafe fn write(&mut self, offset: usize, value: T) {
        core::ptr::write_volatile((self.base_addr() + offset) as *mut T, value)
    }
}

// Core UnstableMatter definition
#[derive(Debug)]
pub struct UnstableMatter<T: 'static> {
    memory: FluidMemory<T>,
    _ufo: UFO<T>,
}

impl<T: 'static> UnstableMatter<T> {
    pub const fn new(addr: usize) -> Self {
        Self {
            memory: FluidMemory::new(addr),
            _ufo: UFO::new(),
        }
    }

    pub const fn addr(&self) -> usize {
        self.memory.base_addr()
    }

    pub unsafe fn read(&self) -> T {
        self.memory.read(0)
    }

    pub unsafe fn write(&mut self, value: T) {
        self.memory.write(0, value)
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

// Fluid space-time mesh
#[derive(Debug)]
pub struct SpaceTime<T: 'static> {
    memory: FluidMemory<T>,
    size: usize,
    offset: usize,
    stride: usize,
    dimensions: Dimensions,
    _ufo: UFO<T>,
}

impl<T: 'static> SpaceTime<T> {
    pub const fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            memory: FluidMemory::new(base_addr),
            size,
            offset,
            stride: core::mem::size_of::<T>(),
            dimensions: Dimensions::new(size, 1, 1),
            _ufo: UFO::new(),
        }
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

    pub unsafe fn read_at(&self, index: usize) -> T {
        assert!(index < self.size);
        self.memory.read(index * self.stride + self.offset)
    }

    pub unsafe fn write_at(&mut self, index: usize, value: T) {
        assert!(index < self.size);
        self.memory.write(index * self.stride + self.offset, value)
    }
}

unsafe impl<T: 'static> Send for SpaceTime<T> {}
unsafe impl<T: 'static> Sync for SpaceTime<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid_memory() {
        let mut memory = FluidMemory::<u32>::new(0x1000);
        assert_eq!(memory.base_addr(), 0x1000);
    }

    #[test]
    fn test_dimensions() {
        let dims = Dimensions::new(10, 20, 30);
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
