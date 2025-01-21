//! Crystal lattice strand formation and manipulation
//! Created: 2025-01-21 15:30:33 UTC
//! Author: @isdood

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use super::node::{LatticeNode, NodeError};
use crate::core::wave_pattern::WavePattern;

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StrandError {
    #[error("Invalid strand configuration: {0}")]
    InvalidConfig(String),
    #[error("Strand formation error: {0}")]
    FormationError(String),
    #[error("Node error: {0}")]
    NodeError(#[from] NodeError),
    #[error("Pattern alignment error: {0}")]
    AlignmentError(String),
}

/// Configuration for crystal strands
#[derive(Debug, Clone)]
pub struct StrandConfig {
    pub min_length: usize,
    pub max_length: usize,
    pub coupling_strength: f64,
    pub alignment_threshold: f64,
    pub stability_threshold: f64,
    pub memory_length: usize,
}

impl Default for StrandConfig {
    fn default() -> Self {
        Self {
            min_length: 3,
            max_length: 12,
            coupling_strength: 0.5,
            alignment_threshold: 0.95,
            stability_threshold: 0.001,
            memory_length: 128,
        }
    }
}

/// Crystal lattice strand manager
pub struct LatticeStrand {
    config: StrandConfig,
    strands: RwLock<HashMap<StrandId, Strand>>,
    history: RwLock<VecDeque<StrandState>>,
    wave_pattern: Arc<WavePattern>,
}

/// Unique strand identifier
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct StrandId(u64);

/// Crystal strand structure
#[derive(Debug, Clone)]
struct Strand {
    id: StrandId,
    nodes: Vec<Arc<LatticeNode>>,
    phase: f64,
    amplitude: f64,
    stability: f64,
    energy: f64,
    alignment: f64,
}

/// Current state of all strands
#[derive(Debug, Clone)]
pub struct StrandState {
    pub active_strands: usize,
    pub total_energy: f64,
    pub mean_stability: f64,
    pub mean_alignment: f64,
}

impl LatticeStrand {
    /// Create new lattice strand manager
    pub fn new(config: StrandConfig, wave_pattern: Arc<WavePattern>) -> Self {
        Self {
            config,
            strands: RwLock::new(HashMap::new()),
            history: RwLock::new(VecDeque::with_capacity(config.memory_length)),
            wave_pattern,
        }
    }

    /// Update strand patterns
    pub fn update(&self, time: f64) -> Result<(), StrandError> {
        // Update existing strands
        self.update_strands(time)?;

        // Clean up unstable strands
        self.cleanup_strands()?;

        // Update state history
        self.update_history()?;

        Ok(())
    }

    /// Form new strand from nodes
    pub fn form_strand(&self, nodes: Vec<Arc<LatticeNode>>) -> Result<StrandId, StrandError> {
        if nodes.len() < self.config.min_length || nodes.len() > self.config.max_length {
            return Err(StrandError::InvalidConfig(
                format!("Invalid strand length: {}", nodes.len())
            ));
        }

        // Verify node connectivity
        self.verify_connectivity(&nodes)?;

        // Calculate strand properties
        let (phase, amplitude) = self.calculate_strand_properties(&nodes)?;
        let stability = self.calculate_strand_stability(&nodes)?;
        let energy = self.calculate_strand_energy(&nodes)?;
        let alignment = self.calculate_strand_alignment(&nodes)?;

        let strand = Strand {
            id: StrandId(self.generate_strand_id()),
            nodes,
            phase,
            amplitude,
            stability,
            energy,
            alignment,
        };

        // Store new strand
        self.strands.write().insert(strand.id.clone(), strand);

        Ok(strand.id)
    }

    /// Update existing strands
    fn update_strands(&self, time: f64) -> Result<(), StrandError> {
        let mut strands = self.strands.write();

        let updates: Vec<(StrandId, Strand)> = strands.par_iter()
        .filter_map(|(id, strand)| {
            self.update_strand(strand, time)
            .ok()
            .map(|updated| (id.clone(), updated))
        })
        .collect();

        // Apply updates
        for (id, updated_strand) in updates {
            strands.insert(id, updated_strand);
        }

        Ok(())
    }

    /// Update single strand
    fn update_strand(&self, strand: &Strand, time: f64) -> Result<Strand, StrandError> {
        // Update nodes
        for node in &strand.nodes {
            node.update(time)?;
        }

        // Recalculate strand properties
        let (phase, amplitude) = self.calculate_strand_properties(&strand.nodes)?;
        let stability = self.calculate_strand_stability(&strand.nodes)?;
        let energy = self.calculate_strand_energy(&strand.nodes)?;
        let alignment = self.calculate_strand_alignment(&strand.nodes)?;

        Ok(Strand {
            id: strand.id.clone(),
           nodes: strand.nodes.clone(),
           phase,
           amplitude,
           stability,
           energy,
           alignment,
        })
    }

    /// Verify node connectivity in strand
    fn verify_connectivity(&self, nodes: &[Arc<LatticeNode>]) -> Result<(), StrandError> {
        for window in nodes.windows(2) {
            if !window[0].is_connected(&window[1]) {
                return Err(StrandError::FormationError(
                    "Nodes are not properly connected".into()
                ));
            }
        }
        Ok(())
    }

    /// Calculate strand properties
    fn calculate_strand_properties(
        &self,
        nodes: &[Arc<LatticeNode>],
    ) -> Result<(f64, f64), StrandError> {
        let mut phase_sum = Complex64::new(0.0, 0.0);
        let mut amplitude_sum = 0.0;

        for node in nodes {
            let state = node.get_state();
            phase_sum += Complex64::from_polar(1.0, state.phase);
            amplitude_sum += state.amplitude.norm();
        }

        let mean_phase = phase_sum.arg();
        let mean_amplitude = amplitude_sum / nodes.len() as f64;

        Ok((mean_phase, mean_amplitude))
    }

    /// Calculate strand stability
    fn calculate_strand_stability(&self, nodes: &[Arc<LatticeNode>]) -> Result<f64, StrandError> {
        let stabilities: Vec<f64> = nodes.iter()
        .map(|node| node.get_state().stability)
        .collect();

        let mean_stability = stabilities.iter().sum::<f64>() / stabilities.len() as f64;
        Ok(mean_stability)
    }

    /// Calculate strand energy
    fn calculate_strand_energy(&self, nodes: &[Arc<LatticeNode>]) -> Result<f64, StrandError> {
        let total_energy: f64 = nodes.iter()
        .map(|node| node.get_state().energy)
        .sum();

        Ok(total_energy)
    }

    /// Calculate strand alignment
    fn calculate_strand_alignment(&self, nodes: &[Arc<LatticeNode>]) -> Result<f64, StrandError> {
        let mut phase_vector = Complex64::new(0.0, 0.0);

        for node in nodes {
            let state = node.get_state();
            phase_vector += Complex64::from_polar(1.0, state.phase);
        }

        let alignment = phase_vector.norm() / nodes.len() as f64;
        Ok(alignment)
    }

    /// Cleanup unstable strands
    fn cleanup_strands(&self) -> Result<(), StrandError> {
        let mut strands = self.strands.write();
        strands.retain(|_, strand|
        strand.stability >= self.config.stability_threshold &&
        strand.alignment >= self.config.alignment_threshold
        );
        Ok(())
    }

    /// Update state history
    fn update_history(&self) -> Result<(), StrandError> {
        let mut history = self.history.write();
        let current_state = self.calculate_state()?;

        if history.len() >= self.config.memory_length {
            history.pop_front();
        }
        history.push_back(current_state);

        Ok(())
    }

    /// Calculate current strand state
    fn calculate_state(&self) -> Result<StrandState, StrandError> {
        let strands = self.strands.read();

        let active_strands = strands.len();
        let total_energy: f64 = strands.values().map(|s| s.energy).sum();

        let mean_stability = if active_strands > 0 {
            strands.values().map(|s| s.stability).sum::<f64>() / active_strands as f64
        } else {
            1.0
        };

        let mean_alignment = if active_strands > 0 {
            strands.values().map(|s| s.alignment).sum::<f64>() / active_strands as f64
        } else {
            1.0
        };

        Ok(StrandState {
            active_strands,
            total_energy,
            mean_stability,
            mean_alignment,
        })
    }

    /// Generate unique strand ID
    fn generate_strand_id(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
    }

    /// Get current strand state
    pub fn get_state(&self) -> Result<StrandState, StrandError> {
        self.calculate_state()
    }

    /// Check if strand formation is stable
    pub fn is_stable(&self) -> bool {
        self.strands.read().values()
        .all(|strand| strand.stability >= self.config.stability_threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_strand_formation() -> Result<(), StrandError> {
        let config = StrandConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let strand_manager = LatticeStrand::new(config.clone(), wave_pattern);

        // Create test nodes
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0])))
        .collect();

        // Connect nodes
        nodes[0].connect(&nodes[1])?;
        nodes[1].connect(&nodes[2])?;

        // Form strand
        let strand_id = strand_manager.form_strand(nodes)?;
        let state = strand_manager.get_state()?;

        assert_eq!(state.active_strands, 1);
        Ok(())
    }

    #[test]
    fn test_strand_stability() -> Result<(), StrandError> {
        let config = StrandConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let strand_manager = LatticeStrand::new(config.clone(), wave_pattern);

        // Create and connect nodes
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0])))
        .collect();

        for window in nodes.windows(2) {
            window[0].connect(&window[1])?;
        }

        // Apply forces to create stability
        for node in &nodes {
            node.apply_force(Complex64::new(1.0, 0.0))?;
        }

        strand_manager.form_strand(nodes)?;
        strand_manager.update(0.1)?;

        assert!(strand_manager.is_stable());
        Ok(())
    }

    #[test]
    fn test_strand_cleanup() -> Result<(), StrandError> {
        let mut config = StrandConfig::default();
        config.stability_threshold = 0.9;
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let strand_manager = LatticeStrand::new(config.clone(), wave_pattern);

        // Create unstable strand
        let nodes: Vec<Arc<LatticeNode>> = (0..3)
        .map(|i| Arc::new(LatticeNode::new(Default::default(), [i as f64, 0.0, 0.0])))
        .collect();

        for window in nodes.windows(2) {
            window[0].connect(&window[1])?;
        }

        strand_manager.form_strand(nodes)?;
        strand_manager.update(0.1)?;

        let state = strand_manager.get_state()?;
        assert_eq!(state.active_strands, 0); // Strand should be cleaned up due to low stability
        Ok(())
    }
}
