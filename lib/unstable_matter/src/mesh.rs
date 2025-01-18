/// Quantum Mesh Module
/// Last Updated: 2025-01-18 17:58:31 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    scribe::{Scribe, ScribePrecision, QuantumString},
    vector::Vector3D,
    phantom::QuantumCell,
    GravityField,
    BlackHole,
    Wormhole,
    WormholeGlitch,
    helium::Helium,
    meshmath::MeshMath,
};

#[derive(Debug, Clone)]
pub struct MeshDimensions {
    width: usize,
    height: usize,
    depth: usize,
}

impl MeshDimensions {
    pub fn new(vec: Vector3D<usize>) -> Self {
        Self {
            width: vec.get_x().clone(),
            height: vec.get_y().clone(),
            depth: vec.get_z().clone(),
        }
    }

    pub fn to_vector(&self) -> Vector3D<usize> {
        Vector3D::new(self.width, self.height, self.depth)
    }

    pub fn x(&self) -> usize { self.width }
    pub fn y(&self) -> usize { self.height }
    pub fn z(&self) -> usize { self.depth }

    pub fn volume(&self) -> usize {
        self.width.mesh_mul(self.height).mesh_mul(self.depth)
    }
}

impl Scribe for MeshDimensions {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("Mesh[");
        output.push_usize(self.width);
        output.push_char('x');
        output.push_usize(self.height);
        output.push_char('x');
        output.push_usize(self.depth);
        output.push_char(']');
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
    Free,
    Entangled,
    QuantumUncertain,
    WormholeConnected,
    Absorbed,
}

#[derive(Debug, Clone)]
pub struct MeshCell {
    position: QuantumCell<Vector3D<f64>>,
    mass: Helium<f64>,
    state: QuantumCell<CellState>,
    coherence: Helium<f64>,
    timestamp: Helium<u64>,
    wormhole_connection: Option<Wormhole>,
}

impl MeshCell {
    pub fn new(position: Vector3D<f64>) -> Self {
        Self {
            position: QuantumCell::new(position),
            mass: Helium::new(1.0),
            state: QuantumCell::new(CellState::Free),
            coherence: Helium::new(1.0),
            timestamp: Helium::new(CURRENT_TIMESTAMP.try_into().unwrap()),
            wormhole_connection: None,
        }
    }

    pub fn apply_force(&mut self, force: Vector3D<f64>) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Cell quantum state unstable");
        }

        let position = self.position.get().clone();
        let new_position = position.mesh_add(force);
        self.position.set(new_position);
        self.decay_coherence();
        self.timestamp.quantum_store(CURRENT_TIMESTAMP.try_into().unwrap(), &HeliumOrdering::Quantum)?;
        Ok(())
    }

    pub fn interact_with_gravity(&mut self, field: &GravityField) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Cell quantum state unstable");
        }

        let position = self.position.get().clone();
        let mass = self.mass.quantum_load(&HeliumOrdering::Quantum)?;
        let force = field.calculate_force_at(position, mass);
        self.apply_force(force)?;

        if force.mesh_magnitude() > GRAVITATIONAL_THRESHOLD {
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
        let distance = position.mesh_sub(blackhole_pos).mesh_magnitude();

        if distance <= blackhole.get_event_horizon_radius() {
            self.state.set(CellState::Absorbed);
            blackhole.absorb_mass(self.mass.quantum_load(&HeliumOrdering::Quantum)?)?;
            return Ok(());
        }

        let force = blackhole.calculate_force_at(&position);
        self.apply_force(force)?;
        Ok(())
    }

    pub fn connect_wormhole(&mut self, wormhole: Wormhole) -> Result<(), WormholeGlitch> {
        if !self.is_quantum_stable() {
            return Err(WormholeGlitch::quantum_state_compromised());
        }

        if self.get_coherence() < WORMHOLE_STABILITY_THRESHOLD {
            return Err(WormholeGlitch::stability_failure());
        }

        self.wormhole_connection = Some(wormhole);
        self.state.set(CellState::WormholeConnected);
        Ok(())
    }

    pub fn get_position(&self) -> Vector3D<f64> {
        self.position.get().clone()
    }

    pub fn get_mass(&self) -> f64 {
        self.mass.quantum_load(&HeliumOrdering::Quantum)
        .expect("Failed to load mass")
    }

    pub fn get_state(&self) -> CellState {
        self.state.get()
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence.quantum_load(&HeliumOrdering::Quantum)
        .expect("Failed to load coherence")
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        let current = self.coherence.quantum_load(&HeliumOrdering::Quantum)
        .expect("Failed to load coherence");
        let new_coherence = current.mesh_mul(COHERENCE_DECAY_FACTOR);
        self.coherence.quantum_store(new_coherence, &HeliumOrdering::Quantum)
        .expect("Failed to store coherence");
    }
}

impl Scribe for MeshCell {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("MeshCell{pos=");
        self.get_position().scribe(precision, output);
        output.push_str(", mass=");
        output.push_f64(self.get_mass(), precision.decimal_places());
        output.push_str(", coherence=");
        output.push_f64(self.get_coherence(), precision.decimal_places());
        output.push_char('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_cell_creation() {
        let position = Vector3D::new(1.0, 2.0, 3.0);
        let cell = MeshCell::new(position.clone());

        assert_eq!(cell.get_state(), CellState::Free);
        assert_eq!(cell.get_position(), position);
        assert!(cell.is_quantum_stable());
    }

    #[test]
    fn test_force_application() {
        let mut cell = MeshCell::new(Vector3D::new(0.0, 0.0, 0.0));
        let force = Vector3D::new(1.0, 0.0, 0.0);

        assert!(cell.apply_force(force).is_ok());
        assert_eq!(cell.get_position(), Vector3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_quantum_stability() {
        let cell = MeshCell::new(Vector3D::new(0.0, 0.0, 0.0));

        // Force decoherence
        for _ in 0..100 {
            cell.decay_coherence();
        }

        assert!(!cell.is_quantum_stable());
    }

    #[test]
    fn test_scribe_output() {
        let cell = MeshCell::new(Vector3D::new(1.0, 2.0, 3.0));
        let mut output = QuantumString::new();
        cell.scribe(ScribePrecision::Standard, &mut output);
        assert!(output.as_str().contains("MeshCell{pos="));
        assert!(output.as_str().contains("mass=1.000000"));
    }
}
