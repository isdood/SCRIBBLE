/// Unstable Matter Library
/// Last Updated: 2025-01-15 03:44:13 UTC
/// Author: isdood
/// Current User: isdood

pub mod constants;
pub mod glitch;
pub mod phantom;
pub mod mesh;
pub mod ufo;
pub mod vector;
pub mod helium;
pub mod grav;
pub mod blackhole;
pub mod wormhole;
pub mod zeronaut;
pub mod scribe;
pub mod unstable;

use crate::mesh::MeshDimensions;

// Re-exports
pub use constants::*;
pub use glitch::WormholeGlitch;  // Changed from WormholeError to WormholeGlitch
pub use vector::Vector3D;
pub use phantom::{PhantomSpace, Quantum, QuantumCell, Horizon};
pub use mesh::MeshCell;
pub use ufo::{UFO, Protected};
pub use helium::{Helium, HeliumOrdering};
pub use grav::{GravityField, GravityFieldRef};
pub use blackhole::BlackHole;
pub use wormhole::Wormhole;
pub use zeronaut::Zeronaut;
pub use scribe::{Scribe, ScribePrecision, QuantumString};
pub use unstable::{UnstableDescriptor, QuantumState};

#[derive(Debug)]
pub struct SpaceTimeMemory {
    phantom_space: PhantomSpace,
    ufo: UFO,
    dimensions: MeshDimensions,
    timestamp: Helium<usize>,
    quantum_descriptor: UnstableDescriptor,
}

impl SpaceTimeMemory {
    pub fn new(dimensions: MeshDimensions) -> Self {
        Self {
            phantom_space: PhantomSpace::new(),
            ufo: UFO::new(),
            dimensions,
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            quantum_descriptor: UnstableDescriptor::new(),
        }
    }

    pub fn is_protected(&self) -> bool {
        self.ufo.is_protected() && self.quantum_descriptor.is_stable()
    }

    pub fn track(&mut self) {
        self.ufo.track();
    }

    pub fn get_quantum_state(&self) -> QuantumState {
        *self.quantum_descriptor.state.get()
    }
}

#[derive(Debug)]
pub struct SpaceTime {
    memory: SpaceTimeMemory,
    mesh: MeshDimensions,
    black_holes: Vec<BlackHole>,
    dimensions: Vector3D<usize>,
    timestamp: Helium<usize>,
}

impl SpaceTime {
    pub fn new(dimensions: Vector3D<usize>) -> Self {
        let mesh = MeshDimensions {
            width: dimensions.x(),
            height: dimensions.y(),
            depth: dimensions.z(),
        };

        Self {
            memory: SpaceTimeMemory::new(mesh.clone()),
            mesh,
            black_holes: Vec::new(),
            dimensions,
            timestamp: Helium::new(CURRENT_TIMESTAMP),
        }
    }

    pub fn track(&mut self) {
        self.memory.track();
    }

    pub fn is_protected(&self) -> bool {
        self.memory.is_protected()
    }

    pub fn get_position(&self) -> Vector3D<f64> {
        *self.memory.quantum_descriptor.position()
    }

    pub fn set_position(&mut self, position: Vector3D<f64>) {
        self.memory.quantum_descriptor.set_position(position);
        self.memory.phantom_space.set_position(position.x(), position.y(), position.z());
    }

    pub fn get_dimensions(&self) -> &Vector3D<usize> {
        &self.dimensions
    }

    pub fn get_mesh_dimensions(&self) -> &MeshDimensions {
        &self.mesh
    }

    pub fn get_timestamp(&self) -> Result<usize, &'static str> {
        self.timestamp.load(&HeliumOrdering::Quantum)
    }

    pub fn update_timestamp(&self) -> Result<(), &'static str> {
        self.timestamp.store(CURRENT_TIMESTAMP, &HeliumOrdering::Quantum)
    }

    pub fn get_coherence(&self) -> f64 {
        self.memory.quantum_descriptor.coherence()
    }

    pub fn calculate_index(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        if x >= self.dimensions.x() || y >= self.dimensions.y() || z >= self.dimensions.z() {
            return None;
        }
        Some(x + y * self.dimensions.x() + z * self.dimensions.x() * self.dimensions.y())
    }

    pub fn get_uncertainty(&self) -> &Vector3D<f64> {
        self.memory.quantum_descriptor.uncertainty()
    }
}

impl Quantum for SpaceTime {
    fn is_quantum_stable(&self) -> bool {
        self.memory.phantom_space.is_quantum_stable() &&
        self.memory.quantum_descriptor.is_stable() &&
        self.is_protected() &&
        self.timestamp.is_quantum_stable()
    }

    fn get_coherence(&self) -> f64 {
        let space_coherence = self.memory.phantom_space.get_coherence();
        let quantum_coherence = self.memory.quantum_descriptor.coherence();
        let time_coherence = self.timestamp.get_coherence();
        (space_coherence + quantum_coherence + time_coherence) / 3.0
    }

    fn decay_coherence(&self) {
        self.memory.phantom_space.decay_coherence();
        self.timestamp.decay_coherence();
        // Quantum descriptor handles its own decay
    }

    fn reset_coherence(&self) {
        self.memory.phantom_space.reset_coherence();
        self.timestamp.reset_coherence();
        // Reset quantum descriptor state
        self.memory.quantum_descriptor.reset();
    }
}

impl Scribe for SpaceTime {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("SpaceTime[");
        output.push_str("pos=");
        self.memory.quantum_descriptor.scribe(precision, output);
        output.push_str(", c=");
        output.push_f64(self.get_coherence(), 6);
        output.push_char(']');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacetime_creation() {
        let dimensions = Vector3D::new(10, 10, 10);
        let spacetime = SpaceTime::new(dimensions);
        assert_eq!(spacetime.get_dimensions(), &dimensions);
    }

    #[test]
    fn test_spacetime_protection() {
        let mut spacetime = SpaceTime::new(Vector3D::new(5, 5, 5));
        assert!(spacetime.is_protected());
        spacetime.track();
        assert!(spacetime.is_protected());
    }

    #[test]
    fn test_spacetime_quantum_stability() {
        let spacetime = SpaceTime::new(Vector3D::new(5, 5, 5));
        assert!(spacetime.is_quantum_stable());
        assert!(spacetime.get_coherence() > 0.0);
    }

    #[test]
    fn test_spacetime_position() {
        let mut spacetime = SpaceTime::new(Vector3D::new(5, 5, 5));
        let position = Vector3D::new(1.0, 2.0, 3.0);
        spacetime.set_position(position.clone());
        assert_eq!(spacetime.get_position(), position);
    }

    #[test]
    fn test_uncertainty() {
        let spacetime = SpaceTime::new(Vector3D::new(5, 5, 5));
        let uncertainty = spacetime.get_uncertainty();
        assert!(uncertainty.magnitude() >= PLANCK_LENGTH);
    }

    #[test]
    fn test_quantum_scribing() {
        let spacetime = SpaceTime::new(Vector3D::new(5, 5, 5));
        let mut output = QuantumString::new();
        spacetime.scribe(ScribePrecision::Standard, &mut output);
        assert!(output.as_str().starts_with("SpaceTime["));
    }
}
