// vector_space.rs

use std::sync::atomic::AtomicUsize;
use crate::morph_tracker::{MorphTracker, FileType};
use crate::ufo_states::UFOState;
use crate::mesh::{MeshCell, SpaceTime};
use crate::space_config::{SpaceConfig, SpaceMetadata};
use crate::tracked_ufo::TrackedUFO;
use crate::vector::Vector3D;

pub struct VectorSpace {
    pub origin: usize,
    pub mesh: SpaceTime<MeshCell>,
    pub config: SpaceConfig,
    pub ufo_state: TrackedUFO,
    pub metadata: SpaceMetadata,
    pub morph_tracker: MorphTracker,
    pub state: UFOState,
}

impl Default for VectorSpace {
    fn default() -> Self {
        Self::new(0, 1024)
    }
}

impl VectorSpace {
    pub fn new(origin: usize, size: usize) -> Self {
        let config = SpaceConfig {
            dimensions: Vector3D::new(size as isize, size as isize, size as isize),
            cell_size: 256,
            cells: Vector3D::new(16, 16, 16),
        };

        let mesh_size = (config.cells.x * config.cells.y * config.cells.z) as usize;
        let current_time = 1705108339; // 2025-01-13 02:32:19 UTC

        let metadata = SpaceMetadata {
            creation_time: AtomicUsize::new(current_time),
            last_modified: AtomicUsize::new(current_time),
            creator: "isdood",
            last_modifier: "isdood",
        };

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
        let mesh_size = (self.config.cells.x * self.config.cells.y * self.config.cells.z) as usize;
        for i in 0..mesh_size {
            let cell = MeshCell::default();
            self.mesh.write_at(i, cell)?;
        }
        Ok(())
    }

    pub fn get_cell(&self, index: usize) -> Result<MeshCell, &'static str> {
        self.mesh.read_at(index)
    }

    pub fn set_cell(&mut self, index: usize, cell: MeshCell) -> Result<(), &'static str> {
        self.mesh.write_at(index, cell)
    }

    pub fn land(mut self) -> Result<Self, &'static str> {
        self.ufo_state.track();
        self.state = self.state.transition_to_landed()?;
        Ok(self)
    }

    pub fn hover(mut self) -> Result<Self, &'static str> {
        self.ufo_state.track();
        self.state = self.state.transition_to_hovering()?;
        Ok(self)
    }

    pub fn take_off(mut self) -> Result<Self, &'static str> {
        self.ufo_state.track();
        self.state = self.state.transition_to_flying()?;
        Ok(self)
    }

    pub fn get_state(&self) -> &UFOState {
        &self.state
    }

    pub fn init_morph_types(&mut self) -> Result<(), &'static str> {
        self.morph_tracker.register_file_type(FileType::Rust)
    }

    pub fn get_mesh_size(&self) -> usize {
        (self.config.cells.x * self.config.cells.y * self.config.cells.z) as usize
    }

    pub fn update_metadata(&mut self) {
        self.metadata.last_modified.store(1705108339, Ordering::SeqCst); // 2025-01-13 02:32:19 UTC
        self.metadata.last_modifier = "isdood";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_space_creation() {
        let space = VectorSpace::new(0, 1024);
        assert_eq!(space.origin, 0);
        assert_eq!(space.get_mesh_size(), 4096);
    }

    #[test]
    fn test_state_transitions() {
        let space = VectorSpace::new(0, 1024);
        let landed_space = space.land().unwrap();
        assert!(matches!(landed_space.state, UFOState::Landed));
    }
}
