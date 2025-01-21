//! Core harmonic pattern analysis and manipulation
//! Created: 2025-01-21 13:58:46 UTC
//! Author: @isdood

use std::{
    collections::VecDeque,
    sync::Arc,
};

use num_complex::Complex64;
use rayon::prelude::*;
use parking_lot::RwLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HarmonicError {
    #[error("Invalid frequency range: {0}")]
    InvalidFrequency(String),
    #[error("Resonance calculation failed: {0}")]
    ResonanceError(String),
    #[error("Pattern synchronization failed: {0}")]
    SyncError(String),
    #[error("Buffer overflow in harmonic analysis")]
    BufferOverflow,
}

/// Configuration for harmonic analysis
#[derive(Debug, Clone)]
pub struct HarmonicConfig {
    pub base_frequency: f64,
    pub harmonic_depth: usize,
    pub resonance_threshold: f64,
    pub phase_coherence: f64,
    pub buffer_size: usize,
}

impl Default for HarmonicConfig {
    fn default() -> Self {
        Self {
            base_frequency: 432.0,
            harmonic_depth: 7,
            resonance_threshold: 0.001,
            phase_coherence: 0.95,
            buffer_size: 1024,
        }
    }
}

/// Core harmonic processor
pub struct HarmonicProcessor {
    config: HarmonicConfig,
    state: Arc<RwLock<HarmonicState>>,
    buffer: VecDeque<Complex64>,
}

/// Current state of harmonic analysis
#[derive(Debug, Clone)]
pub struct HarmonicState {
    pub frequencies: Vec<f64>,
    pub amplitudes: Vec<f64>,
    pub phases: Vec<f64>,
    pub resonance: f64,
    pub coherence: f64,
}

impl HarmonicProcessor {
    /// Create new harmonic processor with configuration
    pub fn new(config: HarmonicConfig) -> Self {
        let state = Arc::new(RwLock::new(HarmonicState {
            frequencies: vec![0.0; config.harmonic_depth],
            amplitudes: vec![0.0; config.harmonic_depth],
            phases: vec![0.0; config.harmonic_depth],
            resonance: 0.0,
            coherence: 1.0,
        }));

        Self {
            config,
            state,
            buffer: VecDeque::with_capacity(config.buffer_size),
        }
    }

    /// Process new wave data
    pub fn process_wave(&mut self, wave_data: &[Complex64]) -> Result<(), HarmonicError> {
        // Add new data to buffer
        for &value in wave_data {
            if self.buffer.len() >= self.config.buffer_size {
                self.buffer.pop_front();
            }
            self.buffer.push_back(value);
        }

        // Analyze harmonics
        self.analyze_harmonics()?;

        // Update resonance and coherence
        self.update_state()?;

        Ok(())
    }

    /// Analyze harmonic patterns in buffer
    fn analyze_harmonics(&self) -> Result<(), HarmonicError> {
        let mut state = self.state.write();

        // Calculate frequencies and amplitudes
        let harmonics = (1..=self.config.harmonic_depth)
        .into_par_iter()
        .map(|n| self.analyze_harmonic_component(n))
        .collect::<Result<Vec<_>, _>>()?;

        // Update state with new harmonics
        for (i, (freq, amp, phase)) in harmonics.into_iter().enumerate() {
            state.frequencies[i] = freq;
            state.amplitudes[i] = amp;
            state.phases[i] = phase;
        }

        Ok(())
    }

    /// Analyze single harmonic component
    fn analyze_harmonic_component(&self, harmonic: usize) -> Result<(f64, f64, f64), HarmonicError> {
        let frequency = self.config.base_frequency * harmonic as f64;
        let buffer: Vec<_> = self.buffer.iter().cloned().collect();

        // Perform frequency analysis
        let (amplitude, phase) = self.calculate_harmonic_params(&buffer, frequency)?;

        Ok((frequency, amplitude, phase))
    }

