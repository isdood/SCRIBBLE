/// Quantum Vector Module
/// Last Updated: 2025-01-15 05:00:15 UTC
/// Author: isdood
/// Current User: isdood

use std::ops::{Add, Sub, Mul};
use crate::scribe::{Scribe, ScribePrecision, QuantumString};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3D<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector4D<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> &T { &self.x }
    pub fn y(&self) -> &T { &self.y }
    pub fn z(&self) -> &T { &self.z }
}

impl<T: Add<Output = T>> Add for Vector3D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl<T: Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl<T: Mul<f64, Output = T>> Mul<f64> for Vector3D<T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
}

impl<T: Scribe> Scribe for Vector3D<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨");
        self.x.scribe(precision, output);
        output.push_str(", ");
        self.y.scribe(precision, output);
        output.push_str(", ");
        self.z.scribe(precision, output);
        output.push_str("⟩");
    }
}

impl<T: Clone + Scribe> Vector3D<T> {
    pub fn magnitude(&self) -> f64 {
        // Implementation for magnitude calculation
        // This is a placeholder - implement actual calculation based on type T
        0.0
    }
}

impl<T> Vector4D<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn x(&self) -> &T { &self.x }
    pub fn y(&self) -> &T { &self.y }
    pub fn z(&self) -> &T { &self.z }
    pub fn w(&self) -> &T { &self.w }
}

impl<T: PartialEq + Add<Output = T>> Add for Vector4D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T: PartialEq + Sub<Output = T>> Sub for Vector4D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl<T: PartialEq + Mul<Output = T> + Copy> Mul<T> for Vector4D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs,
        )
    }
}

impl<T: Scribe> Scribe for Vector4D<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨");
        self.x.scribe(precision, output);
        output.push_str(", ");
        self.y.scribe(precision, output);
        output.push_str(", ");
        self.z.scribe(precision, output);
        output.push_str(", ");
        self.w.scribe(precision, output);
        output.push_str("⟩");
    }
}

impl<T: Scribe + Clone + 'static> Quantum for Vector4D<T> {
    fn get_coherence(&self) -> f64 {
        1.0 // Default implementation
    }

    fn is_quantum_stable(&self) -> bool {
        true // Default implementation
    }

    fn decay_coherence(&self) {
        // Default implementation
    }

    fn reset_coherence(&self) {
        // Default implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(*v.x(), 1.0);
        assert_eq!(*v.y(), 2.0);
        assert_eq!(*v.z(), 3.0);
    }

    #[test]
    fn test_vector3d_addition() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result, Vector3D::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector4d_operations() {
        let v1 = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4D::new(5.0, 6.0, 7.0, 8.0);
        let sum = v1 + v2;
        assert_eq!(sum, Vector4D::new(6.0, 8.0, 10.0, 12.0));
    }
}
