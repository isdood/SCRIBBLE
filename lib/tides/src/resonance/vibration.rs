//! Crystal lattice vibration simulation and analysis with Julia computation backend
//! Created: 2025-01-21 15:48:06 UTC
//! Author: @isdood

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use crate::{
    core::wave_pattern::WavePattern,
    lattice::node::LatticeNode,
    julia::{
        JuliaCompute,
        vibration::{JuliaVibrationCompute, VibrationResult},
        spectrum::{JuliaSpectrumAnalysis, SpectrumResult},
    },
};

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VibrationError {
    #[error("Invalid vibration configuration: {0}")]
    InvalidConfig(String),
    #[error("Vibration mode error: {0}")]
    ModeError(String),
    #[error("Julia computation error: {0}")]
    JuliaError(String),
    #[error("Spectrum analysis error: {0}")]
    SpectrumError(String),
}

/// Configuration for crystal vibrations
#[derive(Debug, Clone)]
pub struct VibrationConfig {
    pub frequency_range: (f64, f64),
    pub damping_factor: f64,
    pub coupling_strength: f64,
    pub mode_threshold: f64,
    pub resolution: usize,
    pub memory_length: usize,
    pub julia_threads: usize,
}

impl Default for VibrationConfig {
    fn default() -> Self {
        Self {
            frequency_range: (20.0, 20000.0),
            damping_factor: 0.995,
            coupling_strength: 0.5,
            mode_threshold: 0.001,
            resolution: 1024,
            memory_length: 256,
            julia_threads: 4,
        }
    }
}

/// Crystal vibration manager with Julia backend
pub struct CrystalVibration {
    config: VibrationConfig,
    state: RwLock<VibrationState>,
    history: RwLock<VecDeque<VibrationState>>,
    wave_pattern: Arc<WavePattern>,
    julia_compute: JuliaVibrationCompute,
    spectrum_analysis: JuliaSpectrumAnalysis,
}

/// Vibration state information
#[derive(Debug, Clone)]
pub struct VibrationState {
    pub time: f64,
    pub modes: Vec<VibrationMode>,
    pub amplitude_field: Vec<Vec<Complex64>>,
    pub frequency_spectrum: Vec<f64>,
    pub phase_spectrum: Vec<f64>,
    pub energy_distribution: Vec<f64>,
    pub total_energy: f64,
    pub coherence: f64,
}

/// Vibration mode information from Julia computation
#[derive(Debug, Clone)]
pub struct VibrationMode {
    pub frequency: f64,
    pub amplitude: Complex64,
    pub phase: f64,
    pub energy: f64,
    pub nodes: Vec<u64>,
    pub stability: f64,
}

impl CrystalVibration {
    /// Create new crystal vibration manager with Julia backend
    pub fn new(config: VibrationConfig, wave_pattern: Arc<WavePattern>) -> Result<Self, VibrationError> {
        let julia_compute = JuliaVibrationCompute::new(config.julia_threads)
        .map_err(|e| VibrationError::JuliaError(e.to_string()))?;

        let spectrum_analysis = JuliaSpectrumAnalysis::new(config.julia_threads)
        .map_err(|e| VibrationError::JuliaError(e.to_string()))?;

        let initial_state = VibrationState {
            time: 0.0,
            modes: Vec::new(),
            amplitude_field: Vec::new(),
            frequency_spectrum: vec![0.0; config.resolution],
            phase_spectrum: vec![0.0; config.resolution],
            energy_distribution: vec![0.0; config.resolution],
            total_energy: 0.0,
            coherence: 1.0,
        };

        Ok(Self {
            config,
            state: RwLock::new(initial_state),
           history: RwLock::new(VecDeque::with_capacity(config.memory_length)),
           wave_pattern,
           julia_compute,
           spectrum_analysis,
        })
    }

    /// Update vibration state using Julia computation
    pub fn update(&self, nodes: &[Arc<LatticeNode>], time: f64) -> Result<(), VibrationError> {
        // Prepare node data for Julia
        let node_data = self.prepare_node_data(nodes)?;

        // Compute vibrations using Julia
        let vibration_result = self.julia_compute.compute_vibrations(
            node_data,
            time,
            self.config.damping_factor,
            self.config.coupling_strength,
        ).map_err(|e| VibrationError::JuliaError(e.to_string()))?;

        // Analyze spectrum using Julia
        let spectrum_result = self.spectrum_analysis.analyze_spectrum(
            &vibration_result.modes,
            self.config.frequency_range,
            self.config.resolution,
        ).map_err(|e| VibrationError::SpectrumError(e.to_string()))?;

        // Update state with Julia results
        self.update_state(vibration_result, spectrum_result, time)?;

        // Update history
        self.update_history()?;

        Ok(())
    }

