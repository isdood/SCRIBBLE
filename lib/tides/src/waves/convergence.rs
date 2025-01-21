//! Crystal wave convergence analysis and optimization
//! Created: 2025-01-21 15:54:14 UTC
//! Author: @isdood

use std::{
    collections::VecDeque,
    sync::Arc,
};

use crate::{
    julia::{
        waves::{JuliaWaveAnalysis, WaveConvergenceResult},
        fft::{JuliaFFTProcessor, FFTResult},
    },
    chapel::{
        parallel::{ChapelDomainMap, ChapelParallelIterator},
        waves::{ChapelWaveConvergence, ConvergenceMetrics},
    },
    core::wave_pattern::WavePattern,
};

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConvergenceError {
    #[error("Invalid convergence configuration: {0}")]
    InvalidConfig(String),
    #[error("Wave analysis error: {0}")]
    WaveError(String),
    #[error("Julia computation error: {0}")]
    JuliaError(String),
    #[error("Chapel computation error: {0}")]
    ChapelError(String),
}

/// Configuration for wave convergence analysis
#[derive(Debug, Clone)]
pub struct ConvergenceConfig {
    pub resolution: usize,
    pub time_window: f64,
    pub tolerance: f64,
    pub max_iterations: usize,
    pub memory_depth: usize,
    pub julia_threads: usize,
    pub chapel_locales: usize,
    pub compute_backend: ComputeBackend,
}

#[derive(Debug, Clone, Copy)]
pub enum ComputeBackend {
    Julia,
    Chapel,
    Hybrid,
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        Self {
            resolution: 1024,
            time_window: 1.0,
            tolerance: 1e-6,
            max_iterations: 1000,
            memory_depth: 256,
            julia_threads: 4,
            chapel_locales: 2,
            compute_backend: ComputeBackend::Hybrid,
        }
    }
}

/// Wave convergence analyzer
pub struct WaveConvergence {
    config: ConvergenceConfig,
    state: RwLock<ConvergenceState>,
    history: RwLock<VecDeque<ConvergenceState>>,
    wave_pattern: Arc<WavePattern>,
    julia_analyzer: JuliaWaveAnalysis,
    julia_fft: JuliaFFTProcessor,
    chapel_convergence: ChapelWaveConvergence,
}

/// Convergence state information
#[derive(Debug, Clone)]
pub struct ConvergenceState {
    pub time: f64,
    pub iteration: usize,
    pub wave_field: Vec<Vec<Complex64>>,
    pub convergence_metric: f64,
    pub phase_coherence: f64,
    pub energy_density: Vec<Vec<f64>>,
    pub frequency_spectrum: Vec<f64>,
    pub convergence_rate: f64,
}

impl WaveConvergence {
    /// Create new wave convergence analyzer
    pub fn new(
        config: ConvergenceConfig,
        wave_pattern: Arc<WavePattern>,
    ) -> Result<Self, ConvergenceError> {
        // Initialize Julia components
        let julia_analyzer = JuliaWaveAnalysis::new(config.julia_threads)
        .map_err(|e| ConvergenceError::JuliaError(e.to_string()))?;

        let julia_fft = JuliaFFTProcessor::new(config.julia_threads)
        .map_err(|e| ConvergenceError::JuliaError(e.to_string()))?;

        // Initialize Chapel components
        let chapel_convergence = ChapelWaveConvergence::new(config.chapel_locales)
        .map_err(|e| ConvergenceError::ChapelError(e.to_string()))?;

        let initial_state = ConvergenceState {
            time: 0.0,
            iteration: 0,
            wave_field: Vec::new(),
            convergence_metric: 1.0,
            phase_coherence: 1.0,
            energy_density: Vec::new(),
            frequency_spectrum: vec![0.0; config.resolution],
            convergence_rate: 0.0,
        };

        Ok(Self {
            config,
            state: RwLock::new(initial_state),
           history: RwLock::new(VecDeque::with_capacity(config.memory_depth)),
           wave_pattern,
           julia_analyzer,
           julia_fft,
           chapel_convergence,
        })
    }

