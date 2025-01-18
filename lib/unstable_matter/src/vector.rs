/// Crystalline Vector Module - Vector3D Implementation
/// Last Updated: 2025-01-18 16:50:21 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    scribe::{Scribe, ScribePrecision, QuantumString},
    quantum::Quantum,
    meshmath::{MeshMath, MeshValue},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<T> where T: MeshValue {
    x: T,
    y: T,
    z: T,
}

impl<T: MeshValue> Vector3D<T> {
    pub fn new_unchecked(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn get_x(&self) -> &T { &self.x }
    pub fn get_y(&self) -> &T { &self.y }
    pub fn get_z(&self) -> &T { &self.z }

    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }

    pub fn mesh_add(&self, other: &Self) -> Self {
        Self::new(
            self.x.mesh_add(other.x),
                  self.y.mesh_add(other.y),
                  self.z.mesh_add(other.z)
        )
    }

    pub fn mesh_sub(&self, other: &Self) -> Self {
        Self::new(
            self.x.mesh_sub(other.x),
                  self.y.mesh_sub(other.y),
                  self.z.mesh_sub(other.z)
        )
    }

    pub fn mesh_mul(&self, scalar: T) -> Self {
        Self::new(
            self.x.mesh_mul(scalar),
                  self.y.mesh_mul(scalar),
                  self.z.mesh_mul(scalar)
        )
    }

    pub fn mesh_div(&self, scalar: T) -> Self {
        Self::new(
            self.x.mesh_div(scalar),
                  self.y.mesh_div(scalar),
                  self.z.mesh_div(scalar)
        )
    }

    pub fn mesh_neg(&self) -> Self {
        Self::new(
            self.x.mesh_neg(),
                  self.y.mesh_neg(),
                  self.z.mesh_neg()
        )
    }

    pub fn mesh_dot(&self, other: &Self) -> T {
        self.x.mesh_mul(other.x)
        .mesh_add(self.y.mesh_mul(other.y))
        .mesh_add(self.z.mesh_mul(other.z))
    }

    pub fn mesh_magnitude_squared(&self) -> T {
        self.mesh_dot(self)
    }

    pub fn mesh_cross(&self, other: &Self) -> Self {
        Self::new(
            self.y.mesh_mul(other.z).mesh_sub(self.z.mesh_mul(other.y)),
                  self.z.mesh_mul(other.x).mesh_sub(self.x.mesh_mul(other.z)),
                  self.x.mesh_mul(other.y).mesh_sub(self.y.mesh_mul(other.x))
        )
    }
}

impl Vector3D<f64> {
    pub fn mesh_magnitude(&self) -> f64 {
        MeshMath::sqrt_f64(
            self.x.mesh_mul(self.x)
            .mesh_add(self.y.mesh_mul(self.y))
            .mesh_add(self.z.mesh_mul(self.z))
        )
    }

    pub fn mesh_normalize(&self) -> Self {
        let mag = self.mesh_magnitude();
        if MeshMath::eq_f64(mag, T::mesh_zero()) {
            *self
        } else {
            Self::new(
                self.x.mesh_div(mag),
                      self.y.mesh_div(mag),
                      self.z.mesh_div(mag)
            )
        }
    }

    pub fn mesh_distance(&self, other: &Self) -> f64 {
        let dx = self.x.mesh_sub(other.x);
        let dy = self.y.mesh_sub(other.y);
        let dz = self.z.mesh_sub(other.z);

        MeshMath::sqrt_f64(
            dx.mesh_mul(dx)
            .mesh_add(dy.mesh_mul(dy))
            .mesh_add(dz.mesh_mul(dz))
        )
    }
}

impl Vector3D<isize> {
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = MeshMath::isize_to_f64(self.x.mesh_sub(other.x));
        let dy = MeshMath::isize_to_f64(self.y.mesh_sub(other.y));
        let dz = MeshMath::isize_to_f64(self.z.mesh_sub(other.z));

        MeshMath::sqrt_f64(
            dx.mesh_mul(dx)
            .mesh_add(dy.mesh_mul(dy))
            .mesh_add(dz.mesh_mul(dz))
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

impl<T: MeshValue + 'static> Quantum for Vector3D<T> {
    fn get_coherence(&self) -> f64 { T::mesh_one() }
    fn is_quantum_stable(&self) -> bool { true }
    fn decay_coherence(&self) {}
    fn reset_coherence(&self) {}
}

// Standard operators implemented using mesh operations
impl std::ops::Add for Vector3D<f64> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self.mesh_add(&other)
    }
}

impl std::ops::Sub for Vector3D<f64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.mesh_sub(&other)
    }
}

impl std::ops::Mul<f64> for Vector3D<f64> {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        self.mesh_mul(scalar)
    }
}

impl std::ops::Div<f64> for Vector3D<f64> {
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        self.mesh_div(scalar)
    }
}

impl std::ops::Neg for Vector3D<f64> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.mesh_neg()
    }
}

