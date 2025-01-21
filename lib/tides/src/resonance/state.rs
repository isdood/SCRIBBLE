//! Crystal state management and synchronization
//! Created: 2025-01-21 15:44:04 UTC
//! Author: @isdood

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use crate::{
    core::wave_pattern::WavePattern,
    harmony::{
        crystalline::CrystallineState,
        resonator::ResonatorState,
    },
    lattice::{
        node::NodeState,
        resonance::LatticeResonance,
    },
};

use num_complex::Complex64;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Invalid state configuration: {0}")]
    InvalidConfig(String),
    #[error("State synchronization failed: {0}")]
    SyncError(String),
    #[error("State transition error: {0}")]
    TransitionError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Crystal state configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    pub memory_depth: usize,
    pub sync_interval: f64,
    pub stability_threshold: f64,
    pub transition_smoothing: f64,
}

impl Default for StateConfig {
    fn default() -> Self {
        Self {
            memory_depth: 256,
            sync_interval: 0.1,
            stability_threshold: 0.95,
            transition_smoothing: 0.5,
        }
    }
}

/// Crystal state manager
pub struct CrystalState {
    config: StateConfig,
    state: RwLock<GlobalState>,
    history: RwLock<VecDeque<GlobalState>>,
    wave_pattern: Arc<WavePattern>,
    resonance: Arc<LatticeResonance>,
    last_sync: std::time::Instant,
}

/// Global crystal state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub timestamp: f64,
    pub energy_field: Vec<Vec<f64>>,
    pub phase_field: Vec<Vec<f64>>,
    pub amplitude_field: Vec<Vec<Complex64>>,
    pub stability: f64,
    pub coherence: f64,
    pub total_energy: f64,
    pub node_states: HashMap<u64, NodeState>,
    pub resonator_state: Option<ResonatorState>,
    pub crystalline_state: Option<CrystallineState>,
}

impl CrystalState {
    /// Create new crystal state manager
    pub fn new(
        config: StateConfig,
        wave_pattern: Arc<WavePattern>,
        resonance: Arc<LatticeResonance>,
    ) -> Result<Self, StateError> {
        let initial_state = GlobalState {
            timestamp: 0.0,
            energy_field: Vec::new(),
            phase_field: Vec::new(),
            amplitude_field: Vec::new(),
            stability: 1.0,
            coherence: 1.0,
            total_energy: 0.0,
            node_states: HashMap::new(),
            resonator_state: None,
            crystalline_state: None,
        };

        Ok(Self {
            config,
            state: RwLock::new(initial_state.clone()),
           history: RwLock::new(VecDeque::with_capacity(config.memory_depth)),
           wave_pattern,
           resonance,
           last_sync: std::time::Instant::now(),
        })
    }

    /// Update global state
    pub fn update(&self, time: f64) -> Result<(), StateError> {
        // Check if synchronization is needed
        if self.should_sync() {
            self.synchronize()?;
        }

        // Update fields
        self.update_fields(time)?;

        // Update history
        self.update_history()?;

        Ok(())
    }

    /// Synchronize state with components
    fn synchronize(&self) -> Result<(), StateError> {
        let mut state = self.state.write();

        // Synchronize with wave pattern
        let wave_state = self.wave_pattern.get_state();
        state.amplitude_field = wave_state.amplitudes;
        state.phase_field = wave_state.phases;

        // Synchronize with resonance
        let resonance_state = self.resonance.get_state();
        state.stability = resonance_state.stability;
        state.coherence = resonance_state.coherence;

        // Update timestamp
        state.timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| StateError::SyncError(e.to_string()))?
        .as_secs_f64();

