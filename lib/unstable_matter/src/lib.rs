#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::mesh;

/// UnstableMatter Core Library
/// Last Updated: 2025-01-13 03:23:37 UTC
/// Author: Caleb J.D. Terkovics (isdood)
/// Current User: isdood

pub mod vector;
pub mod mesh;
pub mod space_config;
pub mod tracked_ufo;
pub mod morph_tracker;
pub mod vector_space;
pub mod ufo_states;
pub mod ufo;

// Re-export core components
pub use vector::{Vector3D, IntVector3D};
pub use mesh::MeshCell;
pub use space_config::{SpaceConfig, SpaceMetadata};
pub use tracked_ufo::TrackedUFO;
pub use morph_tracker::{MorphTracker, FileType};
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

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl Dimensions {
    pub const fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn to_vector(&self) -> IntVector3D {
        IntVector3D::new(
            self.width as isize,
            self.height as isize,
            self.depth as isize,
        )
    }

    pub fn from_vector(vec: &IntVector3D) -> Self {
        Self {
            width: vec.x.max(0) as usize,
            height: vec.y.max(0) as usize,
            depth: vec.z.max(0) as usize,
        }
    }

    pub fn volume(&self) -> usize {
        self.width * self.height * self.depth
    }
}

// Fluid memory space representation
#[derive(Debug)]
pub struct FluidMemory<T: 'static> {
    base: MemoryAddress,
    timestamp: AtomicUsize,
    _ufo: UFO<T>,
}

impl<T: 'static> FluidMemory<T> {
    pub const fn new(base: MemoryAddress) -> Self {
        Self {
            base,
            timestamp: AtomicUsize::new(1705111898), // 2025-01-13 03:11:38 UTC
            _ufo: UFO::new(),
        }
    }

    pub const fn get_base(&self) -> MemoryAddress {
        self.base
    }

    pub const fn base_addr(&self) -> usize {
        self.base.as_usize()
    }
}

impl<T: Copy + 'static> FluidMemory<T> {
    pub unsafe fn read(&self, offset: usize) -> T {
        let addr = self.base.as_usize() + offset;
        core::ptr::read_volatile(addr as *const T)
    }

    pub unsafe fn write(&mut self, offset: usize, value: T) {
        let addr = self.base.as_usize() + offset;
        core::ptr::write_volatile(addr as *mut T, value);
        self.timestamp.store(1705111898, Ordering::SeqCst); // 2025-01-13 03:11:38 UTC
    }
}

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

impl<T: Copy + 'static> SpaceTime<T> {
    pub const fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            memory: FluidMemory::new(MemoryAddress::new(base_addr)),
            size,
            offset,
            stride: core::mem::size_of::<T>(),
            dimensions: Dimensions::new(size, 1, 1),
            timestamp: AtomicUsize::new(1705111898), // 2025-01-13 03:11:38 UTC
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
        self.timestamp.store(1705111898, Ordering::SeqCst); // 2025-01-13 03:11:38 UTC
    }
}

unsafe impl<T: Copy + 'static> Send for SpaceTime<T> {}
unsafe impl<T: Copy + 'static> Sync for SpaceTime<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid_memory() {
        let memory = FluidMemory::<u32>::new(MemoryAddress::new(0x1000));
        assert_eq!(memory.base_addr(), 0x1000);
    }

    #[test]
    fn test_dimensions() {
        let dims = Dimensions::new(10, 20, 30);
        assert_eq!(dims.width, 10);
        assert_eq!(dims.height, 20);
        assert_eq!(dims.depth, 30);
        assert_eq!(dims.volume(), 6000);
    }

    #[test]
    fn test_dimensions_vector_conversion() {
        let dims = Dimensions::new(10, 20, 30);
        let vec = dims.to_vector();
        assert_eq!(vec.x, 10);
        assert_eq!(vec.y, 20);
        assert_eq!(vec.z, 30);

        let dims2 = Dimensions::from_vector(&vec);
        assert_eq!(dims2.width, 10);
        assert_eq!(dims2.height, 20);
        assert_eq!(dims2.depth, 30);
    }

    #[test]
    fn test_space_time() {
        let space: SpaceTime<u32> = SpaceTime::new(0x1000, 100, 0);
        assert_eq!(space.size(), 100);
        assert_eq!(space.stride(), core::mem::size_of::<u32>());
    }
}
