//! Crystalline interference pattern analysis and manipulation
//! Created: 2025-01-21 15:11:57 UTC
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
pub enum InterferenceError {
    #[error("Invalid interference pattern: {0}")]
    InvalidPattern(String),
    #[error("Pattern analysis failed: {0}")]
    AnalysisError(String),
    #[error("Coherence calculation failed: {0}")]
    CoherenceError(String),
    #[error("Memory buffer overflow")]
    BufferOverflow,
}

/// Configuration for interference patterns
#[derive(Debug, Clone)]
pub struct InterferenceConfig {
    pub grid_size: (usize, usize),
    pub wavelength_range: (f64, f64),
    pub coherence_threshold: f64,
    pub interference_depth: usize,
    pub memory_length: usize,
}

impl Default for InterferenceConfig {
    fn default() -> Self {
        Self {
            grid_size: (64, 64),
            wavelength_range: (380.0, 780.0),
            coherence_threshold: 0.95,
            interference_depth: 7,
            memory_length: 128,
        }
    }
}

/// Interference pattern analyzer
pub struct InterferenceAnalyzer {
    config: InterferenceConfig,
    state: Arc<RwLock<InterferenceState>>,
    history: VecDeque<InterferenceState>,
    pattern_cache: HashMap<PatternKey, InterferencePattern>,
}

/// Current state of interference
#[derive(Debug, Clone)]
pub struct InterferenceState {
    pub amplitude_pattern: Vec<Vec<Complex64>>,
    pub phase_pattern: Vec<Vec<f64>>,
    pub interference_map: Vec<Vec<f64>>,
    pub coherence_field: Vec<Vec<f64>>,
    pub total_energy: f64,
}

/// Interference pattern description
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct PatternKey {
    wavelength: u64, // Quantized wavelength for hashing
    phase_angle: i32, // Discretized phase angle
    pattern_type: InterferenceType,
}

/// Types of interference patterns
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum InterferenceType {
    Constructive,
    Destructive,
    Standing,
    Traveling,
}

/// Detailed interference pattern
#[derive(Debug, Clone)]
struct InterferencePattern {
    amplitude: Vec<Vec<f64>>,
    phase: Vec<Vec<f64>>,
    energy_distribution: Vec<Vec<f64>>,
    coherence: f64,
}

impl InterferenceAnalyzer {
    /// Create new interference analyzer
    pub fn new(config: InterferenceConfig) -> Result<Self, InterferenceError> {
        let (width, height) = config.grid_size;
        let initial_state = InterferenceState {
            amplitude_pattern: vec![vec![Complex64::new(0.0, 0.0); width]; height],
            phase_pattern: vec![vec![0.0; width]; height],
            interference_map: vec![vec![0.0; width]; height],
            coherence_field: vec![vec![0.0; width]; height],
            total_energy: 0.0,
        };

        let mut history = VecDeque::with_capacity(config.memory_length);
        history.push_back(initial_state.clone());

        Ok(Self {
            config,
            state: Arc::new(RwLock::new(initial_state)),
           history,
           pattern_cache: HashMap::new(),
        })
    }

    /// Analyze interference pattern
    pub fn analyze_pattern(&mut self, waves: &[Vec<Vec<Complex64>>]) -> Result<(), InterferenceError> {
        let mut state = self.state.write();
        let (width, height) = self.config.grid_size;

        // Calculate interference pattern
        let interference = self.calculate_interference(waves)?;

        // Update state
        state.amplitude_pattern = interference.amplitude_pattern;
        state.phase_pattern = interference.phase_pattern;
        state.interference_map = interference.energy_distribution;
        state.coherence_field = self.calculate_coherence_field(&interference)?;
        state.total_energy = self.calculate_total_energy(&interference.energy_distribution);

        // Update history
        if self.history.len() >= self.config.memory_length {
            self.history.pop_front();
        }
        self.history.push_back((*state).clone());

        Ok(())
    }

    /// Calculate interference between waves
    fn calculate_interference(&self, waves: &[Vec<Vec<Complex64>>]) -> Result<InterferencePattern, InterferenceError> {
        let (width, height) = self.config.grid_size;

        // Initialize result arrays
        let mut amplitude = vec![vec![0.0; width]; height];
        let mut phase = vec![vec![0.0; width]; height];
        let mut energy = vec![vec![0.0; width]; height];

        // Calculate interference pattern in parallel
        (0..height).into_par_iter().try_for_each(|i| {
            for j in 0..width {
                let mut sum = Complex64::new(0.0, 0.0);

                // Sum wave contributions
                for wave in waves {
                    if let Some(value) = wave.get(i).and_then(|row| row.get(j)) {
                        sum += value;
                    }
                }

                amplitude[i][j] = sum.norm();
                phase[i][j] = sum.arg();
                energy[i][j] = sum.norm_sqr();
            }
            Ok::<(), InterferenceError>(())
        })?;

        // Calculate coherence
        let coherence = self.calculate_pattern_coherence(&phase)?;

        Ok(InterferencePattern {
            amplitude,
            phase,
            energy_distribution: energy,
            coherence,
        })
    }

