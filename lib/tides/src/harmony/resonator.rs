//! Crystal resonator pattern management and synchronization
//! Created: 2025-01-21 15:13:47 UTC
//! Author: @isdood

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResonatorError {
    #[error("Invalid resonator configuration: {0}")]
    InvalidConfig(String),
    #[error("Resonance stability error: {0}")]
    StabilityError(String),
    #[error("Pattern synchronization failed: {0}")]
    SyncError(String),
    #[error("Energy conservation violated: {0}")]
    EnergyError(String),
}

/// Configuration for crystal resonator
#[derive(Debug, Clone)]
pub struct ResonatorConfig {
    pub lattice_size: (usize, usize),
    pub frequency_range: (f64, f64),
    pub quality_factor: f64,
    pub coupling_strength: f64,
    pub stability_threshold: f64,
    pub memory_depth: usize,
}

impl Default for ResonatorConfig {
    fn default() -> Self {
        Self {
            lattice_size: (64, 64),
            frequency_range: (20.0, 20000.0),
            quality_factor: 100.0,
            coupling_strength: 0.5,
            stability_threshold: 0.001,
            memory_depth: 256,
        }
    }
}

/// Crystal resonator manager
pub struct CrystalResonator {
    config: ResonatorConfig,
    state: Arc<RwLock<ResonatorState>>,
    history: VecDeque<ResonatorState>,
    resonance_nodes: HashMap<(usize, usize), ResonanceNode>,
    mode_cache: HashMap<ResonanceMode, ModePattern>,
}

/// Current state of resonator
#[derive(Debug, Clone)]
pub struct ResonatorState {
    pub amplitude_field: Vec<Vec<Complex64>>,
    pub frequency_field: Vec<Vec<f64>>,
    pub quality_field: Vec<Vec<f64>>,
    pub energy_distribution: Vec<Vec<f64>>,
    pub total_energy: f64,
    pub stability: f64,
}

/// Resonance node in crystal lattice
#[derive(Debug, Clone)]
struct ResonanceNode {
    frequency: f64,
    amplitude: Complex64,
    quality: f64,
    coupling: Vec<(usize, usize)>,
    mode: ResonanceMode,
}

/// Resonance mode identifier
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum ResonanceMode {
    Fundamental,
    Harmonic(usize),
    Coupled(usize, usize),
    Standing(usize),
}

/// Mode pattern description
#[derive(Debug, Clone)]
struct ModePattern {
    amplitude: Vec<Vec<f64>>,
    phase: Vec<Vec<f64>>,
    frequency_response: Vec<f64>,
    stability: f64,
}

impl CrystalResonator {
    /// Create new crystal resonator
    pub fn new(config: ResonatorConfig) -> Result<Self, ResonatorError> {
        let (width, height) = config.lattice_size;
        let initial_state = ResonatorState {
            amplitude_field: vec![vec![Complex64::new(0.0, 0.0); width]; height],
            frequency_field: vec![vec![0.0; width]; height],
            quality_field: vec![vec![config.quality_factor; width]; height],
            energy_distribution: vec![vec![0.0; width]; height],
            total_energy: 0.0,
            stability: 1.0,
        };

        let mut history = VecDeque::with_capacity(config.memory_depth);
        history.push_back(initial_state.clone());

        Ok(Self {
            config,
            state: Arc::new(RwLock::new(initial_state)),
           history,
           resonance_nodes: HashMap::new(),
           mode_cache: HashMap::new(),
        })
    }

    /// Update resonator state
    pub fn update(&mut self, time: f64) -> Result<(), ResonatorError> {
        // Update resonance nodes
        self.update_resonance_nodes(time)?;

        // Calculate mode patterns
        self.update_mode_patterns()?;

        // Update global state
        self.update_state()?;

        // Ensure energy conservation
        self.verify_energy_conservation()?;

        Ok(())
    }

    /// Update resonance nodes
    fn update_resonance_nodes(&mut self, time: f64) -> Result<(), ResonatorError> {
        let (width, height) = self.config.lattice_size;
        let mut new_nodes = HashMap::new();

        // Update nodes in parallel
        let nodes: Vec<((usize, usize), ResonanceNode)> = (0..height)
        .into_par_iter()
        .flat_map(|i| {
            (0..width).into_par_iter().filter_map(move |j| {
                self.calculate_resonance_node(i, j, time).ok()
                .map(|node| ((i, j), node))
            })
        })
        .collect();

        // Update map with new nodes
        for (coords, node) in nodes {
            new_nodes.insert(coords, node);
        }

        self.resonance_nodes = new_nodes;
        Ok(())
    }

