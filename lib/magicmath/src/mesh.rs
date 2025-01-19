//! Mesh Mathematics Implementation for Crystal Lattice Systems
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 19:02:24 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use crate::constants::{
    HARMONY_STABILITY_THRESHOLD,
    mesh::{RESOLUTION_MIN, RESOLUTION_MAX, PRECISION_DEFAULT},
};
use errors::core::{Error as MathError, ErrorKind};
use scribe::native_string::String;

/// Mesh mathematics operations for crystal lattice systems
#[derive(Debug, Clone)]
pub struct MeshMath {
    /// Resolution of the crystal mesh grid
    resolution: usize,
    /// Precision for numerical operations
    precision: f64,
    /// Current stability factor
    stability: f64,
}

impl MeshMath {
    /// Create new mesh math instance with specified resolution
    pub fn new(resolution: usize) -> Result<Self, MathError> {
        if resolution < RESOLUTION_MIN || resolution > RESOLUTION_MAX {
            return Err(MathError::new(
                ErrorKind::InvalidInput,
                String::from("Mesh resolution out of valid range")
            ));
        }

        Ok(Self {
            resolution,
            precision: PRECISION_DEFAULT,
            stability: 1.0,
        })
    }

    /// Set mesh precision for calculations
    pub fn set_precision(&mut self, precision: f64) -> Result<(), MathError> {
        if precision <= 0.0 {
            return Err(MathError::new(
                ErrorKind::InvalidInput,
                String::from("Precision must be positive")
            ));
        }
        self.precision = precision;
        Ok(())
    }

    /// Get current mesh resolution
    pub fn resolution(&self) -> usize {
        self.resolution
    }

    /// Get current precision setting
    pub fn precision(&self) -> f64 {
        self.precision
    }

    /// Get current stability factor
    pub fn stability(&self) -> f64 {
        self.stability
    }

    /// Interpolate value on crystal mesh
    pub fn interpolate<T: MeshValue>(&mut self, value: T) -> Result<T, MathError> {
        let val = value.to_f64()?;

        // Check for invalid values
        if val.is_nan() || val.is_infinite() {
            return Err(MathError::new(
                ErrorKind::InvalidInput,
                String::from("Cannot interpolate NaN or infinite value on crystal mesh")
            ));
        }

        // Update stability based on operation
        self.stability *= 0.99;
        if self.stability < HARMONY_STABILITY_THRESHOLD {
            return Err(MathError::new(
                ErrorKind::StabilityLoss,
                String::from("Crystal mesh stability lost during interpolation")
            ));
        }

        // Perform mesh-aligned rounding
        let scale = (1.0 / self.precision).round() as i64;
        let mesh_points = self.resolution as f64;
        let normalized = (val * mesh_points).round() / mesh_points;
        let rounded = (normalized * scale as f64).round() / scale as f64;

        Ok(T::from(rounded))
    }

    /// Reset mesh stability
    pub fn reset_stability(&mut self) {
        self.stability = 1.0;
    }
}

impl Default for MeshMath {
    fn default() -> Self {
        Self::new(100).expect("Default mesh resolution should be valid") // 100x100 default mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        assert!(MeshMath::new(50).is_ok());
        assert!(MeshMath::new(0).is_err());
        assert!(MeshMath::new(10001).is_err());
    }

    #[test]
    fn test_interpolation() {
        let mut mesh = MeshMath::default();
        assert!(mesh.interpolate(1.234567f64).is_ok());
        assert!(mesh.interpolate(f64::NAN).is_err());
        assert!(mesh.interpolate(f64::INFINITY).is_err());
    }

    #[test]
    fn test_precision_setting() {
        let mut mesh = MeshMath::default();
        assert!(mesh.set_precision(0.001).is_ok());
        assert!(mesh.set_precision(0.0).is_err());
        assert!(mesh.set_precision(-1.0).is_err());
    }

    #[test]
    fn test_stability_tracking() {
        let mut mesh = MeshMath::default();
        for _ in 0..100 {
            let _ = mesh.interpolate(1.0f64);
        }
        assert!(mesh.stability() < 1.0);
        mesh.reset_stability();
        assert_eq!(mesh.stability(), 1.0);
    }
}
