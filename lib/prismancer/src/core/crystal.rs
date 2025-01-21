// src/core/crystal.rs
// Created: 2025-01-21 18:53:59 UTC
// Author: isdood

use std::{
    sync::atomic::{AtomicU64, AtomicF64, Ordering},
    time::{Duration, Instant},
};

use parking_lot::RwLock;
use rayon::prelude::*;
use glam::{Vec3, Mat4};
use thiserror::Error;

/// Errors that can occur in crystal operations
#[derive(Error, Debug)]
pub enum CrystalError {
    #[error("Crystal coherence below threshold: {0} < {1}")]
    CoherenceLow(f64, f64),
    #[error("Crystal lattice misalignment: {0}")]
    LatticeMisaligned(Vec3),
    #[error("Harmony state decoherence")]
    Decoherence,
    #[error("Reality anchor unstable: {0}")]
    UnstableAnchor(f64),
}

/// Result type for crystal operations
pub type CrystalResult<T> = Result<T, CrystalError>;

/// Configuration for crystal system
#[derive(Debug, Clone)]
pub struct CrystalConfig {
    pub coherence_threshold: f64,
    pub reality_anchor_strength: f64,
    pub harmony_depth: u32,
    pub lattice_alignment_tolerance: f64,
    pub decoherence_rate: f64,
}

impl Default for CrystalConfig {
    fn default() -> Self {
        Self {
            coherence_threshold: 0.95,
            reality_anchor_strength: 0.85,
            harmony_depth: 16,
            lattice_alignment_tolerance: 0.001,
            decoherence_rate: 0.001,
        }
    }
}

/// Represents a harmony state within the crystal
#[derive(Debug)]
pub struct HarmonyState {
    superposition: Vec3,
    phase: f64,
    coherence: AtomicF64,
    last_observed: AtomicU64,
}

impl HarmonyState {
    pub fn new(initial_state: Vec3) -> Self {
        Self {
            superposition: initial_state,
            phase: 0.0,
            coherence: AtomicF64::new(1.0),
            last_observed: AtomicU64::new(Instant::now().elapsed().as_nanos() as u64),
        }
    }

    pub fn collapse(&self) -> Vec3 {
        let now = Instant::now().elapsed().as_nanos() as u64;
        self.last_observed.store(now, Ordering::Release);
        self.superposition
    }
}

/// Core crystal structure
#[derive(Debug)]
pub struct Crystal {
    config: CrystalConfig,
    lattice: RwLock<Mat4>,
    states: Vec<HarmonyState>,
    reality_anchor: AtomicF64,
    coherence: AtomicF64,
}

impl Crystal {
    /// Create a new crystal with given configuration
    pub fn new(config: CrystalConfig) -> Self {
        Self {
            config,
            lattice: RwLock::new(Mat4::IDENTITY),
            states: Vec::new(),
            reality_anchor: AtomicF64::new(1.0),
            coherence: AtomicF64::new(1.0),
        }
    }

    /// Add a harmony state to the crystal
    pub fn add_state(&mut self, state: HarmonyState) -> CrystalResult<()> {
        if self.coherence.load(Ordering::Acquire) < self.config.coherence_threshold {
            return Err(CrystalError::CoherenceLow(
                self.coherence.load(Ordering::Acquire),
                                                  self.config.coherence_threshold,
            ));
        }
        self.states.push(state);
        self.update_coherence();
        Ok(())
    }

    /// Update the crystal's harmony coherence
    fn update_coherence(&self) {
        let new_coherence = self.states.par_iter().map(|state| {
            let time_since_observation = Instant::now().elapsed().as_nanos() as u64
            - state.last_observed.load(Ordering::Acquire);
            let decoherence = (-self.config.decoherence_rate * time_since_observation as f64).exp();
            state.coherence.load(Ordering::Acquire) * decoherence
        }).sum::<f64>() / self.states.len() as f64;

        self.coherence.store(new_coherence, Ordering::Release);
    }

    /// Align the crystal lattice
    pub fn align_lattice(&self, target: Mat4) -> CrystalResult<()> {
        let mut lattice = self.lattice.write();
        let difference = (*lattice * target.inverse()).to_scale_rotation_translation().0;

        if difference.length() > self.config.lattice_alignment_tolerance {
            return Err(CrystalError::LatticeMisaligned(difference));
        }

        *lattice = target;
        Ok(())
    }

    /// Process harmony interactions
    pub fn process_harmony(&self, duration: Duration) -> CrystalResult<Vec<Vec3>> {
        self.update_coherence();

        if self.coherence.load(Ordering::Acquire) < self.config.coherence_threshold {
            return Err(CrystalError::Decoherence);
        }

        let reality_anchor = self.reality_anchor.load(Ordering::Acquire);
        if reality_anchor < self.config.reality_anchor_strength {
            return Err(CrystalError::UnstableAnchor(reality_anchor));
        }

        Ok(self.states.par_iter().map(|state| state.collapse()).collect())
    }

    /// Stabilize the crystal's reality anchor
    pub fn stabilize(&self) -> CrystalResult<()> {
        let current_anchor = self.reality_anchor.load(Ordering::Acquire);
        let new_anchor = (current_anchor + self.config.reality_anchor_strength) / 2.0;

        if new_anchor < self.config.reality_anchor_strength {
            return Err(CrystalError::UnstableAnchor(new_anchor));
        }

        self.reality_anchor.store(new_anchor, Ordering::Release);
        Ok(())
    }

    /// Get current crystal metrics
    pub fn metrics(&self) -> CrystalMetrics {
        CrystalMetrics {
            coherence: self.coherence.load(Ordering::Acquire),
            reality_anchor: self.reality_anchor.load(Ordering::Acquire),
            state_count: self.states.len(),
            lattice: *self.lattice.read(),
        }
    }
}

/// Crystal system metrics
#[derive(Debug, Clone)]
pub struct CrystalMetrics {
    pub coherence: f64,
    pub reality_anchor: f64,
    pub state_count: usize,
    pub lattice: Mat4,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_crystal_creation() {
        let config = CrystalConfig::default();
        let crystal = Crystal::new(config.clone());
        let metrics = crystal.metrics();

        assert_eq!(metrics.coherence, 1.0);
        assert_eq!(metrics.reality_anchor, 1.0);
        assert_eq!(metrics.state_count, 0);
    }

    #[test]
    fn test_harmony_state_addition() {
        let config = CrystalConfig::default();
        let mut crystal = Crystal::new(config);

        let state = HarmonyState::new(Vec3::new(1.0, 0.0, 0.0));
        assert!(crystal.add_state(state).is_ok());

        let metrics = crystal.metrics();
        assert_eq!(metrics.state_count, 1);
    }

    #[test]
    fn test_coherence_decay() {
        let mut config = CrystalConfig::default();
        config.decoherence_rate = 0.1;
        let mut crystal = Crystal::new(config);

        let state = HarmonyState::new(Vec3::new(1.0, 0.0, 0.0));
        crystal.add_state(state).unwrap();

        thread::sleep(Duration::from_millis(100));
        crystal.update_coherence();

        let metrics = crystal.metrics();
        assert!(metrics.coherence < 1.0);
    }

    #[test]
    fn test_lattice_alignment() {
        let config = CrystalConfig::default();
        let crystal = Crystal::new(config);

        let target = Mat4::from_scale(Vec3::new(1.0, 1.0, 1.0));
        assert!(crystal.align_lattice(target).is_ok());

        let metrics = crystal.metrics();
        assert_eq!(metrics.lattice, target);
    }
}
