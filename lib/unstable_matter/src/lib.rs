#![no_std]

/// UnstableMatter Core Library
/// Last Updated: 2025-01-14 01:43:19 UTC
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
pub use {
    vector::{Vector3D, FloatVector3D, IntVector3D},
    align::{Alignment, AlignedRegion, VECTOR_ALIGN, CACHE_LINE},
    helium::{Helium, HeliumSize},
    valence::{ValenceOrder, compare_vectors},
    mesh_clock::{MeshClock, MeshCell, CellState},
    ufo::{UFO, Protected, MemoryTrace},
    phantom::PhantomSpace,
};

// System Constants
pub const QUANTUM_TIMESTAMP: usize = 1705193599; // 2025-01-14 01:43:19 UTC

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

#[derive(Debug)]
pub struct FluidMemory<T: 'static> {
    base: MemoryAddress,
    timestamp: AtomicUsize,
    ufo: UFO<T>,
    phantom_space: PhantomSpace<T>,
}

impl<T: 'static> FluidMemory<T> {
    pub const fn new(base: MemoryAddress) -> Self {
        Self {
            base,
            timestamp: AtomicUsize::new(1705192646), // 2025-01-14 01:37:26 UTC
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
        self.timestamp.store(1705192646, Ordering::SeqCst);
        value
    }

    pub unsafe fn write(&mut self, offset: usize, value: T) {
        self.ufo.protect();
        self.phantom_space.decay_coherence();
        let addr = self.base.as_usize() + offset;
        core::ptr::write_volatile(addr as *mut T, value);
        self.timestamp.store(1705192646, Ordering::SeqCst);
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
    pub const fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            memory: FluidMemory::new(MemoryAddress::new(base_addr)),
            size,
            offset,
            stride: core::mem::size_of::<T>(),
            dimensions: Dimensions::new(size, 1, 1),
            timestamp: AtomicUsize::new(1705192646), // 2025-01-14 01:37:26 UTC
            ufo: UFO::new(),
            phantom_space: PhantomSpace::new(),
        }
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
}
