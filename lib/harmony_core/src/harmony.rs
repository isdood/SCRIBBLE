//! Harmonic State Management
//! ======================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-21 00:20:01 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::{
    MeshValue,
    Vector3D,
    resonance::{Quantum, Phase, Resonance},
    CrystalAdd,
    CrystalSub,
    CrystalMul,
    CrystalDiv,
};

use errors::MathError;
use core::{
    fmt::{self, Display},
    result::Result,
};

use crate::{
    cube::Cube,
    QuantumError,
};

/// A quantum state handler
#[derive(Debug)]
pub struct HarmonicHandler<T> {
    state: Cube<T>,
    resonance: Resonance,
    field: f64,
}

impl<T: Default + Clone + MeshValue> HarmonicHandler<T> {
    /// Create a new quantum handler
    pub fn new(size: usize) -> Self {
        Self {
            state: Cube::new(size),
            resonance: Resonance::new(),
            field: 0.0,
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
    pub fn field(&self) -> f64 {
        self.field
    }

    /// Apply field transformation
    pub fn apply_field(&mut self, pos: &Vector3D) -> Result<(), MathError> {
        self.field = pos.magnitude()?;
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

impl<T: MeshValue + Display> Display for HarmonicHandler<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Harmonic State:")?;
        write!(f, "Resonance: {:?}\nField: {}", self.resonance, self.field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default)]
    struct TestHarmonic {
        value: f64,
    }

    impl CrystalAdd for TestHarmonic {
        fn add(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value + other.value })
        }

        fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value += other.value;
            Ok(())
        }
    }

    impl CrystalSub for TestHarmonic {
        fn sub(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value - other.value })
        }

        fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value -= other.value;
            Ok(())
        }
    }

    impl CrystalMul for TestHarmonic {
        fn mul(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value * other.value })
        }

        fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value *= other.value;
            Ok(())
        }
    }

    impl CrystalDiv for TestHarmonic {
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

    impl MeshValue for TestHarmonic {
        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.value)
        }

        fn from(value: f64) -> Self {
            Self { value }
        }

        fn coherence(&self) -> Result<f64, MathError> {
            Ok(self.value.abs())
        }

        fn energy(&self) -> Result<f64, MathError> {
            Ok(self.value * self.value)
        }

        fn magnitude(&self) -> Result<f64, MathError> {
            Ok(self.value.abs())
        }

        fn to_usize(&self) -> Result<usize, MathError> {
            Ok(self.value as usize)
        }

        fn check_harmony_state(&self) -> bool {
            self.value >= 0.0
        }
    }

    impl Display for TestHarmonic {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
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
        assert_eq!(handler.resonance().position(), pos);
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
        let mut handler = HarmonicHandler::<TestHarmonic>::new(2);

        // Initialize with a stable state
        let pos = Vector3D::new(1.0, 0.0, 0.0);
        handler.apply_field(&pos).expect("Failed to apply field");

        // Set a test state with known coherence
        let test_value = TestHarmonic { value: 1.0 };
        handler.set_state(&pos, test_value).expect("Failed to set state");

        // Check coherence - should succeed with value > 0
        let coherence = handler.check_coherence().expect("Coherence check failed");
        assert!(coherence > 0.0, "Coherence should be positive");
    }
}
