//! Crystal resonance attunement and harmonization
//! Created: 2025-01-21 15:41:07 UTC
//! Author: @isdood

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use crate::{
    core::harmonic::HarmonicProcessor,
    lattice::{
        node::LatticeNode,
        resonance::LatticeResonance,
    },
};

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AttunementError {
    #[error("Invalid attunement configuration: {0}")]
    InvalidConfig(String),
    #[error("Harmonization error: {0}")]
    HarmonizationError(String),
    #[error("Resonance stability error: {0}")]
    StabilityError(String),
    #[error("Phase alignment error: {0}")]
    PhaseError(String),
}

/// Configuration for resonance attunement
#[derive(Debug, Clone)]
pub struct AttunementConfig {
    pub base_frequency: f64,
    pub harmonic_depth: usize,
    pub phase_tolerance: f64,
    pub coupling_strength: f64,
    pub stability_threshold: f64,
    pub memory_depth: usize,
}

impl Default for AttunementConfig {
    fn default() -> Self {
        Self {
            base_frequency: 432.0,
            harmonic_depth: 7,
            phase_tolerance: 0.1,
            coupling_strength: 0.5,
            stability_threshold: 0.95,
            memory_depth: 128,
        }
    }
}

/// Resonance attunement manager
pub struct ResonanceAttunement {
    config: AttunementConfig,
    state: RwLock<AttunementState>,
    history: RwLock<VecDeque<AttunementState>>,
    harmonic_processor: Arc<HarmonicProcessor>,
    resonance_map: RwLock<HashMap<NodeGroup, ResonanceGroup>>,
}

/// Current attunement state
#[derive(Debug, Clone)]
pub struct AttunementState {
    pub harmonics: Vec<HarmonicLevel>,
    pub phase_coherence: f64,
    pub total_energy: f64,
    pub stability: f64,
}

/// Harmonic level information
#[derive(Debug, Clone)]
pub struct HarmonicLevel {
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub nodes: Vec<u64>,
}

/// Group of nodes for resonance
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct NodeGroup(Vec<u64>);

/// Resonance group information
#[derive(Debug, Clone)]
struct ResonanceGroup {
    frequency: f64,
    amplitude: Complex64,
    phase: f64,
    stability: f64,
    energy: f64,
}

impl ResonanceAttunement {
    /// Create new resonance attunement manager
    pub fn new(
        config: AttunementConfig,
        harmonic_processor: Arc<HarmonicProcessor>,
    ) -> Result<Self, AttunementError> {
        let initial_state = AttunementState {
            harmonics: Vec::new(),
            phase_coherence: 1.0,
            total_energy: 0.0,
            stability: 1.0,
        };

        Ok(Self {
            config,
            state: RwLock::new(initial_state.clone()),
           history: RwLock::new(VecDeque::with_capacity(config.memory_depth)),
           harmonic_processor,
           resonance_map: RwLock::new(HashMap::new()),
        })
    }

    /// Update attunement
    pub fn update(
        &self,
        nodes: &[Arc<LatticeNode>],
        resonance: &LatticeResonance,
        time: f64,
    ) -> Result<(), AttunementError> {
        // Update resonance groups
        self.update_resonance_groups(nodes, resonance)?;

        // Process harmonics
        self.process_harmonics(time)?;

        // Update global state
        self.update_state()?;

        // Update history
        self.update_history()?;

        Ok(())
    }

    /// Update resonance groups
    fn update_resonance_groups(
        &self,
        nodes: &[Arc<LatticeNode>],
        resonance: &LatticeResonance,
    ) -> Result<(), AttunementError> {
        let mut resonance_map = self.resonance_map.write();
        resonance_map.clear();

        // Group nodes by resonance
        let groups = self.group_nodes_by_resonance(nodes, resonance)?;

        // Calculate group properties
        for group in groups {
            let properties = self.calculate_group_properties(&group, nodes)?;
            resonance_map.insert(group, properties);
        }

        Ok(())
    }

    /// Group nodes by resonance characteristics
    fn group_nodes_by_resonance(
        &self,
        nodes: &[Arc<LatticeNode>],
        resonance: &LatticeResonance,
    ) -> Result<Vec<NodeGroup>, AttunementError> {
        let mut groups = Vec::new();
        let mut processed = std::collections::HashSet::new();

        for node in nodes {
            if processed.contains(&node.id()) {
                continue;
            }

            let group = self.find_resonant_group(node, nodes, resonance)?;
            processed.extend(group.0.iter());
            groups.push(group);
        }

        Ok(groups)
    }

