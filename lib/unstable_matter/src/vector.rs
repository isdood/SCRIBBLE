/// Vector Space Module
/// Last Updated: 2025-01-15 04:32:42 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    phantom::{Quantum, QuantumCell},
    helium::Helium,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

#[derive(Debug, Clone, Copy)]
pub struct Vector3D<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Copy> Vector3D<T> {
    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }
}

impl<T: Add<Output = T>> Add for Vector3D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Mul<f64, Output = T>> Mul<f64> for Vector3D<T> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag < f64::EPSILON {
            return Self::new(0.0, 0.0, 0.0);
        }
        Self::new(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        libm::sqrt(dx * dx + dy * dy + dz * dz)
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.magnitude() > PLANCK_LENGTH
    }
}

impl Vector3D<isize> {
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        libm::sqrt(dx * dx + dy * dy + dz * dz)
    }

    pub fn to_f64(&self) -> Vector3D<f64> {
        Vector3D::new(
            self.x as f64,
            self.y as f64,
            self.z as f64,
        )
    }
}

#[derive(Debug)]
pub struct Vector4D<T: 'static> {
    t: T,
    x: T,
    y: T,
    z: T,
    coherence: QuantumCell<f64>,
    timestamp: Helium<usize>,
}

impl<T: 'static> Quantum for Vector4D<T> {
    fn get_coherence(&self) -> f64 {
        self.coherence.get_coherence()
    }

    fn is_quantum_stable(&self) -> bool {
        self.coherence.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        self.coherence.decay_coherence();
    }

    fn reset_coherence(&self) {
        self.coherence.reset_coherence();
    }
}

impl Clone for Vector4D<f64> {
    fn clone(&self) -> Self {
        Self {
            t: self.t,
            x: self.x,
            y: self.y,
            z: self.z,
            aligned_space: self.aligned_space.clone(),
        }
    }
}

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

        let mut result = Self {
            t: gamma * (self.t - beta * self.x / LIGHT_SPEED),
            x: gamma * (self.x - beta * self.t * LIGHT_SPEED),
            y: self.y,
            z: self.z,
            aligned_space: self.aligned_space.clone(),
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

        let mut result = Self {
            t: libm::floor(self.t / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            x: libm::floor(self.x / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            y: libm::floor(self.y / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            z: libm::floor(self.z / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            aligned_space: self.aligned_space.clone(),
        };
        result.aligned_space.reset_coherence();
        result
    }

    pub fn quantum_coherence(&self, other: &Self) -> f64 {
        (self.aligned_space.get_coherence() + other.aligned_space.get_coherence()) / 2.0
    }
}

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
    fn test_vector_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(2.0, 3.0, 4.0);

        let sum = v1.clone() + v2.clone();
        assert_eq!(sum.x(), 3.0);
        assert_eq!(sum.y(), 5.0);
        assert_eq!(sum.z(), 7.0);

        let scaled = v1 * 2.0;
        assert_eq!(scaled.x(), 2.0);
        assert_eq!(scaled.y(), 4.0);
        assert_eq!(scaled.z(), 6.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector3D::new(3.0, 0.0, 0.0);
        let normalized = v.normalize();
        assert_eq!(normalized.x(), 1.0);
        assert_eq!(normalized.magnitude(), 1.0);
    }

    #[test]
    fn test_quantum_stability() {
        let v1 = Vector3D::new(PLANCK_LENGTH * 2.0, 0.0, 0.0);
        assert!(v1.is_quantum_stable());

        let v2 = Vector3D::new(PLANCK_LENGTH * 0.5, 0.0, 0.0);
        assert!(!v2.is_quantum_stable());
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
