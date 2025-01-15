/// Quantum Black Hole Module
/// Last Updated: 2025-01-15 05:05:20 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    helium::Helium,
    phantom::QuantumCell,
    vector::Vector3D,
    mesh::MeshCell,
};

const SCHWARZSCHILD_CONSTANT: f64 = 2.0 * 6.67430e-11; // 2G/c^2
const HAWKING_TEMPERATURE_CONSTANT: f64 = 1.227e-11;
const EVENT_HORIZON_COHERENCE: f64 = 0.9;
const QUANTUM_COHERENCE_THRESHOLD: f64 = 0.5;

#[derive(Debug, Clone, Copy)]
pub enum BlackHoleState {
    Stable,
    Evaporating,
    Entangled,
    Decoherent,
    Singularity,
}

#[derive(Debug)]
pub struct BlackHole {
    position: QuantumCell<Vector3D<f64>>,
    mass: Helium<f64>,
    radius: Helium<f64>,
    coherence: Helium<f64>,
    temperature: Helium<f64>,
    affected_cells: QuantumCell<Vec<MeshCell>>,
    quantum_state: QuantumCell<BlackHoleState>,
    timestamp: Helium<usize>,
}

impl BlackHole {
    pub fn new(mass: f64, position: Vector3D<f64>) -> Self {
        let mut bh = Self {
            position: QuantumCell::new(position),
            mass: Helium::new(mass),
            radius: Helium::new(SCHWARZSCHILD_CONSTANT * mass),
            coherence: Helium::new(1.0),
            temperature: Helium::new(HAWKING_TEMPERATURE_CONSTANT / mass),
            affected_cells: QuantumCell::new(Vec::new()),
            quantum_state: QuantumCell::new(BlackHoleState::Stable),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
        };
        bh.update_event_horizon();
        bh
    }

    fn update_event_horizon(&mut self) {
        let mass = self.mass.quantum_load();
        self.radius.quantum_store(SCHWARZSCHILD_CONSTANT * mass);
    }

