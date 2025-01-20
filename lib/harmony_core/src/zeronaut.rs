//! Zeronaut Quantum State Management
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 20:32:11 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::{
    traits::MeshValue,
    geometry::{Field, Mesh},
    types::Vector3D,
    resonance::{Quantum, Phase, Resonance},
};

use errors::{
    Error as MathError,
    types::QuantumError,
};

use scribe::{
    Scribe,
    native_string::String,
};

/// Zero-point energy state handler
#[derive(Debug)]
pub struct Zeronaut<T> {
    field: Field,
    mesh: Mesh<T>,
    resonance: Resonance,
    position: Vector3D,
}

impl<T: Default + Clone + MeshValue> Zeronaut<T> {
    /// Create a new zeronaut handler
    pub fn new(size: usize) -> Self {
        Self {
            field: Field::default(),
            mesh: Mesh::new(size),
            resonance: Resonance::new(),
            position: Vector3D::new(0.0, 0.0, 0.0),
        }
    }

    /// Get the zero-point state at position
    pub fn get_state(&self, pos: &Vector3D) -> Result<T, QuantumError> {
        self.mesh.get_value_at(pos)
        .map_err(|_| QuantumError::BoundaryViolation)
    }

    /// Set the zero-point state at position
    pub fn set_state(&mut self, pos: &Vector3D, value: T) -> Result<(), QuantumError> {
        self.mesh.set_value_at(pos, value)
        .map_err(|_| QuantumError::BoundaryViolation)
    }

    /// Apply field transformation at current position
    pub fn apply_field(&mut self) -> Result<(), MathError> {
        self.field.transform(&self.position)?;
        self.resonance.set_position(self.position);
        Ok(())
    }

    /// Move to new position
    pub fn move_to(&mut self, pos: Vector3D) -> Result<(), MathError> {
        self.position = pos;
        self.apply_field()
    }

    /// Get current position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Get current resonance state
    pub fn resonance(&self) -> &Resonance {
        &self.resonance
    }

    /// Calculate zero-point energy
    pub fn zero_point_energy(&self) -> Result<f64, MathError> {
        let field_energy = self.field.energy()?;
        let resonance_energy = self.resonance.energy()?;
        Ok(0.5 * (field_energy + resonance_energy))
    }
}

impl<T: MeshValue> Quantum for Zeronaut<T> {
    fn energy(&self) -> Result<f64, MathError> {
        self.zero_point_energy()
    }

    fn phase(&self) -> Result<f64, MathError> {
        self.resonance.phase()
    }
}

impl<T: MeshValue> Phase for Zeronaut<T> {
    fn phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        self.resonance.phase_shift(shift)
    }
}

impl<T: MeshValue> Scribe for Zeronaut<T> {
    fn scribe(&self) -> String {
        let mut result = String::new();
        result.push_str("Zeronaut State:\n");
        result.push_str("Position: ");
        result.push_str(&self.position.to_string());
        result.push_str("\nResonance: ");
        result.push_str(&self.resonance.to_string());
        result.push_str("\nField Energy: ");
        result.push_str(&self.field.energy().unwrap_or(0.0).to_string());
        result
    }
}
