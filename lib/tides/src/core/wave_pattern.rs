//! Wave pattern generation and analysis for crystal structures
//! Created: 2025-01-21 14:02:37 UTC
//! Author: @isdood

use std::{
    collections::VecDeque,
    sync::Arc,
};

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WavePatternError {
    #[error("Invalid pattern configuration: {0}")]
    InvalidConfig(String),
    #[error("Pattern generation failed: {0}")]
    GenerationError(String),
    #[error("Analysis failed: {0}")]
    AnalysisError(String),
    #[error("Pattern synchronization failed: {0}")]
    SyncError(String),
}

/// Wave pattern configuration
#[derive(Debug, Clone)]
pub struct WavePatternConfig {
    pub dimensions: (usize, usize),
    pub base_frequency: f64,
    pub harmonic_count: usize,
    pub interference_threshold: f64,
    pub coherence_threshold: f64,
    pub memory_depth: usize,
}

impl Default for WavePatternConfig {
    fn default() -> Self {
        Self {
            dimensions: (64, 64),
            base_frequency: 432.0,
            harmonic_count: 7,
            interference_threshold: 0.001,
            coherence_threshold: 0.95,
            memory_depth: 128,
        }
    }
}

/// Wave pattern generator and analyzer
pub struct WavePattern {
    config: WavePatternConfig,
    state: Arc<RwLock<PatternState>>,
    history: VecDeque<PatternState>,
}

/// Current state of wave pattern
#[derive(Debug, Clone)]
pub struct PatternState {
    pub amplitudes: Vec<Vec<Complex64>>,
    pub phases: Vec<Vec<f64>>,
    pub harmonics: Vec<HarmonicLayer>,
    pub interference: Vec<Vec<f64>>,
    pub coherence: f64,
}

/// Harmonic layer in the wave pattern
#[derive(Debug, Clone)]
pub struct HarmonicLayer {
    pub frequency: f64,
    pub amplitude: Vec<Vec<f64>>,
    pub phase: Vec<Vec<f64>>,
}

impl WavePattern {
    /// Create new wave pattern generator
    pub fn new(config: WavePatternConfig) -> Result<Self, WavePatternError> {
        let (width, height) = config.dimensions;
        if width == 0 || height == 0 {
            return Err(WavePatternError::InvalidConfig("Dimensions must be positive".into()));
        }

        let initial_state = Self::create_initial_state(&config)?;
        let mut history = VecDeque::with_capacity(config.memory_depth);
        history.push_back(initial_state.clone());

        Ok(Self {
            config,
            state: Arc::new(RwLock::new(initial_state)),
           history,
        })
    }

    /// Create initial pattern state
    fn create_initial_state(config: &WavePatternConfig) -> Result<PatternState, WavePatternError> {
        let (width, height) = config.dimensions;

        // Initialize empty state
        let state = PatternState {
            amplitudes: vec![vec![Complex64::new(0.0, 0.0); width]; height],
            phases: vec![vec![0.0; width]; height],
            harmonics: (0..config.harmonic_count)
            .map(|n| HarmonicLayer {
                frequency: config.base_frequency * (n + 1) as f64,
                 amplitude: vec![vec![0.0; width]; height],
                 phase: vec![vec![0.0; width]; height],
            })
            .collect(),
            interference: vec![vec![0.0; width]; height],
            coherence: 1.0,
        };

        Ok(state)
    }

    /// Generate new wave pattern
    pub fn generate_pattern(&mut self, time: f64) -> Result<(), WavePatternError> {
        let mut state = self.state.write();
        let (width, height) = self.config.dimensions;

        // Generate harmonic layers in parallel
        let new_harmonics: Vec<HarmonicLayer> = (0..self.config.harmonic_count)
        .into_par_iter()
        .map(|n| self.generate_harmonic_layer(n, time))
        .collect::<Result<Vec<_>, _>>()?;

        // Calculate interference pattern
        let interference = self.calculate_interference(&new_harmonics)?;

        // Update amplitudes and phases
        let (amplitudes, phases) = self.calculate_wave_components(&new_harmonics, &interference)?;

        // Calculate coherence
        let coherence = self.calculate_coherence(&phases)?;

        // Update state
        state.harmonics = new_harmonics;
        state.interference = interference;
        state.amplitudes = amplitudes;
        state.phases = phases;
        state.coherence = coherence;

        // Update history
        if self.history.len() >= self.config.memory_depth {
            self.history.pop_front();
        }
        self.history.push_back((*state).clone());

        Ok(())
    }

