/// Space Configuration Implementation
/// Last Updated: 2025-01-14 21:33:40 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    Vector3D,
    helium::{Helium, HeliumOrdering},
    phantom::QuantumCell,
    constants::CURRENT_TIMESTAMP,
};

const QUANTUM_COHERENCE_THRESHOLD: f64 = 0.5;

#[derive(Debug)]
pub struct SpaceConfig {
    mesh_density: QuantumCell<Vector3D<usize>>,
    cell_size: QuantumCell<Vector3D<usize>>,
    timestamp: Helium<usize>,
    coherence: Helium<f64>,
}

impl Clone for SpaceConfig {
    fn clone(&self) -> Self {
        Self {
            mesh_density: QuantumCell::new(*self.mesh_density.get()),
            cell_size: QuantumCell::new(*self.cell_size.get()),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            coherence: Helium::new(1.0),
        }
    }
}

#[derive(Debug)]
pub struct SpaceMetadata {
    size: Helium<usize>,
    vector_space: QuantumCell<Vector3D<usize>>,
    config: SpaceConfig,
    timestamp: Helium<usize>,
    coherence: Helium<f64>,
}

impl Clone for SpaceMetadata {
    fn clone(&self) -> Self {
        Self {
            size: Helium::new(self.get_size()),
            vector_space: QuantumCell::new(*self.vector_space.get()),
            config: self.config.clone(),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            coherence: Helium::new(self.get_coherence()),
        }
    }
}

impl SpaceConfig {
    pub fn new(mesh_density: Vector3D<usize>, cell_size: Vector3D<usize>) -> Self {
        Self {
            mesh_density: QuantumCell::new(mesh_density),
            cell_size: QuantumCell::new(cell_size),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            coherence: Helium::new(1.0),
        }
    }

    pub fn get_mesh_density(&self) -> Vector3D<usize> {
        *self.mesh_density.get()
    }

    pub fn get_cell_size(&self) -> Vector3D<usize> {
        *self.cell_size.get()
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(HeliumOrdering::Relaxed)
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence.load(HeliumOrdering::Relaxed)
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_COHERENCE_THRESHOLD
    }

    pub fn update_mesh_density(&mut self, new_density: Vector3D<usize>) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        self.mesh_density.set(new_density);
        self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
        self.decay_coherence();
        Ok(())
    }

    pub fn update_cell_size(&mut self, new_size: Vector3D<usize>) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        self.cell_size.set(new_size);
        self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
        self.decay_coherence();
        Ok(())
    }

    fn decay_coherence(&self) {
        let current = self.coherence.load(HeliumOrdering::Acquire);
        self.coherence.store(current * 0.99, HeliumOrdering::Release);
    }
}

impl SpaceMetadata {
    pub fn new(size: usize) -> Self {
        Self {
            size: Helium::new(size),
            vector_space: QuantumCell::new(Vector3D::new(size, size, size)),
            config: SpaceConfig::new(
                Vector3D::new(16, 16, 16),
                                     Vector3D::new(4, 4, 4)
            ),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            coherence: Helium::new(1.0),
        }
    }

    pub fn get_size(&self) -> usize {
        self.size.load(HeliumOrdering::Relaxed)
    }

    pub fn get_vector_space(&self) -> Vector3D<usize> {
        *self.vector_space.get()
    }

    pub fn get_config(&self) -> &SpaceConfig {
        &self.config
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(HeliumOrdering::Relaxed)
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence.load(HeliumOrdering::Relaxed)
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_COHERENCE_THRESHOLD
    }

    pub fn update_size(&mut self, new_size: usize) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        self.size.store(new_size, HeliumOrdering::Release);
        self.vector_space.set(Vector3D::new(new_size, new_size, new_size));
        self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
        self.decay_coherence();
        Ok(())
    }

    fn decay_coherence(&self) {
        let current = self.coherence.load(HeliumOrdering::Acquire);
        self.coherence.store(current * 0.99, HeliumOrdering::Release);
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
        assert_eq!(config.get_timestamp(), CURRENT_TIMESTAMP);
        assert!(config.is_quantum_stable());
    }

    #[test]
    fn test_space_metadata() {
        let metadata = SpaceMetadata::new(0x1000);
        let cloned = metadata.clone();

        assert_eq!(metadata.get_size(), cloned.get_size());
        assert_eq!(metadata.get_vector_space(), cloned.get_vector_space());
        assert_eq!(metadata.get_timestamp(), CURRENT_TIMESTAMP);
        assert!(metadata.is_quantum_stable());
    }

    #[test]
    fn test_space_config_updates() {
        let mesh_density = Vector3D::new(16, 16, 16);
        let cell_size = Vector3D::new(4, 4, 4);
        let mut config = SpaceConfig::new(mesh_density, cell_size);

        let new_density = Vector3D::new(32, 32, 32);
        let initial_coherence = config.get_coherence();
        assert!(config.update_mesh_density(new_density).is_ok());

        assert_eq!(config.get_mesh_density(), new_density);
        assert!(config.get_coherence() < initial_coherence);
    }

    #[test]
    fn test_space_metadata_updates() {
        let mut metadata = SpaceMetadata::new(0x1000);
        let initial_coherence = metadata.get_coherence();

        assert!(metadata.update_size(0x2000).is_ok());
        assert_eq!(metadata.get_size(), 0x2000);
        assert!(metadata.get_coherence() < initial_coherence);
    }

    #[test]
    fn test_quantum_stability() {
        let mut config = SpaceConfig::new(
            Vector3D::new(16, 16, 16),
                                          Vector3D::new(4, 4, 4)
        );

        // Force decoherence
        for _ in 0..100 {
            let _ = config.update_mesh_density(Vector3D::new(32, 32, 32));
        }

        assert!(!config.is_quantum_stable());
        assert!(config.update_mesh_density(Vector3D::new(64, 64, 64)).is_err());
    }
}
