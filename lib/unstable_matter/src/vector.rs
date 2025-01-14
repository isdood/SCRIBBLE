/// Vector-Mesh Memory System Implementation
/// Last Updated: 2025-01-14 15:18:29 UTC
/// Author: isdood
/// Current User: isdood

use core::{
    ops::{Add, Sub, Mul},
    marker::Copy,
    default::Default,
};

use libm;
use crate::helium::Helium;

const LIGHT_SPEED: f64 = 299_792_458.0; // m/s
const PLANCK_LENGTH: f64 = 1.616255e-35; // meters
const QUANTUM_THRESHOLD: f64 = 1e-10;
const MESH_TIMESTAMP: f64 = 1705241909.0; // 2025-01-14 15:18:29 UTC

/// 3D Vector with coherence tracking for mesh memory
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// 4D Spacetime Vector with coherence tracking for mesh memory
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4D<T> {
    pub t: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Standard operations for Vector3D
impl<T> Vector3D<T>
where
T: PartialEq + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/// Specialized implementation for Vector3D<f64>
impl Vector3D<f64> {
    /// Calculate magnitude of 3D vector
    pub fn magnitude(&self) -> f64 {
        let squared = self.x * self.x + self.y * self.y + self.z * self.z;
        libm::sqrt(squared)
    }
}


/// Standard operations for Vector4D
impl<T> Vector4D<T>
where
T: PartialEq + Add<Output = T> + Mul<Output = T> + Copy + Default
{
    pub const fn new(t: T, x: T, y: T, z: T) -> Self {
        Self { t, x, y, z }
    }

    /// Minkowski inner product (metric signature: -+++)
    pub fn inner_product(&self, other: &Self) -> T
    where
    T: Sub<Output = T>
    {
        let spatial = self.x * other.x + self.y * other.y + self.z * other.z;
        spatial - (self.t * other.t) // Note the minus sign for time component
    }

    /// Squared spacetime interval
    pub fn interval_squared(&self) -> T
    where
    T: Sub<Output = T>
    {
        self.inner_product(self)
    }
}

/// Specialized implementation for f64 vectors
impl Vector4D<f64> {
    /// Computes proper time interval
    pub fn proper_time(&self) -> f64 {
        let interval = self.interval_squared();
        if interval < -PLANCK_LENGTH {
            libm::sqrt(-interval)
        } else {
            0.0
        }
    }

    /// Lorentz boost in x-direction
    pub fn boost_x(&self, beta: f64) -> Self {
        assert!(beta.abs() < 1.0, "Speed must be less than light speed");
        let gamma = 1.0 / libm::sqrt(1.0 - beta * beta);
        Self {
            t: gamma * (self.t - beta * self.x / LIGHT_SPEED),
            x: gamma * (self.x - beta * self.t * LIGHT_SPEED),
            y: self.y,
            z: self.z,
        }
    }

    /// Check if vector is timelike
    pub fn is_timelike(&self) -> bool {
        self.interval_squared() < -QUANTUM_THRESHOLD
    }

    /// Check if vector is spacelike
    pub fn is_spacelike(&self) -> bool {
        self.interval_squared() > QUANTUM_THRESHOLD
    }

    /// Check if vector is null (lightlike)
    pub fn is_null(&self) -> bool {
        self.interval_squared().abs() < QUANTUM_THRESHOLD
    }

    /// Convert to quantum state
    pub fn to_quantum(&self) -> Helium<Self> {
        Helium::new(*self)
    }

    /// Get spatial components
    pub fn spatial_part(&self) -> Vector3D<f64> {
        Vector3D::new(self.x, self.y, self.z)
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> f64 {
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

    /// Check if quantum scale
    pub fn is_quantum(&self) -> bool {
        self.magnitude() < PLANCK_LENGTH
    }

    /// Apply quantum corrections
    pub fn quantize(&self) -> Self {
        if !self.is_quantum() {
            *self
        } else {
            Self {
                t: libm::floor(self.t / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
                x: libm::floor(self.x / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
                y: libm::floor(self.y / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
                z: libm::floor(self.z / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            }
        }
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
        }
    }
}

/// Metric tensor for spacetime calculations
#[derive(Debug, Clone, Copy)]
pub struct MetricTensor {
    components: [[f64; 4]; 4],
}

impl MetricTensor {
    /// Creates Minkowski metric
    pub fn minkowski() -> Self {
        let mut components = [[0.0; 4]; 4];
        components[0][0] = -1.0; // Time component
        components[1][1] = 1.0;  // Spatial components
        components[2][2] = 1.0;
        components[3][3] = 1.0;
        Self { components }
    }

    /// Metric contraction
    pub fn contract(&self, v1: &Vector4D<f64>, v2: &Vector4D<f64>) -> f64 {
        let v1_components = [v1.t, v1.x, v1.y, v1.z];
        let v2_components = [v2.t, v2.x, v2.y, v2.z];

        let mut result = 0.0;
        for i in 0..4 {
            for j in 0..4 {
                result += self.components[i][j] * v1_components[i] * v2_components[j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_time() {
        let v = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        assert!(v.proper_time() > 0.0);

        let quantum_v = Vector4D::new(PLANCK_LENGTH/2.0, 0.0, 0.0, 0.0);
        assert_eq!(quantum_v.proper_time(), 0.0);
    }

    #[test]
    fn test_boost() {
        let v = Vector4D::new(1.0, 0.5, 0.0, 0.0);
        let boosted = v.boost_x(0.5);
        assert!(boosted.is_timelike());
        assert_eq!(
            v.interval_squared(),
                   boosted.interval_squared(),
                   "Lorentz invariance violation"
        );
    }

    #[test]
    fn test_metric_tensor() {
        let metric = MetricTensor::minkowski();
        let v1 = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        let v2 = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(metric.contract(&v1, &v2), -1.0);
    }
}
