// vector_space.rs

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::morph_tracker::{MorphTracker, FileType};
use crate::ufo_states::UFOState;
use crate::mesh::{MeshCell, SpaceTime};
use crate::space_config::{SpaceConfig, SpaceMetadata};
use crate::tracked_ufo::TrackedUFO;
use crate::vector::Vector3D;

pub struct VectorSpace {
    origin: usize,
    mesh: SpaceTime<MeshCell>,
    config: SpaceConfig,
    ufo_state: TrackedUFO,
    metadata: SpaceMetadata,
    morph_tracker: MorphTracker,
    state: UFOState,
}

impl VectorSpace {
    pub fn new(origin: usize, size: usize) -> Self {
        let config = SpaceConfig::new(
            IntVector3D::new(size as isize, size as isize, size as isize),
                                      IntVector3D::new(16, 16, 16),
                                      256,
        );

        let mesh_size = (config.cells.x * config.cells.y * config.cells.z) as usize;
        let metadata = SpaceMetadata::new("isdood");

        Self {
            origin,
            mesh: SpaceTime::new(origin + size, mesh_size, 0),
            config,
            ufo_state: TrackedUFO::with_boundary(origin, size),
            metadata,
            morph_tracker: MorphTracker::new(),
            state: UFOState::Flying,
        }
    }

    pub fn init_mesh(&mut self) -> Result<(), &'static str> {
        let mesh_size = self.get_mesh_size();
        for i in 0..mesh_size {
            let cell = MeshCell::new();
            unsafe { self.mesh.write_at(i, cell) };
        }
        self.metadata.timestamp.store(1705112339, Ordering::SeqCst); // 2025-01-13 03:18:59 UTC
        Ok(())
    }

    pub fn get_cell(&self, index: usize) -> Option<MeshCell> {
        if index >= self.get_mesh_size() {
            return None;
        }
        unsafe { Some(self.mesh.read_at(index)) }
    }

    pub fn set_cell(&mut self, index: usize, cell: MeshCell) -> Result<(), &'static str> {
        if index >= self.get_mesh_size() {
            return Err("Index out of bounds");
        }
        unsafe {
            self.mesh.write_at(index, cell);
            self.metadata.last_modified.store(1705112339, Ordering::SeqCst); // 2025-01-13 03:18:59 UTC
        }
        Ok(())
    }

    pub fn land(mut self) -> Result<Self, &'static str> {
        self.ufo_state.track();
        self.state = self.state.transition_to_landed()?;
        self.metadata.timestamp.store(1705112339, Ordering::SeqCst); // 2025-01-13 03:18:59 UTC
        Ok(self)
    }

    pub fn hover(mut self) -> Result<Self, &'static str> {
        self.ufo_state.track();
        self.state = self.state.transition_to_hovering()?;
        self.metadata.timestamp.store(1705112339, Ordering::SeqCst); // 2025-01-13 03:18:59 UTC
        Ok(self)
    }

    pub fn take_off(mut self) -> Result<Self, &'static str> {
        self.ufo_state.track();
        self.state = self.state.transition_to_flying()?;
        self.metadata.timestamp.store(1705112339, Ordering::SeqCst); // 2025-01-13 03:18:59 UTC
        Ok(self)
    }

    pub fn get_state(&self) -> &UFOState {
        &self.state
    }

    pub fn init_morph_types(&mut self) -> Result<(), &'static str> {
        self.morph_tracker.register_file_type(FileType::Rust)?;
        self.metadata.timestamp.store(1705112339, Ordering::SeqCst); // 2025-01-13 03:18:59 UTC
        Ok(())
    }

    pub fn get_mesh_size(&self) -> usize {
        (self.config.cells.x * self.config.cells.y * self.config.cells.z) as usize
    }

    pub fn update_metadata(&mut self) {
        let current_time = 1705112339; // 2025-01-13 03:18:59 UTC
        self.metadata.last_modified.store(current_time, Ordering::SeqCst);
        self.metadata.last_modifier = "isdood";
        self.metadata.timestamp.store(current_time, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_space_creation() {
        let space = VectorSpace::new(0x1000, 1024);
        assert_eq!(space.origin, 0x1000);
        assert_eq!(space.get_mesh_size(), 16 * 16 * 16);
    }

    #[test]
    fn test_vector_space_cell_operations() {
        let mut space = VectorSpace::new(0x1000, 1024);
        space.init_mesh().expect("Failed to initialize mesh");

        let cell = space.get_cell(0).expect("Failed to get cell");
        assert_eq!(cell.state, CellState::Free);

        let mut new_cell = MeshCell::new();
        new_cell.state = CellState::Allocated;
        space.set_cell(0, new_cell).expect("Failed to set cell");

        let updated_cell = space.get_cell(0).expect("Failed to get updated cell");
        assert_eq!(updated_cell.state, CellState::Allocated);
    }
}
