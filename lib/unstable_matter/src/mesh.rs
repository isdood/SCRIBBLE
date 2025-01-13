use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone)]
pub struct MeshCell {
    pub state: CellState,
    pub timestamp: AtomicUsize,
}

#[derive(Debug, Clone)]
pub enum CellState {
    Free,
    Allocated,
    Reserved,
}

impl Default for MeshCell {
    fn default() -> Self {
        Self {
            state: CellState::Free,
            timestamp: AtomicUsize::new(1705108505), // 2025-01-13 02:35:05 UTC
        }
    }
}

pub struct SpaceTime<T> {
    cells: Vec<T>,
    size: usize,
}

impl<T: Clone> SpaceTime<T> {
    pub fn new(origin: usize, size: usize, default: usize) -> Self {
        Self {
            cells: Vec::with_capacity(size),
            size,
        }
    }

    pub fn read_at(&self, index: usize) -> Result<T, &'static str> {
        self.cells.get(index).cloned().ok_or("Index out of bounds")
    }

    pub fn write_at(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index out of bounds");
        }
        if index >= self.cells.len() {
            self.cells.resize_with(index + 1, || value.clone());
        } else {
            self.cells[index] = value;
        }
        Ok(())
    }
}