        Ok(())
    }

    /// Update state fields
    fn update_fields(&self, time: f64) -> Result<(), StateError> {
        let mut state = self.state.write();

        // Calculate energy field
        state.energy_field = state.amplitude_field.iter()
        .map(|row| row.iter().map(|c| c.norm_sqr()).collect())
        .collect();

        // Calculate total energy
        state.total_energy = state.energy_field.iter()
        .map(|row| row.iter().sum::<f64>())
        .sum();

        Ok(())
    }

    /// Update state history
    fn update_history(&self) -> Result<(), StateError> {
        let mut history = self.history.write();
        let current_state = self.state.read().clone();

        if history.len() >= self.config.memory_depth {
            history.pop_front();
        }
        history.push_back(current_state);

        Ok(())
    }

    /// Check if synchronization is needed
    fn should_sync(&self) -> bool {
        self.last_sync.elapsed().as_secs_f64() >= self.config.sync_interval
    }

    /// Get current global state
    pub fn get_state(&self) -> GlobalState {
        self.state.read().clone()
    }

    /// Get state history
    pub fn get_history(&self) -> Vec<GlobalState> {
        self.history.read().iter().cloned().collect()
    }

    /// Check if state is stable
    pub fn is_stable(&self) -> bool {
        self.state.read().stability >= self.config.stability_threshold
    }

    /// Serialize state to JSON
    pub fn serialize(&self) -> Result<String, StateError> {
        serde_json::to_string(&*self.state.read())
        .map_err(|e| StateError::SerializationError(e.to_string()))
    }

    /// Deserialize state from JSON
    pub fn deserialize(&self, json: &str) -> Result<(), StateError> {
        let new_state: GlobalState = serde_json::from_str(json)
        .map_err(|e| StateError::SerializationError(e.to_string()))?;
        *self.state.write() = new_state;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_state_initialization() -> Result<(), StateError> {
        let config = StateConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let resonance = Arc::new(LatticeResonance::new(Default::default()));

        let state_manager = CrystalState::new(config, wave_pattern, resonance)?;
        let state = state_manager.get_state();

        assert_relative_eq!(state.stability, 1.0);
        assert_relative_eq!(state.coherence, 1.0);
        assert_relative_eq!(state.total_energy, 0.0);
        Ok(())
    }

    #[test]
    fn test_state_update() -> Result<(), StateError> {
        let config = StateConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let resonance = Arc::new(LatticeResonance::new(Default::default()));

        let state_manager = CrystalState::new(config, wave_pattern, resonance)?;

        // Initial update
        state_manager.update(0.0)?;
        let initial_state = state_manager.get_state();

        // Second update
        state_manager.update(0.1)?;
        let updated_state = state_manager.get_state();

        assert!(updated_state.timestamp > initial_state.timestamp);
        Ok(())
    }

    #[test]
    fn test_history_management() -> Result<(), StateError> {
        let mut config = StateConfig::default();
        config.memory_depth = 3;
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let resonance = Arc::new(LatticeResonance::new(Default::default()));

        let state_manager = CrystalState::new(config.clone(), wave_pattern, resonance)?;

        // Perform multiple updates
        for i in 0..5 {
            state_manager.update(i as f64)?;
        }

        let history = state_manager.get_history();
        assert!(history.len() <= config.memory_depth);
        Ok(())
    }

    #[test]
    fn test_serialization() -> Result<(), StateError> {
        let config = StateConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let resonance = Arc::new(LatticeResonance::new(Default::default()));

        let state_manager = CrystalState::new(config, wave_pattern, resonance)?;

        // Serialize
        let json = state_manager.serialize()?;
        assert!(!json.is_empty());

        // Deserialize
        state_manager.deserialize(&json)?;
        Ok(())
    }

    #[test]
    fn test_stability_threshold() -> Result<(), StateError> {
        let mut config = StateConfig::default();
        config.stability_threshold = 0.8;
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let resonance = Arc::new(LatticeResonance::new(Default::default()));

        let state_manager = CrystalState::new(config, wave_pattern, resonance)?;

        {
            let mut state = state_manager.state.write();
            state.stability = 0.9;
        }

        assert!(state_manager.is_stable());
        Ok(())
    }

    #[test]
    fn test_energy_conservation() -> Result<(), StateError> {
        let config = StateConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let resonance = Arc::new(LatticeResonance::new(Default::default()));

        let state_manager = CrystalState::new(config, wave_pattern, resonance)?;

        // Set initial energy field
        {
            let mut state = state_manager.state.write();
            state.amplitude_field = vec![vec![Complex64::new(1.0, 0.0); 2]; 2];
        }

        state_manager.update(0.0)?;
        let state = state_manager.get_state();

        // Check energy calculation
        assert_relative_eq!(state.total_energy, 4.0); // 2x2 field with amplitude 1.0
        Ok(())
    }

    #[test]
    fn test_sync_interval() -> Result<(), StateError> {
        let mut config = StateConfig::default();
        config.sync_interval = 0.1;
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let resonance = Arc::new(LatticeResonance::new(Default::default()));

        let state_manager = CrystalState::new(config, wave_pattern, resonance)?;

        // Initial sync should be needed
        assert!(state_manager.should_sync());

        // Update to trigger sync
        state_manager.update(0.0)?;

        // Immediate sync should not be needed
        assert!(!state_manager.should_sync());
        Ok(())
    }
}
