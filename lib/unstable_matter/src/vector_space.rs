// lib/unstable_matter/src/lib.rs
#![no_std]

/// Unstable Matter Core Library
/// Last Updated: 2025-01-13 03:58:20 UTC
/// Author: isdood
/// Current User: isdood

pub mod mesh;
pub mod morph_tracker;
pub mod space_config;
pub mod tracked_ufo;
mod vector_space;

pub use self::{
    mesh::{MeshCell, CellState},
    vector_space::VectorSpace,
    space_config::{SpaceConfig, SpaceMetadata},
};

// Re-export vector types
#[derive(Debug, Clone, Copy)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UFOState {
    Flying,
    Hovering,
    Landed,
}

// lib/unstable_matter/src/vector_space.rs
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    space_config::SpaceMetadata,
    tracked_ufo::TrackedUFO,
    morph_tracker::MorphTracker,
    UFOState,
};

#[derive(Debug)]
pub struct VectorSpace {
    origin: AtomicUsize,
    ufo_state: TrackedUFO,
    metadata: SpaceMetadata,
    morph_tracker: MorphTracker,
    state: UFOState,
    timestamp: AtomicUsize,
}

impl Clone for VectorSpace {
    fn clone(&self) -> Self {
        Self {
            origin: AtomicUsize::new(self.get_origin()),
            ufo_state: self.ufo_state.clone(),
            metadata: self.metadata.clone(),
            morph_tracker: self.morph_tracker.clone(),
            state: self.state,
            timestamp: AtomicUsize::new(1705114700), // 2025-01-13 03:58:20 UTC
        }
    }
}

impl VectorSpace {
    pub fn new(origin: usize, metadata: SpaceMetadata) -> Self {
        Self {
            origin: AtomicUsize::new(origin),
            ufo_state: TrackedUFO::new(origin, metadata.get_size()),
            metadata,
            morph_tracker: MorphTracker::new(),
            state: UFOState::Flying,
            timestamp: AtomicUsize::new(1705114700), // 2025-01-13 03:58:20 UTC
        }
    }

    pub fn get_origin(&self) -> usize {
        self.origin.load(Ordering::SeqCst)
    }

    pub fn get_ufo_state(&self) -> &TrackedUFO {
        &self.ufo_state
    }

    pub fn get_metadata(&self) -> &SpaceMetadata {
        &self.metadata
    }

    pub fn get_morph_tracker(&self) -> &MorphTracker {
        &self.morph_tracker
    }

    pub fn get_state(&self) -> UFOState {
        self.state
    }

    pub fn update_origin(&self, new_origin: usize) {
        self.origin.store(new_origin, Ordering::SeqCst);
        self.ufo_state.update_origin(new_origin);
        self.timestamp.store(1705114700, Ordering::SeqCst); // 2025-01-13 03:58:20 UTC
    }

    pub fn transition_state(&mut self, new_state: UFOState) {
        self.state = new_state;
        self.timestamp.store(1705114700, Ordering::SeqCst); // 2025-01-13 03:58:20 UTC
    }

    pub fn is_valid_address(&self, addr: usize) -> bool {
        self.ufo_state.is_within_bounds(addr)
    }
}

// lib/unstable_matter/src/mesh.rs
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::Vector3D;

#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Empty,
    Occupied,
    Reserved,
}

#[derive(Debug)]
pub struct MeshCell<T> {
    data: T,
    state: CellState,
    position: Vector3D<usize>,
    timestamp: AtomicUsize,
}

impl<T: Clone> Clone for MeshCell<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            state: self.state,
            position: self.position,
            timestamp: AtomicUsize::new(1705114700), // 2025-01-13 03:58:20 UTC
        }
    }
}

impl<T> MeshCell<T> {
    pub fn new(data: T, position: Vector3D<usize>) -> Self {
        Self {
            data,
            state: CellState::Empty,
            position,
            timestamp: AtomicUsize::new(1705114700), // 2025-01-13 03:58:20 UTC
        }
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn get_state(&self) -> CellState {
        self.state
    }

    pub fn get_position(&self) -> Vector3D<usize> {
        self.position
    }

    pub fn set_state(&mut self, new_state: CellState) {
        self.state = new_state;
        self.timestamp.store(1705114700, Ordering::SeqCst); // 2025-01-13 03:58:20 UTC
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_cell_clone() {
        let pos = Vector3D::new(1, 2, 3);
        let cell1 = MeshCell::new(42u32, pos);
        let cell2 = cell1.clone();

        assert_eq!(*cell1.get_data(), *cell2.get_data());
        assert_eq!(cell1.get_position().x, cell2.get_position().x);
        assert_eq!(cell1.get_state() as u8, cell2.get_state() as u8);
    }
}
