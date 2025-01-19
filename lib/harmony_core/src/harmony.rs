//! Harmonic State Management
//! ======================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 21:29:20 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::{
    traits::MeshValue,
    math::Field,
    errors::MathError,
};

use crate::{
    errors::QuantumError,
    vector::Vector3D,
    cube::Cube,
    align::{Alignment, AlignmentState},
    resonance::{Quantum, Phase, Resonance},
    scribe::Scribe,
};

/// A quantum state handler
#[derive(Debug)]
pub struct HarmonicHandler<T> {
    state: Cube<T>,
    resonance: Resonance,
    field: Field,
}

impl<T: Default + Clone + MeshValue> HarmonicHandler<T> {
    /// Create a new quantum handler
    pub fn new(size: usize) -> Self {
        Self {
            state: Cube::new(size),
            resonance: Resonance::new(),
            field: Field::default(),
        }
    }

    /// Get the quantum state at position
    pub fn get_state(&self, pos: &Vector3D) -> Result<T, QuantumError> {
        self.state.get_state_at(pos)
    }

    /// Set the quantum state at position
    pub fn set_state(&mut self, pos: &Vector3D, value: T) -> Result<(), QuantumError> {
        self.state.set_state_at(pos, value)
    }

    /// Get the current resonance
    pub fn resonance(&self) -> &Resonance {
        &self.resonance
    }

    /// Get the current field
    pub fn field(&self) -> &Field {
        &self.field
    }

    /// Apply field transformation
    pub fn apply_field(&mut self, pos: &Vector3D) -> Result<(), MathError> {
        self.field.transform(pos)?;
        self.resonance.set_position(*pos);
        Ok(())
    }

    /// Check quantum coherence
    pub fn check_coherence(&self) -> Result<f64, MathError> {
        self.resonance.coherence()
    }
}

impl<T: MeshValue> Quantum for HarmonicHandler<T> {
    fn energy(&self) -> Result<f64, MathError> {
        self.resonance.energy()
    }

    fn phase(&self) -> Result<f64, MathError> {
        self.resonance.phase()
    }
}

impl<T: MeshValue> Phase for HarmonicHandler<T> {
    fn phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        self.resonance.phase_shift(shift)
    }
}

impl<T: MeshValue + Scribe> Scribe for HarmonicHandler<T> {
    fn scribe(&self) -> String {
        let mut result = String::new();
        write_str!(result, "Harmonic State:\n");
        write_str!(result, "Resonance: ");
        write_str!(result, &self.resonance.scribe());
        write_str!(result, "\nField: ");
        write_str!(result, &self.field.scribe());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default)]
    struct TestHarmonic {
        value: f64,
    }

    impl MeshValue for TestHarmonic {
        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.value)
        }

        fn from(value: f64) -> Self {
            Self { value }
        }
    }

    impl Scribe for TestHarmonic {
        fn scribe(&self) -> String {
            let mut result = String::new();
            write_str!(result, &self.value.scribe());
            result
        }
    }

    #[test]
    fn test_harmonic_handler() {
        let mut handler = HarmonicHandler::<TestHarmonic>::new(4);
        let pos = Vector3D::new(1.0, 1.0, 1.0);
        let value = TestHarmonic { value: 42.0 };

        assert!(handler.set_state(&pos, value).is_ok());
        assert_eq!(handler.get_state(&pos).unwrap().value, 42.0);
    }

    #[test]
    fn test_field_transform() {
        let mut handler = HarmonicHandler::<TestHarmonic>::new(2);
        let pos = Vector3D::new(1.0, 0.0, 0.0);

        assert!(handler.apply_field(&pos).is_ok());
        assert_eq!(handler.resonance().position(), &pos);
    }

    #[test]
    fn test_quantum_traits() {
        let mut handler = HarmonicHandler::<TestHarmonic>::new(2);

        assert!(handler.energy().is_ok());
        assert!(handler.phase().is_ok());
        assert!(handler.phase_shift(0.5).is_ok());
    }

    #[test]
    fn test_coherence() {
        let handler = HarmonicHandler::<TestHarmonic>::new(2);
        assert!(handler.check_coherence().is_ok());
    }
}