    /// Find group of resonant nodes
    fn find_resonant_group(
        &self,
        seed: &LatticeNode,
        nodes: &[Arc<LatticeNode>],
        resonance: &LatticeResonance,
    ) -> Result<NodeGroup, AttunementError> {
        let mut group = vec![seed.id()];
        let seed_state = seed.get_state();

        for node in nodes {
            if node.id() == seed.id() {
                continue;
            }

            let node_state = node.get_state();
            if self.are_nodes_resonant(&seed_state, &node_state)? {
                group.push(node.id());
            }
        }

        Ok(NodeGroup(group))
    }

    /// Check if nodes are in resonance
    fn are_nodes_resonant(
        &self,
        state1: &crate::lattice::node::NodeState,
        state2: &crate::lattice::node::NodeState,
    ) -> Result<bool, AttunementError> {
        let freq_diff = (state1.frequency - state2.frequency).abs();
        let phase_diff = (state1.phase - state2.phase).abs();
        let normalized_phase = phase_diff % (2.0 * std::f64::consts::PI);

        Ok(freq_diff <= self.config.phase_tolerance &&
        normalized_phase <= self.config.phase_tolerance)
    }

    /// Calculate resonance group properties
    fn calculate_group_properties(
        &self,
        group: &NodeGroup,
        nodes: &[Arc<LatticeNode>],
    ) -> Result<ResonanceGroup, AttunementError> {
        let mut sum_amplitude = Complex64::new(0.0, 0.0);
        let mut sum_energy = 0.0;
        let mut sum_frequency = 0.0;
        let mut count = 0;

        for &node_id in &group.0 {
            if let Some(node) = nodes.iter().find(|n| n.id() == node_id) {
                let state = node.get_state();
                sum_amplitude += state.amplitude;
                sum_energy += state.energy;
                sum_frequency += state.frequency;
                count += 1;
            }
        }

        if count == 0 {
            return Err(AttunementError::HarmonizationError(
                "Empty resonance group".into()
            ));
        }

        let mean_amplitude = sum_amplitude / count as f64;
        let mean_frequency = sum_frequency / count as f64;
        let stability = self.calculate_group_stability(group, nodes)?;

        Ok(ResonanceGroup {
            frequency: mean_frequency,
            amplitude: mean_amplitude,
            phase: mean_amplitude.arg(),
           stability,
           energy: sum_energy,
        })
    }

    /// Calculate stability of resonance group
    fn calculate_group_stability(
        &self,
        group: &NodeGroup,
        nodes: &[Arc<LatticeNode>],
    ) -> Result<f64, AttunementError> {
        let mut sum_stability = 0.0;
        let mut count = 0;

        for &node_id in &group.0 {
            if let Some(node) = nodes.iter().find(|n| n.id() == node_id) {
                sum_stability += node.get_state().stability;
                count += 1;
            }
        }

        if count == 0 {
            return Ok(0.0);
        }

        Ok(sum_stability / count as f64)
    }

    /// Process harmonics for current groups
    fn process_harmonics(&self, time: f64) -> Result<(), AttunementError> {
        let resonance_map = self.resonance_map.read();
        let mut harmonics = Vec::new();

        // Calculate harmonics for each frequency level
        for harmonic_level in 1..=self.config.harmonic_depth {
            let level = self.calculate_harmonic_level(
                &resonance_map,
                harmonic_level,
                time,
            )?;
            harmonics.push(level);
        }

        // Update state with new harmonics
        let mut state = self.state.write();
        state.harmonics = harmonics;

        Ok(())
    }

    /// Calculate harmonic level
    fn calculate_harmonic_level(
        &self,
        resonance_map: &HashMap<NodeGroup, ResonanceGroup>,
        level: usize,
        time: f64,
    ) -> Result<HarmonicLevel, AttunementError> {
        let target_freq = self.config.base_frequency * level as f64;
        let mut nodes = Vec::new();
        let mut sum_amplitude = Complex64::new(0.0, 0.0);

        for (group, resonance) in resonance_map.iter() {
            if (resonance.frequency - target_freq).abs() <= self.config.phase_tolerance {
                nodes.extend(group.0.iter().cloned());
                sum_amplitude += resonance.amplitude;
            }
        }

        Ok(HarmonicLevel {
            frequency: target_freq,
            amplitude: sum_amplitude.norm(),
           phase: sum_amplitude.arg(),
           nodes,
        })
    }

