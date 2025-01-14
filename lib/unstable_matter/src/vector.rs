/// Vector4D Implementation for Physical Spacetime
/// Last Updated: 2025-01-14 06:13:43 UTC
/// Author: isdood
/// Current User: isdood

use core::{
    ops::{Add, Sub, Mul},
    marker::Copy,
    default::Default,
        cmp::PartialEq,
};

use crate::{
    helium::Helium,
    QUANTUM_TIMESTAMP,
};

const LIGHT_SPEED: f64 = 299_792_458.0; // m/s
const PLANCK_LENGTH: f64 = 1.616255e-35; // meters
const QUANTUM_THRESHOLD: f64 = 1e-10;

/// 3D Vector implementation for spatial components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<T: PartialEq> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T>
where
T: PartialEq + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        libm::sqrt(self.magnitude_squared())
    }
}

/// 4D Vector implementation specialized for quantum spacetime operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4D<T: PartialEq> {
    pub t: T,  // Time component
    pub x: T,
    pub y: T,
    pub z: T,
}

// Core Vector4D implementation
impl<T> Vector4D<T>
where
T: PartialEq + Add<Output = T> + Mul<Output = T> + Copy + Default
{
    pub const fn new(t: T, x: T, y: T, z: T) -> Self {
        Self { t, x, y, z }
    }

    /// Minkowski inner product (metric signature: -+++)
    pub fn inner_product(&self, other: &Self) -> T {
        let spatial = self.x * other.x + self.y * other.y + self.z * other.z;
        spatial - (self.t * other.t) // Note the minus sign for time component
    }

    /// Squared spacetime interval
    pub fn interval_squared(&self) -> T {
        self.inner_product(self)
    }
}

// Specialized implementation for f64 with quantum-aware operations
impl Vector4D<f64> {
    /// Computes proper time interval
    pub fn proper_time(&self) -> f64 {
        let interval = self.interval_squared();
        if interval < 0.0 {
            libm::sqrt(-interval) // Timelike interval
        } else {
            0.0 // Spacelike or null interval
        }
    }

    /// Lorentz boost in x-direction
    pub fn boost_x(&self, beta: f64) -> Self {
        let gamma = 1.0 / libm::sqrt(1.0 - beta * beta);
        Self {
            t: gamma * (self.t - beta * self.x),
            x: gamma * (self.x - beta * self.t),
            y: self.y,
            z: self.z,
        }
    }

    /// Check if vector is timelike
    pub fn is_timelike(&self) -> bool {
        self.interval_squared() < 0.0
    }

    /// Check if vector is spacelike
    pub fn is_spacelike(&self) -> bool {
        self.interval_squared() > 0.0
    }

    /// Check if vector is null (lightlike)
    pub fn is_null(&self) -> bool {
        self.interval_squared().abs() < 1e-10
    }

    /// Convert to quantum state
    pub fn to_quantum(&self) -> Helium<Self> {
        Helium::new(*self)
    }

    /// Get spatial components as Vector3D
    pub fn spatial_part(&self) -> Vector3D<f64> {
        Vector3D::new(self.x, self.y, self.z)
    }
}

/// Metric tensor for 4D spacetime calculations
#[derive(Debug, Clone, Copy)]
pub struct MetricTensor {
    components: [[f64; 4]; 4],
}

impl MetricTensor {
    /// Creates Minkowski metric (flat spacetime)
    pub fn minkowski() -> Self {
        let mut components = [[0.0; 4]; 4];
        components[0][0] = -1.0; // Time component
        components[1][1] = 1.0;  // Spatial components
        components[2][2] = 1.0;
        components[3][3] = 1.0;
        Self { components }
    }

    /// Computes metric contraction with two vectors
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

// Standard operations for Vector4D
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

impl Vector4D<f64> {
    /// Computes proper time interval with quantum corrections
    pub fn proper_time(&self) -> f64 {
        let interval = self.interval_squared();
        if interval < -PLANCK_LENGTH {
            libm::sqrt(-interval) // Timelike interval with quantum correction
        } else {
            0.0 // Spacelike or null interval
        }
    }

    /// Lorentz boost in x-direction with quantum coherence
    pub fn boost_x(&self, beta: f64) -> Self {
        assert!(beta.abs() < 1.0, "Speed must be less than light speed");
        let gamma = 1.0 / libm::sqrt(1.0 - beta * beta);
        let boosted = Self {
            t: gamma * (self.t - beta * self.x / LIGHT_SPEED),
            x: gamma * (self.x - beta * self.t * LIGHT_SPEED),
            y: self.y,
            z: self.z,
        };

        // Apply quantum corrections
        if boosted.magnitude() < PLANCK_LENGTH {
            Self {
                t: 1705207623.0 * PLANCK_LENGTH, // 2025-01-14 06:13:43 UTC
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            boosted
        }
    }

    /// Check if vector is timelike (causally connected)
    pub fn is_timelike(&self) -> bool {
        self.interval_squared() < -QUANTUM_THRESHOLD
    }

    /// Check if vector is spacelike (causally separated)
    pub fn is_spacelike(&self) -> bool {
        self.interval_squared() > QUANTUM_THRESHOLD
    }

    /// Check if vector is null (lightlike) within quantum uncertainty
    pub fn is_null(&self) -> bool {
        self.interval_squared().abs() < QUANTUM_THRESHOLD
    }

    /// Convert to quantum state with coherence tracking
    pub fn to_quantum(&self) -> Helium<Self> {
        let mut quantum_state = Helium::new(*self);
        if self.magnitude() < PLANCK_LENGTH {
            quantum_state.set_coherence(0.5);
        }
        quantum_state
    }

    /// Get spatial components as Vector3D
    pub fn spatial_part(&self) -> Vector3D<f64> {
        Vector3D::new(self.x, self.y, self.z)
    }

    /// Calculate 4-vector magnitude with quantum corrections
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

    /// Check if vector represents a quantum-scale event
    pub fn is_quantum(&self) -> bool {
        self.magnitude() < PLANCK_LENGTH
    }

    /// Apply quantum corrections to coordinates
    pub fn quantize(&self) -> Self {
        if !self.is_quantum() {
            *self
        } else {
            Self {
                t: (self.t / PLANCK_LENGTH).round() * PLANCK_LENGTH,
                x: (self.x / PLANCK_LENGTH).round() * PLANCK_LENGTH,
                y: (self.y / PLANCK_LENGTH).round() * PLANCK_LENGTH,
                z: (self.z / PLANCK_LENGTH).round() * PLANCK_LENGTH,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TIMESTAMP: usize = 1705207752; // 2025-01-14 06:09:12 UTC

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
    fn test_quantum_effects() {
        let v = Vector4D::new(PLANCK_LENGTH/2.0, PLANCK_LENGTH/2.0, 0.0, 0.0);
        assert!(v.is_quantum());

        let quantized = v.quantize();
        assert_eq!(quantized.x, 0.0);
        assert_eq!(quantized.y, 0.0);
    }

    #[test]
    fn test_quantum_coherence() {
        let v = Vector4D::new(1.0, 1.0, 1.0, 1.0);
        let quantum_state = v.to_quantum();
        assert!(quantum_state.get_coherence() > 0.5);

        let tiny_v = Vector4D::new(
            PLANCK_LENGTH/2.0,
            PLANCK_LENGTH/2.0,
            PLANCK_LENGTH/2.0,
            PLANCK_LENGTH/2.0
        );
        let quantum_tiny = tiny_v.to_quantum();
        assert!(quantum_tiny.get_coherence() <= 0.5);
    }
}
