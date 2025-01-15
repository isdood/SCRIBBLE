/// Quantum Mesh Cell Implementation
/// Last Updated: 2025-01-15 01:21:42 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::{CURRENT_TIMESTAMP, MESH_COHERENCE_THRESHOLD, GRAVITATIONAL_THRESHOLD},
    grav::{GravityField, GravityFieldRef},
    helium::{Helium, HeliumOrdering},
    phantom::QuantumCell,
    ufo::UFO,
    Vector3D,
    wormhole::{ProtectedWormhole, WormholeError},
    blackhole::BlackHole,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

#[derive(Debug, Clone)]
pub struct MeshDimensions {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl MeshDimensions {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn from_vector(vec: Vector3D<usize>) -> Self {
        Self {
            width: vec.x(),
            height: vec.y(),
            depth: vec.z(),
        }
    }

    pub fn to_vector(&self) -> Vector3D<usize> {
        Vector3D::new(self.width, self.height, self.depth)
    }

    pub fn volume(&self) -> usize {
        self.width * self.height * self.depth
    }
}

impl Scribe for MeshDimensions {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("Mesh[");
        output.push_str(&format!("{}x{}x{}", self.width, self.height, self.depth));
        output.push_char(']');
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
    Free,
    Absorbed,
    WormholeConnected,
    QuantumUncertain,
}

pub struct MeshCell<T: 'static> {
    state: QuantumCell<CellState>,
    position: QuantumCell<Vector3D<f64>>,
    mass: Helium<f64>,
    timestamp: Helium<usize>,
    coherence: QuantumCell<f64>,
    gravity_influence: QuantumCell<Vector3D<f64>>,
    wormhole_connection: Option<ProtectedWormhole<T>>,
    _ufo: UFO<T>,
}

impl<T: 'static> MeshCell<T> {
    pub fn new(position: Vector3D<f64>) -> Self {
        Self {
            state: QuantumCell::new(CellState::Free),
            position: QuantumCell::new(position),
            mass: Helium::new(1.0),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            coherence: QuantumCell::new(1.0),
            gravity_influence: QuantumCell::new(Vector3D::new(0.0, 0.0, 0.0)),
            wormhole_connection: None,
            _ufo: UFO::new(),
        }
    }

    pub fn get_state(&self) -> CellState {
        self.state.get().clone()
    }

    pub fn set_state(&self, new_state: CellState) {
        self.state.set(new_state);
        self.timestamp.quantum_store(CURRENT_TIMESTAMP);
        self.decay_coherence();
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.quantum_load()
    }

    pub fn get_coherence(&self) -> f64 {
        *self.coherence.get()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > MESH_COHERENCE_THRESHOLD
    }

    fn decay_coherence(&self) {
        let current = *self.coherence.get();
        self.coherence.set(current * 0.99);
    }

    pub fn reset_coherence(&self) {
        self.coherence.set(1.0);
        self.timestamp.quantum_store(CURRENT_TIMESTAMP);
    }

    pub fn apply_force(&mut self, force: Vector3D<f64>) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Cell quantum state unstable");
        }

        let current_pos = self.position.get().clone();
        self.position.set(current_pos + force.clone());

        let current_influence = self.gravity_influence.get().clone();
        self.gravity_influence.set(current_influence + force);

        self.decay_coherence();
        self.timestamp.quantum_store(CURRENT_TIMESTAMP);
        Ok(())
    }

    pub fn interact_with_gravity(&mut self, field: &GravityField) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Cell quantum state unstable");
        }

        let position = self.position.get().clone();
        let mass = self.mass.quantum_load();
        let force = field.calculate_force_at(position, mass);
        self.apply_force(force)?;

        if force.magnitude() > GRAVITATIONAL_THRESHOLD {
            self.state.set(CellState::QuantumUncertain);
        }

        Ok(())
    }

    pub fn interact_with_blackhole(&mut self, blackhole: &mut BlackHole) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Cell quantum state unstable");
        }

        let position = self.position.get().clone();
        let blackhole_pos = blackhole.get_position();
        let distance = (position - blackhole_pos).magnitude();

        if distance <= blackhole.get_event_horizon_radius() {
            self.state.set(CellState::Absorbed);
            blackhole.absorb_mass(self.mass.quantum_load())?;
            return Ok(());
        }

        let force = blackhole.calculate_force_at(position);
        self.apply_force(force)?;
        Ok(())
    }

    pub fn connect_wormhole(&mut self, wormhole: ProtectedWormhole<T>) -> Result<(), WormholeError> {
        if !self.is_quantum_stable() {
            return Err(WormholeError::QuantumStateCompromised);
        }

        if self.get_coherence() < WORMHOLE_STABILITY_THRESHOLD {
            return Err(WormholeError::StabilityFailure);
        }

        self.wormhole_connection = Some(wormhole);
        self.state.set(CellState::WormholeConnected);
        self.timestamp.quantum_store(CURRENT_TIMESTAMP);
        Ok(())
    }

    pub fn get_position(&self) -> Vector3D<f64> {
        self.position.get().clone()
    }

    pub fn get_mass(&self) -> f64 {
        self.mass.quantum_load()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_cell_creation() {
        let position = Vector3D::new(1.0, 2.0, 3.0);
        let cell = MeshCell::<()>::new(position.clone());

        assert_eq!(cell.get_state(), CellState::Free);
        assert_eq!(cell.get_position(), position);
        assert!(cell.is_quantum_stable());
    }

    #[test]
    fn test_force_application() {
        let mut cell = MeshCell::<()>::new(Vector3D::new(0.0, 0.0, 0.0));
        let force = Vector3D::new(1.0, 0.0, 0.0);

        assert!(cell.apply_force(force).is_ok());
        assert_eq!(cell.get_position(), Vector3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_quantum_stability() {
        let cell = MeshCell::<()>::new(Vector3D::new(0.0, 0.0, 0.0));

        // Force decoherence
        for _ in 0..100 {
            cell.decay_coherence();
        }

        assert!(!cell.is_quantum_stable());
    }
}
