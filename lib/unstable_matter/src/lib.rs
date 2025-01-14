#![no_std]

/// UnstableMatter Core Library
/// Last Updated: 2025-01-14 16:26:27 UTC
/// Author: isdood
/// Current User: isdood

pub mod vector;
pub mod align;
pub mod helium;
pub mod valence;
pub mod mesh_clock;
pub mod ufo;
pub mod phantom;

use core::sync::atomic::{AtomicUsize, Ordering};

// Re-exports
pub use {
    vector::Vector3D,
    align::{Alignment, AlignedSpace},
    helium::{Helium, HeliumSize},
    valence::{ValenceOrder, compare_vectors},
    mesh_clock::{MeshClock, MeshCell, CellState},
    ufo::{UFO, Protected, MemoryTrace},
    phantom::PhantomSpace,
};

// System Constants
pub const QUANTUM_TIMESTAMP: usize = 1705245987; // 2025-01-14 16:26:27 UTC
pub const VECTOR_ALIGN: usize = 16;
pub const CACHE_LINE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryAddress(usize);

impl MemoryAddress {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub const fn as_usize(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn to_vector(&self) -> Vector3D<isize> {
        Vector3D::new(
            self.width as isize,
            self.height as isize,
            self.depth as isize,
        )
    }

    pub fn from_vector(vec: &Vector3D<isize>) -> Self {
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

#[derive(Debug)]
pub struct FluidMemory<T: 'static> {
    base: MemoryAddress,
    timestamp: AtomicUsize,
    ufo: UFO<T>,
    phantom_space: PhantomSpace<T>,
}

impl<T: 'static> FluidMemory<T> {
    pub fn new(base: MemoryAddress) -> Self {
        Self {
            base,
            timestamp: AtomicUsize::new(QUANTUM_TIMESTAMP),
            ufo: UFO::new(),
            phantom_space: PhantomSpace::new(),
        }
    }

    pub const fn get_base(&self) -> MemoryAddress {
        self.base
    }

    pub const fn base_addr(&self) -> usize {
        self.base.as_usize()
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    pub fn is_protected(&self) -> bool {
        self.ufo.is_protected()
    }

    pub fn get_position(&self) -> Vector3D<isize> {
        self.phantom_space.get_position()
    }

    pub fn set_position(&mut self, x: isize, y: isize, z: isize) {
        self.phantom_space.set_position(x, y, z);
        self.ufo.track();
    }
}

impl<T: Copy + 'static> FluidMemory<T> {
    pub unsafe fn read(&mut self, offset: usize) -> T {
        self.ufo.protect();
        self.phantom_space.decay_coherence();
        let addr = self.base.as_usize() + offset;
        let value = core::ptr::read_volatile(addr as *const T);
        self.timestamp.store(QUANTUM_TIMESTAMP, Ordering::SeqCst);
        value
    }

    pub unsafe fn write(&mut self, offset: usize, value: T) {
        self.ufo.protect();
        self.phantom_space.decay_coherence();
        let addr = self.base.as_usize() + offset;
        core::ptr::write_volatile(addr as *mut T, value);
        self.timestamp.store(QUANTUM_TIMESTAMP, Ordering::SeqCst);
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
    ufo: UFO<T>,
    phantom_space: PhantomSpace<T>,
}

impl<T: Copy + 'static> SpaceTime<T> {
    pub fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            memory: FluidMemory::new(MemoryAddress::new(base_addr)),
            size,
            offset,
            stride: core::mem::size_of::<T>(),
            dimensions: Dimensions::new(size, 1, 1),
            timestamp: AtomicUsize::new(QUANTUM_TIMESTAMP),
            ufo: UFO::new(),
            phantom_space: PhantomSpace::new(),
        }
    }

    // Add getters for all fields to fix "never read" warnings
    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn get_stride(&self) -> usize {
        self.stride
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    pub fn get_position(&self) -> Vector3D<isize> {
        self.phantom_space.get_position()
    }

    pub fn set_position(&mut self, x: isize, y: isize, z: isize) {
        self.phantom_space.set_position(x, y, z);
        self.memory.set_position(x, y, z);
        self.ufo.track();
    }

    pub fn is_protected(&self) -> bool {
        self.ufo.is_protected() && self.memory.is_protected()
    }

    pub fn get_coherence(&self) -> f64 {
        (self.phantom_space.get_coherence() +
        self.memory.phantom_space.get_coherence()) / 2.0
    }

    pub fn calculate_index(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.dimensions.width * self.dimensions.height +
        y * self.dimensions.width +
        x + self.offset) * self.stride
    }
}
