/// Crystalline Vector Module
/// Last Updated: 2025-01-18 16:04:49 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    scribe::{Scribe, ScribePrecision, QuantumString},
    quantum::Quantum,
    meshmath::{MeshMath, MeshAdd, MeshSub, MeshMul, MeshDiv, MeshNeg, MeshVec, MeshScalar},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<T> {
    x: T,
    y: T,
    z: T,
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
            MeshMath::add_f64(self.x, rhs.x),
                  MeshMath::add_f64(self.y, rhs.y),
                  MeshMath::add_f64(self.z, rhs.z),
        )
    }
}

impl<T: Copy + MeshSub<Output = T>> MeshSub for Vector3D<T> {
    type Output = Self;
    fn mesh_sub(self, rhs: Self) -> Self::Output {
        Self::new(
            MeshMath::sub_f64(self.x, rhs.x),
                  MeshMath::sub_f64(self.y, rhs.y),
                  MeshMath::sub_f64(self.z, rhs.z),
        )
    }
}

impl<T: Copy + MeshMul<f64, Output = T>> MeshMul<f64> for Vector3D<T> {
    type Output = Self;
    fn mesh_mul(self, scalar: f64) -> Self::Output {
        Self::new(
            MeshMath::mul_f64(self.x, scalar),
                  MeshMath::mul_f64(self.y, scalar),
                  MeshMath::mul_f64(self.z, scalar),
        )
    }
}

impl<T: Copy + MeshDiv<f64, Output = T>> MeshDiv<f64> for Vector3D<T> {
    type Output = Self;
    fn mesh_div(self, scalar: f64) -> Self::Output {
        Self::new(
            MeshMath::div_f64(self.x, scalar),
                  MeshMath::div_f64(self.y, scalar),
                  MeshMath::div_f64(self.z, scalar),
        )
    }
}

impl<T: Copy + MeshNeg<Output = T>> MeshNeg for Vector3D<T> {
    type Output = Self;
    fn mesh_neg(self) -> Self::Output {
        Self::new(
            MeshMath::neg_f64(self.x),
                  MeshMath::neg_f64(self.y),
                  MeshMath::neg_f64(self.z),
        )
    }
}

impl<T: Copy + MeshAdd<Output = T> + MeshMul<Output = T>> Vector3D<T> {
    pub fn mesh_dot(&self, other: &Self) -> T {
        MeshMath::add_f64(
            MeshMath::add_f64(
                MeshMath::mul_f64(self.x, other.x),
                              MeshMath::mul_f64(self.y, other.y),
            ),
            MeshMath::mul_f64(self.z, other.z),
        )
    }

    pub fn mesh_magnitude(&self) -> T {
        let x_sq = MeshMath::mul_f64(self.x, self.x);
        let y_sq = MeshMath::mul_f64(self.y, self.y);
        let z_sq = MeshMath::mul_f64(self.z, self.z);

        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(x_sq, y_sq),
                              z_sq
            )
        )
    }

    pub fn mesh_normalize(&self) -> Self
    where T: MeshDiv<f64, Output = T> {
        let mag = self.mesh_magnitude();
        if MeshMath::gt_f64(mag, MeshMath::zero_f64()) {
            self.mesh_div(mag)
        } else {
            *self
        }
    }

    pub fn mesh_cross(&self, other: &Self) -> Self
    where T: MeshSub<Output = T> {
        Self::new(
            MeshMath::sub_f64(
                MeshMath::mul_f64(self.y, other.z),
                              MeshMath::mul_f64(self.z, other.y)
            ),
            MeshMath::sub_f64(
                MeshMath::mul_f64(self.z, other.x),
                              MeshMath::mul_f64(self.x, other.z)
            ),
            MeshMath::sub_f64(
                MeshMath::mul_f64(self.x, other.y),
                              MeshMath::mul_f64(self.y, other.x)
            ),
        )
    }

    pub fn mesh_distance(&self, other: &Self) -> T
    where T: MeshSub<Output = T> {
        let dx = MeshMath::sub_f64(self.x, other.x);
        let dy = MeshMath::sub_f64(self.y, other.y);
        let dz = MeshMath::sub_f64(self.z, other.z);

        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::mul_f64(dx, dx),
                                  MeshMath::mul_f64(dy, dy)
                ),
                MeshMath::mul_f64(dz, dz)
            )
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

impl<T: Copy + 'static> Quantum for Vector3D<T> {
    fn get_coherence(&self) -> f64 { MeshMath::one_f64() }
    fn is_quantum_stable(&self) -> bool { true }
    fn decay_coherence(&self) {}
    fn reset_coherence(&self) {}
}

impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::mul_f64(self.x, self.x),
                                  MeshMath::mul_f64(self.y, self.y)
                ),
                MeshMath::mul_f64(self.z, self.z)
            )
        )
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if MeshMath::gt_f64(mag, MeshMath::zero_f64()) {
            self.mesh_div(mag)
        } else {
            *self
        }
    }

    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = MeshMath::sub_f64(self.x, other.x);
        let dy = MeshMath::sub_f64(self.y, other.y);
        let dz = MeshMath::sub_f64(self.z, other.z);

        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::mul_f64(dx, dx),
                                  MeshMath::mul_f64(dy, dy)
                ),
                MeshMath::mul_f64(dz, dz)
            )
        )
    }

    pub fn mesh_from_tuple(t: (f64, f64, f64)) -> Self {
        Self::new(
            MeshMath::isize_to_f64(t.0 as isize),
                  MeshMath::isize_to_f64(t.1 as isize),
                  MeshMath::isize_to_f64(t.2 as isize)
        )
    }

    pub fn mesh_to_tuple(&self) -> (f64, f64, f64) {
        (
            MeshMath::isize_to_f64(self.x as isize),
         MeshMath::isize_to_f64(self.y as isize),
         MeshMath::isize_to_f64(self.z as isize)
        )
    }
}

impl Vector3D<isize> {
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = MeshMath::isize_to_f64(MeshMath::sub_isize(self.x, other.x));
        let dy = MeshMath::isize_to_f64(MeshMath::sub_isize(self.y, other.y));
        let dz = MeshMath::isize_to_f64(MeshMath::sub_isize(self.z, other.z));

        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::mul_f64(dx, dx),
                                  MeshMath::mul_f64(dy, dy)
                ),
                MeshMath::mul_f64(dz, dz)
            )
        )
    }
}

impl Scribe for Vector3D<isize> {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨");
        output.push_str(&self.x.to_string());
        output.push_str(", ");
        output.push_str(&self.y.to_string());
        output.push_str(", ");
        output.push_str(&self.z.to_string());
        output.push_str("⟩");
    }
}

/// Crystalline Vector Module - Vector4D Implementation
/// Last Updated: 2025-01-18 16:06:18 UTC
/// Author: isdood
/// Current User: isdood

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4D<T> {
    x: T,
    y: T,
    z: T,
    w: T,
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
            MeshMath::add_f64(self.x, rhs.x),
                  MeshMath::add_f64(self.y, rhs.y),
                  MeshMath::add_f64(self.z, rhs.z),
                  MeshMath::add_f64(self.w, rhs.w),
        )
    }
}

impl<T: Copy + MeshSub<Output = T>> MeshSub for Vector4D<T> {
    type Output = Self;
    fn mesh_sub(self, rhs: Self) -> Self::Output {
        Self::new(
            MeshMath::sub_f64(self.x, rhs.x),
                  MeshMath::sub_f64(self.y, rhs.y),
                  MeshMath::sub_f64(self.z, rhs.z),
                  MeshMath::sub_f64(self.w, rhs.w),
        )
    }
}

impl<T: Copy + MeshMul<f64, Output = T>> MeshMul<f64> for Vector4D<T> {
    type Output = Self;
    fn mesh_mul(self, scalar: f64) -> Self::Output {
        Self::new(
            MeshMath::mul_f64(self.x, scalar),
                  MeshMath::mul_f64(self.y, scalar),
                  MeshMath::mul_f64(self.z, scalar),
                  MeshMath::mul_f64(self.w, scalar),
        )
    }
}

impl<T: Copy + MeshDiv<f64, Output = T>> MeshDiv<f64> for Vector4D<T> {
    type Output = Self;
    fn mesh_div(self, scalar: f64) -> Self::Output {
        Self::new(
            MeshMath::div_f64(self.x, scalar),
                  MeshMath::div_f64(self.y, scalar),
                  MeshMath::div_f64(self.z, scalar),
                  MeshMath::div_f64(self.w, scalar),
        )
    }
}

impl<T: Copy + MeshNeg<Output = T>> MeshNeg for Vector4D<T> {
    type Output = Self;
    fn mesh_neg(self) -> Self::Output {
        Self::new(
            MeshMath::neg_f64(self.x),
                  MeshMath::neg_f64(self.y),
                  MeshMath::neg_f64(self.z),
                  MeshMath::neg_f64(self.w),
        )
    }
}

