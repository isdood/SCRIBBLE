/// Quantum Mesh Cell Implementation
/// Last Updated: 2025-01-15 01:21:42 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    vector::Vector3D,
    phantom::QuantumCell,
    grav::GravityField,
    helium::Helium,
    wormhole::Wormhole,
    glitch::WormholeGlitch,
    scribe::{Scribe, ScribePrecision, QuantumString},
    blackhole::BlackHole,
    wormhole::Wormhole,
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

#[derive(Debug)]
pub enum CellState {
    Free,
    Entangled,
    QuantumUncertain,
    WormholeConnected,
    Absorbed,
}

pub struct MeshCell<T: 'static> {
    position: QuantumCell<Vector3D<f64>>,
    mass: Helium<f64>,
    state: QuantumCell<CellState>,
    coherence: Helium<f64>,
    timestamp: Helium<usize>,
    wormhole_connection: Option<Wormhole>,  // Changed from ProtectedWormhole to Wormhole
}

impl<T> MeshCell<T> {
    pub fn new(position: Vector3D<f64>) -> Self {
        Self {
            position: QuantumCell::new(position),
            mass: Helium::new(1.0),
            state: QuantumCell::new(CellState::Free),
            coherence: Helium::new(1.0),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            wormhole_connection: None,
        }
    }

    pub fn apply_force(&mut self, force: Vector3D<f64>) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Cell quantum state unstable");
        }

        let position = self.position.get().clone();
        let new_position = position + force;
        self.position.set(new_position);
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

    pub fn connect_wormhole(&mut self, wormhole: Wormhole) -> Result<(), WormholeGlitch> {
        if !self.is_quantum_stable() {
            return Err(WormholeGlitch::QuantumStateCompromised);
        }

        if self.get_coherence() < WORMHOLE_STABILITY_THRESHOLD {
            return Err(WormholeGlitch::StabilityFailure);
        }

        self.wormhole_connection = Some(wormhole);
        self.state.set(CellState::WormholeConnected);
        Ok(())
    }

    pub fn get_position(&self) -> Vector3D<f64> {
        self.position.get().clone()
    }

    pub fn get_mass(&self) -> f64 {
        self.mass.quantum_load()
    }

    pub fn get_state(&self) -> CellState {
        self.state.get().clone()
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence.quantum_load()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        let current = self.coherence.quantum_load();
        let new_coherence = current * COHERENCE_DECAY_FACTOR;
        self.coherence.quantum_store(new_coherence);
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