    pub fn affect_mesh_cell(&mut self, cell: MeshCell) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        let mut cells = self.affected_cells.get();
        cells.push(cell);
        self.affected_cells.set(cells);
        self.apply_gravitational_effects()?;
        Ok(())
    }

    pub fn apply_gravitational_effects(&mut self) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        let mut cells = self.affected_cells.get();
        let position = self.position.get();
        let mass = self.mass.quantum_load();

        for cell in cells.iter_mut() {
            let cell_pos = cell.get_position();
            let distance = (position - cell_pos).magnitude();

            if distance <= self.radius.quantum_load() {
                self.absorb_cell(cell)?;
                continue;
            }

            let force = mass * 6.67430e-11 / (distance * distance);
            let direction = (position - cell_pos).normalize();
            let displacement = direction * force;

            cell.apply_force(displacement)?;
        }

        self.evaporate()?;
        self.decay_coherence();
        Ok(())
    }

    fn absorb_cell(&mut self, cell: &MeshCell) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        let mass = self.mass.quantum_load();
        let cell_mass = cell.get_mass();

        self.mass.quantum_store(mass + cell_mass);
        self.update_properties();
        self.quantum_state.set(BlackHoleState::Stable);

        Ok(())
    }

    fn evaporate(&mut self) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        let mass = self.mass.quantum_load();
        let temperature = self.temperature.quantum_load();

        // Hawking radiation mass loss
        let mass_loss = temperature * 1e-21; // Simplified model
        let new_mass = (mass - mass_loss).max(0.0);

        if new_mass <= 0.0 {
            self.quantum_state.set(BlackHoleState::Decoherent);
            return Err("Black hole has evaporated");
        }

        self.mass.quantum_store(new_mass);
        self.update_properties();
        self.quantum_state.set(BlackHoleState::Evaporating);

        Ok(())
    }

    fn update_properties(&mut self) {
        let mass = self.mass.quantum_load();
        self.radius.quantum_store(SCHWARZSCHILD_CONSTANT * mass);
        self.temperature.quantum_store(HAWKING_TEMPERATURE_CONSTANT / mass);
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence.quantum_load()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_COHERENCE_THRESHOLD
    }

    fn decay_coherence(&self) {
        let current = self.coherence.quantum_load();
        let new_coherence = current * 0.99;
        self.coherence.quantum_store(new_coherence);
        self.timestamp.quantum_store(CURRENT_TIMESTAMP);

        let new_state = match (new_coherence, self.radius.quantum_load()) {
            (c, _) if c < QUANTUM_COHERENCE_THRESHOLD => BlackHoleState::Decoherent,
            (c, r) if c > EVENT_HORIZON_COHERENCE && r > 0.0 => BlackHoleState::Stable,
            (_, r) if r <= 0.0 => BlackHoleState::Singularity,
            (c, _) if c > QUANTUM_COHERENCE_THRESHOLD => BlackHoleState::Evaporating,
            _ => BlackHoleState::Decoherent,
        };

        self.quantum_state.set(new_state);
    }

    pub fn entangle_with(&mut self, other: &mut BlackHole) -> Result<(), &'static str> {
        if !self.is_quantum_stable() || !other.is_quantum_stable() {
            return Err("One or both black holes are quantum unstable");
        }

        let combined_mass = (self.mass.quantum_load() + other.mass.quantum_load()) / 2.0;
        let combined_coherence = (self.get_coherence() + other.get_coherence()) / 2.0;

        self.mass.quantum_store(combined_mass);
        other.mass.quantum_store(combined_mass);

        self.coherence.quantum_store(combined_coherence);
        other.coherence.quantum_store(combined_coherence);

        self.quantum_state.set(BlackHoleState::Entangled);
        other.quantum_state.set(BlackHoleState::Entangled);

        self.update_properties();
        other.update_properties();

        Ok(())
    }

    pub fn get_event_horizon_radius(&self) -> f64 {
        self.radius.quantum_load()
    }

    pub fn get_hawking_temperature(&self) -> f64 {
        self.temperature.quantum_load()
    }

    pub fn get_mass(&self) -> f64 {
        self.mass.quantum_load()
    }

    pub fn get_position(&self) -> Vector3D<f64> {
        self.position.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_hole_creation() {
        let black_hole = BlackHole::new(
            1.0e30, // 1 solar mass
            Vector3D::new(0.0, 0.0, 0.0)
        );
        assert!(black_hole.is_quantum_stable());
        assert_eq!(*black_hole.quantum_state.get(), BlackHoleState::Stable);
    }

    #[test]
    fn test_hawking_evaporation() {
        let mut black_hole = BlackHole::new(
            1.0e20, // Small black hole
            Vector3D::new(0.0, 0.0, 0.0)
        );

        assert!(black_hole.evaporate().is_ok());
        assert_eq!(*black_hole.quantum_state.get(), BlackHoleState::Evaporating);
        assert!(black_hole.get_mass() < 1.0e20);
    }

    #[test]
    fn test_black_hole_entanglement() {
        let mut bh1 = BlackHole::new(1.0e30, Vector3D::new(0.0, 0.0, 0.0));
        let mut bh2 = BlackHole::new(2.0e30, Vector3D::new(1.0, 0.0, 0.0));

        assert!(bh1.entangle_with(&mut bh2).is_ok());
        assert_eq!(bh1.get_mass(), bh2.get_mass());
        assert_eq!(*bh1.quantum_state.get(), BlackHoleState::Entangled);
    }

    #[test]
    fn test_quantum_stability() {
        let mut black_hole = BlackHole::new(
            1.0e30,
            Vector3D::new(0.0, 0.0, 0.0)
        );

        // Force decoherence
        for _ in 0..100 {
            let _ = black_hole.evaporate();
        }

        assert!(!black_hole.is_quantum_stable());
        assert!(black_hole.evaporate().is_err());
    }
}
