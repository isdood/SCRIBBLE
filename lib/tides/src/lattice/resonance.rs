//! Crystal lattice resonance management and analysis
//! Created: 2025-01-21 15:24:00 UTC
//! Author: @isdood

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use super::node::{LatticeNode, NodeState, NodeError};
use crate::harmony::resonator::ResonatorState;

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResonanceError {
    #[error("Invalid resonance configuration: {0}")]
    InvalidConfig(String),
    #[error("Pattern formation error: {0}")]
    PatternError(String),
    #[error("Synchronization error: {0}")]
    SyncError(String),
    #[error("Node error: {0}")]
    NodeError(#[from] NodeError),
}

/// Resonance pattern configuration
#[derive(Debug, Clone)]
pub struct ResonanceConfig {
    pub frequency_range: (f64, f64),
    pub coupling_threshold: f64,
    pub sync_tolerance: f64,
    pub pattern_depth: usize,
    pub stability_threshold: f64,
}

impl Default for ResonanceConfig {
    fn default() -> Self {
        Self {
            frequency_range: (20.0, 20000.0),
            coupling_threshold: 0.001,
            sync_tolerance: 0.1,
            pattern_depth: 7,
            stability_threshold: 0.95,
        }
    }
}

/// Lattice resonance manager
pub struct LatticeResonance {
    config: ResonanceConfig,
    nodes: HashMap<u64, Arc<LatticeNode>>,
    patterns: RwLock<HashMap<PatternId, ResonancePattern>>,
    state: RwLock<ResonanceState>,
}

/// Unique pattern identifier
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct PatternId {
    frequency: u64,  // Quantized frequency for hashing
    nodes: HashSet<u64>,
}

/// Resonance pattern information
#[derive(Debug, Clone)]
struct ResonancePattern {
    frequency: f64,
    amplitude: f64,
    phase: f64,
    nodes: HashSet<u64>,
    stability: f64,
    energy: f64,
}

/// Current resonance state
#[derive(Debug, Clone)]
struct ResonanceState {
    active_patterns: HashSet<PatternId>,
    total_energy: f64,
    stability: f64,
    coherence: f64,
}

impl LatticeResonance {
    /// Create new lattice resonance manager
    pub fn new(config: ResonanceConfig) -> Self {
        Self {
            config,
            nodes: HashMap::new(),
            patterns: RwLock::new(HashMap::new()),
            state: RwLock::new(ResonanceState {
                active_patterns: HashSet::new(),
                               total_energy: 0.0,
                               stability: 1.0,
                               coherence: 1.0,
            }),
        }
    }

    /// Add node to resonance network
    pub fn add_node(&mut self, node: Arc<LatticeNode>) -> Result<(), ResonanceError> {
        self.nodes.insert(node.id(), node);
        Ok(())
    }

    /// Remove node from resonance network
    pub fn remove_node(&mut self, node_id: u64) -> Result<(), ResonanceError> {
        self.nodes.remove(&node_id);
        self.cleanup_patterns(node_id)?;
        Ok(())
    }

    /// Update resonance patterns
    pub fn update(&mut self, time: f64) -> Result<(), ResonanceError> {
        // Update nodes
        self.update_nodes(time)?;

        // Detect and update patterns
        self.update_patterns()?;

        // Update global state
        self.update_state()?;

        Ok(())
    }

    /// Update all nodes
    fn update_nodes(&self, time: f64) -> Result<(), ResonanceError> {
        self.nodes.par_iter().try_for_each(|(_, node)| {
            node.update(time).map_err(ResonanceError::from)
        })
    }

    /// Update resonance patterns
    fn update_patterns(&mut self) -> Result<(), ResonanceError> {
        let mut new_patterns = HashMap::new();

        // Find resonant nodes
        let resonant_nodes: HashSet<u64> = self.nodes.par_iter()
        .filter(|(_, node)| node.is_resonant())
        .map(|(&id, _)| id)
        .collect();

        // Detect patterns
        let patterns = self.detect_patterns(&resonant_nodes)?;

        // Update patterns
        for pattern in patterns {
            let pattern_id = PatternId {
                frequency: self.quantize_frequency(pattern.frequency),
                nodes: pattern.nodes.clone(),
            };
            new_patterns.insert(pattern_id, pattern);
        }

        // Update pattern storage
        *self.patterns.write() = new_patterns;

        Ok(())
    }

    /// Detect resonance patterns
    fn detect_patterns(&self, resonant_nodes: &HashSet<u64>) -> Result<Vec<ResonancePattern>, ResonanceError> {
        let mut patterns = Vec::new();
        let mut processed = HashSet::new();

        for &node_id in resonant_nodes {
            if processed.contains(&node_id) {
                continue;
            }

            if let Some(pattern) = self.grow_pattern(node_id, resonant_nodes)? {
                processed.extend(pattern.nodes.iter());
                patterns.push(pattern);
            }
        }

        Ok(patterns)
    }

    /// Grow resonance pattern from seed node
    fn grow_pattern(&self, seed_id: u64, resonant_nodes: &HashSet<u64>) -> Result<Option<ResonancePattern>, ResonanceError> {
        let seed_node = self.nodes.get(&seed_id)
        .ok_or_else(|| ResonanceError::InvalidConfig("Seed node not found".into()))?;

        let mut pattern_nodes = HashSet::new();
        let mut to_process = vec![seed_id];
        let seed_state = seed_node.get_state();

        while let Some(current_id) = to_process.pop() {
            if !pattern_nodes.insert(current_id) {
                continue;
            }

            let current_node = &self.nodes[&current_id];

            // Find connected resonant nodes
            for &other_id in resonant_nodes {
                if pattern_nodes.contains(&other_id) {
                    continue;
                }

                let other_node = &self.nodes[&other_id];
                if self.are_nodes_synchronized(current_node, other_node)? {
                    to_process.push(other_id);
                }
            }
        }

        if pattern_nodes.len() < 2 {
            return Ok(None);
        }

        Ok(Some(ResonancePattern {
            frequency: seed_state.frequency,
            amplitude: seed_state.amplitude.norm(),
                phase: seed_state.phase,
                nodes: pattern_nodes,
                stability: self.calculate_pattern_stability(&pattern_nodes)?,
                energy: self.calculate_pattern_energy(&pattern_nodes)?,
        }))
    }

    /// Check if nodes are synchronized
    fn are_nodes_synchronized(&self, node1: &LatticeNode, node2: &LatticeNode) -> Result<bool, ResonanceError> {
        if !node1.is_connected(node2) {
            return Ok(false);
        }

        let state1 = node1.get_state();
        let state2 = node2.get_state();

        let freq_diff = (state1.frequency - state2.frequency).abs();
        let phase_diff = (state1.phase - state2.phase).abs();
        let normalized_phase = phase_diff % (2.0 * std::f64::consts::PI);

        Ok(freq_diff <= self.config.sync_tolerance &&
        normalized_phase <= self.config.sync_tolerance)
    }

    /// Calculate pattern stability
    fn calculate_pattern_stability(&self, nodes: &HashSet<u64>) -> Result<f64, ResonanceError> {
        let stabilities: Vec<f64> = nodes.iter()
        .filter_map(|&id| self.nodes.get(&id))
        .map(|node| node.get_state().stability)
        .collect();

        if stabilities.is_empty() {
            return Ok(0.0);
        }

        let avg_stability = stabilities.iter().sum::<f64>() / stabilities.len() as f64;
        Ok(avg_stability)
    }

    /// Calculate pattern energy
    fn calculate_pattern_energy(&self, nodes: &HashSet<u64>) -> Result<f64, ResonanceError> {
        Ok(nodes.iter()
        .filter_map(|&id| self.nodes.get(&id))
        .map(|node| node.get_state().energy)
        .sum())
    }

    /// Update global resonance state
    fn update_state(&mut self) -> Result<(), ResonanceError> {
        let patterns = self.patterns.read();
        let mut state = self.state.write();

        // Update active patterns
        state.active_patterns = patterns.keys().cloned().collect();

        // Calculate total energy
        state.total_energy = patterns.values()
        .map(|p| p.energy)
        .sum();

        // Calculate global stability
        state.stability = self.calculate_global_stability(&patterns)?;

        // Calculate coherence
        state.coherence = self.calculate_global_coherence(&patterns)?;

        Ok(())
    }

    /// Calculate global stability
    fn calculate_global_stability(&self, patterns: &HashMap<PatternId, ResonancePattern>) -> Result<f64, ResonanceError> {
        if patterns.is_empty() {
            return Ok(1.0);
        }

        let stability = patterns.values()
        .map(|p| p.stability)
        .sum::<f64>() / patterns.len() as f64;

        Ok(stability)
    }

    /// Calculate global coherence
    fn calculate_global_coherence(&self, patterns: &HashMap<PatternId, ResonancePattern>) -> Result<f64, ResonanceError> {
        if patterns.is_empty() {
            return Ok(1.0);
        }

        let mut total_phase = Complex64::new(0.0, 0.0);
        let mut total_weight = 0.0;

        for pattern in patterns.values() {
            let weight = pattern.amplitude;
            total_phase += Complex64::from_polar(weight, pattern.phase);
            total_weight += weight;
        }

        if total_weight == 0.0 {
            return Ok(0.0);
        }

        Ok(total_phase.norm() / total_weight)
    }

    /// Cleanup patterns after node removal
    fn cleanup_patterns(&mut self, removed_node: u64) -> Result<(), ResonanceError> {
        let mut patterns = self.patterns.write();
        patterns.retain(|_, pattern| !pattern.nodes.contains(&removed_node));
        Ok(())
    }

    /// Quantize frequency for pattern identification
    fn quantize_frequency(&self, freq: f64) -> u64 {
        (freq * 100.0).round() as u64
    }

    /// Get current resonance state
    pub fn get_state(&self) -> ResonanceState {
        self.state.read().clone()
    }

    /// Check if lattice is in resonance
    pub fn is_resonant(&self) -> bool {
        let state = self.state.read();
        !state.active_patterns.is_empty() &&
        state.stability >= self.config.stability_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_resonance_initialization() {
        let config = ResonanceConfig::default();
        let resonance = LatticeResonance::new(config);
        let state = resonance.get_state();

        assert!(state.active_patterns.is_empty());
        assert_relative_eq!(state.total_energy, 0.0);
        assert_relative_eq!(state.stability, 1.0);
        assert_relative_eq!(state.coherence, 1.0);
    }

    #[test]
    fn test_node_management() -> Result<(), ResonanceError> {
        let config = ResonanceConfig::default();
        let mut resonance = LatticeResonance::new(config);

        let node = Arc::new(LatticeNode::new(Default::default(), [0.0, 0.0, 0.0]));
        let node_id = node.id();

        resonance.add_node(node)?;
        assert!(resonance.nodes.contains_key(&node_id));

        resonance.remove_node(node_id)?;
        assert!(!resonance.nodes.contains_key(&node_id));

        Ok(())
    }

    #[test]
    fn test_pattern_detection() -> Result<(), ResonanceError> {
        let config = ResonanceConfig::default();
        let mut resonance = LatticeResonance::new(config);

        // Create connected resonant nodes
        let node1 = Arc::new(LatticeNode::new(Default::default(), [0.0, 0.0, 0.0]));
        let node2 = Arc::new(LatticeNode::new(Default::default(), [1.0, 0.0, 0.0]));

        node1.connect(&node2)?;
        node1.apply_force(Complex64::new(1.0, 0.0))?;
        node2.apply_force(Complex64::new(1.0, 0.0))?;

        resonance.add_node(node1)?;
        resonance.add_node(node2)?;

        resonance.update(0.0)?;
        let state = resonance.get_state();

        assert!(!state.active_patterns.is_empty());
        Ok(())
    }

    #[test]
    fn test_synchronization() -> Result<(), ResonanceError> {
        let config = ResonanceConfig::default();
        let mut resonance = LatticeResonance::new(config);

        let node1 = Arc::new(LatticeNode::new(Default::default(), [0.0, 0.0, 0.0]));
        let node2 = Arc::new(LatticeNode::new(Default::default(), [1.0, 0.0, 0.0]));

        node1.connect(&node2)?;

        // Apply similar forces to create synchronization
        node1.apply_force(Complex64::new(1.0, 0.0))?;
        node2.apply_force(Complex64::new(1.0, 0.0))?;

        assert!(resonance.are_nodes_synchronized(&node1, &node2)?);
        Ok(())
    }
}
