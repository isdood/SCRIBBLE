//! Facet Type Definitions
//! Author: @isdood
//! Created: 2025-01-21 13:13:14 UTC

use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for crystal computations
pub type Result<T> = std::result::Result<T, FacetError>;

/// Error types for crystal operations
#[derive(Error, Debug, Clone)]
pub enum FacetError {
    #[error("Crystal clarity too low: {0}")]
    LowClarity(f64),
    #[error("Resonance disruption: {0}")]
    ResonanceDisruption(f64),
    #[error("Computation failed: {0}")]
    ComputeError(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Memory allocation failed")]
    AllocationError,
    #[error("Bridge error: {0}")]
    BridgeError(String),
}

/// 3D vector for crystal coordinates
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    /// Create new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Convert slice to Vec3d
    pub fn from_slice(slice: &[u8]) -> Self {
        if slice.len() >= 24 {
            let x = f64::from_le_bytes(slice[0..8].try_into().unwrap());
            let y = f64::from_le_bytes(slice[8..16].try_into().unwrap());
            let z = f64::from_le_bytes(slice[16..24].try_into().unwrap());
            Self { x, y, z }
        } else {
            Self::default()
        }
    }

    /// Convert Vec3d to vector
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(24);
        vec.extend_from_slice(&self.x.to_le_bytes());
        vec.extend_from_slice(&self.y.to_le_bytes());
        vec.extend_from_slice(&self.z.to_le_bytes());
        vec
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize vector
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag != 0.0 {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }
}

impl Default for Vec3d {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add for Vec3d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3d {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f64> for Vec3d {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        if scalar == 0.0 {
            Self::default()
        } else {
            Self {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
            }
        }
    }
}

/// Crystal metrics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CrystalMetrics {
    /// Crystal clarity level
    pub clarity: f64,
    /// Resonance strength
    pub resonance: f64,
    /// Pattern stability
    pub stability: f64,
    /// Computation timestamp
    pub timestamp: i64,
}

impl Default for CrystalMetrics {
    fn default() -> Self {
        Self {
            clarity: 1.0,
            resonance: 1.0,
            stability: 1.0,
            timestamp: 0,
        }
    }
}

/// Computation operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "Add"),
            Operation::Subtract => write!(f, "Subtract"),
            Operation::Multiply => write!(f, "Multiply"),
            Operation::Divide => write!(f, "Divide"),
            Operation::Power => write!(f, "Power"),
            Operation::Root => write!(f, "Root"),
        }
    }
}

/// Crystal pattern type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    /// Basic crystal pattern
    Basic,
    /// Enhanced resonance pattern
    Resonant,
    /// High clarity pattern
    Clear,
    /// Quantum entangled pattern
    Quantum,
    /// Whimsical pattern
    Whimsy,
}

impl Pattern {
    /// Get pattern efficiency
    pub fn efficiency(&self) -> f64 {
        match self {
            Pattern::Basic => 1.0,
            Pattern::Resonant => 1.2,
            Pattern::Clear => 1.15,
            Pattern::Quantum => 1.3,
            Pattern::Whimsy => 1.25,
        }
    }
}

/// Computation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeConfig {
    /// Minimum clarity requirement
    pub min_clarity: f64,
    /// Minimum resonance requirement
    pub min_resonance: f64,
    /// Enable SIMD operations
    pub enable_simd: bool,
    /// Enable whimsy
    pub enable_whimsy: bool,
    /// Pattern type
    pub pattern: Pattern,
}

impl Default for ComputeConfig {
    fn default() -> Self {
        Self {
            min_clarity: 0.85,
            min_resonance: 0.85,
            enable_simd: true,
            enable_whimsy: false,
            pattern: Pattern::Basic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3d_operations() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(2.0, 3.0, 4.0);

        let sum = v1 + v2;
        assert_eq!(sum, Vec3d::new(3.0, 5.0, 7.0));

        let scaled = v1 * 2.0;
        assert_eq!(scaled, Vec3d::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_pattern_efficiency() {
        assert!(Pattern::Quantum.efficiency() > Pattern::Basic.efficiency());
        assert!(Pattern::Whimsy.efficiency() > Pattern::Basic.efficiency());
    }

    #[test]
    fn test_vec3d_serialization() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        let data = v.to_vec();
        let v2 = Vec3d::from_slice(&data);
        assert_eq!(v, v2);
    }
}
