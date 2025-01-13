// lib/unstable_matter/src/mesh.rs
#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::align::Align;
use crate::vector::Vector3D;
use crate::ufo::UFO;

#[derive(Debug)]
pub struct SpaceTime<T> {
    memory: Align,
    dimensions: Vector3D,
    stride: usize,
    offset: usize,
    size: usize,
    timestamp: AtomicUsize,
    _ufo: UFO,
}

impl<T> SpaceTime<T> {
    pub fn new(origin: usize, size: usize, offset: usize) -> Self {
        let dimensions = Vector3D::new(size as isize, 1, 1);
        let stride = core::mem::size_of::<T>();
        let base = MemoryAddress::new(origin);

        Self {
            memory: Align::new(base, size * stride),
            dimensions,
            stride,
            offset,
            size,
            timestamp: AtomicUsize::new(1705109638), // 2025-01-13 02:47:18 UTC
            _ufo: UFO::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn dimensions(&self) -> Vector3D {
        self.dimensions
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn timestamp(&self) -> &AtomicUsize {
        &self.timestamp
    }

    pub fn read_at(&self, index: usize) -> Result<T, &'static str> {
        if index >= self.size {
            return Err("Index out of bounds");
        }

        let addr = self.memory.read_at(index * self.stride + self.offset)
        .ok_or("Memory read failed")?;

        // Safety: We trust that the memory at this address contains a valid T
        Ok(unsafe { *(addr.as_usize() as *const T) })
    }

    pub fn write_at(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index out of bounds");
        }

        let addr = MemoryAddress::new(
            self.memory.get_base().as_usize() + index * self.stride + self.offset
        );

        if !self.memory.write_at(index * self.stride + self.offset, addr) {
            return Err("Memory write failed");
        }

        // Safety: We trust that the memory at this address can hold a T
        unsafe {
            *(addr.as_usize() as *mut T) = value;
        }

        self.timestamp.store(1705109638, Ordering::SeqCst); // 2025-01-13 02:47:18 UTC
        Ok(())
    }
}

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
            timestamp: AtomicUsize::new(1705112019), // 2025-01-13 03:13:39 UTC
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
