/// Quantum Gravity Implementation
/// Last Updated: 2025-01-15 05:04:15 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    vector::Vector3D,
    phantom::QuantumCell,
    mesh::MeshCell,
    horizon::Horizon,
    helium::Helium,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GravityState {
    Stable,
    Warped,
    Entangled,
    Decoherent,
}

#[derive(Debug, Clone)]
pub struct GravityFieldData {
    force_vector: Vector3D<f64>,
        field_strength: f64,
        quantum_coherence: f64,
}

#[derive(Debug, Clone)]
pub struct GravityField {
    data: Horizon<GravityFieldData>,
    affected_cells: QuantumCell<Vec<MeshCell>>,
    timestamp: Helium<usize>,
}

impl GravityField {
    pub fn new(force_vector: Vector3D<f64>) -> Self {
        Self {
            data: Horizon::new(GravityFieldData {
                force_vector,
                    field_strength: force_vector.magnitude(),
                               quantum_coherence: 1.0,
            }),
            affected_cells: QuantumCell::new(Vec::new()),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
        }
    }

    pub fn calculate_force_at(&self, position: Vector3D<f64>, mass: f64) -> Vector3D<f64> {
        let distance = position.magnitude();
        if distance < f64::EPSILON {
            return Vector3D::new(0.0, 0.0, 0.0);
        }

        let data = self.data.observe();
        let force_magnitude = GRAVITATIONAL_CONSTANT * mass * data.field_strength
        / (distance * distance);
        let direction = position.normalize();
        direction * force_magnitude
    }

    pub fn get_field_strength(&self) -> f64 {
        self.data.observe().field_strength
    }

    pub fn get_quantum_coherence(&self) -> f64 {
        self.data.observe().quantum_coherence
    }

    pub fn get_force_vector(&self) -> Vector3D<f64> {
        self.data.observe().force_vector.clone()
    }
}

#[derive(Debug, Clone)]
pub struct GravityFieldRef {
    field: Horizon<GravityFieldData>,
}

impl From<GravityField> for GravityFieldRef {
    fn from(field: GravityField) -> Self {
        Self {
            field: field.data,
        }
    }
}

impl GravityFieldRef {
    pub fn calculate_force_at(&self, position: Vector3D<f64>, mass: f64) -> Vector3D<f64> {
        let distance = position.magnitude();
        if distance < f64::EPSILON {
            return Vector3D::new(0.0, 0.0, 0.0);
        }

        let data = self.field.observe();
        let force_magnitude = GRAVITATIONAL_CONSTANT * mass * data.field_strength
        / (distance * distance);
        let direction = position.normalize();
        direction * force_magnitude
    }
}

pub struct MeshGravity {
    field: GravityField,
    affected_cells: QuantumCell<Vec<MeshCell>>,
    quantum_state: QuantumCell<GravityState>,
    timestamp: Helium<usize>,
}

impl MeshGravity {
    pub fn new() -> Self {
        Self {
            field: GravityField::new(Vector3D::new(0.0, -1.0, 0.0)),
            affected_cells: QuantumCell::new(Vec::new()),
            quantum_state: QuantumCell::new(GravityState::Stable),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
        }
    }

    pub fn affect_cell(&mut self, cell: MeshCell) -> Result<(), &'static str> {
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
        let field_strength = self.field.get_field_strength();

        for cell in cells.iter_mut() {
            let position = cell.get_position();
            let distance = position.magnitude();

            if distance < PLANCK_LENGTH {
                self.quantum_state.set(GravityState::Warped);
                return Err("Quantum warp detected");
            }

            let force = self.field.calculate_force_at(position, cell.get_mass());
            cell.apply_force(force)?;
        }

        self.decay_coherence();
        Ok(())
    }

    pub fn get_coherence(&self) -> f64 {
        self.field.get_quantum_coherence()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        let current = self.field.get_quantum_coherence();
        let new_coherence = current * COHERENCE_DECAY_FACTOR;

        let new_state = match new_coherence {
            c if c > 0.9 => GravityState::Stable,
            c if c > 0.7 => GravityState::Warped,
            c if c > QUANTUM_STABILITY_THRESHOLD => GravityState::Entangled,
            _ => GravityState::Decoherent,
        };

        self.quantum_state.set(new_state);
        self.timestamp.quantum_store(CURRENT_TIMESTAMP);
    }

    pub fn entangle_with(&mut self, other: &mut MeshGravity) -> Result<(), &'static str> {
        if !self.is_quantum_stable() || !other.is_quantum_stable() {
            return Err("One or both gravity fields are quantum unstable");
        }

        let combined_coherence = (self.get_coherence() + other.get_coherence()) / 2.0;

        self.quantum_state.set(GravityState::Entangled);
        other.quantum_state.set(GravityState::Entangled);

        Ok(())
    }
}

impl Scribe for GravityField {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("GravityField{force=");
        self.get_force_vector().scribe(precision, output);
        output.push_str(", strength=");
        output.push_f64(self.get_field_strength(), precision.decimal_places());
        output.push_str(", coherence=");
        output.push_f64(self.get_quantum_coherence(), precision.decimal_places());
        output.push_char('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_field_creation() {
        let field = GravityField::new(Vector3D::new(0.0, -1.0, 0.0));
        assert!(field.get_quantum_coherence() > 0.9);
    }

    #[test]
    fn test_mesh_gravity_stability() {
        let mut gravity = MeshGravity::new();

        for _ in 0..100 {
            let _ = gravity.apply_gravitational_effects();
        }

        assert!(!gravity.is_quantum_stable());
        assert!(gravity.apply_gravitational_effects().is_err());
    }

    #[test]
    fn test_gravity_entanglement() {
        let mut gravity1 = MeshGravity::new();
        let mut gravity2 = MeshGravity::new();

        assert!(gravity1.entangle_with(&mut gravity2).is_ok());
        assert_eq!(gravity1.quantum_state.get(), &GravityState::Entangled);
        assert_eq!(gravity2.quantum_state.get(), &GravityState::Entangled);
    }
}
