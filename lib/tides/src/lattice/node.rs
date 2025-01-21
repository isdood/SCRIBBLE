//! Crystal lattice node representation and manipulation
//! Created: 2025-01-21 15:22:06 UTC
//! Author: @isdood

use std::{
    collections::HashSet,
    sync::atomic::{AtomicU64, Ordering},
};

use num_complex::Complex64;
use parking_lot::RwLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Invalid node configuration: {0}")]
    InvalidConfig(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("State transition error: {0}")]
    StateError(String),
    #[error("Energy balance error: {0}")]
    EnergyError(String),
}

/// Static node counter for unique IDs
static NODE_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Node configuration parameters
#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub resonance_threshold: f64,
    pub coupling_strength: f64,
    pub damping_factor: f64,
    pub phase_tolerance: f64,
    pub max_connections: usize,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            resonance_threshold: 0.001,
            coupling_strength: 0.5,
            damping_factor: 0.995,
            phase_tolerance: 0.1,
            max_connections: 6,
        }
    }
}

/// Crystal lattice node
pub struct LatticeNode {
    id: u64,
    config: NodeConfig,
    state: RwLock<NodeState>,
    connections: RwLock<HashSet<u64>>,
}

/// Node state information
#[derive(Debug, Clone)]
pub struct NodeState {
    pub position: [f64; 3],
    pub amplitude: Complex64,
    pub frequency: f64,
    pub phase: f64,
    pub energy: f64,
    pub stability: f64,
}

