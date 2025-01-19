//! Mesh Mathematics Implementation for Crystal Lattice Systems
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 20:47:51 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::MeshValue;
use crate::constants::{
    mesh::{
        RESOLUTION_MIN,
        RESOLUTION_MAX,
        PRECISION_DEFAULT,
        STABILITY_THRESHOLD,
        COHERENCE_THRESHOLD,
        PRECISION_THRESHOLD,
        CACHE_LINE_SIZE,
        PREFETCH_DISTANCE,
    },
    HARMONY_STABILITY_THRESHOLD,
};
use errors::{MathError, MathResult};
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
    /// Current coherence level
    coherence: f64,
}

impl MeshMath {
    /// Create new mesh math instance with specified resolution
    pub fn new(resolution: usize) -> MathResult<Self> {
        if resolution < RESOLUTION_MIN || resolution > RESOLUTION_MAX {
            return Err(MathError::InvalidParameter(format!(
                "Mesh resolution must be between {} and {}",
                RESOLUTION_MIN, RESOLUTION_MAX
            )));
        }

        Ok(Self {
            resolution,
            precision: PRECISION_DEFAULT,
            stability: 1.0,
            coherence: 1.0,
        })
    }

    /// Set mesh precision for calculations
    pub fn set_precision(&mut self, precision: f64) -> MathResult<()> {
        if precision <= 0.0 || precision < PRECISION_THRESHOLD {
            return Err(MathError::InvalidParameter(
                "Precision must be positive and above threshold".to_string()
            ));
        }
        self.precision = precision;
        Ok(())
    }

    /// Get current mesh resolution
    #[inline]
    pub fn resolution(&self) -> usize {
        self.resolution
    }

    /// Get current precision setting
    #[inline]
    pub fn precision(&self) -> f64 {
        self.precision
    }

    /// Get current stability factor
    #[inline]
    pub fn stability(&self) -> f64 {
        self.stability
    }

    /// Get current coherence level
    #[inline]
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Check if mesh state is stable
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.stability >= STABILITY_THRESHOLD &&
        self.coherence >= COHERENCE_THRESHOLD
    }

    /// Interpolate value on crystal mesh
    pub fn interpolate<T: MeshValue>(&mut self, value: T) -> MathResult<T> {
        let val = value.to_f64()?;

        // Check for invalid values
        if val.is_nan() || val.is_infinite() {
            return Err(MathError::InvalidParameter(
                "Cannot interpolate NaN or infinite value on crystal mesh".to_string()
            ));
        }

        // Update stability and coherence
        self.stability *= HARMONY_STABILITY_THRESHOLD.sqrt();
        self.coherence *= 0.99;

        // Check stability conditions
        if !self.is_stable() {
            return Err(MathError::HarmonyStateUnstable);
        }

        // Perform mesh-aligned rounding
        let scale = (1.0 / self.precision).round() as i64;
        let mesh_points = self.resolution as f64;
        let normalized = (val * mesh_points).round() / mesh_points;
        let rounded = (normalized * scale as f64).round() / scale as f64;

        Ok(T::from(rounded))
    }

    /// Reset mesh stability and coherence
    pub fn reset_state(&mut self) {
        self.stability = 1.0;
        self.coherence = 1.0;
    }

    /// Prefetch mesh data into cache (noop in stable rust)
    #[inline]
    pub fn prefetch(&self, _index: usize) {
        // Prefetching disabled in stable rust
    }
}

impl Default for MeshMath {
    fn default() -> Self {
        Self::new(100).expect("Default mesh resolution should be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        assert!(MeshMath::new(50).is_ok());
        assert!(MeshMath::new(0).is_err());
        assert!(MeshMath::new(RESOLUTION_MAX + 1).is_err());
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
        assert!(mesh.is_stable());

        for _ in 0..100 {
            let _ = mesh.interpolate(1.0f64);
        }

        assert!(mesh.stability() < 1.0);
        assert!(mesh.coherence() < 1.0);

        mesh.reset_state();
        assert_eq!(mesh.stability(), 1.0);
        assert_eq!(mesh.coherence(), 1.0);
    }
}
