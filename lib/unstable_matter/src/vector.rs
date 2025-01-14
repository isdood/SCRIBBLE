// lib/unstable_matter/src/vector.rs
/// Last Updated: 2025-01-14 20:42:13 UTC
/// Author: isdood
/// Current User: isdood

// lib/unstable_matter/src/vector.rs

use core::cell::UnsafeCell;
use core::ops::{Add, Sub, Mul};

use crate::{
    align::Alignment,
    constants::{
        CURRENT_TIMESTAMP,
        PLANCK_LENGTH,
        QUANTUM_THRESHOLD,
        LIGHT_SPEED,
    },
};

#[derive(Debug)]
pub struct Vector3D<T> {
    x: T,
    y: T,
    z: T,
    timestamp: UnsafeCell<usize>,
    alignment: Alignment,
}

impl<T> Vector3D<T>
where
T: PartialEq + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            timestamp: UnsafeCell::new(CURRENT_TIMESTAMP),
            alignment: Alignment::new(8),
        }
    }

    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }

    pub fn set_x(&mut self, x: T) {
        self.x = x;
        unsafe { *self.timestamp.get() = CURRENT_TIMESTAMP; }
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y;
        unsafe { *self.timestamp.get() = CURRENT_TIMESTAMP; }
    }

    pub fn set_z(&mut self, z: T) {
        self.z = z;
        unsafe { *self.timestamp.get() = CURRENT_TIMESTAMP; }
    }

    pub fn quantum_coherence(&self) -> f64 {
        self.alignment.get_coherence()
    }

    pub fn magnitude(&self) -> T
    where T: Mul<Output = T> + Add<Output = T> {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.quantum_coherence() > QUANTUM_THRESHOLD
    }
}

impl<T: Clone> Clone for Vector3D<T>
where T: PartialEq + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: PartialEq> PartialEq for Vector3D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

// Implement Vector3D<f64> specific operations
impl Vector3D<f64> {
    pub fn magnitude(&mut self) -> f64 {
        self.update_timestamp();
        self.aligned_space.decay_coherence();
        libm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&mut self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            return self.clone();
        }
        let mut result = self.clone();
        result.x /= mag;
        result.y /= mag;
        result.z /= mag;
        result.update_timestamp();
        result
    }

    pub fn dot(&mut self, other: &mut Self) -> f64 {
        self.update_timestamp();
        other.update_timestamp();
        self.aligned_space.decay_coherence();
        other.aligned_space.decay_coherence();
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&mut self, other: &mut Self) -> Self {
        let alignment = vector_align();
        let mut result = Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            timestamp: UnsafeCell::new(MESH_TIMESTAMP),
            aligned_space: Box::new(AlignedSpace::new(
                MESH_TIMESTAMP,
                core::mem::size_of::<f64>() * 3,
                                                      alignment,
            )),
        };
        result.update_timestamp();
        result.aligned_space.reset_coherence();
        result
    }

    pub fn quantum_distance(&mut self, other: &mut Self) -> (f64, f64) {
        self.update_timestamp();
        other.update_timestamp();

        let distance = {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            let dz = self.z - other.z;
            libm::sqrt(dx * dx + dy * dy + dz * dz)
        };

        let space_coherence = (self.aligned_space.get_coherence()
        + other.aligned_space.get_coherence())
        / 2.0;

        let t1 = self.get_timestamp();
        let t2 = other.get_timestamp();
        let time_diff = if t1 > t2 { t1 - t2 } else { t2 - t1 } as f64;

        let time_coherence = (VECTOR_QUANTUM_STATE as f64)
        / (time_diff + VECTOR_QUANTUM_STATE as f64);

        let coherence = (space_coherence + time_coherence) / 2.0;
        (distance, coherence)
    }

    pub fn is_quantum(&self) -> bool {
        let mag = self.x * self.x + self.y * self.y + self.z * self.z;
        mag < PLANCK_LENGTH * PLANCK_LENGTH && self.aligned_space.is_quantum_stable()
    }

    pub fn quantize(&mut self) -> Self {
        if !self.is_quantum() {
            return self.clone();
        }

        let alignment = vector_align();
        let mut result = Self {
            x: libm::floor(self.x / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            y: libm::floor(self.y / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            z: libm::floor(self.z / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            timestamp: UnsafeCell::new(MESH_TIMESTAMP),
            aligned_space: Box::new(AlignedSpace::new(
                MESH_TIMESTAMP,
                core::mem::size_of::<f64>() * 3,
                                                      alignment,
            )),
        };
        result.update_timestamp();
        result.aligned_space.reset_coherence();
        result
    }

    pub fn quantum_coherence(&self, other: &Self) -> f64 {
        let space_coherence = (self.aligned_space.get_coherence()
        + other.aligned_space.get_coherence())
        / 2.0;

        let t1 = self.get_timestamp();
        let t2 = other.get_timestamp();
        let time_diff = if t1 > t2 { t1 - t2 } else { t2 - t1 } as f64;

        let time_coherence = (VECTOR_QUANTUM_STATE as f64)
        / (time_diff + VECTOR_QUANTUM_STATE as f64);

        (space_coherence + time_coherence) / 2.0
    }
}

// Implement standard arithmetic operations
impl<T: Copy + Add<Output = T>> Add for Vector3D<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        let mut result = Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        );
        result.update_timestamp();
        result
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        let mut result = Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        );
        result.update_timestamp();
        result
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector4D<T> {
    pub t: T,
    pub x: T,
    pub y: T,
    pub z: T,
    aligned_space: Box<AlignedSpace>,
}

