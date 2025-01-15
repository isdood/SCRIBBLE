/// Quantum Vector Module
/// Last Updated: 2025-01-15 05:06:34 UTC
/// Author: isdood
/// Current User: isdood

use std::ops::{Add, Sub, Mul};
use crate::{
    scribe::{Scribe, ScribePrecision, QuantumString},
    phantom::Quantum,
};

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

// Single Scribe implementation for Vector3D
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

impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.clone() * (1.0 / mag)
        } else {
            self.clone()
        }
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
    fn test_vector3d_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let sum = v1.clone() + v2.clone();
        let diff = v2 - v1;

        assert_eq!(sum, Vector3D::new(5.0, 7.0, 9.0));
        assert_eq!(diff, Vector3D::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_vector3d_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector3d_normalize() {
        let v = Vector3D::new(3.0, 0.0, 0.0);
        let normalized = v.normalize();
        assert_eq!(normalized.magnitude(), 1.0);
    }

    #[test]
    fn test_vector4d_operations() {
        let v1 = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4D::new(5.0, 6.0, 7.0, 8.0);
        let sum = v1 + v2;

        assert_eq!(sum, Vector4D::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn test_scribe_output() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let mut output = QuantumString::new();
        v.scribe(ScribePrecision::Standard, &mut output);
        assert_eq!(output.as_str(), "⟨1.000000, 2.000000, 3.000000⟩");
    }
}
