//! Zeronaut Quantum State Management
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 16:50:49 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::{
    core::{Field, Mesh},
    traits::{MeshValue, CrystalAdd, CrystalSub, CrystalMul, CrystalDiv},
    vector3d::Vector3D,
    resonance::{Quantum, Phase, Resonance}
};

use errors::core::{MathError, QuantumError};
use scribe::{Write as Scribe, native::String, native::Vec};

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

impl<T: MeshValue + Scribe> Scribe for Zeronaut<T> {
    fn write(&self, f: &mut scribe::Formatter) -> scribe::Result {
        f.write_str("Zeronaut State:\n")?;
        f.write_str("Position: ")?;
        self.position.write(f)?;
        f.write_str("\nResonance: ")?;
        self.resonance.write(f)?;
        f.write_str("\nField Energy: ")?;
        write!(f, "{}", self.field.energy().unwrap_or(0.0))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default)]
    struct TestZero {
        value: f64,
    }

    impl MeshValue for TestZero {
        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.value)
        }

        fn from(value: f64) -> Self {
            Self { value }
        }

        fn coherence(&self) -> Result<f64, MathError> {
            Ok(1.0)
        }

        fn energy(&self) -> Result<f64, MathError> {
            Ok(self.value.abs())
        }

        fn magnitude(&self) -> Result<f64, MathError> {
            Ok(self.value.abs())
        }

        fn to_usize(&self) -> Result<usize, MathError> {
            Ok(self.value as usize)
        }

        fn check_harmony_state(&self) -> bool {
            true
        }
    }

    impl CrystalAdd for TestZero {
        fn add(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value + other.value })
        }

        fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value += other.value;
            Ok(())
        }
    }

    impl CrystalSub for TestZero {
        fn sub(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value - other.value })
        }

        fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value -= other.value;
            Ok(())
        }
    }

    impl CrystalMul for TestZero {
        fn mul(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value * other.value })
        }

        fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value *= other.value;
            Ok(())
        }
    }

    impl CrystalDiv for TestZero {
        fn div(&self, other: &Self) -> Result<Self, MathError> {
            if other.value == 0.0 {
                return Err(MathError::DivisionByZero);
            }
            Ok(Self { value: self.value / other.value })
        }

        fn div_assign(&mut self, other: &Self) -> Result<(), MathError> {
            if other.value == 0.0 {
                return Err(MathError::DivisionByZero);
            }
            self.value /= other.value;
            Ok(())
        }
    }

    impl Scribe for TestZero {
        fn write(&self, f: &mut scribe::Formatter) -> scribe::Result {
            write!(f, "{}", self.value)
        }
    }

    #[test]
    fn test_zeronaut_creation() {
        let zeronaut = Zeronaut::<TestZero>::new(4);
        assert_eq!(zeronaut.position().x, 0.0);
    }

    #[test]
    fn test_zeronaut_movement() {
        let mut zeronaut = Zeronaut::<TestZero>::new(4);
        let pos = Vector3D::new(1.0, 1.0, 1.0);
        assert!(zeronaut.move_to(pos).is_ok());
        assert_eq!(zeronaut.position(), &pos);
    }

    #[test]
    fn test_state_access() {
        let mut zeronaut = Zeronaut::<TestZero>::new(4);
        let pos = Vector3D::new(1.0, 1.0, 1.0);
        let value = TestZero { value: 42.0 };

        assert!(zeronaut.set_state(&pos, value).is_ok());
        assert_eq!(zeronaut.get_state(&pos).unwrap().value, 42.0);
    }

    #[test]
    fn test_zero_point_energy() {
        let zeronaut = Zeronaut::<TestZero>::new(4);
        assert!(zeronaut.zero_point_energy().is_ok());
    }

    #[test]
    fn test_quantum_traits() {
        let mut zeronaut = Zeronaut::<TestZero>::new(4);
        assert!(zeronaut.energy().is_ok());
        assert!(zeronaut.phase().is_ok());
        assert!(zeronaut.phase_shift(0.5).is_ok());
    }
}
