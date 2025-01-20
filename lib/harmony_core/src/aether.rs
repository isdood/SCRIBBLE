//! Aether Module
//! =================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 21:19:08 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::{
    MeshValue,
    Vector3D,
    CrystalAdd,
    CrystalSub,
    CrystalMul,
    CrystalDiv,
};

use errors::{MathError, QuantumError};

use core::{
    fmt::{self, Display, Formatter, Result as FmtResult},
    result::Result,
};

/// Test field for aether operations
#[derive(Debug, Clone, Copy)]
pub struct TestField {
    /// Field position
    position: Vector3D,
    /// Field value
    value: f64,
}

impl TestField {
    /// Create a new test field
    pub fn new(position: Vector3D, value: f64) -> Self {
        Self { position, value }
    }

    /// Get field position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Get field value
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Display for TestField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "TestField(position: {:?}, value: {})", self.position, self.value)
    }
}

impl CrystalAdd for TestField {
    fn add(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self {
            position: self.position.clone(),
           value: self.value + other.value,
        })
    }

    fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
        self.value += other.value;
        Ok(())
    }
}

impl CrystalSub for TestField {
    fn sub(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self {
            position: self.position.clone(),
           value: self.value - other.value,
        })
    }

    fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
        self.value -= other.value;
        Ok(())
    }
}

impl CrystalMul for TestField {
    fn mul(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self {
            position: self.position.clone(),
           value: self.value * other.value,
        })
    }

    fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
        self.value *= other.value;
        Ok(())
    }
}

impl CrystalDiv for TestField {
    fn div(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self {
            position: self.position.clone(),
           value: self.value / other.value,
        })
    }

    fn div_assign(&mut self, other: &Self) -> Result<(), MathError> {
        self.value /= other.value;
        Ok(())
    }
}

impl MeshValue for TestField {
    fn to_f64(&self) -> Result<f64, MathError> {
        Ok(self.value)
    }

    fn from(value: f64) -> Self {
        Self {
            position: Vector3D::new(0.0, 0.0, 0.0),
            value,
        }
    }

    fn coherence(&self) -> Result<f64, MathError> {
        Ok(self.value)
    }

    fn energy(&self) -> Result<f64, MathError> {
        Ok(self.value)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_creation() {
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        let field = TestField::new(pos, 1.0);
        assert_eq!(field.position(), &pos);
        assert_eq!(field.value(), 1.0);
    }

    #[test]
    fn test_field_operations() {
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        let field1 = TestField::new(pos.clone(), 1.0);
        let field2 = TestField::new(pos.clone(), 2.0);

        let field_add = field1.add(&field2).expect("Addition failed");
        assert_eq!(field_add.value(), 3.0);

        let field_sub = field1.sub(&field2).expect("Subtraction failed");
        assert_eq!(field_sub.value(), -1.0);

        let field_mul = field1.mul(&field2).expect("Multiplication failed");
        assert_eq!(field_mul.value(), 2.0);

        let field_div = field1.div(&field2).expect("Division failed");
        assert_eq!(field_div.value(), 0.5);
    }
}