    /// Prepare node data for Julia computation
    fn prepare_node_data(&self, nodes: &[Arc<LatticeNode>]) -> Result<Vec<(u64, [f64; 3], Complex64)>, VibrationError> {
        nodes.iter().map(|node| {
            let state = node.get_state();
            Ok((node.id(), state.position, state.amplitude))
        }).collect()
    }

    /// Update state with Julia computation results
    fn update_state(
        &self,
        vibration_result: VibrationResult,
        spectrum_result: SpectrumResult,
        time: f64,
    ) -> Result<(), VibrationError> {
        let mut state = self.state.write();

        state.time = time;
        state.modes = vibration_result.modes;
        state.amplitude_field = vibration_result.amplitude_field;
        state.frequency_spectrum = spectrum_result.frequency_spectrum;
        state.phase_spectrum = spectrum_result.phase_spectrum;
        state.energy_distribution = spectrum_result.energy_distribution;
        state.total_energy = spectrum_result.total_energy;
        state.coherence = vibration_result.coherence;

        Ok(())
    }

    /// Update state history
    fn update_history(&self) -> Result<(), VibrationError> {
        let mut history = self.history.write();
        let current_state = self.state.read().clone();

        if history.len() >= self.config.memory_length {
            history.pop_front();
        }
        history.push_back(current_state);

        Ok(())
    }

    /// Get current vibration state
    pub fn get_state(&self) -> VibrationState {
        self.state.read().clone()
    }

    /// Get vibration history
    pub fn get_history(&self) -> Vec<VibrationState> {
        self.history.read().iter().cloned().collect()
    }

    /// Check if vibrations are coherent
    pub fn is_coherent(&self) -> bool {
        self.state.read().coherence >= self.config.mode_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_vibration_initialization() -> Result<(), VibrationError> {
        let config = VibrationConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let vibration = CrystalVibration::new(config.clone(), wave_pattern)?;

        let state = vibration.get_state();
        assert!(state.modes.is_empty());
        assert_eq!(state.frequency_spectrum.len(), config.resolution);
        assert_relative_eq!(state.total_energy, 0.0);
        assert_relative_eq!(state.coherence, 1.0);
        Ok(())
    }

    #[test]
    fn test_julia_computation() -> Result<(), VibrationError> {
        let config = VibrationConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let vibration = CrystalVibration::new(config, wave_pattern)?;

        // Create test nodes
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| {
            let node = Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0]));
            node.apply_force(Complex64::new(1.0, 0.0)).unwrap();
            node
        })
        .collect();

        vibration.update(&nodes, 0.0)?;
        let state = vibration.get_state();

        assert!(!state.modes.is_empty());
        assert!(state.total_energy > 0.0);
        Ok(())
    }

    #[test]
    fn test_spectrum_analysis() -> Result<(), VibrationError> {
        let config = VibrationConfig {
            resolution: 10,
            ..Default::default()
        };
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let vibration = CrystalVibration::new(config, wave_pattern)?;

        let node = Arc::new(LatticeNode::new(Default::default(), [0.0, 0.0, 0.0]));
        node.apply_force(Complex64::new(1.0, 0.0))?;

        vibration.update(&[node], 0.0)?;
        let state = vibration.get_state();

        assert_eq!(state.frequency_spectrum.len(), 10);
        assert_eq!(state.phase_spectrum.len(), 10);
        Ok(())
    }

    #[test]
    fn test_julia_threading() -> Result<(), VibrationError> {
        let config = VibrationConfig {
            julia_threads: 2,
            ..Default::default()
        };
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let vibration = CrystalVibration::new(config, wave_pattern)?;

        let nodes: Vec<Arc<LatticeNode>> = (0..10)
        .map(|i| {
            let node = Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0]));
            node.apply_force(Complex64::new(1.0, 0.0)).unwrap();
            node
        })
        .collect();

        vibration.update(&nodes, 0.0)?;
        Ok(())
    }
}