    /// Update convergence analysis
    pub fn update(&self, time: f64) -> Result<(), ConvergenceError> {
        match self.config.compute_backend {
            ComputeBackend::Julia => {
                self.update_with_julia(time)?;
            }
            ComputeBackend::Chapel => {
                self.update_with_chapel(time)?;
            }
            ComputeBackend::Hybrid => {
                self.update_hybrid(time)?;
            }
        }

        self.update_history()?;
        Ok(())
    }

    /// Update using Julia backend
    fn update_with_julia(&self, time: f64) -> Result<(), ConvergenceError> {
        // Get current wave pattern
        let wave_state = self.wave_pattern.get_state();

        // Analyze wave convergence using Julia
        let convergence_result = self.julia_analyzer
        .analyze_convergence(&wave_state.amplitudes, time, self.config.tolerance)
        .map_err(|e| ConvergenceError::JuliaError(e.to_string()))?;

        // Compute FFT using Julia
        let fft_result = self.julia_fft
        .compute_spectrum(&wave_state.amplitudes, self.config.resolution)
        .map_err(|e| ConvergenceError::JuliaError(e.to_string()))?;

        // Update state with Julia results
        self.update_state_from_julia(convergence_result, fft_result, time)?;

        Ok(())
    }

    /// Update using Chapel backend
    fn update_with_chapel(&self, time: f64) -> Result<(), ConvergenceError> {
        // Get current wave pattern
        let wave_state = self.wave_pattern.get_state();

        // Analyze convergence using Chapel's parallel capabilities
        let metrics = self.chapel_convergence
        .compute_metrics(&wave_state.amplitudes, time, self.config.tolerance)
        .map_err(|e| ConvergenceError::ChapelError(e.to_string()))?;

        // Update state with Chapel results
        self.update_state_from_chapel(metrics, time)?;

        Ok(())
    }

    /// Update using hybrid Julia/Chapel approach
    fn update_hybrid(&self, time: f64) -> Result<(), ConvergenceError> {
        let wave_state = self.wave_pattern.get_state();

        // Parallel computation using both backends
        let (julia_result, chapel_metrics) = rayon::join(
            || {
                self.julia_analyzer.analyze_convergence(
                    &wave_state.amplitudes,
                    time,
                    self.config.tolerance,
                )
            },
            || {
                self.chapel_convergence.compute_metrics(
                    &wave_state.amplitudes,
                    time,
                    self.config.tolerance,
                )
            },
        );

        let julia_result = julia_result
        .map_err(|e| ConvergenceError::JuliaError(e.to_string()))?;
        let chapel_metrics = chapel_metrics
        .map_err(|e| ConvergenceError::ChapelError(e.to_string()))?;

        // FFT computation using Julia
        let fft_result = self.julia_fft
        .compute_spectrum(&wave_state.amplitudes, self.config.resolution)
        .map_err(|e| ConvergenceError::JuliaError(e.to_string()))?;

        // Merge and update results
        self.merge_and_update_results(julia_result, chapel_metrics, fft_result, time)?;

        Ok(())
    }

    /// Update state from Julia results
    fn update_state_from_julia(
        &self,
        convergence: WaveConvergenceResult,
        fft: FFTResult,
        time: f64,
    ) -> Result<(), ConvergenceError> {
        let mut state = self.state.write();
        state.time = time;
        state.iteration += 1;
        state.wave_field = convergence.wave_field;
        state.convergence_metric = convergence.metric;
        state.phase_coherence = convergence.phase_coherence;
        state.energy_density = convergence.energy_density;
        state.frequency_spectrum = fft.spectrum;
        state.convergence_rate = convergence.convergence_rate;
        Ok(())
    }

    /// Update state from Chapel results
    fn update_state_from_chapel(
        &self,
        metrics: ConvergenceMetrics,
        time: f64,
    ) -> Result<(), ConvergenceError> {
        let mut state = self.state.write();
        state.time = time;
        state.iteration += 1;
        state.wave_field = metrics.wave_field;
        state.convergence_metric = metrics.convergence;
        state.phase_coherence = metrics.coherence;
        state.energy_density = metrics.energy_density;
        state.convergence_rate = metrics.convergence_rate;
        Ok(())
    }

