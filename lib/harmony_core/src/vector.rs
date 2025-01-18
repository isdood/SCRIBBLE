//! Crystalline Vector Implementation
//! ==============================
//!
//! Provides quantum-safe vector operations through crystalline
//! lattice structures and harmonic resonance.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:37:02 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    harmony::{MeshValue, MeshOps},
    scribe::{Scribe, ScribePrecision, QuantumString},
    constants::QUANTUM_STABILITY_THRESHOLD
};

/// A three-dimensional vector in crystalline space
#[derive(Debug, Clone, Copy)]
pub struct Vector3D<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: MeshValue + Copy> Vector3D<T> {
    /// Creates a new Vector3D in crystalline space
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Creates a zero vector with perfect crystalline symmetry
    pub fn zero() -> Self where T: MeshValue {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    /// Computes the quantum dot product preserving crystalline coherence
    pub fn dot(&self, other: &Self) -> T {
        self.x.mesh_mul(&other.x)
        .mesh_add(&self.y.mesh_mul(&other.y))
        .mesh_add(&self.z.mesh_mul(&other.z))
    }

    /// Returns the squared length in crystalline space
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }

    /// Computes the cross product with quantum stability
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y.mesh_mul(&other.z).mesh_sub(&self.z.mesh_mul(&other.y)),
                  self.z.mesh_mul(&other.x).mesh_sub(&self.x.mesh_mul(&other.z)),
                  self.x.mesh_mul(&other.y).mesh_sub(&self.y.mesh_mul(&other.x))
        )
    }
}

impl<T: MeshValue + Copy> MeshOps for Vector3D<T> {
    type Output = Self;

    fn mesh_add(&self, rhs: &Self) -> Self {
        Self::new(
            self.x.mesh_add(&rhs.x),
                  self.y.mesh_add(&rhs.y),
                  self.z.mesh_add(&rhs.z)
        )
    }

    fn mesh_sub(&self, rhs: &Self) -> Self {
        Self::new(
            self.x.mesh_sub(&rhs.x),
                  self.y.mesh_sub(&rhs.y),
                  self.z.mesh_sub(&rhs.z)
        )
    }

    fn mesh_mul(&self, scalar: &f64) -> Self {
        Self::new(
            self.x.mesh_mul(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                  self.y.mesh_mul(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                  self.z.mesh_mul(&T::from_f64(*scalar).unwrap_or_else(T::zero))
        )
    }

    fn mesh_div(&self, scalar: &f64) -> Self {
        if *scalar != 0.0 {
            Self::new(
                self.x.mesh_div(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                      self.y.mesh_div(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                      self.z.mesh_div(&T::from_f64(*scalar).unwrap_or_else(T::zero))
            )
        } else {
            Self::zero()
        }
    }

    fn mesh_neg(&self) -> Self {
        Self::new(
            self.x.mesh_neg(),
                  self.y.mesh_neg(),
                  self.z.mesh_neg()
        )
    }
}

/// A four-dimensional vector in crystalline space
#[derive(Debug, Clone, Copy)]
pub struct Vector4D<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: MeshValue + Copy> Vector4D<T> {
    /// Creates a new Vector4D in crystalline space
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a zero vector with perfect crystalline symmetry
    pub fn zero() -> Self where T: MeshValue {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }

    /// Computes the quantum dot product preserving crystalline coherence
    pub fn dot(&self, other: &Self) -> T {
        self.x.mesh_mul(&other.x)
        .mesh_add(&self.y.mesh_mul(&other.y))
        .mesh_add(&self.z.mesh_mul(&other.z))
        .mesh_add(&self.w.mesh_mul(&other.w))
    }

    /// Returns the squared length in crystalline space
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }

    /// Projects to 3D crystalline space by dividing by w
    pub fn project_3d(&self) -> Vector3D<T> {
        Vector3D::new(
            self.x.mesh_div(&self.w),
                      self.y.mesh_div(&self.w),
                      self.z.mesh_div(&self.w)
        )
    }
}

impl<T: MeshValue + Copy> MeshOps for Vector4D<T> {
    type Output = Self;

    fn mesh_add(&self, rhs: &Self) -> Self {
        Self::new(
            self.x.mesh_add(&rhs.x),
                  self.y.mesh_add(&rhs.y),
                  self.z.mesh_add(&rhs.z),
                  self.w.mesh_add(&rhs.w)
        )
    }

    fn mesh_sub(&self, rhs: &Self) -> Self {
        Self::new(
            self.x.mesh_sub(&rhs.x),
                  self.y.mesh_sub(&rhs.y),
                  self.z.mesh_sub(&rhs.z),
                  self.w.mesh_sub(&rhs.w)
        )
    }

    fn mesh_mul(&self, scalar: &f64) -> Self {
        Self::new(
            self.x.mesh_mul(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                  self.y.mesh_mul(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                  self.z.mesh_mul(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                  self.w.mesh_mul(&T::from_f64(*scalar).unwrap_or_else(T::zero))
        )
    }

    fn mesh_div(&self, scalar: &f64) -> Self {
        if *scalar != 0.0 {
            Self::new(
                self.x.mesh_div(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                      self.y.mesh_div(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                      self.z.mesh_div(&T::from_f64(*scalar).unwrap_or_else(T::zero)),
                      self.w.mesh_div(&T::from_f64(*scalar).unwrap_or_else(T::zero))
            )
        } else {
            Self::zero()
        }
    }

    fn mesh_neg(&self) -> Self {
        Self::new(
            self.x.mesh_neg(),
                  self.y.mesh_neg(),
                  self.z.mesh_neg(),
                  self.w.mesh_neg()
        )
    }
}

impl<T: MeshValue + Scribe> Scribe for Vector3D<T> {
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

impl<T: MeshValue + Scribe> Scribe for Vector4D<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        let sum = v1.mesh_add(&v2);
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);

        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);

        let cross = v1.cross(&v2);
        assert_eq!(cross.x, -3.0);
        assert_eq!(cross.y, 6.0);
        assert_eq!(cross.z, -3.0);
    }

    #[test]
    fn test_vector4d_operations() {
        let v1 = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4D::new(5.0, 6.0, 7.0, 8.0);

        let diff = v2.mesh_sub(&v1);
        assert_eq!(diff.x, 4.0);
        assert_eq!(diff.y, 4.0);
        assert_eq!(diff.z, 4.0);
        assert_eq!(diff.w, 4.0);

        let scaled = v1.mesh_mul(&2.0);
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
        assert_eq!(scaled.z, 6.0);
        assert_eq!(scaled.w, 8.0);
    }

    #[test]
    fn test_vector_projection() {
        let v4 = Vector4D::new(2.0, 4.0, 6.0, 2.0);
        let v3 = v4.project_3d();

        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 2.0);
        assert_eq!(v3.z, 3.0);
    }

    #[test]
    fn test_zero_vectors() {
        let v3 = Vector3D::<f64>::zero();
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, 0.0);

        let v4 = Vector4D::<f64>::zero();
        assert_eq!(v4.x, 0.0);
        assert_eq!(v4.y, 0.0);
        assert_eq!(v4.z, 0.0);
        assert_eq!(v4.w, 0.0);
    }
}