    /// Calculate amplitude and phase for given frequency
    fn calculate_harmonic_params(&self, data: &[Complex64], frequency: f64) -> Result<(f64, f64), HarmonicError> {
        if data.is_empty() {
            return Ok((0.0, 0.0));
        }

        let sample_rate = 44100.0; // Standard sample rate
        let dt = 1.0 / sample_rate;
        let omega = 2.0 * std::f64::consts::PI * frequency;

        // Calculate DFT for specific frequency
        let mut sum = Complex64::new(0.0, 0.0);
        for (i, &sample) in data.iter().enumerate() {
            let t = dt * i as f64;
            let basis = Complex64::new(0.0, -omega * t).exp();
            sum += sample * basis;
        }

        let amplitude = (2.0 * sum.norm()) / data.len() as f64;
        let phase = sum.arg();

        Ok((amplitude, phase))
    }

    /// Update resonance and coherence state
    fn update_state(&self) -> Result<(), HarmonicError> {
        let mut state = self.state.write();

        // Calculate resonance
        state.resonance = self.calculate_resonance(&state.amplitudes)?;

        // Calculate phase coherence
        state.coherence = self.calculate_coherence(&state.phases)?;

        Ok(())
    }

    /// Calculate resonance from amplitudes
    fn calculate_resonance(&self, amplitudes: &[f64]) -> Result<f64, HarmonicError> {
        if amplitudes.is_empty() {
            return Ok(0.0);
        }

        // Calculate weighted sum of harmonics
        let total_weight: f64 = (1..=amplitudes.len()).map(|i| 1.0 / i as f64).sum();
        let weighted_sum: f64 = amplitudes.iter()
        .enumerate()
        .map(|(i, &amp)| amp * (1.0 / (i + 1) as f64))
        .sum();

        Ok(weighted_sum / total_weight)
    }

    /// Calculate phase coherence
    fn calculate_coherence(&self, phases: &[f64]) -> Result<f64, HarmonicError> {
        if phases.is_empty() {
            return Ok(1.0);
        }

        // Calculate phase differences
        let mut coherence = 1.0;
        for i in 1..phases.len() {
            let phase_diff = (phases[i] - phases[i-1]).abs();
            let normalized_diff = phase_diff / std::f64::consts::PI;
            coherence *= (1.0 - normalized_diff).max(0.0);
        }

        Ok(coherence)
    }

    /// Get current harmonic state
    pub fn get_state(&self) -> HarmonicState {
        self.state.read().clone()
    }

    /// Check if system is in resonance
    pub fn is_resonant(&self) -> bool {
        let state = self.state.read();
        state.resonance >= self.config.resonance_threshold
    }

    /// Get dominant frequencies
    pub fn get_dominant_frequencies(&self) -> Vec<f64> {
        let state = self.state.read();
        let mut freq_amp: Vec<_> = state.frequencies.iter()
        .zip(state.amplitudes.iter())
        .collect();

        freq_amp.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        freq_amp.iter().map(|&(f, _)| *f).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_harmonic_analysis() {
        let config = HarmonicConfig {
            base_frequency: 432.0,
            harmonic_depth: 7,
            resonance_threshold: 0.001,
            phase_coherence: 0.95,
            buffer_size: 1024,
        };

        let mut processor = HarmonicProcessor::new(config);
        let test_data: Vec<Complex64> = (0..100)
        .map(|i| {
            let t = i as f64 / 44100.0;
            Complex64::new((432.0 * t * 2.0 * std::f64::consts::PI).sin(), 0.0)
        })
        .collect();

        processor.process_wave(&test_data).unwrap();
        let state = processor.get_state();

        assert!(state.resonance > 0.0);
        assert!(state.coherence > 0.0);
    }

    #[test]
    fn test_resonance_detection() {
        let config = HarmonicConfig::default();
        let mut processor = HarmonicProcessor::new(config);

        let test_data: Vec<Complex64> = (0..100)
        .map(|i| {
            let t = i as f64 / 44100.0;
            Complex64::new((432.0 * t * 2.0 * std::f64::consts::PI).sin(), 0.0)
        })
        .collect();

        processor.process_wave(&test_data).unwrap();
        assert!(processor.get_state().resonance >= 0.0);
    }
}