    /// Update global state
    fn update_state(&self) -> Result<(), AttunementError> {
        let mut state = self.state.write();
        let resonance_map = self.resonance_map.read();

        // Calculate total energy
        state.total_energy = resonance_map.values()
        .map(|group| group.energy)
        .sum();

        // Calculate stability
        state.stability = self.calculate_global_stability(&resonance_map)?;

        // Calculate phase coherence
        state.phase_coherence = self.calculate_phase_coherence(&resonance_map)?;

        Ok(())
    }

    /// Calculate global stability
    fn calculate_global_stability(
        &self,
        resonance_map: &HashMap<NodeGroup, ResonanceGroup>,
    ) -> Result<f64, AttunementError> {
        if resonance_map.is_empty() {
            return Ok(1.0);
        }

        let stability = resonance_map.values()
        .map(|group| group.stability)
        .sum::<f64>() / resonance_map.len() as f64;

        Ok(stability)
    }

    /// Calculate phase coherence
    fn calculate_phase_coherence(
        &self,
        resonance_map: &HashMap<NodeGroup, ResonanceGroup>,
    ) -> Result<f64, AttunementError> {
        if resonance_map.is_empty() {
            return Ok(1.0);
        }

        let mut phase_vector = Complex64::new(0.0, 0.0);
        let mut total_amplitude = 0.0;

        for group in resonance_map.values() {
            phase_vector += Complex64::from_polar(group.amplitude.norm(), group.phase);
            total_amplitude += group.amplitude.norm();
        }

        if total_amplitude == 0.0 {
            return Ok(0.0);
        }

        Ok(phase_vector.norm() / total_amplitude)
    }

    /// Update history
    fn update_history(&self) -> Result<(), AttunementError> {
        let mut history = self.history.write();
        let current_state = self.state.read().clone();

        if history.len() >= self.config.memory_depth {
            history.pop_front();
        }
        history.push_back(current_state);

        Ok(())
    }

    /// Get current attunement state
    pub fn get_state(&self) -> AttunementState {
        self.state.read().clone()
    }