/// Crystalline Vector Module - Vector4D Implementation
/// Last Updated: 2025-01-18 16:51:21 UTC
/// Author: isdood
/// Current User: isdood

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4D<T> where T: MeshValue {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: MeshValue> Vector4D<T> {
    pub fn new_unchecked(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn get_x(&self) -> &T { &self.x }
    pub fn get_y(&self) -> &T { &self.y }
    pub fn get_z(&self) -> &T { &self.z }
    pub fn get_w(&self) -> &T { &self.w }

    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }
    pub fn w(&self) -> T { self.w }

    pub fn mesh_add(&self, other: &Self) -> Self {
        Self::new(
            self.x.mesh_add(other.x),
                  self.y.mesh_add(other.y),
                  self.z.mesh_add(other.z),
                  self.w.mesh_add(other.w)
        )
    }

    pub fn mesh_sub(&self, other: &Self) -> Self {
        Self::new(
            self.x.mesh_sub(other.x),
                  self.y.mesh_sub(other.y),
                  self.z.mesh_sub(other.z),
                  self.w.mesh_sub(other.w)
        )
    }

    pub fn mesh_mul(&self, scalar: T) -> Self {
        Self::new(
            self.x.mesh_mul(scalar),
                  self.y.mesh_mul(scalar),
                  self.z.mesh_mul(scalar),
                  self.w.mesh_mul(scalar)
        )
    }

    pub fn mesh_div(&self, scalar: T) -> Self {
        Self::new(
            self.x.mesh_div(scalar),
                  self.y.mesh_div(scalar),
                  self.z.mesh_div(scalar),
                  self.w.mesh_div(scalar)
        )
    }

    pub fn mesh_neg(&self) -> Self {
        Self::new(
            self.x.mesh_neg(),
                  self.y.mesh_neg(),
                  self.z.mesh_neg(),
                  self.w.mesh_neg()
        )
    }

    pub fn mesh_dot(&self, other: &Self) -> T {
        self.x.mesh_mul(other.x)
        .mesh_add(self.y.mesh_mul(other.y))
        .mesh_add(self.z.mesh_mul(other.z))
        .mesh_add(self.w.mesh_mul(other.w))
    }

    pub fn mesh_magnitude_squared(&self) -> T {
        self.mesh_dot(self)
    }
}

impl Vector4D<f64> {
    pub fn mesh_magnitude(&self) -> f64 {
        MeshMath::sqrt_f64(
            self.x.mesh_mul(self.x)
            .mesh_add(self.y.mesh_mul(self.y))
            .mesh_add(self.z.mesh_mul(self.z))
            .mesh_add(self.w.mesh_mul(self.w))
        )
    }

    pub fn mesh_normalize(&self) -> Self {
        let mag = self.mesh_magnitude();
        if MeshMath::eq_f64(mag, T::mesh_zero()) {
            *self
        } else {
            Self::new(
                self.x.mesh_div(mag),
                      self.y.mesh_div(mag),
                      self.z.mesh_div(mag),
                      self.w.mesh_div(mag)
            )
        }
    }

    pub fn mesh_distance(&self, other: &Self) -> f64 {
        let dx = self.x.mesh_sub(other.x);
        let dy = self.y.mesh_sub(other.y);
        let dz = self.z.mesh_sub(other.z);
        let dw = self.w.mesh_sub(other.w);

        MeshMath::sqrt_f64(
            dx.mesh_mul(dx)
            .mesh_add(dy.mesh_mul(dy))
            .mesh_add(dz.mesh_mul(dz))
            .mesh_add(dw.mesh_mul(dw))
        )
    }
}

impl Vector4D<isize> {
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = MeshMath::isize_to_f64(self.x.mesh_sub(other.x));
        let dy = MeshMath::isize_to_f64(self.y.mesh_sub(other.y));
        let dz = MeshMath::isize_to_f64(self.z.mesh_sub(other.z));
        let dw = MeshMath::isize_to_f64(self.w.mesh_sub(other.w));

        MeshMath::sqrt_f64(
            dx.mesh_mul(dx)
            .mesh_add(dy.mesh_mul(dy))
            .mesh_add(dz.mesh_mul(dz))
            .mesh_add(dw.mesh_mul(dw))
        )
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

impl<T: MeshValue + 'static> Quantum for Vector4D<T> {
    fn get_coherence(&self) -> f64 { T::mesh_one() }
    fn is_quantum_stable(&self) -> bool { true }
    fn decay_coherence(&self) {}
    fn reset_coherence(&self) {}
}

// Standard operators implemented using mesh operations
impl std::ops::Add for Vector4D<f64> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self.mesh_add(&other)
    }
}

impl std::ops::Sub for Vector4D<f64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.mesh_sub(&other)
    }
}

impl std::ops::Mul<f64> for Vector4D<f64> {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        self.mesh_mul(scalar)
    }
}

impl std::ops::Div<f64> for Vector4D<f64> {
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        self.mesh_div(scalar)
    }
}

impl std::ops::Neg for Vector4D<f64> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.mesh_neg()
    }
}