impl<T> Vector4D<T>
where
T: PartialEq + Add<Output = T> + Mul<Output = T> + Copy + Default,
{
    pub fn new(t: T, x: T, y: T, z: T) -> Self {
        let alignment = vector_align();
        let aligned_space = Box::new(AlignedSpace::new(
            MESH_TIMESTAMP,
            core::mem::size_of::<T>() * 4,
                                                       alignment,
        ));

        Self { t, x, y, z, aligned_space }
    }

    pub fn inner_product(&mut self, other: &Self) -> T
    where
    T: Sub<Output = T>,
    {
        self.aligned_space.decay_coherence();

        let spatial = self.x * other.x + self.y * other.y + self.z * other.z;
        spatial - (self.t * other.t) // Note the minus sign for time component
    }

    pub fn interval_squared(&mut self) -> T
    where
    T: Sub<Output = T>,
    {
        self.aligned_space.decay_coherence();
        let copy = *self; // Create a copy since T implements Copy
        self.inner_product(&copy)
    }
}

/// Specialized implementation for f64 vectors
impl Vector4D<f64> {
    pub fn proper_time(&mut self) -> f64 {
        self.aligned_space.decay_coherence();
        let interval = self.interval_squared();
        if interval < -PLANCK_LENGTH {
            libm::sqrt(-interval)
        } else {
            0.0
        }
    }

    pub fn boost_x(&mut self, beta: f64) -> Self {
        assert!(beta.abs() < 1.0, "Speed must be less than light speed");
        let gamma = 1.0 / libm::sqrt(1.0 - beta * beta);

        let alignment = vector_align();
        let aligned_space = Box::new(AlignedSpace::new(
            MESH_TIMESTAMP,
            core::mem::size_of::<f64>() * 4,
                                                       alignment,
        ));

        let mut result = Self {
            t: gamma * (self.t - beta * self.x / LIGHT_SPEED),
            x: gamma * (self.x - beta * self.t * LIGHT_SPEED),
            y: self.y,
            z: self.z,
            aligned_space,
        };
        result.aligned_space.reset_coherence();
        result
    }

    pub fn is_timelike(&mut self) -> bool {
        self.aligned_space.decay_coherence();
        self.interval_squared() < -QUANTUM_THRESHOLD
    }

    pub fn is_spacelike(&mut self) -> bool {
        self.aligned_space.decay_coherence();
        self.interval_squared() > QUANTUM_THRESHOLD
    }

    pub fn is_null(&mut self) -> bool {
        self.aligned_space.decay_coherence();
        self.interval_squared().abs() < QUANTUM_THRESHOLD
    }

    pub fn to_quantum(&self) -> Helium<Self> {
        self.aligned_space.reset_coherence();
        Helium::new(self.clone())
    }

