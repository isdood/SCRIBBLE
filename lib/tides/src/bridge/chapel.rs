//! Chapel FFI bridge for wave distribution and crystal mesh operations
//! Created: 2025-01-21 13:56:23 UTC
//! Author: @isdood

use std::{
    ffi::{c_void, CStr, CString},
    sync::Arc,
    os::raw::{c_char, c_int, c_double},
};

use anyhow::{bail, Result};
use parking_lot::RwLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChapelError {
    #[error("Chapel runtime initialization failed")]
    InitializationError,
    #[error("Wave distribution failed: {0}")]
    WaveDistributionError(String),
    #[error("Mesh synchronization failed: {0}")]
    MeshSyncError(String),
    #[error("Invalid crystal configuration: {0}")]
    ConfigurationError(String),
    #[error("Memory allocation failed")]
    AllocationError,
}

/// External Chapel functions
#[link(name = "chapel")]
extern "C" {
    fn chapel_init() -> c_int;
    fn chapel_finalize() -> c_int;
    fn create_wave_mesh(size: c_int, dimensions: c_int) -> *mut c_void;
    fn destroy_wave_mesh(mesh: *mut c_void);
    fn distribute_wave_pattern(
        mesh: *mut c_void,
        pattern: *const c_double,
        length: c_int,
        resonance: c_double,
    ) -> c_int;
    fn synchronize_crystal_mesh(
        mesh: *mut c_void,
        harmonics: c_int,
        threshold: c_double,
    ) -> c_int;
    fn get_mesh_status(mesh: *mut c_void, status: *mut c_char) -> c_int;
}

/// Chapel bridge for crystal wave operations
pub struct ChapelBridge {
    mesh: *mut c_void,
    size: i32,
    dimensions: i32,
    config: CrystalConfig,
    status: Arc<RwLock<MeshStatus>>,
}

/// Crystal mesh configuration
#[derive(Debug, Clone)]
pub struct CrystalConfig {
    pub harmonics: i32,
    pub resonance_threshold: f64,
    pub flow_decay: f64,
    pub stability_factor: f64,
}

/// Mesh status information
#[derive(Debug, Clone)]
pub struct MeshStatus {
    pub active_nodes: i32,
    pub resonance_level: f64,
    pub stability: f64,
    pub flow_patterns: Vec<FlowPattern>,
}

/// Wave flow pattern
#[derive(Debug, Clone)]
pub struct FlowPattern {
    pub intensity: f64,
    pub direction: (f64, f64),
    pub frequency: f64,
    pub phase: f64,
}

impl ChapelBridge {
    /// Create a new Chapel bridge instance
    pub fn new(size: i32, dimensions: i32, config: CrystalConfig) -> Result<Self> {
        // Initialize Chapel runtime
        if unsafe { chapel_init() } != 0 {
            bail!(ChapelError::InitializationError);
        }

        // Create wave mesh
        let mesh = unsafe { create_wave_mesh(size, dimensions) };
        if mesh.is_null() {
            bail!(ChapelError::AllocationError);
        }

        Ok(Self {
            mesh,
            size,
            dimensions,
            config,
            status: Arc::new(RwLock::new(MeshStatus {
                active_nodes: 0,
                resonance_level: 0.0,
                stability: 1.0,
                flow_patterns: Vec::new(),
            })),
        })
    }

    /// Distribute wave pattern across the mesh
    pub fn distribute_pattern(&self, pattern: &[f64], resonance: f64) -> Result<()> {
        if pattern.len() != (self.size * self.size) as usize {
            bail!(ChapelError::ConfigurationError(
                "Pattern size doesn't match mesh dimensions".into()
            ));
        }

        let result = unsafe {
            distribute_wave_pattern(
                self.mesh,
                pattern.as_ptr(),
                                    pattern.len() as c_int,
                                    resonance,
            )
        };

        if result != 0 {
            bail!(ChapelError::WaveDistributionError(
                "Failed to distribute wave pattern".into()
            ));
        }

        self.update_status()?;
        Ok(())
    }

