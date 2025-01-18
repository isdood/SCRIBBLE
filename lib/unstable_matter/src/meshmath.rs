/// MeshSpace Mathematics Implementation
/// Last Updated: 2025-01-18 15:58:40 UTC
/// Author: isdood
/// Current User: isdood

use std::f64::consts::PI;

// Operation traits
pub trait MeshAdd<Rhs = Self> {
    type Output;
    fn mesh_add(self, rhs: Rhs) -> Self::Output;
}

pub trait MeshSub<Rhs = Self> {
    type Output;
    fn mesh_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait MeshMul<Rhs = Self> {
    type Output;
    fn mesh_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait MeshDiv<Rhs = Self> {
    type Output;
    fn mesh_div(self, rhs: Rhs) -> Self::Output;
}

pub trait MeshNeg {
    type Output;
    fn mesh_neg(self) -> Self::Output;
}

// Vector-specific traits
pub trait MeshVec<T> {
    fn mesh_dot(&self, other: &Self) -> T;
    fn mesh_magnitude(&self) -> T;
    fn mesh_normalize(&self) -> Self;
    fn mesh_cross(&self, other: &Self) -> Self;
}

pub trait MeshScalar<T> {
    fn mesh_scale(&self, scalar: T) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeshMath;

impl MeshMath {
    // Type conversion functions
    pub fn isize_to_f64(x: isize) -> f64 {
        x as f64
    }

    pub fn f64_to_isize(x: f64) -> isize {
        x as isize
    }

    // Basic f64 operations
    pub fn add_f64(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn sub_f64(a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }

    pub fn div_f64(a: f64, b: f64) -> f64 {
        a / b
    }

    pub fn neg_f64(a: f64) -> f64 {
        -a
    }

    // Basic isize operations
    pub fn add_isize(a: isize, b: isize) -> isize {
        a + b
    }

    pub fn sub_isize(a: isize, b: isize) -> isize {
        a - b
    }

    pub fn mul_isize(a: isize, b: isize) -> isize {
        a * b
    }

    pub fn div_isize(a: isize, b: isize) -> isize {
        a / b
    }

    pub fn neg_isize(a: isize) -> isize {
        -a
    }

    // Mathematical functions for f64
    pub fn sqrt_f64(x: f64) -> f64 {
        x.sqrt()
    }

    pub fn abs_f64(x: f64) -> f64 {
        x.abs()
    }

    pub fn sin_f64(x: f64) -> f64 {
        x.sin()
    }

    pub fn cos_f64(x: f64) -> f64 {
        x.cos()
    }

    pub fn tan_f64(x: f64) -> f64 {
        x.tan()
    }

    pub fn exp_f64(x: f64) -> f64 {
        x.exp()
    }

    pub fn ln_f64(x: f64) -> f64 {
        x.ln()
    }

    pub fn pow_f64(x: f64, y: f64) -> f64 {
        x.powf(y)
    }

    // Comparison operations
    pub fn gt_f64(a: f64, b: f64) -> bool {
        a > b
    }

    pub fn lt_f64(a: f64, b: f64) -> bool {
        a < b
    }

    pub fn eq_f64(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    pub fn gt_isize(a: isize, b: isize) -> bool {
        a > b
    }

    pub fn lt_isize(a: isize, b: isize) -> bool {
        a < b
    }

    pub fn eq_isize(a: isize, b: isize) -> bool {
        a == b
    }

    // Constants
    pub fn zero_f64() -> f64 {
        0.0
    }

    pub fn one_f64() -> f64 {
        1.0
    }

    pub fn pi_f64() -> f64 {
        PI
    }

    pub fn zero_isize() -> isize {
        0
    }

    pub fn one_isize() -> isize {
        1
    }

    // Vector operations
    pub fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter()).map(|(&x, &y)| Self::mul_f64(x, y)).sum()
    }

    pub fn cross_product_f64(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
        [
            Self::sub_f64(Self::mul_f64(a[1], b[2]), Self::mul_f64(a[2], b[1])),
            Self::sub_f64(Self::mul_f64(a[2], b[0]), Self::mul_f64(a[0], b[2])),
            Self::sub_f64(Self::mul_f64(a[0], b[1]), Self::mul_f64(a[1], b[0])),
        ]
    }

    pub fn vector_magnitude_f64(v: &[f64]) -> f64 {
        Self::sqrt_f64(
            v.iter()
            .map(|&x| Self::mul_f64(x, x))
            .fold(Self::zero_f64(), Self::add_f64)
        )
    }

    pub fn normalize_vector_f64(v: &[f64]) -> Vec<f64> {
        let mag = Self::vector_magnitude_f64(v);
        if Self::gt_f64(mag, Self::zero_f64()) {
            v.iter().map(|&x| Self::div_f64(x, mag)).collect()
        } else {
            v.to_vec()
        }
    }

    // Angle operations
    pub fn normalize_angle_f64(angle: f64) -> f64 {
        let two_pi = Self::mul_f64(2.0, Self::pi_f64());
        let mut normalized = angle % two_pi;
        if Self::gt_f64(normalized, Self::pi_f64()) {
            normalized = Self::sub_f64(normalized, two_pi);
        } else if Self::lt_f64(normalized, Self::neg_f64(Self::pi_f64())) {
            normalized = Self::add_f64(normalized, two_pi);
        }
        normalized
    }
}

// Implement mesh traits for primitive types
impl MeshAdd for f64 {
    type Output = f64;
    fn mesh_add(self, rhs: f64) -> f64 {
        MeshMath::add_f64(self, rhs)
    }
}

impl MeshSub for f64 {
    type Output = f64;
    fn mesh_sub(self, rhs: f64) -> f64 {
        MeshMath::sub_f64(self, rhs)
    }
}

impl MeshMul for f64 {
    type Output = f64;
    fn mesh_mul(self, rhs: f64) -> f64 {
        MeshMath::mul_f64(self, rhs)
    }
}

impl MeshDiv for f64 {
    type Output = f64;
    fn mesh_div(self, rhs: f64) -> f64 {
        MeshMath::div_f64(self, rhs)
    }
}

impl MeshNeg for f64 {
    type Output = f64;
    fn mesh_neg(self) -> f64 {
        MeshMath::neg_f64(self)
    }
}

// Implement mesh traits for isize
impl MeshAdd for isize {
    type Output = isize;
    fn mesh_add(self, rhs: isize) -> isize {
        MeshMath::add_isize(self, rhs)
    }
}

impl MeshSub for isize {
    type Output = isize;
    fn mesh_sub(self, rhs: isize) -> isize {
        MeshMath::sub_isize(self, rhs)
    }
}

impl MeshMul for isize {
    type Output = isize;
    fn mesh_mul(self, rhs: isize) -> isize {
        MeshMath::mul_isize(self, rhs)
    }
}

impl MeshDiv for isize {
    type Output = isize;
    fn mesh_div(self, rhs: isize) -> isize {
        MeshMath::div_isize(self, rhs)
    }
}

impl MeshNeg for isize {
    type Output = isize;
    fn mesh_neg(self) -> isize {
        MeshMath::neg_isize(self)
    }
}