impl LatticeNode {
    /// Create new lattice node
    pub fn new(config: NodeConfig, position: [f64; 3]) -> Self {
        let id = NODE_COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            id,
            config,
            state: RwLock::new(NodeState {
                position,
                amplitude: Complex64::new(0.0, 0.0),
                               frequency: 0.0,
                               phase: 0.0,
                               energy: 0.0,
                               stability: 1.0,
            }),
            connections: RwLock::new(HashSet::new()),
        }
    }

    /// Get node ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Update node state
    pub fn update(&self, time: f64) -> Result<(), NodeError> {
        let mut state = self.state.write();

        // Apply damping
        state.amplitude *= self.config.damping_factor;

        // Update phase
        state.phase += 2.0 * std::f64::consts::PI * state.frequency * time;
        state.phase %= 2.0 * std::f64::consts::PI;

        // Calculate energy
        state.energy = state.amplitude.norm_sqr();

        // Update stability based on energy and connections
        state.stability = self.calculate_stability(&state)?;

        Ok(())
    }

    /// Connect to another node
    pub fn connect(&self, other: &Self) -> Result<(), NodeError> {
        let mut connections = self.connections.write();

        if connections.len() >= self.config.max_connections {
            return Err(NodeError::ConnectionError(
                "Maximum connections reached".into()
            ));
        }

        connections.insert(other.id());
        Ok(())
    }

    /// Disconnect from another node
    pub fn disconnect(&self, other: &Self) -> Result<(), NodeError> {
        let mut connections = self.connections.write();

        if !connections.remove(&other.id()) {
            return Err(NodeError::ConnectionError(
                "Node not connected".into()
            ));
        }

        Ok(())
    }

    /// Apply external force to node
    pub fn apply_force(&self, force: Complex64) -> Result<(), NodeError> {
        let mut state = self.state.write();
        state.amplitude += force;
        state.energy = state.amplitude.norm_sqr();

        Ok(())
    }

    /// Calculate coupling force with another node
    pub fn calculate_coupling(&self, other: &Self) -> Result<Complex64, NodeError> {
        if !self.is_connected(other) {
            return Err(NodeError::ConnectionError(
                "Nodes not connected".into()
            ));
        }

        let self_state = self.state.read();
        let other_state = other.state.read();

        let distance = self.calculate_distance(&self_state.position, &other_state.position);
        let phase_diff = (self_state.phase - other_state.phase + std::f64::consts::PI)
        % (2.0 * std::f64::consts::PI) - std::f64::consts::PI;

        let coupling = self.config.coupling_strength * (-distance).exp()
        * Complex64::from_polar(1.0, phase_diff);

        Ok(coupling)
    }

    /// Check if node is connected to another
    pub fn is_connected(&self, other: &Self) -> bool {
        self.connections.read().contains(&other.id())
    }

    /// Get number of connections
    pub fn connection_count(&self) -> usize {
        self.connections.read().len()
    }

    /// Get node state
    pub fn get_state(&self) -> NodeState {
        self.state.read().clone()
    }

    /// Check if node is in resonance
    pub fn is_resonant(&self) -> bool {
        let state = self.state.read();
        state.energy >= self.config.resonance_threshold
    }

    /// Get node position
    pub fn position(&self) -> [f64; 3] {
        self.state.read().position
    }

    /// Set node position
    pub fn set_position(&self, position: [f64; 3]) -> Result<(), NodeError> {
        let mut state = self.state.write();
        state.position = position;
        Ok(())
    }

    /// Calculate stability factor
    fn calculate_stability(&self, state: &NodeState) -> Result<f64, NodeError> {
        let connection_factor = self.connection_count() as f64 / self.config.max_connections as f64;
        let energy_factor = if state.energy > 0.0 {
            (-state.energy.ln()).exp()
        } else {
            0.0
        };

        let stability = connection_factor * energy_factor * self.config.damping_factor;
        Ok(stability.min(1.0))
    }

    /// Calculate distance between positions
    fn calculate_distance(&self, pos1: &[f64; 3], pos2: &[f64; 3]) -> f64 {
        let dx = pos1[0] - pos2[0];
        let dy = pos1[1] - pos2[1];
        let dz = pos1[2] - pos2[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl PartialEq for LatticeNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for LatticeNode {}

impl std::hash::Hash for LatticeNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_node_creation() {
        let config = NodeConfig::default();
        let position = [0.0, 0.0, 0.0];
        let node = LatticeNode::new(config, position);

        let state = node.get_state();
        assert_eq!(state.position, position);
        assert_relative_eq!(state.energy, 0.0);
        assert_relative_eq!(state.stability, 1.0);
    }

    #[test]
    fn test_node_connections() -> Result<(), NodeError> {
        let config = NodeConfig::default();
        let node1 = LatticeNode::new(config.clone(), [0.0, 0.0, 0.0]);
        let node2 = LatticeNode::new(config, [1.0, 0.0, 0.0]);

        node1.connect(&node2)?;
        assert!(node1.is_connected(&node2));
        assert_eq!(node1.connection_count(), 1);

        node1.disconnect(&node2)?;
        assert!(!node1.is_connected(&node2));
        assert_eq!(node1.connection_count(), 0);

        Ok(())
    }

    #[test]
    fn test_force_application() -> Result<(), NodeError> {
        let config = NodeConfig::default();
        let node = LatticeNode::new(config, [0.0, 0.0, 0.0]);

        let force = Complex64::new(1.0, 0.0);
        node.apply_force(force)?;

        let state = node.get_state();
        assert_relative_eq!(state.amplitude.re, 1.0);
        assert_relative_eq!(state.energy, 1.0);

        Ok(())
    }

    #[test]
    fn test_coupling_calculation() -> Result<(), NodeError> {
        let config = NodeConfig::default();
        let node1 = LatticeNode::new(config.clone(), [0.0, 0.0, 0.0]);
        let node2 = LatticeNode::new(config, [1.0, 0.0, 0.0]);

        node1.connect(&node2)?;
        let coupling = node1.calculate_coupling(&node2)?;

        assert!(coupling.norm() <= 1.0);
        Ok(())
    }

    #[test]
    fn test_stability_calculation() -> Result<(), NodeError> {
        let config = NodeConfig::default();
        let node = LatticeNode::new(config, [0.0, 0.0, 0.0]);

        // Test stability with no energy
        let initial_state = node.get_state();
        assert_relative_eq!(initial_state.stability, 1.0);

        // Apply force and update
        node.apply_force(Complex64::new(1.0, 0.0))?;
        node.update(0.1)?;

        let updated_state = node.get_state();
        assert!(updated_state.stability > 0.0);
        assert!(updated_state.stability <= 1.0);

        Ok(())
    }

    #[test]
    fn test_max_connections() -> Result<(), NodeError> {
        let mut config = NodeConfig::default();
        config.max_connections = 2;
        let main_node = LatticeNode::new(config.clone(), [0.0, 0.0, 0.0]);

        // Create and connect max number of nodes
        let node1 = LatticeNode::new(config.clone(), [1.0, 0.0, 0.0]);
        let node2 = LatticeNode::new(config.clone(), [0.0, 1.0, 0.0]);
        let node3 = LatticeNode::new(config, [0.0, 0.0, 1.0]);

        main_node.connect(&node1)?;
        main_node.connect(&node2)?;

        // Attempting to connect one more should fail
        assert!(main_node.connect(&node3).is_err());
        assert_eq!(main_node.connection_count(), 2);

        Ok(())
    }
}
