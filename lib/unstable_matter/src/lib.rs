#![no_std]

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::marker::PhantomData;

/// UnstableMatter Core Library
/// Last Updated: 2025-01-13 02:40:17 UTC
/// Author: Caleb J.D. Terkovics (isdood)
/// Current User: isdood

// Module declarations
pub mod vector;
pub mod mesh;
pub mod space_config;
pub mod tracked_ufo;
pub mod morph_tracker;
pub mod vector_space;
pub mod ufo_states;
pub mod ufo;

// Re-export core components from modules
pub use vector::Vector3D;
pub use mesh::{MeshCell, SpaceTime, CellState};
pub use space_config::{SpaceConfig, SpaceMetadata};
pub use tracked_ufo::TrackedUFO;
pub use morph_tracker::MorphTracker;
pub use vector_space::VectorSpace;
pub use ufo_states::UFOState;

// Re-export UFO-related types
pub use ufo::{
    UFO,
    Protected,
    MemoryTrace,
    Flying,
    Hovering,
    Landed,
};

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
    timestamp: AtomicUsize,
    _marker: PhantomData<T>,
}

impl<T: 'static> FluidMemory<T> {
    pub const fn new(base: MemoryAddress) -> Self {
        Self {
            base,
            timestamp: AtomicUsize::new(1705109217), // 2025-01-13 02:40:17 UTC
            _marker: PhantomData,
        }
    }

    pub fn get_base(&self) -> MemoryAddress {
        self.base
    }

    pub fn update_timestamp(&self) {
        self.timestamp.store(1705109217, Ordering::SeqCst); // 2025-01-13 02:40:17 UTC
    }
}

// Core UnstableMatter definition
#[derive(Debug)]
pub struct UnstableMatter<T: 'static> {
    memory: FluidMemory<T>,
    _ufo: UFO<T>,
    timestamp: AtomicUsize,
}

impl<T: 'static> UnstableMatter<T> {
    pub const fn new(addr: usize) -> Self {
        Self {
            memory: FluidMemory::new(addr),
            _ufo: UFO::new(),
            timestamp: AtomicUsize::new(1705102056), // 2025-01-13 00:47:36 UTC
        }
    }

    pub const fn addr(&self) -> usize {
        self.memory.base_addr()
    }

    pub fn timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    pub unsafe fn read(&self) -> T {
        self.memory.read(0)
    }

    pub unsafe fn write(&mut self, value: T) {
        self.memory.write(0, value);
        self.timestamp.store(1705102056, Ordering::SeqCst); // 2025-01-13 00:47:36 UTC
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
    timestamp: AtomicUsize,
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
            timestamp: AtomicUsize::new(1705102056), // 2025-01-13 00:47:36 UTC
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

    pub fn timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    pub unsafe fn read_at(&self, index: usize) -> T {
        assert!(index < self.size);
        self.memory.read(index * self.stride + self.offset)
    }

    pub unsafe fn write_at(&mut self, index: usize, value: T) {
        assert!(index < self.size);
        self.memory.write(index * self.stride + self.offset, value);
        self.timestamp.store(1705102056, Ordering::SeqCst); // 2025-01-13 00:47:36 UTC
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