    pub fn spatial_part(&mut self) -> Vector3D<f64> {
        self.aligned_space.decay_coherence();
        Vector3D::new(self.x, self.y, self.z)
    }

    pub fn magnitude(&mut self) -> f64 {
        self.aligned_space.decay_coherence();
        let squared = self.x * self.x + self.y * self.y +
        self.z * self.z - self.t * self.t * LIGHT_SPEED * LIGHT_SPEED;
        if squared.abs() < PLANCK_LENGTH * PLANCK_LENGTH {
            PLANCK_LENGTH
        } else if squared < 0.0 {
            0.0
        } else {
            libm::sqrt(squared)
        }
    }

    pub fn is_quantum(&mut self) -> bool {
        self.magnitude() < PLANCK_LENGTH && self.aligned_space.is_quantum_stable()
    }

    pub fn quantize(&mut self) -> Self {
        if !self.is_quantum() {
            return self.clone();
        }

        let alignment = vector_align();
        let aligned_space = Box::new(AlignedSpace::new(
            MESH_TIMESTAMP,
            core::mem::size_of::<f64>() * 4,
                                                       alignment,
        ));

        let mut result = Self {
            t: libm::floor(self.t / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            x: libm::floor(self.x / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            y: libm::floor(self.y / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            z: libm::floor(self.z / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            aligned_space,
        };
        result.aligned_space.reset_coherence();
        result
    }

    pub fn quantum_coherence(&self, other: &Self) -> f64 {
        (self.aligned_space.get_coherence()
        + other.aligned_space.get_coherence())
        / 2.0
    }
}

/// Standard arithmetic operations
impl<T: PartialEq + Add<Output = T>> Add for Vector4D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            t: self.t + other.t,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            aligned_space: self.aligned_space.clone(),
        }
    }
}

impl<T: PartialEq + Sub<Output = T>> Sub for Vector4D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            t: self.t - other.t,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            aligned_space: self.aligned_space.clone(),
        }
    }
}

impl<T: PartialEq + Mul<Output = T> + Copy> Mul<T> for Vector4D<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            t: self.t * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            aligned_space: self.aligned_space.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector4d_creation() {
        let v = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.t, 1.0);
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 4.0);
    }

    #[test]
    fn test_vector3d_magnitude() {
        let mut v = Vector3D::new(1.0, 2.0, 2.0);
        assert_eq!(v.magnitude(), 3.0);
    }

    #[test]
    fn test_vector3d_normalize() {
        let mut v = Vector3D::new(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        assert!((normalized.x - 0.6).abs() < 1e-6);
        assert!((normalized.y - 0.8).abs() < 1e-6);
        assert!((normalized.z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_vector3d_dot() {
        let mut v1 = Vector3D::new(1.0, 2.0, 3.0);
        let mut v2 = Vector3D::new(4.0, -5.0, 6.0);
        assert_eq!(v1.dot(&mut v2), 12.0);
    }

    #[test]
    fn test_vector3d_cross() {
        let mut v1 = Vector3D::new(1.0, 2.0, 3.0);
        let mut v2 = Vector3D::new(4.0, 5.0, 6.0);
        let v3 = v1.cross(&mut v2);
        assert_eq!(v3.x, -3.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, -3.0);
    }

    #[test]
    fn test_vector4d_proper_time() {
        let mut v = Vector4D::new(1.0, 2.0, 2.0, 2.0);
        assert!(v.proper_time() > 0.0);
    }

    #[test]
    fn test_vector4d_boost_x() {
        let mut v = Vector4D::new(1.0, 0.5, 0.0, 0.0);
        let boosted = v.boost_x(0.5);
        assert!(boosted.is_timelike());
        assert_eq!(
            v.interval_squared(),
                   boosted.interval_squared(),
                   "Lorentz invariance violation"
        );
    }

    #[test]
    fn test_vector4d_quantum_coherence() {
        let v1 = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        let v2 = Vector4D::new(0.0, 1.0, 0.0, 0.0);
        let coherence = v1.quantum_coherence(&v2);
        assert!(coherence > 0.0);
    }
}
