// lib/unstable_matter/src/space_config.rs
/// Space Configuration Implementation
/// Last Updated: 2025-01-13 04:26:27 UTC
/// Author: isdood
/// Current User: isdood

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::Vector3D;

#[derive(Debug)]
pub struct SpaceConfig {
    mesh_density: Vector3D<usize>,
    cell_size: Vector3D<usize>,
    timestamp: AtomicUsize,
}

impl Clone for SpaceConfig {
    fn clone(&self) -> Self {
        Self {
            mesh_density: self.mesh_density,
            cell_size: self.cell_size,
            timestamp: AtomicUsize::new(1705116387), // 2025-01-13 04:26:27 UTC
        }
    }
}

#[derive(Debug)]
pub struct SpaceMetadata {
    size: AtomicUsize,
    vector_space: Vector3D<usize>,
    config: SpaceConfig,
    timestamp: AtomicUsize,
}

impl Clone for SpaceMetadata {
    fn clone(&self) -> Self {
        Self {
            size: AtomicUsize::new(self.get_size()),
            vector_space: self.vector_space,
            config: self.config.clone(),
            timestamp: AtomicUsize::new(1705116387), // 2025-01-13 04:26:27 UTC
        }
    }
}

impl SpaceConfig {
    pub fn new(mesh_density: Vector3D<usize>, cell_size: Vector3D<usize>) -> Self {
        Self {
            mesh_density,
            cell_size,
            timestamp: AtomicUsize::new(1705116387), // 2025-01-13 04:26:27 UTC
        }
    }

    pub fn get_mesh_density(&self) -> Vector3D<usize> {
        self.mesh_density
    }

    pub fn get_cell_size(&self) -> Vector3D<usize> {
        self.cell_size
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    pub fn update_mesh_density(&mut self, new_density: Vector3D<usize>) {
        self.mesh_density = new_density;
        self.timestamp.store(1705116387, Ordering::SeqCst); // 2025-01-13 04:26:27 UTC
    }

    pub fn update_cell_size(&mut self, new_size: Vector3D<usize>) {
        self.cell_size = new_size;
        self.timestamp.store(1705116387, Ordering::SeqCst); // 2025-01-13 04:26:27 UTC
    }
}

impl SpaceMetadata {
    pub fn new(size: usize) -> Self {
        Self {
            size: AtomicUsize::new(size),
            vector_space: Vector3D::new(size, size, size),
            config: SpaceConfig::new(
                Vector3D::new(16, 16, 16),
                                     Vector3D::new(4, 4, 4)
            ),
            timestamp: AtomicUsize::new(1705116387), // 2025-01-13 04:26:27 UTC
        }
    }

    pub fn get_size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }

    pub fn get_vector_space(&self) -> Vector3D<usize> {
        self.vector_space
    }

    pub fn get_config(&self) -> &SpaceConfig {
        &self.config
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    pub fn update_size(&mut self, new_size: usize) {
        self.size.store(new_size, Ordering::SeqCst);
        self.vector_space = Vector3D::new(new_size, new_size, new_size);
        self.timestamp.store(1705116387, Ordering::SeqCst); // 2025-01-13 04:26:27 UTC
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_config() {
        let mesh_density = Vector3D::new(16, 16, 16);
        let cell_size = Vector3D::new(4, 4, 4);
        let config = SpaceConfig::new(mesh_density, cell_size);
        let cloned = config.clone();

        assert_eq!(config.get_mesh_density(), cloned.get_mesh_density());
        assert_eq!(config.get_cell_size(), cloned.get_cell_size());
        assert!(config.get_timestamp() > 0);
    }

    #[test]
    fn test_space_metadata() {
        let metadata = SpaceMetadata::new(0x1000);
        let cloned = metadata.clone();

        assert_eq!(metadata.get_size(), cloned.get_size());
        assert_eq!(metadata.get_vector_space(), cloned.get_vector_space());
        assert!(metadata.get_timestamp() > 0);
    }

    #[test]
    fn test_space_config_updates() {
        let mesh_density = Vector3D::new(16, 16, 16);
        let cell_size = Vector3D::new(4, 4, 4);
        let mut config = SpaceConfig::new(mesh_density, cell_size);

        let new_density = Vector3D::new(32, 32, 32);
        let initial_timestamp = config.get_timestamp();
        config.update_mesh_density(new_density);

        assert_eq!(config.get_mesh_density(), new_density);
        assert!(config.get_timestamp() >= initial_timestamp);
    }

    #[test]
    fn test_space_metadata_updates() {
        let mut metadata = SpaceMetadata::new(0x1000);
        let initial_timestamp = metadata.get_timestamp();

        metadata.update_size(0x2000);
        assert_eq!(metadata.get_size(), 0x2000);
        assert!(metadata.get_timestamp() >= initial_timestamp);
    }
}