    /// Synchronize crystal mesh with given parameters
    pub fn synchronize_mesh(&self) -> Result<()> {
        let result = unsafe {
            synchronize_crystal_mesh(
                self.mesh,
                self.config.harmonics,
                self.config.resonance_threshold,
            )
        };

        if result != 0 {
            bail!(ChapelError::MeshSyncError(
                "Failed to synchronize crystal mesh".into()
            ));
        }

        self.update_status()?;
        Ok(())
    }

    /// Update mesh status information
    fn update_status(&self) -> Result<()> {
        let mut status_buf = vec![0i8; 1024];
        let result = unsafe {
            get_mesh_status(self.mesh, status_buf.as_mut_ptr())
        };

        if result != 0 {
            bail!(ChapelError::MeshSyncError(
                "Failed to get mesh status".into()
            ));
        }

        // Parse status string
        let status_str = unsafe {
            CStr::from_ptr(status_buf.as_ptr())
            .to_string_lossy()
            .to_string()
        };

        let mut status = self.status.write();
        *status = self.parse_status(&status_str)?;

        Ok(())
    }

    /// Parse mesh status string
    fn parse_status(&self, status_str: &str) -> Result<MeshStatus> {
        let parts: Vec<&str> = status_str.split('|').collect();
        if parts.len() < 4 {
            bail!(ChapelError::MeshSyncError(
                "Invalid status string format".into()
            ));
        }

        let active_nodes = parts[0].parse()?;
        let resonance_level = parts[1].parse()?;
        let stability = parts[2].parse()?;

        let flow_patterns = parts[3]
        .split(';')
        .filter(|p| !p.is_empty())
        .map(|p| self.parse_flow_pattern(p))
        .collect::<Result<Vec<_>>>()?;

        Ok(MeshStatus {
            active_nodes,
            resonance_level,
            stability,
            flow_patterns,
        })
    }

    /// Parse flow pattern string
    fn parse_flow_pattern(&self, pattern_str: &str) -> Result<FlowPattern> {
        let parts: Vec<&str> = pattern_str.split(',').collect();
        if parts.len() != 4 {
            bail!(ChapelError::ConfigurationError(
                "Invalid flow pattern format".into()
            ));
        }

        Ok(FlowPattern {
            intensity: parts[0].parse()?,
           direction: (parts[1].parse()?, parts[2].parse()?),
           frequency: parts[3].parse()?,
           phase: 0.0, // Calculated from frequency
        })
    }

    /// Get current mesh status
    pub fn get_status(&self) -> MeshStatus {
        self.status.read().clone()
    }

    /// Check if mesh is in resonance
    pub fn is_resonant(&self) -> bool {
        let status = self.status.read();
        status.resonance_level >= self.config.resonance_threshold
    }

    /// Calculate mesh stability
    pub fn calculate_stability(&self) -> f64 {
        let status = self.status.read();
        status.stability * self.config.stability_factor
    }

    /// Apply flow decay to mesh
    pub fn apply_flow_decay(&self) -> Result<()> {
        let mut status = self.status.write();
        for pattern in &mut status.flow_patterns {
            pattern.intensity *= self.config.flow_decay;
        }
        Ok(())
    }
}

impl Drop for ChapelBridge {
    fn drop(&mut self) {
        unsafe {
            destroy_wave_mesh(self.mesh);
            chapel_finalize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapel_bridge_creation() -> Result<()> {
        let config = CrystalConfig {
            harmonics: 7,
            resonance_threshold: 0.001,
            flow_decay: 0.95,
            stability_factor: 1.0,
        };

        let bridge = ChapelBridge::new(64, 2, config)?;
        assert!(bridge.calculate_stability() > 0.0);
        Ok(())
    }

    #[test]
    fn test_wave_distribution() -> Result<()> {
        let config = CrystalConfig {
            harmonics: 7,
            resonance_threshold: 0.001,
            flow_decay: 0.95,
            stability_factor: 1.0,
        };

        let bridge = ChapelBridge::new(8, 2, config)?;
        let pattern = vec![1.0; 64];
        bridge.distribute_pattern(&pattern, 0.5)?;

        let status = bridge.get_status();
        assert!(status.active_nodes > 0);
        Ok(())
    }
}
