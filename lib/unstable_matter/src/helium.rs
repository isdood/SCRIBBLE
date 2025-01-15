/// Quantum Vector Module
/// Last Updated: 2025-01-15 05:29:18 UTC
/// Author: isdood
/// Current User: isdood

use std::ops::{Add, Sub, Mul};
use crate::{
    quantum::Quantum,
    scribe::{Scribe, ScribePrecision, QuantumString},
    constants::QUANTUM_STABILITY_THRESHOLD,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3D<T> {
    x: T,
    y: T,
    z: T,
    coherence: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector4D<T> {
    x: T,
    y: T,
    z: T,
    w: T,
    coherence: f64,
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            coherence: 1.0,
        }
    }

    pub fn x(&self) -> &T { &self.x }
    pub fn y(&self) -> &T { &self.y }
    pub fn z(&self) -> &T { &self.z }
}

impl<T: Add<Output = T> + Clone> Add for Vector3D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl<T: Sub<Output = T> + Clone> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl<T: Mul<f64, Output = T> + Clone> Mul<f64> for Vector3D<T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
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

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let diff = self.clone() - other.clone();
        diff.magnitude() * (self.coherence.min(other.coherence))
    }
}

impl<T> Vector4D<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x,
            y,
            z,
            w,
            coherence: 1.0,
        }
    }

    pub fn x(&self) -> &T { &self.x }
    pub fn y(&self) -> &T { &self.y }
    pub fn z(&self) -> &T { &self.z }
    pub fn w(&self) -> &T { &self.w }
}

impl<T: Add<Output = T> + Clone> Add for Vector4D<T> {
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

impl<T: Sub<Output = T> + Clone> Sub for Vector4D<T> {
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

impl<T: Mul<Output = T> + Clone> Mul<T> for Vector4D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.x * rhs.clone(),
                  self.y * rhs.clone(),
                  self.z * rhs.clone(),
                  self.w * rhs,
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

impl<T> Quantum for Vector3D<T> {
    fn get_coherence(&self) -> f64 {
        self.coherence
    }

    fn is_quantum_stable(&self) -> bool {
        self.coherence > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        // Note: In a fully immutable implementation, this would return a new vector
        // For this example, we'll use interior mutability
        unsafe {
            let coherence_ptr = &self.coherence as *const f64 as *mut f64;
            *coherence_ptr *= 0.9;
        }
    }

    fn reset_coherence(&self) {
        unsafe {
            let coherence_ptr = &self.coherence as *const f64 as *mut f64;
            *coherence_ptr = 1.0;
        }
    }
}

impl<T> Quantum for Vector4D<T> {
    fn get_coherence(&self) -> f64 {
        self.coherence
    }

    fn is_quantum_stable(&self) -> bool {
        self.coherence > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        unsafe {
            let coherence_ptr = &self.coherence as *const f64 as *mut f64;
            *coherence_ptr *= 0.9;
        }
    }

    fn reset_coherence(&self) {
        unsafe {
            let coherence_ptr = &self.coherence as *const f64 as *mut f64;
            *coherence_ptr = 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

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

        assert_eq!(*sum.x(), 5.0);
        assert_eq!(*sum.y(), 7.0);
        assert_eq!(*sum.z(), 9.0);

        assert_eq!(*diff.x(), 3.0);
        assert_eq!(*diff.y(), 3.0);
        assert_eq!(*diff.z(), 3.0);
    }

    #[test]
    fn test_vector3d_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_vector3d_normalize() {
        let v = Vector3D::new(3.0, 0.0, 0.0);
        let normalized = v.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_quantum_operations() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert!(v.is_quantum_stable());
        v.decay_coherence();
        assert!(v.get_coherence() < 1.0);
        v.reset_coherence();
        assert_eq!(v.get_coherence(), 1.0);
    }

    #[test]
    fn test_quantum_distance() {
        let v1 = Vector3D::new(0.0, 0.0, 0.0);
        let v2 = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v1.quantum_distance(&v2), 5.0);
    }
}