    /// Calculate coherence field
    fn calculate_coherence_field(&self, pattern: &InterferencePattern) -> Result<Vec<Vec<f64>>, InterferenceError> {
        let (width, height) = self.config.grid_size;
        let mut coherence_field = vec![vec![0.0; width]; height];

        // Calculate local coherence in parallel
        coherence_field.par_iter_mut().enumerate().try_for_each(|(i, row)| {
            for j in 0..width {
                row[j] = self.calculate_local_coherence(pattern, i, j)?;
            }
            Ok::<(), InterferenceError>(())
        })?;

        Ok(coherence_field)
    }

    /// Calculate local coherence at a point
    fn calculate_local_coherence(&self, pattern: &InterferencePattern, i: usize, j: usize) -> Result<f64, InterferenceError> {
        let (width, height) = self.config.grid_size;
        let mut phase_sum = Complex64::new(0.0, 0.0);
        let mut count = 0;

        // Sum phase contributions from neighbors
        for di in -1..=1 {
            for dj in -1..=1 {
                let ni = (i as i32 + di) as usize;
                let nj = (j as i32 + dj) as usize;

                if ni < height && nj < width {
                    phase_sum += Complex64::from_polar(1.0, pattern.phase[ni][nj]);
                    count += 1;
                }
            }
        }

        if count == 0 {
            return Ok(0.0);
        }

        Ok(phase_sum.norm() / count as f64)
    }

    /// Calculate total energy in interference pattern
    fn calculate_total_energy(&self, energy_distribution: &[Vec<f64>]) -> f64 {
        energy_distribution.par_iter()
        .map(|row| row.iter().sum::<f64>())
        .sum()
    }

    /// Identify interference type
    fn identify_interference_type(&self, pattern: &InterferencePattern) -> InterferenceType {
        let energy_variation = self.calculate_energy_variation(&pattern.energy_distribution);
        let phase_gradient = self.calculate_phase_gradient(&pattern.phase);

        match (energy_variation < 0.1, phase_gradient < 0.1) {
            (true, true) => InterferenceType::Standing,
            (true, false) => InterferenceType::Traveling,
            (false, true) => InterferenceType::Constructive,
            (false, false) => InterferenceType::Destructive,
        }
    }

    /// Calculate energy variation
    fn calculate_energy_variation(&self, energy: &[Vec<f64>]) -> f64 {
        let mean = energy.par_iter()
        .map(|row| row.iter().sum::<f64>())
        .sum::<f64>() / (energy.len() * energy[0].len()) as f64;

        let variance = energy.par_iter()
        .map(|row| {
            row.iter()
            .map(|&e| (e - mean).powi(2))
            .sum::<f64>()
        })
        .sum::<f64>() / (energy.len() * energy[0].len()) as f64;

        variance.sqrt() / mean
    }

    /// Calculate phase gradient
    fn calculate_phase_gradient(&self, phase: &[Vec<f64>]) -> f64 {
        let (width, height) = self.config.grid_size;
        let mut total_gradient = 0.0;
        let mut count = 0;

        for i in 1..height-1 {
            for j in 1..width-1 {
                let dx = phase[i][j+1] - phase[i][j-1];
                let dy = phase[i+1][j] - phase[i-1][j];
                total_gradient += (dx * dx + dy * dy).sqrt();
                count += 1;
            }
        }

        if count == 0 {
            0.0
        } else {
            total_gradient / count as f64
        }
    }

    /// Get current interference state
    pub fn get_state(&self) -> InterferenceState {
        self.state.read().clone()
    }

    /// Check if interference is coherent
    pub fn is_coherent(&self) -> bool {
        let state = self.state.read();
        let mean_coherence = state.coherence_field.iter()
        .map(|row| row.iter().sum::<f64>())
        .sum::<f64>() / (state.coherence_field.len() * state.coherence_field[0].len()) as f64;

        mean_coherence >= self.config.coherence_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_interference_analysis() -> Result<(), InterferenceError> {
        let config = InterferenceConfig::default();
        let mut analyzer = InterferenceAnalyzer::new(config)?;

        let (width, height) = config.grid_size;
        let wave1 = vec![vec![Complex64::new(1.0, 0.0); width]; height];
        let wave2 = vec![vec![Complex64::new(0.0, 1.0); width]; height];

        analyzer.analyze_pattern(&[wave1, wave2])?;
        let state = analyzer.get_state();

        assert!(state.total_energy > 0.0);
        Ok(())
    }

    #[test]
    fn test_coherence_calculation() -> Result<(), InterferenceError> {
        let config = InterferenceConfig::default();
        let mut analyzer = InterferenceAnalyzer::new(config)?;

        let (width, height) = config.grid_size;
        let wave = vec![vec![Complex64::new(1.0, 0.0); width]; height];

        analyzer.analyze_pattern(&[wave.clone()])?;
        assert!(analyzer.is_coherent());
        Ok(())
    }
}
