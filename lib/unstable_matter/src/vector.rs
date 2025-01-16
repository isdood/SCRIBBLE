/// Quantum Vector Module
/// Last Updated: 2025-01-16 22:58:59 UTC
/// Author: isdood
/// Current User: isdood

use std::ops::{Add, Sub, Mul};
use crate::{
    quantum::Quantum,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4D<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: Copy> Vector3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }

    pub fn as_ptr(&self) -> *const T {
        &self.x as *const T
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.x as *mut T
    }

    pub fn as_isize(&self) -> Vector3D<isize>
    where T: Into<isize> + Copy {
        Vector3D::new(
            self.x.into(),
                      self.y.into(),
                      self.z.into()
        )
    }

    pub fn as_usize(&self) -> Vector3D<usize>
    where T: Into<usize> + Copy {
        Vector3D::new(
            self.x.into(),
                      self.y.into(),
                      self.z.into()
        )
    }
}

impl<T: Copy + Add<Output = T>> Add for Vector3D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl<T: Copy + Mul<f64, Output = T>> Mul<f64> for Vector3D<T> {
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

impl<T: Scribe + Clone + 'static> Quantum for Vector3D<T> {
    fn get_coherence(&self) -> f64 {
        1.0
    }

    fn is_quantum_stable(&self) -> bool {
        true
    }

    fn decay_coherence(&self) {}

    fn reset_coherence(&self) {}
}

impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self::new(
                self.x / mag,
                self.y / mag,
                self.z / mag,
            )
        } else {
            *self
        }
    }

    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Vector3D<isize> {
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl<T> Vector4D<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self
    where
    T: Copy
    {
        Self { x, y, z, w }
    }

    pub fn x(&self) -> T where T: Copy { self.x }
    pub fn y(&self) -> T where T: Copy { self.y }
    pub fn z(&self) -> T where T: Copy { self.z }
    pub fn w(&self) -> T where T: Copy { self.w }

    pub fn as_ptr(&self) -> *const T {
        &self.x as *const T
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.x as *mut T
    }
}

impl<T: Copy + Add<Output = T>> Add for Vector4D<T> {
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

impl<T: Copy + Sub<Output = T>> Sub for Vector4D<T> {
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

impl<T: Copy + Mul<Output = T>> Mul<T> for Vector4D<T> {
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
        1.0
    }

    fn is_quantum_stable(&self) -> bool {
        true
    }

    fn decay_coherence(&self) {}

    fn reset_coherence(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        let sum = v1 + v2;
        assert_eq!(sum, Vector3D::new(5.0, 7.0, 9.0));

        let diff = v2 - v1;
        assert_eq!(diff, Vector3D::new(3.0, 3.0, 3.0));

        let scaled = v1 * 2.0;
        assert_eq!(scaled, Vector3D::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vector3d_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector3d_normalize() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector4d_operations() {
        let v1 = Vector4D::new(1, 2, 3, 4);
        let v2 = Vector4D::new(5, 6, 7, 8);

        let sum = v1 + v2;
        assert_eq!(sum, Vector4D::new(6, 8, 10, 12));

        let diff = v2 - v1;
        assert_eq!(diff, Vector4D::new(4, 4, 4, 4));

        let scaled = v1 * 2;
        assert_eq!(scaled, Vector4D::new(2, 4, 6, 8));
    }

    #[test]
    fn test_vector_conversions() {
        let v = Vector3D::new(1, 2, 3);
        let v_isize = v.as_isize();
        let v_usize = v.as_usize();

        assert_eq!(v_isize, Vector3D::new(1isize, 2isize, 3isize));
        assert_eq!(v_usize, Vector3D::new(1usize, 2usize, 3usize));
    }

    #[test]
    fn test_vector_ptr_access() {
        let mut v = Vector3D::new(1, 2, 3);
        let ptr = v.as_ptr();
        let mut_ptr = v.as_mut_ptr();

        unsafe {
            assert_eq!(*ptr, 1);
            *mut_ptr = 42;
        }
        assert_eq!(v.x(), 42);
    }
}
