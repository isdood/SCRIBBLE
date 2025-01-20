//! Crystal node implementation for quantum-coherent networking
//!
//! This module provides the core node functionality for the Mycelium
//! crystal computing network, including quantum state management,
//! reality anchoring, and harmony maintenance.
//!
//! Author: isdood
//! Last Updated: 2025-01-20 02:00:45 UTC

use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::error::{NetworkError, NetworkResult};
use crate::transport::{TransportChannel, QuantumState};
use crate::harmony::{HarmonyMonitor, Harmonizable};
use crate::topology::TopologyType;

/// Unique identifier for crystal nodes
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct NodeId(Uuid);

impl NodeId {
    /// Creates a new random node ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Gets the underlying UUID
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Configuration for crystal nodes
#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Minimum harmony level required
    pub harmony_threshold: f64,
    /// Reality anchor strength
    pub reality_anchor: f64,
    /// Preferred topology type
    pub topology_preference: TopologyType,
    /// Maximum number of connections
    pub max_connections: usize,
    /// Auto-stabilization enabled
    pub auto_stabilize: bool,
    /// Node capabilities
    pub capabilities: NodeCapabilities,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            harmony_threshold: 0.87,
            reality_anchor: 0.93,
            topology_preference: TopologyType::Mesh,
            max_connections: 16,
            auto_stabilize: true,
            capabilities: NodeCapabilities::default(),
        }
    }
}

/// Node operational state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    /// Initial state after creation
    Initializing,
    /// Stabilizing quantum state
    Stabilizing,
    /// Normal operation
    Operating,
    /// Processing quantum state change
    Transitioning,
    /// Harmony level critical
    Destabilized,
    /// Node is shutting down
    ShuttingDown,
}

/// Node capabilities and features
#[derive(Debug, Clone)]
pub struct NodeCapabilities {
    /// Supports quantum bridging
    pub quantum_bridging: bool,
    /// Supports reality anchoring
    pub reality_anchoring: bool,
    /// Supports harmony stabilization
    pub harmony_stabilization: bool,
    /// Maximum quantum channels
    pub max_quantum_channels: usize,
    /// Supported topology types
    pub supported_topologies: Vec<TopologyType>,
}

impl Default for NodeCapabilities {
    fn default() -> Self {
        Self {
            quantum_bridging: true,
            reality_anchoring: true,
            harmony_stabilization: true,
            max_quantum_channels: 8,
            supported_topologies: vec![
                TopologyType::Mesh,
                TopologyType::Ring,
                TopologyType::Star,
            ],
        }
    }
}

/// Statistics for node operation
#[derive(Debug, Clone)]
pub struct NodeStats {
    /// Current harmony level
    pub harmony_level: f64,
    /// Current quantum stability
    pub quantum_stability: f64,
    /// Number of active connections
    pub active_connections: usize,
    /// Number of quantum state transitions
    pub state_transitions: u64,
    /// Total uptime in seconds
    pub uptime: u64,
    /// Current reality anchor strength
    pub reality_anchor: f64,
}

/// Core crystal node implementation
pub struct CrystalNode {
    /// Unique node identifier
    id: NodeId,
    /// Node configuration
    config: NodeConfig,
    /// Current node state
    state: RwLock<NodeState>,
    /// Current quantum state
    quantum_state: RwLock<QuantumState>,
    /// Active connections
    connections: RwLock<HashMap<NodeId, TransportChannel>>,
    /// Harmony monitor
    harmony_monitor: HarmonyMonitor,
    /// Operation statistics
    stats: RwLock<NodeStats>,
    /// State transition counter
    transition_counter: AtomicU64,
    /// Start timestamp
    start_time: std::time::Instant,
}

impl CrystalNode {
    /// Creates a new crystal node with the given configuration
    pub fn new(config: NodeConfig) -> NetworkResult<Self> {
        if config.harmony_threshold < 0.0 || config.harmony_threshold > 1.0 {
            return Err(NetworkError::ConfigurationError(
                "Harmony threshold must be between 0.0 and 1.0".into()
            ));
        }

        Ok(Self {
            id: NodeId::new(),
           config: config.clone(),
           state: RwLock::new(NodeState::Initializing),
           quantum_state: RwLock::new(QuantumState::new()),
           connections: RwLock::new(HashMap::new()),
           harmony_monitor: HarmonyMonitor::new(config.harmony_threshold),
           stats: RwLock::new(NodeStats {
               harmony_level: 1.0,
               quantum_stability: 1.0,
               active_connections: 0,
               state_transitions: 0,
               uptime: 0,
               reality_anchor: config.reality_anchor,
           }),
           transition_counter: AtomicU64::new(0),
           start_time: std::time::Instant::now(),
        })
    }