impl<T: Copy + MeshAdd<Output = T> + MeshMul<Output = T>> Vector4D<T> {
    pub fn mesh_dot(&self, other: &Self) -> T {
        MeshMath::add_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::mul_f64(self.x, other.x),
                                  MeshMath::mul_f64(self.y, other.y)
                ),
                MeshMath::mul_f64(self.z, other.z)
            ),
            MeshMath::mul_f64(self.w, other.w)
        )
    }

    pub fn mesh_magnitude(&self) -> T {
        let x_sq = MeshMath::mul_f64(self.x, self.x);
        let y_sq = MeshMath::mul_f64(self.y, self.y);
        let z_sq = MeshMath::mul_f64(self.z, self.z);
        let w_sq = MeshMath::mul_f64(self.w, self.w);

        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::add_f64(x_sq, y_sq),
                                  z_sq
                ),
                w_sq
            )
        )
    }

    pub fn mesh_normalize(&self) -> Self
    where T: MeshDiv<f64, Output = T> {
        let mag = self.mesh_magnitude();
        if MeshMath::gt_f64(mag, MeshMath::zero_f64()) {
            self.mesh_div(mag)
        } else {
            *self
        }
    }

    pub fn mesh_distance(&self, other: &Self) -> T
    where T: MeshSub<Output = T> {
        let dx = MeshMath::sub_f64(self.x, other.x);
        let dy = MeshMath::sub_f64(self.y, other.y);
        let dz = MeshMath::sub_f64(self.z, other.z);
        let dw = MeshMath::sub_f64(self.w, other.w);

        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::add_f64(
                        MeshMath::mul_f64(dx, dx),
                                      MeshMath::mul_f64(dy, dy)
                    ),
                    MeshMath::mul_f64(dz, dz)
                ),
                MeshMath::mul_f64(dw, dw)
            )
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

impl<T: Copy + 'static> Quantum for Vector4D<T> {
    fn get_coherence(&self) -> f64 { MeshMath::one_f64() }
    fn is_quantum_stable(&self) -> bool { true }
    fn decay_coherence(&self) {}
    fn reset_coherence(&self) {}
}

impl Vector4D<f64> {
    pub fn magnitude(&self) -> f64 {
        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::add_f64(
                        MeshMath::mul_f64(self.x, self.x),
                                      MeshMath::mul_f64(self.y, self.y)
                    ),
                    MeshMath::mul_f64(self.z, self.z)
                ),
                MeshMath::mul_f64(self.w, self.w)
            )
        )
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if MeshMath::gt_f64(mag, MeshMath::zero_f64()) {
            self.mesh_div(mag)
        } else {
            *self
        }
    }

    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = MeshMath::sub_f64(self.x, other.x);
        let dy = MeshMath::sub_f64(self.y, other.y);
        let dz = MeshMath::sub_f64(self.z, other.z);
        let dw = MeshMath::sub_f64(self.w, other.w);

        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::add_f64(
                        MeshMath::mul_f64(dx, dx),
                                      MeshMath::mul_f64(dy, dy)
                    ),
                    MeshMath::mul_f64(dz, dz)
                ),
                MeshMath::mul_f64(dw, dw)
            )
        )
    }

    pub fn mesh_from_tuple(t: (f64, f64, f64, f64)) -> Self {
        Self::new(
            MeshMath::isize_to_f64(t.0 as isize),
                  MeshMath::isize_to_f64(t.1 as isize),
                  MeshMath::isize_to_f64(t.2 as isize),
                  MeshMath::isize_to_f64(t.3 as isize)
        )
    }

    pub fn mesh_to_tuple(&self) -> (f64, f64, f64, f64) {
        (
            MeshMath::isize_to_f64(self.x as isize),
         MeshMath::isize_to_f64(self.y as isize),
         MeshMath::isize_to_f64(self.z as isize),
         MeshMath::isize_to_f64(self.w as isize)
        )
    }
}

impl Vector4D<isize> {
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = MeshMath::isize_to_f64(MeshMath::sub_isize(self.x, other.x));
        let dy = MeshMath::isize_to_f64(MeshMath::sub_isize(self.y, other.y));
        let dz = MeshMath::isize_to_f64(MeshMath::sub_isize(self.z, other.z));
        let dw = MeshMath::isize_to_f64(MeshMath::sub_isize(self.w, other.w));

        MeshMath::sqrt_f64(
            MeshMath::add_f64(
                MeshMath::add_f64(
                    MeshMath::add_f64(
                        MeshMath::mul_f64(dx, dx),
                                      MeshMath::mul_f64(dy, dy)
                    ),
                    MeshMath::mul_f64(dz, dz)
                ),
                MeshMath::mul_f64(dw, dw)
            )
        )
    }
}

impl Scribe for Vector4D<isize> {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨");
        output.push_str(&self.x.to_string());
        output.push_str(", ");
        output.push_str(&self.y.to_string());
        output.push_str(", ");
        output.push_str(&self.z.to_string());
        output.push_str(", ");
        output.push_str(&self.w.to_string());
        output.push_str("⟩");
    }
}
