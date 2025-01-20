//! Phantom State Operations
//! =====================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 20:13:06 UTC
//! Version: 0.1.0
//! License: MIT

use core::fmt::{Display, Formatter, Result as FmtResult};

use magicmath::{
    traits::MeshValue,
    operations::{
        Field,
        Mesh,
        PhaseField,
    },
    geometry::{Vector3D, Vector4D},
    resonance::{Quantum, Phase, Resonance},
};

use errors::{
    quantum::QuantumError,
    core::Error as MathError,
};

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

impl<T: MeshValue + Display> Display for Phantom<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Phantom State:")?;
        writeln!(f, "4D Position: {}", self.position)?;
        writeln!(f, "3D Projection: {}", self.project())?;
        writeln!(f, "Resonance: {}", self.resonance)?;
        write!(f, "Phase Field: {}", self.phase_field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use magicmath::traits::{CrystalAdd, CrystalSub, CrystalMul, CrystalDiv};

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

    impl CrystalAdd for TestPhantom {
        fn add(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value + other.value })
        }

        fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value += other.value;
            Ok(())
        }
    }

    impl CrystalSub for TestPhantom {
        fn sub(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value - other.value })
        }

        fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value -= other.value;
            Ok(())
        }
    }

    impl CrystalMul for TestPhantom {
        fn mul(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value * other.value })
        }

        fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value *= other.value;
            Ok(())
        }
    }

    impl CrystalDiv for TestPhantom {
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

    impl Display for TestPhantom {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "{}", self.value)
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
