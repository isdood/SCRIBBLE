/// Quantum Vector Module
/// Last Updated: 2025-01-16 02:56:21 UTC
/// Author: isdood
/// Current User: isdood

use std::ops::{Add, Sub, Mul};
use crate::{
    constants::*,
    quantum::Quantum,
    scribe::{Scribe, ScribePrecision, QuantumString},
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

    pub fn x(&self) -> T where T: Copy { self.x }
    pub fn y(&self) -> T where T: Copy { self.y }
    pub fn z(&self) -> T where T: Copy { self.z }
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

impl<T: Copy + PartialEq + Add<Output = T>> Vector4D<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }
    pub fn w(&self) -> T { self.w }
}

impl<T: Copy + PartialEq + Add<Output = T>> Add for Vector4D<T> {
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

impl<T: Copy + PartialEq + Sub<Output = T>> Sub for Vector4D<T> {
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

impl<T: Copy + PartialEq + Mul<Output = T>> Mul<T> for Vector4D<T> {
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
