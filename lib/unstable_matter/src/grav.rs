/// Quantum Gravity Implementation
/// Last Updated: 2025-01-14 23:28:28 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    phantom::QuantumCell,
    Vector3D,
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
    data: Arc<GravityFieldData>,
}

impl GravityField {
    pub fn new(force_vector: Vector3D<f64>) -> Self {
        Self {
            data: Arc::new(GravityFieldData {
                force_vector,
                    field_strength: force_vector.magnitude(),
                           quantum_coherence: 1.0,
            }),
        }
    }

    pub fn calculate_force_at(&self, position: Vector3D<f64>, mass: f64) -> Vector3D<f64> {
        let distance = position.magnitude();
        if distance < f64::EPSILON {
            return Vector3D::new(0.0, 0.0, 0.0);
        }

        let force_magnitude = GRAVITATIONAL_CONSTANT * mass * self.data.field_strength
        / (distance * distance);
        let direction = position.normalize();
        direction * force_magnitude
    }

    pub fn get_field_strength(&self) -> f64 {
        self.data.field_strength
    }

    pub fn get_quantum_coherence(&self) -> f64 {
        self.data.quantum_coherence
    }

    pub fn get_force_vector(&self) -> Vector3D<f64> {
        self.data.force_vector.clone()
    }
}

// Now create a reference type for use in Helium
#[derive(Debug, Clone)]
pub struct GravityFieldRef {
    field: Arc<GravityFieldData>,
}

impl From<GravityField> for GravityFieldRef {
    fn from(field: GravityField) -> Self {
        Self {
            field: field.data.clone(),
        }
    }
}

impl GravityFieldRef {
    pub fn calculate_force_at(&self, position: Vector3D<f64>, mass: f64) -> Vector3D<f64> {
        let distance = position.magnitude();
        if distance < f64::EPSILON {
            return Vector3D::new(0.0, 0.0, 0.0);
        }

        let force_magnitude = GRAVITATIONAL_CONSTANT * mass * self.field.field_strength
        / (distance * distance);
        let direction = position.normalize();
        direction * force_magnitude
    }
}

impl MeshGravity {
    pub fn new() -> Self {
        Self {
            field: GravityField::new(GRAVITATIONAL_CONSTANT, Vector3D::new(0.0, -1.0, 0.0)),
            affected_cells: QuantumCell::new(Vec::new()),
            quantum_state: QuantumCell::new(GravityState::Stable),
            timestamp: Helium::new(1705270965), // 2025-01-14 21:42:45 UTC
        }
    }

    pub fn affect_cell(&mut self, cell: MeshCell) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        let mut cells = self.affected_cells.get_mut();
        cells.push(cell);
        self.apply_gravitational_effects()?;
        Ok(())
    }

    pub fn apply_gravitational_effects(&mut self) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        let cells = self.affected_cells.get_mut();
        let field_strength = self.field.get_warped_strength();

        for cell in cells.iter_mut() {
            let position = cell.get_position();
            let distance = position.magnitude();

            if distance < PLANCK_LENGTH {
                self.quantum_state.set(GravityState::Warped);
                self.field.apply_quantum_warp(1.1)?;
            }

            let force = field_strength / (distance * distance);
            let direction = *self.field.direction.get();
            let displacement = direction * force;

            cell.apply_force(displacement)?;
        }

        self.decay_coherence();
        Ok(())
    }

    pub fn get_coherence(&self) -> f64 {
        self.field.get_coherence()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.field.is_quantum_stable()
    }

    fn decay_coherence(&self) {
        self.field.decay_coherence();

        // Update quantum state based on coherence
        let new_state = match self.field.get_coherence() {
            c if c > 0.9 => GravityState::Stable,
            c if c > 0.7 => GravityState::Warped,
            c if c > QUANTUM_COHERENCE_THRESHOLD => GravityState::Entangled,
            _ => GravityState::Decoherent,
        };

        self.quantum_state.set(new_state);
    }

    pub fn entangle_with(&mut self, other: &mut MeshGravity) -> Result<(), &'static str> {
        if !self.is_quantum_stable() || !other.is_quantum_stable() {
            return Err("One or both gravity fields are quantum unstable");
        }

        let combined_coherence = (self.get_coherence() + other.get_coherence()) / 2.0;
        let combined_warp = (self.field.warp_factor.quantum_load() +
        other.field.warp_factor.quantum_load()) / 2.0;

        self.field.coherence.quantum_store(combined_coherence);
        other.field.coherence.quantum_store(combined_coherence);

        self.field.warp_factor.quantum_store(combined_warp);
        other.field.warp_factor.quantum_store(combined_warp);

        self.quantum_state.set(GravityState::Entangled);
        other.quantum_state.set(GravityState::Entangled);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_field_creation() {
        let field = GravityField::new(
            GRAVITATIONAL_CONSTANT,
            Vector3D::new(0.0, -1.0, 0.0)
        );
        assert!(field.is_quantum_stable());
        assert!(field.get_coherence() > 0.9);
    }

    #[test]
    fn test_quantum_warp() {
        let field = GravityField::new(
            GRAVITATIONAL_CONSTANT,
            Vector3D::new(0.0, -1.0, 0.0)
        );

        assert!(field.apply_quantum_warp(1.1).is_ok());
        assert!(field.get_warped_strength() > GRAVITATIONAL_CONSTANT);
    }

    #[test]
    fn test_mesh_gravity_stability() {
        let mut gravity = MeshGravity::new();

        // Force decoherence
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
        assert_eq!(gravity1.get_coherence(), gravity2.get_coherence());
    }
}
