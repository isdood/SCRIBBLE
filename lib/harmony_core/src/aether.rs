//! Aether Field Operations
//! ====================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 17:19:22 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::{
    ops::{Field, Mesh, PhaseField, AetherField},
    traits::{MeshValue, CrystalAdd, CrystalSub, CrystalMul, CrystalDiv},
    vector3d::Vector3D,
    vector4d::Vector4D,
    resonance::{Quantum, Phase, Resonance}
};

use errors::MathError;
use crate::{QuantumError, String};
use core::fmt::{self, Write, Formatter, Result as FmtResult};
use alloc::string::ToString;

/// Aether field state handler
#[derive(Debug)]
pub struct Aether<T> {
    field: AetherField,
    phase_field: PhaseField,
    mesh: Mesh<T>,
    resonance: Resonance,
    position: Vector4D,
    density: f64,
}

impl<T: Default + Clone + MeshValue> Aether<T> {
    /// Create a new aether field handler
    pub fn new(size: usize, density: f64) -> Self {
        Self {
            field: AetherField::new(density),
            phase_field: PhaseField::new(),
            mesh: Mesh::new(size),
            resonance: Resonance::new(),
            position: Vector4D::new(0.0, 0.0, 0.0, 1.0),
            density,
        }
    }

    /// Get the aether state at position
    pub fn get_state(&self, pos: &Vector4D) -> Result<T, QuantumError> {
        self.mesh.get_value_at(&Vector3D::new(pos.x, pos.y, pos.z))
        .map_err(|_| QuantumError::BoundaryViolation)
    }

    /// Set the aether state at position
    pub fn set_state(&mut self, pos: &Vector4D, value: T) -> Result<(), QuantumError> {
        self.mesh.set_value_at(&Vector3D::new(pos.x, pos.y, pos.z), value)
        .map_err(|_| QuantumError::BoundaryViolation)
    }

    /// Apply aether field transformation
    pub fn apply_field(&mut self) -> Result<(), MathError> {
        self.field.transform(&self.position)?;
        let phase = self.field.phase()?;
        self.phase_field.apply_shift(phase)?;
        self.resonance.phase_shift(phase)?;
        Ok(())
    }

    /// Move to new 4D position
    pub fn move_to(&mut self, pos: Vector4D) -> Result<(), MathError> {
        self.position = pos;
        self.apply_field()
    }

    /// Get current density
    pub fn density(&self) -> f64 {
        self.density
    }

    /// Set new density
    pub fn set_density(&mut self, density: f64) -> Result<(), MathError> {
        if density <= 0.0 {
            return Err(MathError::InvalidValue);
        }
        self.density = density;
        self.field.set_density(density);
        Ok(())
    }

    /// Calculate aether potential
    pub fn potential(&self) -> Result<f64, MathError> {
        let field_energy = self.field.energy()?;
        let density_factor = self.density.sqrt();
        Ok(field_energy * density_factor)
    }
}

impl<T: MeshValue> Quantum for Aether<T> {
    fn energy(&self) -> Result<f64, MathError> {
        self.potential()
    }

    fn phase(&self) -> Result<f64, MathError> {
        self.field.phase()
    }
}

impl<T: MeshValue> Phase for Aether<T> {
    fn phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        self.phase_field.apply_shift(shift)?;
        self.resonance.phase_shift(shift)
    }
}

impl<T: MeshValue> fmt::Display for Aether<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Aether Field State:")?;
        writeln!(f, "Position: {}", self.position)?;
        writeln!(f, "Density: {}", self.density)?;
        writeln!(f, "Potential: {}", self.potential().unwrap_or(0.0))?;
        write!(f, "Resonance: {}", self.resonance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default)]
    struct TestAether {
        value: f64,
    }

    impl MeshValue for TestAether {
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

    impl CrystalAdd for TestAether {
        fn add(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value + other.value })
        }

        fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value += other.value;
            Ok(())
        }
    }

    impl CrystalSub for TestAether {
        fn sub(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value - other.value })
        }

        fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value -= other.value;
            Ok(())
        }
    }

    impl CrystalMul for TestAether {
        fn mul(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value * other.value })
        }

        fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value *= other.value;
            Ok(())
        }
    }

    impl CrystalDiv for TestAether {
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

    impl fmt::Display for TestAether {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "{}", self.value)
        }
    }

    #[test]
    fn test_aether_creation() {
        let aether = Aether::<TestAether>::new(4, 1.0);
        assert_eq!(aether.density(), 1.0);
    }

    #[test]
    fn test_density_update() {
        let mut aether = Aether::<TestAether>::new(4, 1.0);
        assert!(aether.set_density(2.0).is_ok());
        assert_eq!(aether.density(), 2.0);
        assert!(aether.set_density(0.0).is_err());
    }

    #[test]
    fn test_state_access() {
        let mut aether = Aether::<TestAether>::new(4, 1.0);
        let pos = Vector4D::new(1.0, 1.0, 1.0, 1.0);
        let value = TestAether { value: 42.0 };

        assert!(aether.set_state(&pos, value).is_ok());
        assert_eq!(aether.get_state(&pos).unwrap().value, 42.0);
    }

    #[test]
    fn test_field_application() {
        let mut aether = Aether::<TestAether>::new(4, 1.0);
        assert!(aether.apply_field().is_ok());
    }

    #[test]
    fn test_quantum_traits() {
        let mut aether = Aether::<TestAether>::new(4, 1.0);
        assert!(aether.energy().is_ok());
        assert!(aether.phase().is_ok());
        assert!(aether.phase_shift(0.5).is_ok());
    }
}