    /// Calculate resonance node properties
    fn calculate_resonance_node(&self, i: usize, j: usize, time: f64) -> Result<ResonanceNode, ResonatorError> {
        let state = self.state.read();
        let current_amp = state.amplitude_field[i][j];
        let current_freq = state.frequency_field[i][j];
        let current_q = state.quality_field[i][j];

        // Find coupled nodes
        let coupling = self.find_coupled_nodes(i, j)?;

        // Calculate resonance properties
        let (frequency, amplitude, quality) = self.calculate_resonance(
            current_amp,
            current_freq,
            current_q,
            &coupling,
            time,
        )?;

        // Determine resonance mode
        let mode = self.determine_resonance_mode(frequency, &coupling);

        Ok(ResonanceNode {
            frequency,
            amplitude,
            quality,
            coupling,
            mode,
        })
    }

    /// Find coupled resonance nodes
    fn find_coupled_nodes(&self, i: usize, j: usize) -> Result<Vec<(usize, usize)>, ResonatorError> {
        let (width, height) = self.config.lattice_size;
        let mut coupled = Vec::new();
        let threshold = self.config.stability_threshold;

        // Check neighboring nodes
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 { continue; }

                let ni = (i as i32 + di) as usize;
                let nj = (j as i32 + dj) as usize;

                if ni < height && nj < width {
                    if let Some(node) = self.resonance_nodes.get(&(ni, nj)) {
                        if node.quality >= threshold {
                            coupled.push((ni, nj));
                        }
                    }
                }
            }
        }

        Ok(coupled)
    }

    /// Calculate resonance properties
    fn calculate_resonance(
        &self,
        current_amp: Complex64,
        current_freq: f64,
        current_q: f64,
        coupling: &[(usize, usize)],
                           time: f64,
    ) -> Result<(f64, Complex64, f64), ResonatorError> {
        let omega = 2.0 * std::f64::consts::PI * current_freq;
        let coupling_factor = self.config.coupling_strength * coupling.len() as f64;

        // Calculate new amplitude with coupling
        let mut new_amp = current_amp * (-time / current_q).exp();
        for &(ni, nj) in coupling {
            if let Some(node) = self.resonance_nodes.get(&(ni, nj)) {
                new_amp += node.amplitude * coupling_factor;
            }
        }

        // Update frequency based on coupling
        let new_freq = current_freq * (1.0 + coupling_factor * (omega * time).sin());

        // Update quality factor
        let new_q = current_q * (1.0 + coupling_factor * 0.1);

        Ok((new_freq, new_amp, new_q))
    }

    /// Determine resonance mode
    fn determine_resonance_mode(&self, frequency: f64, coupling: &[(usize, usize)]) -> ResonanceMode {
        let base_freq = self.config.frequency_range.0;
        let harmonic = (frequency / base_freq).round() as usize;

        match (harmonic, coupling.len()) {
            (1, 0) => ResonanceMode::Fundamental,
            (h, 0) => ResonanceMode::Harmonic(h),
            (h, 1) => ResonanceMode::Standing(h),
            (h, _) => ResonanceMode::Coupled(h, coupling.len()),
        }
    }

    /// Update mode patterns
    fn update_mode_patterns(&mut self) -> Result<(), ResonatorError> {
        let mut new_cache = HashMap::new();

        // Calculate patterns for each mode
        for node in self.resonance_nodes.values() {
            if !new_cache.contains_key(&node.mode) {
                let pattern = self.calculate_mode_pattern(&node.mode)?;
                new_cache.insert(node.mode.clone(), pattern);
            }
        }

        self.mode_cache = new_cache;
        Ok(())
    }

    /// Calculate mode pattern
    fn calculate_mode_pattern(&self, mode: &ResonanceMode) -> Result<ModePattern, ResonatorError> {
        let (width, height) = self.config.lattice_size;
        let mut amplitude = vec![vec![0.0; width]; height];
        let mut phase = vec![vec![0.0; width]; height];
        let mut freq_response = vec![0.0; 1024];

        // Calculate pattern based on mode
        match mode {
            ResonanceMode::Fundamental => {
                self.calculate_fundamental_pattern(&mut amplitude, &mut phase)?;
            }
            ResonanceMode::Harmonic(n) => {
                self.calculate_harmonic_pattern(*n, &mut amplitude, &mut phase)?;
            }
            ResonanceMode::Coupled(h, c) => {
                self.calculate_coupled_pattern(*h, *c, &mut amplitude, &mut phase)?;
            }
            ResonanceMode::Standing(n) => {
                self.calculate_standing_pattern(*n, &mut amplitude, &mut phase)?;
            }
        }

        // Calculate frequency response
        self.calculate_frequency_response(mode, &mut freq_response)?;

        // Calculate pattern stability
        let stability = self.calculate_pattern_stability(&amplitude, &phase)?;

        Ok(ModePattern {
            amplitude,
            phase,
            frequency_response: freq_response,
            stability,
        })
    }

    /// Calculate fundamental pattern
    fn calculate_fundamental_pattern(
        &self,
        amplitude: &mut [Vec<f64>],
        phase: &mut [Vec<f64>],
    ) -> Result<(), ResonatorError> {
        let (width, height) = self.config.lattice_size;
        let base_freq = self.config.frequency_range.0;

        for i in 0..height {
            for j in 0..width {
                let r = ((i as f64 / height as f64 - 0.5).powi(2) +
                (j as f64 / width as f64 - 0.5).powi(2)).sqrt();
                amplitude[i][j] = (-r * base_freq).exp();
                phase[i][j] = r * 2.0 * std::f64::consts::PI;
            }
        }

        Ok(())
    }

    /// Update global state
    fn update_state(&mut self) -> Result<(), ResonatorError> {
        let mut state = self.state.write();
        let (width, height) = self.config.lattice_size;

        // Update fields
        for i in 0..height {
            for j in 0..width {
                if let Some(node) = self.resonance_nodes.get(&(i, j)) {
                    state.amplitude_field[i][j] = node.amplitude;
                    state.frequency_field[i][j] = node.frequency;
                    state.quality_field[i][j] = node.quality;
                    state.energy_distribution[i][j] = node.amplitude.norm_sqr();
                }
            }
        }

        // Update global properties
        state.total_energy = state.energy_distribution.iter()
        .map(|row| row.iter().sum::<f64>())
        .sum();

        state.stability = self.calculate_global_stability()?;

        // Update history
        if self.history.len() >= self.config.memory_depth {
            self.history.pop_front();
        }
        self.history.push_back((*state).clone());

        Ok(())
    }

    /// Calculate global stability
    fn calculate_global_stability(&self) -> Result<f64, ResonatorError> {
        let stable_nodes = self.resonance_nodes.values()
        .filter(|node| node.quality >= self.config.stability_threshold)
        .count();

        let total_nodes = self.resonance_nodes.len();
        if total_nodes == 0 {
            return Ok(0.0);
        }

        Ok((stable_nodes as f64 / total_nodes as f64).min(1.0))
    }

    /// Verify energy conservation
    fn verify_energy_conservation(&self) -> Result<(), ResonatorError> {
        let state = self.state.read();
        let current_energy = state.total_energy;
        let previous_energy = self.history.back()
        .map(|prev| prev.total_energy)
        .unwrap_or(current_energy);

        let energy_change = (current_energy - previous_energy).abs();
        if energy_change > self.config.stability_threshold * previous_energy {
            return Err(ResonatorError::EnergyError(
                format!("Energy conservation violated: change = {}", energy_change)
            ));
        }

        Ok(())
    }

    /// Get current resonator state
    pub fn get_state(&self) -> ResonatorState {
        self.state.read().clone()
    }

    /// Check if resonator is stable
    pub fn is_stable(&self) -> bool {
        self.state.read().stability >= self.config.stability_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_resonator_initialization() -> Result<(), ResonatorError> {
        let config = ResonatorConfig::default();
        let resonator = CrystalResonator::new(config)?;
        let state = resonator.get_state();

        assert_eq!(state.amplitude_field.len(), 64);
        assert_eq!(state.frequency_field.len(), 64);
        assert_eq!(state.quality_field.len(), 64);
        assert_relative_eq!(state.total_energy, 0.0);
        assert_relative_eq!(state.stability, 1.0);
        Ok(())
    }

    #[test]
    fn test_resonator_update() -> Result<(), ResonatorError> {
        let config = ResonatorConfig::default();
        let mut resonator = CrystalResonator::new(config)?;

        // Initial update
        resonator.update(0.0)?;
        let initial_state = resonator.get_state();

        // Second update
        resonator.update(0.1)?;
        let updated_state = resonator.get_state();

        // Check that energy is conserved within threshold
        let energy_diff = (updated_state.total_energy - initial_state.total_energy).abs();
        assert!(energy_diff <= config.stability_threshold);
        Ok(())
    }

    #[test]
    fn test_resonance_node_coupling() -> Result<(), ResonatorError> {
        let mut config = ResonatorConfig::default();
        config.coupling_strength = 0.5;
        let mut resonator = CrystalResonator::new(config)?;

        // Initialize with a single excited node
        {
            let mut state = resonator.state.write();
            state.amplitude_field[32][32] = Complex64::new(1.0, 0.0);
        }

        // Update and check coupling
        resonator.update(0.1)?;
        let state = resonator.get_state();

        // Check that energy has spread to neighboring nodes
        assert!(state.amplitude_field[31][32].norm() > 0.0);
        assert!(state.amplitude_field[33][32].norm() > 0.0);
        Ok(())
    }

    #[test]
    fn test_mode_patterns() -> Result<(), ResonatorError> {
        let config = ResonatorConfig::default();
        let mut resonator = CrystalResonator::new(config)?;

        // Initialize with fundamental mode
        {
            let mut state = resonator.state.write();
            for i in 0..64 {
                for j in 0..64 {
                    let r = ((i as f64 - 32.0).powi(2) + (j as f64 - 32.0).powi(2)).sqrt() / 32.0;
                    state.amplitude_field[i][j] = Complex64::new((-r).exp(), 0.0);
                }
            }
        }

        resonator.update(0.0)?;

        // Check that fundamental mode is recognized
        assert!(resonator.mode_cache.contains_key(&ResonanceMode::Fundamental));
        Ok(())
    }

    #[test]
    fn test_stability_threshold() -> Result<(), ResonatorError> {
        let mut config = ResonatorConfig::default();
        config.stability_threshold = 0.1;
        let mut resonator = CrystalResonator::new(config)?;

        // Initialize with unstable state
        {
            let mut state = resonator.state.write();
            for i in 0..64 {
                for j in 0..64 {
                    state.quality_field[i][j] = 0.05;
                }
            }
        }

        resonator.update(0.0)?;
        assert!(!resonator.is_stable());

        // Transition to stable state
        {
            let mut state = resonator.state.write();
            for i in 0..64 {
                for j in 0..64 {
                    state.quality_field[i][j] = 0.2;
                }
            }
        }

        resonator.update(0.0)?;
        assert!(resonator.is_stable());
        Ok(())
    }

    #[test]
    fn test_energy_conservation() -> Result<(), ResonatorError> {
        let config = ResonatorConfig::default();
        let mut resonator = CrystalResonator::new(config)?;

        // Initialize with some energy
        {
            let mut state = resonator.state.write();
            state.amplitude_field[32][32] = Complex64::new(1.0, 0.0);
            state.total_energy = 1.0;
        }

        // Multiple updates should conserve energy within threshold
        for t in 0..10 {
            resonator.update(t as f64 * 0.1)?;
            let state = resonator.get_state();
            assert!(state.total_energy > 0.0);
            assert!(state.total_energy <= 1.0);
        }
        Ok(())
    }

    #[test]
    fn test_resonance_modes() -> Result<(), ResonatorError> {
        let config = ResonatorConfig::default();
        let mut resonator = CrystalResonator::new(config)?;

        // Test different modes
        let modes = vec![
            ResonanceMode::Fundamental,
            ResonanceMode::Harmonic(2),
            ResonanceMode::Standing(3),
            ResonanceMode::Coupled(2, 4),
        ];

        for mode in modes {
            let pattern = resonator.calculate_mode_pattern(&mode)?;
            assert!(pattern.stability >= 0.0 && pattern.stability <= 1.0);
            assert!(!pattern.amplitude.is_empty());
            assert!(!pattern.phase.is_empty());
            assert!(!pattern.frequency_response.is_empty());
        }
        Ok(())
    }

    #[test]
    fn test_quality_factor_evolution() -> Result<(), ResonatorError> {
        let config = ResonatorConfig::default();
        let mut resonator = CrystalResonator::new(config)?;

        let initial_q = resonator.get_state().quality_field[32][32];

        // Update multiple times
        for _ in 0..5 {
            resonator.update(0.1)?;
        }

        let final_q = resonator.get_state().quality_field[32][32];

        // Quality factor should change with coupling
        assert_ne!(initial_q, final_q);
        assert!(final_q > 0.0);
        Ok(())
    }
}
