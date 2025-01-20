//! Aether - Quantum Field Interface
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 20:39:33 UTC
//! Version: 0.1.1
//! License: MIT

use core::{
    fmt::{self, Display},
    result::Result,
};

use magicmath::{
    MeshValue,
    Vector3D,
    Vector4D,
    resonance::Resonance,
};

use errors::MathError;

/// A quantum field density
#[derive(Debug, Clone, Copy)]
pub struct Density(f64);

impl Density {
    /// Create new density value
    pub fn new(value: f64) -> Result<Self, MathError> {
        if value < 0.0 {
            return Err(MathError::InvalidRange);
        }
        Ok(Self(value))
    }

    /// Get density value
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A quantum field interface
#[derive(Debug)]
pub struct Aether<T> {
    /// Field position
    position: Vector3D,
    /// Field momentum
    momentum: Vector4D,
    /// Field density
    density: Density,
    /// Quantum resonance
    resonance: Resonance,
    /// Field value
    value: T,
}

impl<T: Default + Clone + MeshValue> Aether<T> {
    /// Create new quantum field
    pub fn new() -> Result<Self, MathError> {
        Ok(Self {
            position: Vector3D::new(0.0, 0.0, 0.0),
           momentum: Vector4D::new(0.0, 0.0, 0.0, 0.0),
           density: Density::new(1.0)?,
           resonance: Resonance::new(),
           value: T::default(),
        })
    }

    /// Get field position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Get field momentum
    pub fn momentum(&self) -> &Vector4D {
        &self.momentum
    }

    /// Get field density
    pub fn density(&self) -> f64 {
        self.density.value()
    }

    /// Set field position
    pub fn set_position(&mut self, pos: Vector3D) {
        self.position = pos;
    }

    /// Calculate field energy
    pub fn energy(&self) -> Result<f64, MathError> {
        let pos_mag = self.position.magnitude()?;
        let mom_mag = self.momentum.magnitude()?;
        Ok(pos_mag * pos_mag + mom_mag * mom_mag)
    }
}

impl<T: MeshValue + Display> Display for Aether<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Quantum Field:")?;
        writeln!(f, "Position: {:?}", self.position)?;
        writeln!(f, "Momentum: {:?}", self.momentum)?;
        writeln!(f, "Density: {}", self.density.value())?;
        write!(f, "Resonance: {:?}", self.resonance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default)]
    struct TestField(f64);

    impl CrystalAdd for TestField {
        fn add(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self(self.0 + other.0))
        }

        fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.0 += other.0;
            Ok(())
        }
    }

    impl CrystalSub for TestField {
        fn sub(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self(self.0 - other.0))
        }

        fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.0 -= other.0;
            Ok(())
        }
    }

    impl CrystalMul for TestField {
        fn mul(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self(self.0 * other.0))
        }

        fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.0 *= other.0;
            Ok(())
        }
    }

    impl CrystalDiv for TestField {
        fn div(&self, other: &Self) -> Result<Self, MathError> {
            if other.0 == 0.0 {
                return Err(MathError::DivisionByZero);
            }
            Ok(Self(self.0 / other.0))
        }

        fn div_assign(&mut self, other: &Self) -> Result<(), MathError> {
            if other.0 == 0.0 {
                return Err(MathError::DivisionByZero);
            }
            self.0 /= other.0;
            Ok(())
        }
    }

    impl MeshValue for TestField {
        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.0)
        }

        fn from(value: f64) -> Self {
            Self(value)
        }

        fn magnitude(&self) -> Result<f64, MathError> {
            Ok(self.0.abs())
        }

        fn coherence(&self) -> Result<f64, MathError> {
            Ok(1.0)
        }

        fn energy(&self) -> Result<f64, MathError> {
            Ok(self.0 * self.0)
        }

        fn to_usize(&self) -> Result<usize, MathError> {
            Ok(self.0 as usize)
        }

        fn check_harmony_state(&self) -> bool {
            self.0 >= 0.0
        }
    }

    impl Display for TestField {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn test_field_creation() -> Result<(), MathError> {
        let field: Aether<TestField> = Aether::new()?;
        assert!(field.density() > 0.0);
        Ok(())
    }

    #[test]
    fn test_field_position() -> Result<(), MathError> {
        let mut field: Aether<TestField> = Aether::new()?;
        let pos = Vector3D::new(1.0, 0.0, 0.0);
        field.set_position(pos);
        assert_eq!(format!("{:?}", field.position()), format!("{:?}", &pos));
        Ok(())
    }

    #[test]
    fn test_field_energy() -> Result<(), MathError> {
        let field: Aether<TestField> = Aether::new()?;
        assert!(field.energy()? >= 0.0);
        Ok(())
    }

    #[test]
    fn test_density_limits() {
        assert!(Density::new(-1.0).is_err());
        assert!(Density::new(1.0).is_ok());
    }
}
