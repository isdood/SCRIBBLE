/// Crystalline Vector Module
/// Last Updated: 2025-01-18 15:07:33 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    scribe::{Scribe, ScribePrecision, QuantumString},
    quantum::Quantum,
    meshmath::{MeshMath, MeshAdd, MeshSub, MeshMul, MeshDiv, MeshNeg},
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

impl<T> Vector3D<T> {
    pub fn new_unchecked(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn get_x(&self) -> &T { &self.x }
    pub fn get_y(&self) -> &T { &self.y }
    pub fn get_z(&self) -> &T { &self.z }
}

impl<T: Copy> Vector3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }
}

impl<T: Clone> Vector3D<T> {
    pub fn get(&self) -> (T, T, T) {
        (self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: Copy + MeshAdd<Output = T>> MeshAdd for Vector3D<T> {
    type Output = Self;
    fn mesh_add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x.mesh_add(rhs.x),
                  self.y.mesh_add(rhs.y),
                  self.z.mesh_add(rhs.z),
        )
    }
}

impl<T: Copy + MeshSub<Output = T>> MeshSub for Vector3D<T> {
    type Output = Self;
    fn mesh_sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x.mesh_sub(rhs.x),
                  self.y.mesh_sub(rhs.y),
                  self.z.mesh_sub(rhs.z),
        )
    }
}

impl<T: Copy + MeshMul<f64, Output = T>> MeshMul<f64> for Vector3D<T> {
    type Output = Self;
    fn mesh_mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x.mesh_mul(rhs),
                  self.y.mesh_mul(rhs),
                  self.z.mesh_mul(rhs),
        )
    }
}

impl<T: Copy + MeshDiv<f64, Output = T>> MeshDiv<f64> for Vector3D<T> {
    type Output = Self;
    fn mesh_div(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x.mesh_div(rhs),
                  self.y.mesh_div(rhs),
                  self.z.mesh_div(rhs),
        )
    }
}

impl<T: Copy + MeshNeg<Output = T>> MeshNeg for Vector3D<T> {
    type Output = Self;
    fn mesh_neg(self) -> Self::Output {
        Self::new(
            self.x.mesh_neg(),
                  self.y.mesh_neg(),
                  self.z.mesh_neg(),
        )
    }
}

impl<T> Vector4D<T> {
    pub fn new_unchecked(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn get_x(&self) -> &T { &self.x }
    pub fn get_y(&self) -> &T { &self.y }
    pub fn get_z(&self) -> &T { &self.z }
    pub fn get_w(&self) -> &T { &self.w }
}

impl<T: Copy> Vector4D<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }
    pub fn w(&self) -> T { self.w }
}

impl<T: Copy + MeshAdd<Output = T>> MeshAdd for Vector4D<T> {
    type Output = Self;
    fn mesh_add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x.mesh_add(rhs.x),
                  self.y.mesh_add(rhs.y),
                  self.z.mesh_add(rhs.z),
                  self.w.mesh_add(rhs.w),
        )
    }
}

impl<T: Copy + MeshSub<Output = T>> MeshSub for Vector4D<T> {
    type Output = Self;
    fn mesh_sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x.mesh_sub(rhs.x),
                  self.y.mesh_sub(rhs.y),
                  self.z.mesh_sub(rhs.z),
                  self.w.mesh_sub(rhs.w),
        )
    }
}

impl<T: Copy + MeshMul<f64, Output = T>> MeshMul<f64> for Vector4D<T> {
    type Output = Self;
    fn mesh_mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x.mesh_mul(rhs),
                  self.y.mesh_mul(rhs),
                  self.z.mesh_mul(rhs),
                  self.w.mesh_mul(rhs),
        )
    }
}

impl<T: Copy + MeshDiv<f64, Output = T>> MeshDiv<f64> for Vector4D<T> {
    type Output = Self;
    fn mesh_div(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x.mesh_div(rhs),
                  self.y.mesh_div(rhs),
                  self.z.mesh_div(rhs),
                  self.w.mesh_div(rhs),
        )
    }
}

impl<T: Copy + MeshNeg<Output = T>> MeshNeg for Vector4D<T> {
    type Output = Self;
    fn mesh_neg(self) -> Self::Output {
        Self::new(
            self.x.mesh_neg(),
                  self.y.mesh_neg(),
                  self.z.mesh_neg(),
                  self.w.mesh_neg(),
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

impl<T: Scribe + Clone + Copy + 'static> Quantum for Vector4D<T> {
    fn get_coherence(&self) -> f64 { 1.0 }
    fn is_quantum_stable(&self) -> bool { true }
    fn decay_coherence(&self) {}
    fn reset_coherence(&self) {}
}

impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        MeshMath::sqrt(
            self.x.mesh_mul(self.x).mesh_add(
                self.y.mesh_mul(self.y).mesh_add(
                    self.z.mesh_mul(self.z)
                )
            )
        )
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.mesh_div(mag)
        } else {
            *self
        }
    }

    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = self.x.mesh_sub(other.x);
        let dy = self.y.mesh_sub(other.y);
        let dz = self.z.mesh_sub(other.z);
        MeshMath::sqrt(
            dx.mesh_mul(dx).mesh_add(
                dy.mesh_mul(dy).mesh_add(
                    dz.mesh_mul(dz)
                )
            )
        )
    }
}

impl Vector4D<f64> {
    pub fn magnitude(&self) -> f64 {
        MeshMath::sqrt(
            self.x.mesh_mul(self.x).mesh_add(
                self.y.mesh_mul(self.y).mesh_add(
                    self.z.mesh_mul(self.z).mesh_add(
                        self.w.mesh_mul(self.w)
                    )
                )
            )
        )
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.mesh_div(mag)
        } else {
            *self
        }
    }

    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = self.x.mesh_sub(other.x);
        let dy = self.y.mesh_sub(other.y);
        let dz = self.z.mesh_sub(other.z);
        let dw = self.w.mesh_sub(other.w);
        MeshMath::sqrt(
            dx.mesh_mul(dx).mesh_add(
                dy.mesh_mul(dy).mesh_add(
                    dz.mesh_mul(dz).mesh_add(
                        dw.mesh_mul(dw)
                    )
                )
            )
        )
    }
}

impl Vector3D<isize> {
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = MeshMath::to_f64(self.x.mesh_sub(other.x));
        let dy = MeshMath::to_f64(self.y.mesh_sub(other.y));
        let dz = MeshMath::to_f64(self.z.mesh_sub(other.z));
        MeshMath::sqrt(
            dx.mesh_mul(dx).mesh_add(
                dy.mesh_mul(dy).mesh_add(
                    dz.mesh_mul(dz)
                )
            )
        )
    }
}

impl Vector4D<isize> {
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = MeshMath::to_f64(self.x.mesh_sub(other.x));
        let dy = MeshMath::to_f64(self.y.mesh_sub(other.y));
        let dz = MeshMath::to_f64(self.z.mesh_sub(other.z));
        let dw = MeshMath::to_f64(self.w.mesh_sub(other.w));
        MeshMath::sqrt(
            dx.mesh_mul(dx).mesh_add(
                dy.mesh_mul(dy).mesh_add(
                    dz.mesh_mul(dz).mesh_add(
                        dw.mesh_mul(dw)
                    )
                )
            )
        )
    }
}
