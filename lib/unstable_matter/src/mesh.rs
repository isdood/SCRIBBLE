// lib/unstable_matter/src/mesh.rs
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    MemoryAddress,
    vector::IntVector3D,
    ufo::UFO,
};

#[derive(Debug)]
pub struct MeshCell<T: 'static> {
    pub state: CellState,
    pub timestamp: AtomicUsize,
    _ufo: UFO<T>,
}

impl<T: 'static> MeshCell<T> {
    pub const fn new() -> Self {
        Self {
            state: CellState::Free,
            timestamp: AtomicUsize::new(1705112617), // 2025-01-13 03:23:37 UTC
            _ufo: UFO::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CellState {
    Free,
    Allocated,
    Reserved,
}

#[derive(Debug)]
pub struct SpaceTime<T: 'static> {
    base: MemoryAddress,
    dimensions: IntVector3D,
    timestamp: AtomicUsize,
    _ufo: UFO<T>,
}

impl<T: 'static> SpaceTime<T> {
    pub const fn new(origin: usize, size: usize, _offset: usize) -> Self {
        Self {
            base: MemoryAddress::new(origin),
            dimensions: IntVector3D::new(size as isize, size as isize, size as isize),
            timestamp: AtomicUsize::new(1705113009), // 2025-01-13 03:26:49 UTC
            _ufo: UFO::new(),
        }
    }

    pub fn dimensions(&self) -> IntVector3D {
        self.dimensions
    }

    pub unsafe fn read_at(&self, index: usize) -> T {
        let addr = self.base.as_usize() + index;
        core::ptr::read_volatile(addr as *const T)
    }

    pub unsafe fn write_at(&mut self, index: usize, value: T) {
        let addr = self.base.as_usize() + index;
        core::ptr::write_volatile(addr as *mut T, value);
        self.timestamp.store(1705112617, Ordering::SeqCst); // 2025-01-13 03:23:37 UTC
    }
}
