/// Quantum Matter Library
/// Last Updated: 2025-01-16 04:15:39 UTC
/// Author: isdood
/// Current User: isdood

// Core traits and types
pub mod quantum;
pub mod scribe;

// Core modules
pub mod align;
pub mod vector;
pub mod unstable;
pub mod mesh;
pub mod ufo;
pub mod blackhole;
pub mod wormhole;
pub mod zeronaut;
pub mod horizon;
pub mod phantom;
pub mod grav;
pub mod helium;
pub mod glitch;
pub mod mesh_clock;
pub mod meshmath;  // Added meshmath module

// Constants
pub mod constants;

// Re-exports
pub use quantum::Quantum;
pub use scribe::{Scribe, ScribePrecision, QuantumString};
pub use vector::{Vector3D, Vector4D};
pub use unstable::{UnstableDescriptor, QuantumState};
pub use mesh::{MeshCell, MeshDimensions};
pub use ufo::{UFO, Protected};
pub use blackhole::BlackHole;
pub use wormhole::{Wormhole, WormholeError};
pub use zeronaut::Zeronaut;
pub use horizon::Horizon;
pub use phantom::{PhantomSpace, QuantumCell};
pub use grav::{GravityField, GravityFieldRef};
pub use helium::{Helium, HeliumOrdering};
pub use constants::*;
pub use glitch::WormholeGlitch;
pub use meshmath::MeshMath;  // Export MeshMath

/// SpaceTime Memory System
#[derive(Debug)]
pub struct SpaceTimeMemory {
    phantom_space: PhantomSpace,
    ufo: UFO,
    _dimensions: MeshDimensions,
    _timestamp: Helium<usize>,
    quantum_descriptor: UnstableDescriptor,
}

impl SpaceTimeMemory {
    pub fn new(dimensions: MeshDimensions) -> Self {
        Self {
            phantom_space: PhantomSpace::new(),
            ufo: UFO::new(),
            _dimensions: dimensions,
            _timestamp: Helium::new(CURRENT_TIMESTAMP),
            quantum_descriptor: UnstableDescriptor::new(),
        }
    }

    pub fn is_protected(&self) -> bool {
        self.ufo.protect().is_ok() && self.quantum_descriptor.is_stable()
    }

    pub fn track(&mut self) -> Result<(), &'static str> {
        self.ufo.track()
    }

    pub fn get_quantum_state(&self) -> QuantumState {
        self.quantum_descriptor.get_state()
    }
}

/// SpaceTime Quantum System
#[derive(Debug)]
pub struct SpaceTime {
    memory: SpaceTimeMemory,
    mesh: MeshDimensions,
    _black_holes: Vec<BlackHole>,
    dimensions: Vector3D<usize>,
    timestamp: Helium<usize>,
}

impl SpaceTime {
    pub fn new(dimensions: Vector3D<usize>) -> Self {
        let mesh = MeshDimensions::new(dimensions.clone());

        Self {
            memory: SpaceTimeMemory::new(mesh.clone()),
            mesh,
            _black_holes: Vec::new(),
            dimensions,
            timestamp: Helium::new(CURRENT_TIMESTAMP),
        }
    }

    pub fn track(&mut self) -> Result<(), &'static str> {
        self.memory.track()
    }

    pub fn is_protected(&self) -> bool {
        self.memory.is_protected()
    }

    pub fn get_position(&self) -> Vector3D<f64> {
        self.memory.quantum_descriptor.position()
    }

    pub fn set_position(&mut self, position: Vector3D<f64>) {
        self.memory.quantum_descriptor.set_position(position.clone());
        self.memory.phantom_space.set_position(
            position.x(),
                                               position.y(),
                                               position.z()
        );
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

    pub fn get_uncertainty(&self) -> Vector3D<f64> {
        self.memory.quantum_descriptor.uncertainty()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.memory.quantum_descriptor.is_stable()
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
        let spacetime = SpaceTime::new(dimensions.clone());
        assert_eq!(spacetime.get_dimensions(), &dimensions);
    }

    #[test]
    fn test_spacetime_protection() {
        let mut spacetime = SpaceTime::new(Vector3D::new(5, 5, 5));
        assert!(spacetime.is_protected());
        assert!(spacetime.track().is_ok());
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
