//! Phantom State Operations
//! =====================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 21:31:34 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::{
    traits::MeshValue,
    math::{Field, Mesh, PhaseField},
    errors::MathError,
};

use crate::{
    errors::QuantumError,
    vector::{Vector3D, Vector4D},
    resonance::{Quantum, Phase, Resonance},
    native::{Box, Vec},
};

use scribe::{Scribe, native_string::String};

/// Phantom state handler for higher-dimensional operations
#[derive(Debug)]
pub struct Phantom<T> {
    field: Field,
    phase_field: PhaseField,
    mesh: Mesh<T>,
    resonance: Resonance,
    position: Vector4D,
}

impl<T: Default + Clone + MeshValue> Phantom<T> {
    /// Create a new phantom handler
    pub fn new(size: usize) -> Self {
        Self {
            field: Field::default(),
            phase_field: PhaseField::new(),
            mesh: Mesh::new(size),
            resonance: Resonance::new(),
            position: Vector4D::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    /// Get the phantom state at position
    pub fn get_state(&self, pos: &Vector4D) -> Result<T, QuantumError> {
        self.mesh.get_value_at(&Vector3D::new(pos.x, pos.y, pos.z))
        .map_err(|_| QuantumError::BoundaryViolation)
    }

    /// Set the phantom state at position
    pub fn set_state(&mut self, pos: &Vector4D, value: T) -> Result<(), QuantumError> {
        self.mesh.set_value_at(&Vector3D::new(pos.x, pos.y, pos.z), value)
        .map_err(|_| QuantumError::BoundaryViolation)
    }

    /// Apply phase field transformation
    pub fn apply_phase_field(&mut self) -> Result<(), MathError> {
        self.phase_field.transform(&self.position)?;
        let phase = self.phase_field.phase()?;
        self.resonance.phase_shift(phase)?;
        Ok(())
    }

    /// Move to new 4D position
    pub fn move_to(&mut self, pos: Vector4D) -> Result<(), MathError> {
        self.position = pos;
        self.apply_phase_field()
    }

    /// Project to 3D space
    pub fn project(&self) -> Vector3D {
        let w = if self.position.w == 0.0 { 1.0 } else { self.position.w };
        Vector3D::new(
            self.position.x / w,
            self.position.y / w,
            self.position.z / w,
        )
    }

    /// Get current 4D position
    pub fn position(&self) -> &Vector4D {
        &self.position
    }

    /// Get current resonance state
    pub fn resonance(&self) -> &Resonance {
        &self.resonance
    }

    /// Calculate phantom energy
    pub fn phantom_energy(&self) -> Result<f64, MathError> {
        let field_energy = self.field.energy()?;
        let phase_energy = self.phase_field.energy()?;
        let resonance_energy = self.resonance.energy()?;
        Ok((field_energy + phase_energy + resonance_energy) / 3.0)
    }
}

impl<T: MeshValue> Quantum for Phantom<T> {
    fn energy(&self) -> Result<f64, MathError> {
        self.phantom_energy()
    }

    fn phase(&self) -> Result<f64, MathError> {
        self.phase_field.phase()
    }
}

impl<T: MeshValue> Phase for Phantom<T> {
    fn phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        self.phase_field.apply_shift(shift)?;
        self.resonance.phase_shift(shift)
    }
}

impl<T: MeshValue + Scribe> Scribe for Phantom<T> {
    fn scribe(&self) -> String {
        let mut result = String::new();
        write_str!(result, "Phantom State:\n");
        write_str!(result, "4D Position: ");
        write_str!(result, &self.position.scribe());
        write_str!(result, "\n3D Projection: ");
        write_str!(result, &self.project().scribe());
        write_str!(result, "\nResonance: ");
        write_str!(result, &self.resonance.scribe());
        write_str!(result, "\nPhase Field: ");
        write_str!(result, &self.phase_field.scribe());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default)]
    struct TestPhantom {
        value: f64,
    }

    impl MeshValue for TestPhantom {
        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.value)
        }

        fn from(value: f64) -> Self {
            Self { value }
        }
    }

    impl Scribe for TestPhantom {
        fn scribe(&self) -> String {
            let mut result = String::new();
            write_str!(result, &self.value.scribe());
            result
        }
    }

    #[test]
    fn test_phantom_creation() {
        let phantom = Phantom::<TestPhantom>::new(4);
        assert_eq!(phantom.position().w, 1.0);
    }

    #[test]
    fn test_phantom_projection() {
        let mut phantom = Phantom::<TestPhantom>::new(4);
        let pos = Vector4D::new(2.0, 2.0, 2.0, 2.0);
        phantom.move_to(pos).unwrap();

        let proj = phantom.project();
        assert_eq!(proj.x, 1.0);
        assert_eq!(proj.y, 1.0);
        assert_eq!(proj.z, 1.0);
    }

    #[test]
    fn test_state_access() {
        let mut phantom = Phantom::<TestPhantom>::new(4);
        let pos = Vector4D::new(1.0, 1.0, 1.0, 1.0);
        let value = TestPhantom { value: 42.0 };

        assert!(phantom.set_state(&pos, value).is_ok());
        assert_eq!(phantom.get_state(&pos).unwrap().value, 42.0);
    }

    #[test]
    fn test_phase_field() {
        let mut phantom = Phantom::<TestPhantom>::new(4);
        assert!(phantom.apply_phase_field().is_ok());
    }

    #[test]
    fn test_quantum_traits() {
        let mut phantom = Phantom::<TestPhantom>::new(4);
        assert!(phantom.energy().is_ok());
        assert!(phantom.phase().is_ok());
        assert!(phantom.phase_shift(0.5).is_ok());
    }
}