    /// Merge and update results from both backends
    fn merge_and_update_results(
        &self,
        julia_result: WaveConvergenceResult,
        chapel_metrics: ConvergenceMetrics,
        fft: FFTResult,
        time: f64,
    ) -> Result<(), ConvergenceError> {
        let mut state = self.state.write();
        state.time = time;
        state.iteration += 1;

        // Average results from both backends
        state.convergence_metric = (julia_result.metric + chapel_metrics.convergence) / 2.0;
        state.phase_coherence = (julia_result.phase_coherence + chapel_metrics.coherence) / 2.0;
        state.convergence_rate = (julia_result.convergence_rate + chapel_metrics.convergence_rate) / 2.0;

        // Use Julia results for wave field and FFT
        state.wave_field = julia_result.wave_field;
        state.frequency_spectrum = fft.spectrum;

        // Use Chapel results for energy density
        state.energy_density = chapel_metrics.energy_density;

        Ok(())
    }

    /// Update convergence history
    fn update_history(&self) -> Result<(), ConvergenceError> {
        let mut history = self.history.write();
        let current_state = self.state.read().clone();

        if history.len() >= self.config.memory_depth {
            history.pop_front();
        }
        history.push_back(current_state);

        Ok(())
    }

    /// Get current convergence state
    pub fn get_state(&self) -> ConvergenceState {
        self.state.read().clone()
    }

    /// Get convergence history
    pub fn get_history(&self) -> Vec<ConvergenceState> {
        self.history.read().iter().cloned().collect()
    }

    /// Check if waves have converged
    pub fn has_converged(&self) -> bool {
        let state = self.state.read();
        state.convergence_metric <= self.config.tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_convergence_initialization() -> Result<(), ConvergenceError> {
        let config = ConvergenceConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let convergence = WaveConvergence::new(config.clone(), wave_pattern)?;

        let state = convergence.get_state();
        assert_eq!(state.iteration, 0);
        assert_relative_eq!(state.convergence_metric, 1.0);
        assert_eq!(state.frequency_spectrum.len(), config.resolution);
        Ok(())
    }

    #[test]
    fn test_julia_backend() -> Result<(), ConvergenceError> {
        let config = ConvergenceConfig {
            compute_backend: ComputeBackend::Julia,
            ..Default::default()
        };
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let convergence = WaveConvergence::new(config, wave_pattern)?;

        convergence.update(0.0)?;
        assert!(convergence.get_state().iteration > 0);
        Ok(())
    }

    #[test]
    fn test_chapel_backend() -> Result<(), ConvergenceError> {
        let config = ConvergenceConfig {
            compute_backend: ComputeBackend::Chapel,
            ..Default::default()
        };
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let convergence = WaveConvergence::new(config, wave_pattern)?;

        convergence.update(0.0)?;
        assert!(convergence.get_state().iteration > 0);
        Ok(())
    }

    #[test]
    fn test_hybrid_backend() -> Result<(), ConvergenceError> {
        let config = ConvergenceConfig {
            compute_backend: ComputeBackend::Hybrid,
            ..Default::default()
        };
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let convergence = WaveConvergence::new(config, wave_pattern)?;

        convergence.update(0.0)?;
        assert!(convergence.get_state().iteration > 0);
        Ok(())
    }

    #[test]
    fn test_convergence_criteria() -> Result<(), ConvergenceError> {
        let mut config = ConvergenceConfig::default();
        config.tolerance = 0.1;
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let convergence = WaveConvergence::new(config, wave_pattern)?;

        for i in 0..10 {
            convergence.update(i as f64 * 0.1)?;
            if convergence.has_converged() {
                break;
            }
        }

        let state = convergence.get_state();
        assert!(state.convergence_metric <= config.tolerance || state.iteration == 10);
        Ok(())
    }

    #[test]
    fn test_history_management() -> Result<(), ConvergenceError> {
        let mut config = ConvergenceConfig::default();
        config.memory_depth = 3;
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let convergence = WaveConvergence::new(config, wave_pattern)?;

        for i in 0..5 {
            convergence.update(i as f64)?;
        }

        let history = convergence.get_history();
        assert!(history.len() <= 3);
        Ok(())
    }
}