    /// Check if attunement is stable
    pub fn is_stable(&self) -> bool {
        let state = self.state.read();
        state.stability >= self.config.stability_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_attunement_initialization() -> Result<(), AttunementError> {
        let config = AttunementConfig::default();
        let harmonic_processor = Arc::new(HarmonicProcessor::new(Default::default())?);
        let attunement = ResonanceAttunement::new(config, harmonic_processor)?;

        let state = attunement.get_state();
        assert!(state.harmonics.is_empty());
        assert_relative_eq!(state.phase_coherence, 1.0);
        assert_relative_eq!(state.total_energy, 0.0);
        assert_relative_eq!(state.stability, 1.0);
        Ok(())
    }

    #[test]
    fn test_resonance_grouping() -> Result<(), AttunementError> {
        let config = AttunementConfig::default();
        let harmonic_processor = Arc::new(HarmonicProcessor::new(Default::default())?);
        let attunement = ResonanceAttunement::new(config, harmonic_processor)?;

        // Create test nodes with similar frequencies
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0])))
        .collect();

        // Apply similar forces to create resonance
        for node in &nodes {
            node.apply_force(Complex64::new(1.0, 0.0))?;
        }

        let resonance = LatticeResonance::new(Default::default());
        attunement.update(&nodes, &resonance, 0.0)?;

        let resonance_map = attunement.resonance_map.read();
        assert!(!resonance_map.is_empty());
        Ok(())
    }

    #[test]
    fn test_harmonic_processing() -> Result<(), AttunementError> {
        let config = AttunementConfig {
            harmonic_depth: 3,
            ..Default::default()
        };
        let harmonic_processor = Arc::new(HarmonicProcessor::new(Default::default())?);
        let attunement = ResonanceAttunement::new(config.clone(), harmonic_processor)?;

        // Create nodes at harmonic frequencies
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| {
            let mut node = Arc::new(LatticeNode::new(Default::default(), [0.0, i as f64, 0.0]));
            let frequency = config.base_frequency * (i + 1) as f64;
            node.get_state().frequency = frequency;
            node
        })
        .collect();

        let resonance = LatticeResonance::new(Default::default());
        attunement.update(&nodes, &resonance, 0.0)?;

        let state = attunement.get_state();
        assert_eq!(state.harmonics.len(), config.harmonic_depth);
        Ok(())
    }

    #[test]
    fn test_phase_coherence() -> Result<(), AttunementError> {
        let config = AttunementConfig::default();
        let harmonic_processor = Arc::new(HarmonicProcessor::new(Default::default())?);
        let attunement = ResonanceAttunement::new(config, harmonic_processor)?;

        // Create nodes with aligned phases
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| {
            let node = Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0]));
            node.apply_force(Complex64::from_polar(1.0, 0.0)).unwrap();
            node
        })
        .collect();

        let resonance = LatticeResonance::new(Default::default());
        attunement.update(&nodes, &resonance, 0.0)?;

        let state = attunement.get_state();
        assert!(state.phase_coherence > 0.9);
        Ok(())
    }

    #[test]
    fn test_stability_threshold() -> Result<(), AttunementError> {
        let mut config = AttunementConfig::default();
        config.stability_threshold = 0.8;
        let harmonic_processor = Arc::new(HarmonicProcessor::new(Default::default())?);
        let attunement = ResonanceAttunement::new(config, harmonic_processor)?;

        // Create stable nodes
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| {
            let node = Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0]));
            node.apply_force(Complex64::new(1.0, 0.0)).unwrap();
            node
        })
        .collect();

        let resonance = LatticeResonance::new(Default::default());
        attunement.update(&nodes, &resonance, 0.0)?;

        assert!(attunement.is_stable());
        Ok(())
    }

    #[test]
    fn test_energy_conservation() -> Result<(), AttunementError> {
        let config = AttunementConfig::default();
        let harmonic_processor = Arc::new(HarmonicProcessor::new(Default::default())?);
        let attunement = ResonanceAttunement::new(config, harmonic_processor)?;

        // Create energetic nodes
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| {
            let node = Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0]));
            node.apply_force(Complex64::new(1.0, 0.0)).unwrap();
            node
        })
        .collect();

        let resonance = LatticeResonance::new(Default::default());

        // Initial update
        attunement.update(&nodes, &resonance, 0.0)?;
        let initial_energy = attunement.get_state().total_energy;

        // Second update
        attunement.update(&nodes, &resonance, 0.1)?;
        let final_energy = attunement.get_state().total_energy;

        // Energy should be conserved or decrease
        assert!(final_energy <= initial_energy);
        Ok(())
    }

    #[test]
    fn test_harmonic_levels() -> Result<(), AttunementError> {
        let config = AttunementConfig {
            base_frequency: 440.0,
            harmonic_depth: 4,
            ..Default::default()
        };
        let harmonic_processor = Arc::new(HarmonicProcessor::new(Default::default())?);
        let attunement = ResonanceAttunement::new(config.clone(), harmonic_processor)?;

        // Create nodes at harmonic frequencies
        let nodes: Vec<Arc<LatticeNode>> = (0..4)
        .map(|i| {
            let mut node = Arc::new(LatticeNode::new(Default::default(), [0.0, i as f64, 0.0]));
            let frequency = config.base_frequency * (i + 1) as f64;
            node.get_state().frequency = frequency;
            node.apply_force(Complex64::new(1.0, 0.0)).unwrap();
            node
        })
        .collect();

        let resonance = LatticeResonance::new(Default::default());
        attunement.update(&nodes, &resonance, 0.0)?;

        let state = attunement.get_state();

        // Check harmonic frequencies
        for (i, harmonic) in state.harmonics.iter().enumerate() {
            assert_relative_eq!(
                harmonic.frequency,
                config.base_frequency * (i + 1) as f64,
                                epsilon = 1e-6
            );
        }
        Ok(())
    }

    #[test]
    fn test_history_management() -> Result<(), AttunementError> {
        let mut config = AttunementConfig::default();
        config.memory_depth = 3;
        let harmonic_processor = Arc::new(HarmonicProcessor::new(Default::default())?);
        let attunement = ResonanceAttunement::new(config, harmonic_processor)?;

        let nodes: Vec<Arc<LatticeNode>> = vec![
            Arc::new(LatticeNode::new(Default::default(), [0.0, 0.0, 0.0]))
        ];
        let resonance = LatticeResonance::new(Default::default());

        // Perform multiple updates
        for i in 0..5 {
            attunement.update(&nodes, &resonance, i as f64)?;
        }

        let history = attunement.history.read();
        assert!(history.len() <= config.memory_depth);
        Ok(())
    }
}
