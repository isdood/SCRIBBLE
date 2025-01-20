//! Phantom State Management
//! ======================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 20:37:40 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::{
    traits::MeshValue,
    base::{Field, Mesh, PhaseField},
    vectors::{Vector3D, Vector4D},
    resonance::{Resonance, Phase},
};

use errors::{
    core::{Error as MathError, Result as MathResult},
    quantum::QuantumError,
};

use scribe::{
    Scribe,
    native_string::String,
};

use crate::constants;

/// Phantom state container for quantum field manipulation
#[derive(Debug)]
pub struct PhantomState<T> {
    field: PhaseField,
    mesh: Mesh<T>,
    resonance: Resonance,
    position: Vector3D,
    momentum: Vector4D,
}

impl<T: Default + Clone + MeshValue> PhantomState<T> {
    /// Create new phantom state
    pub fn new(size: usize) -> Self {
        Self {
            field: PhaseField::new(constants::PHANTOM_PHASE_SHIFT),
            mesh: Mesh::new(size),
            resonance: Resonance::new(),
            position: Vector3D::new(0.0, 0.0, 0.0),
            momentum: Vector4D::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    /// Get current position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Get current momentum
    pub fn momentum(&self) -> &Vector4D {
        &self.momentum
    }

    /// Get phase field state
    pub fn field(&self) -> &PhaseField {
        &self.field
    }

    /// Get resonance state
    pub fn resonance(&self) -> &Resonance {
        &self.resonance
    }

    /// Set new position
    pub fn set_position(&mut self, pos: Vector3D) -> MathResult<()> {
        self.position = pos;
        self.update_field()
    }

    /// Set new momentum
    pub fn set_momentum(&mut self, mom: Vector4D) -> MathResult<()> {
        self.momentum = mom;
        self.update_field()
    }

    /// Update field state
    fn update_field(&mut self) -> MathResult<()> {
        self.field.transform(&self.position)?;
        self.field.adjust_phase(self.momentum.w())?;
        self.resonance.phase_shift(self.field.phase()?)?;
        Ok(())
    }

    /// Calculate total energy
    pub fn energy(&self) -> MathResult<f64> {
        let field_energy = self.field.energy()?;
        let resonance_energy = self.resonance.energy()?;
        Ok(field_energy + resonance_energy)
    }
}

impl<T: MeshValue> Phase for PhantomState<T> {
    fn phase_shift(&mut self, shift: f64) -> MathResult<()> {
        self.field.adjust_phase(shift)?;
        self.resonance.phase_shift(shift)?;
        Ok(())
    }
}

impl<T: MeshValue> Scribe for PhantomState<T> {
    fn scribe(&self) -> String {
        let mut result = String::new();
        result.push_str("Phantom State:\n");
        result.push_str("Position: ");
        result.push_str(&format!("{:?}", self.position));
        result.push_str("\nMomentum: ");
        result.push_str(&format!("{:?}", self.momentum));
        result.push_str("\nPhase: ");
        result.push_str(&format!("{:?}", self.field.phase()));
        result.push_str("\nResonance: ");
        result.push_str(&format!("{:?}", self.resonance));
        result.push_str("\nEnergy: ");
        result.push_str(&self.energy().unwrap_or(0.0).to_string());
        result
    }
}