    /// Generate single harmonic layer
    fn generate_harmonic_layer(&self, n: usize, time: f64) -> Result<HarmonicLayer, WavePatternError> {
        let (width, height) = self.config.dimensions;
        let frequency = self.config.base_frequency * (n + 1) as f64;
        let omega = 2.0 * std::f64::consts::PI * frequency;

        let mut amplitude = vec![vec![0.0; width]; height];
        let mut phase = vec![vec![0.0; width]; height];

        // Generate harmonic pattern
        for i in 0..height {
            for j in 0..width {
                let x = j as f64 / width as f64;
                let y = i as f64 / height as f64;
                let r = (x * x + y * y).sqrt();

                amplitude[i][j] = (1.0 / (n + 1) as f64) * (-r * omega).exp();
                phase[i][j] = omega * (time + r);
            }
        }

        Ok(HarmonicLayer {
            frequency,
            amplitude,
            phase,
        })
    }

    /// Calculate interference pattern between harmonics
    fn calculate_interference(&self, harmonics: &[HarmonicLayer]) -> Result<Vec<Vec<f64>>, WavePatternError> {
        let (width, height) = self.config.dimensions;
        let mut interference = vec![vec![0.0; width]; height];

        // Calculate interference in parallel
        interference.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..width {
                let mut sum = 0.0;
                for h1 in harmonics.iter() {
                    for h2 in harmonics.iter() {
                        sum += h1.amplitude[i][j] * h2.amplitude[i][j] *
                        (h1.phase[i][j] - h2.phase[i][j]).cos();
                    }
                }
                row[j] = sum;
            }
        });

        Ok(interference)
    }

    /// Calculate wave components from harmonics and interference
    fn calculate_wave_components(
        &self,
        harmonics: &[HarmonicLayer],
        interference: &[Vec<f64>],
    ) -> Result<(Vec<Vec<Complex64>>, Vec<Vec<f64>>), WavePatternError> {
        let (width, height) = self.config.dimensions;
        let mut amplitudes = vec![vec![Complex64::new(0.0, 0.0); width]; height];
        let mut phases = vec![vec![0.0; width]; height];

        // Calculate components in parallel
        amplitudes.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..width {
                let mut sum = Complex64::new(0.0, 0.0);
                for harmonic in harmonics {
                    let amp = harmonic.amplitude[i][j];
                    let phase = harmonic.phase[i][j];
                    sum += Complex64::from_polar(amp, phase);
                }
                row[j] = sum;
                phases[i][j] = sum.arg();
            }
        });

        Ok((amplitudes, phases))
    }

    /// Calculate wave pattern coherence
    fn calculate_coherence(&self, phases: &[Vec<f64>]) -> Result<f64, WavePatternError> {
        let (width, height) = self.config.dimensions;
        let mut phase_sum = Complex64::new(0.0, 0.0);

        // Sum phase vectors
        for i in 0..height {
            for j in 0..width {
                phase_sum += Complex64::from_polar(1.0, phases[i][j]);
            }
        }

        let coherence = phase_sum.norm() / (width * height) as f64;
        Ok(coherence)
    }

    /// Get current pattern state
    pub fn get_state(&self) -> PatternState {
        self.state.read().clone()
    }

    /// Check if pattern is coherent
    pub fn is_coherent(&self) -> bool {
        self.state.read().coherence >= self.config.coherence_threshold
    }

    /// Get interference at point
    pub fn get_interference(&self, x: usize, y: usize) -> Option<f64> {
        self.state.read().interference.get(y)?.get(x).cloned()
    }

    /// Get dominant frequencies
    pub fn get_dominant_frequencies(&self) -> Vec<f64> {
        self.state.read()
        .harmonics
        .iter()
        .map(|h| h.frequency)
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_pattern_generation() -> Result<(), WavePatternError> {
        let config = WavePatternConfig::default();
        let mut pattern = WavePattern::new(config)?;

        pattern.generate_pattern(0.0)?;
        let state = pattern.get_state();

        assert!(!state.amplitudes.is_empty());
        assert!(state.coherence >= 0.0 && state.coherence <= 1.0);
        Ok(())
    }

    #[test]
    fn test_interference_calculation() -> Result<(), WavePatternError> {
        let config = WavePatternConfig::default();
        let mut pattern = WavePattern::new(config)?;

        pattern.generate_pattern(0.0)?;
        let state = pattern.get_state();

        // Check interference pattern
        for row in &state.interference {
            for &value in row {
                assert!(!value.is_nan());
            }
        }
        Ok(())
    }
}
