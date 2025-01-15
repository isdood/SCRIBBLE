/// Quantum Vector Module
/// Last Updated: 2025-01-15 05:17:55 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    quantum::Quantum,
    helium::Helium,
    scribe::{Scribe, ScribePrecision, QuantumString},
};
use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
pub struct Vector3D<T: 'static> {
    x: Helium<T>,
    y: Helium<T>,
    z: Helium<T>,
}

#[derive(Debug)]
pub struct Vector4D<T: 'static> {
    x: Helium<T>,
    y: Helium<T>,
    z: Helium<T>,
    w: Helium<T>,
}

impl<T: 'static> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x: Helium::new(x),
            y: Helium::new(y),
            z: Helium::new(z),
        }
    }

    pub fn x(&self) -> T where T: Copy { self.x.quantum_load() }
    pub fn y(&self) -> T where T: Copy { self.y.quantum_load() }
    pub fn z(&self) -> T where T: Copy { self.z.quantum_load() }
}

impl<T: Add<Output = T> + Copy + 'static> Add for Vector3D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z(),
        )
    }
}

impl<T: Sub<Output = T> + Copy + 'static> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z(),
        )
    }
}

impl<T: Mul<f64, Output = T> + Copy + 'static> Mul<f64> for Vector3D<T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x() * rhs,
                  self.y() * rhs,
                  self.z() * rhs,
        )
    }
}

impl<T: Scribe + Copy + 'static> Scribe for Vector3D<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨");
        output.push_f64(self.x().into(), precision.decimal_places());
        output.push_str(", ");
        output.push_f64(self.y().into(), precision.decimal_places());
        output.push_str(", ");
        output.push_f64(self.z().into(), precision.decimal_places());
        output.push_str("⟩");
    }
}

impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        (x * x + y * y + z * z).sqrt()
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

impl<T: 'static> Vector4D<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x: Helium::new(x),
            y: Helium::new(y),
            z: Helium::new(z),
            w: Helium::new(w),
        }
    }

    pub fn x(&self) -> T where T: Copy { self.x.quantum_load() }
    pub fn y(&self) -> T where T: Copy { self.y.quantum_load() }
    pub fn z(&self) -> T where T: Copy { self.z.quantum_load() }
    pub fn w(&self) -> T where T: Copy { self.w.quantum_load() }
}

impl<T: Add<Output = T> + Copy + 'static> Add for Vector4D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z(),
                  self.w() + rhs.w(),
        )
    }
}

impl<T: Sub<Output = T> + Copy + 'static> Sub for Vector4D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z(),
                  self.w() - rhs.w(),
        )
    }
}

impl<T: Mul<Output = T> + Copy + 'static> Mul<T> for Vector4D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.x() * rhs,
                  self.y() * rhs,
                  self.z() * rhs,
                  self.w() * rhs,
        )
    }
}

impl<T: Scribe + Copy + 'static> Scribe for Vector4D<T> {
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

impl<T: Scribe + Copy + 'static> Quantum for Vector4D<T> {
    fn get_coherence(&self) -> f64 {
        (self.x.get_coherence() +
        self.y.get_coherence() +
        self.z.get_coherence() +
        self.w.get_coherence()) / 4.0
    }

    fn is_quantum_stable(&self) -> bool {
        self.x.is_quantum_stable() &&
        self.y.is_quantum_stable() &&
        self.z.is_quantum_stable() &&
        self.w.is_quantum_stable()
    }

    fn decay_coherence(&self) {
        self.x.decay_coherence();
        self.y.decay_coherence();
        self.z.decay_coherence();
        self.w.decay_coherence();
    }

    fn reset_coherence(&self) {
        self.x.reset_coherence();
        self.y.reset_coherence();
        self.z.reset_coherence();
        self.w.reset_coherence();
    }
}

impl<T: Copy + 'static> Clone for Vector3D<T> {
    fn clone(&self) -> Self {
        Self::new(self.x(), self.y(), self.z())
    }
}

impl<T: Copy + 'static> Clone for Vector4D<T> {
    fn clone(&self) -> Self {
        Self::new(self.x(), self.y(), self.z(), self.w())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_vector3d_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert!((v.x() - 1.0).abs() < EPSILON);
        assert!((v.y() - 2.0).abs() < EPSILON);
        assert!((v.z() - 3.0).abs() < EPSILON);
    }

    #[test]
    fn test_vector3d_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let sum = v1.clone() + v2.clone();
        let diff = v2 - v1;

        assert!((sum.x() - 5.0).abs() < EPSILON);
        assert!((sum.y() - 7.0).abs() < EPSILON);
        assert!((sum.z() - 9.0).abs() < EPSILON);

        assert!((diff.x() - 3.0).abs() < EPSILON);
        assert!((diff.y() - 3.0).abs() < EPSILON);
        assert!((diff.z() - 3.0).abs() < EPSILON);
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
    fn test_quantum_stability() {
        let v = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        assert!(v.is_quantum_stable());
        v.decay_coherence();
        assert!(v.get_coherence() < 1.0);
    }
}