    /// Returns the node's unique identifier
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the current quantum state
    pub fn quantum_state(&self) -> QuantumState {
        self.quantum_state.blocking_read().clone()
    }

    /// Updates the node's quantum state
    pub async fn update_quantum_state(&self, new_state: QuantumState) -> NetworkResult<()> {
        let mut state = self.state.write().await;
        *state = NodeState::Transitioning;

        let mut quantum_state = self.quantum_state.write().await;
        *quantum_state = new_state;

        self.transition_counter.fetch_add(1, Ordering::SeqCst);
        self.update_stats().await?;

        *state = NodeState::Operating;
        Ok(())
    }

    /// Establishes a connection with another node
    pub async fn connect(&self, target: NodeId, channel: TransportChannel) -> NetworkResult<()> {
        let mut connections = self.connections.write().await;
        if connections.len() >= self.config.max_connections {
            return Err(NetworkError::CapacityExceeded {
                current: connections.len(),
                       maximum: self.config.max_connections,
            });
        }

        connections.insert(target, channel);
        self.update_stats().await?;
        Ok(())
    }

    /// Updates node statistics
    async fn update_stats(&self) -> NetworkResult<()> {
        let mut stats = self.stats.write().await;
        stats.harmony_level = self.harmony_monitor.get_metrics().harmony_level;
        stats.active_connections = self.connections.read().await.len();
        stats.state_transitions = self.transition_counter.load(Ordering::SeqCst);
        stats.uptime = self.start_time.elapsed().as_secs();
        Ok(())
    }

    /// Attempts to stabilize the node
    pub async fn stabilize(&self) -> NetworkResult<()> {
        let mut state = self.state.write().await;
        *state = NodeState::Stabilizing;

        self.harmony_monitor.stabilize()?;
        self.update_stats().await?;

        *state = NodeState::Operating;
        Ok(())
    }

    /// Returns current node statistics
    pub async fn get_stats(&self) -> NodeStats {
        self.stats.read().await.clone()
    }

    /// Initiates node shutdown
    pub async fn shutdown(&self) -> NetworkResult<()> {
        let mut state = self.state.write().await;
        *state = NodeState::ShuttingDown;

        let mut connections = self.connections.write().await;
        connections.clear();

        self.update_stats().await?;
        Ok(())
    }
}

impl Harmonizable for CrystalNode {
    fn harmonize(&mut self) -> NetworkResult<()> {
        if !self.config.capabilities.harmony_stabilization {
            return Err(NetworkError::ConfigurationError(
                "Node does not support harmony stabilization".into()
            ));
        }

        tokio::task::block_in_place(|| {
            self.harmony_monitor.stabilize()
        })
    }

    fn harmony_level(&self) -> f64 {
        self.harmony_monitor.get_metrics().harmony_level
    }

    fn is_harmonized(&self) -> bool {
        self.harmony_level() >= self.config.harmony_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_creation() {
        let config = NodeConfig::default();
        let node = CrystalNode::new(config).unwrap();
        assert_eq!(node.quantum_state().stability(), 1.0);
    }

    #[tokio::test]
    async fn test_quantum_state_update() {
        let config = NodeConfig::default();
        let node = CrystalNode::new(config).unwrap();

        let new_state = QuantumState::new();
        node.update_quantum_state(new_state.clone()).await.unwrap();

        assert_eq!(node.quantum_state(), new_state);
    }

    #[tokio::test]
    async fn test_connection_capacity() {
        let config = NodeConfig {
            max_connections: 1,
            ..Default::default()
        };
        let node = CrystalNode::new(config).unwrap();

        node.connect(NodeId::new(), TransportChannel::new_quantum())
        .await
        .unwrap();

        let result = node.connect(NodeId::new(), TransportChannel::new_quantum()).await;
        assert!(matches!(result, Err(NetworkError::CapacityExceeded { .. })));
    }

    #[tokio::test]
    async fn test_stats_update() {
        let config = NodeConfig::default();
        let node = CrystalNode::new(config).unwrap();

        let initial_stats = node.get_stats().await;
        assert_eq!(initial_stats.state_transitions, 0);

        node.update_quantum_state(QuantumState::new()).await.unwrap();
        let updated_stats = node.get_stats().await;
        assert_eq!(updated_stats.state_transitions, 1);
    }

    #[tokio::test]
    async fn test_harmonization() {
        let config = NodeConfig::default();
        let mut node = CrystalNode::new(config).unwrap();

        assert!(node.is_harmonized());
        node.harmonize().unwrap();
        assert!(node.harmony_level() >= node.config.harmony_threshold);
    }
}
