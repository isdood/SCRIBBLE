//! Crystal-based network management and coordination
//!
//! This module provides the core networking functionality for crystal-based
//! distributed computing systems, including node discovery, quantum state
//! synchronization, and harmony maintenance.
//!
//! Author: isdood
//! Last Updated: 2025-01-20 01:59:20 UTC

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, watch};
use uuid::Uuid;

use crate::error::{NetworkError, NetworkResult, ConnectionId};
use crate::node::{CrystalNode, NodeId, NodeConfig};
use crate::transport::{TransportChannel, QuantumState};
use crate::topology::{TopologyType, NetworkTopology};
use crate::harmony::{HarmonyMonitor, StabilityMetrics};

/// Version of the network protocol
const PROTOCOL_VERSION: &str = "0.1.0";
/// Default quantum stability threshold
const DEFAULT_QUANTUM_STABILITY: f64 = 0.87;
/// Default reality anchor strength
const DEFAULT_REALITY_ANCHOR: f64 = 0.93;
/// Maximum number of nodes in a single crystal network
const MAX_NODES: usize = 1024;

/// Events that can occur in the crystal network
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// New node joined the network
    NodeJoined {
        node_id: NodeId,
        quantum_state: QuantumState,
    },
    /// Node left the network
    NodeLeft(NodeId),
    /// Quantum state changed
    QuantumStateChanged {
        node_id: NodeId,
        old_state: QuantumState,
        new_state: QuantumState,
    },
    /// Harmony level changed
    HarmonyChanged {
        network_harmony: f64,
        affected_nodes: Vec<NodeId>,
    },
    /// Reality anchor adjusted
    RealityAnchorAdjusted {
        strength: f64,
        stabilized: bool,
    },
    /// Topology changed
    TopologyChanged {
        old_type: TopologyType,
        new_type: TopologyType,
    },
}

/// Configuration for the crystal network
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Preferred network topology
    pub topology: TopologyType,
    /// Minimum harmony level required
    pub min_harmony: f64,
    /// Reality anchor strength
    pub reality_anchor: f64,
    /// Maximum nodes allowed
    pub max_nodes: usize,
    /// Auto-stabilization enabled
    pub auto_stabilize: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            topology: TopologyType::Mesh,
            min_harmony: DEFAULT_QUANTUM_STABILITY,
            reality_anchor: DEFAULT_REALITY_ANCHOR,
            max_nodes: MAX_NODES,
            auto_stabilize: true,
        }
    }
}

/// Manages a crystal-based network of nodes
pub struct CrystalNetwork {
    /// Unique identifier for this network
    id: Uuid,
    /// Network configuration
    config: NetworkConfig,
    /// Connected nodes
    nodes: Arc<RwLock<HashMap<NodeId, CrystalNode>>>,
    /// Network topology manager
    topology: Arc<RwLock<NetworkTopology>>,
    /// Harmony monitor
    harmony: Arc<RwLock<HarmonyMonitor>>,
    /// Active connections
    connections: Arc<RwLock<HashMap<ConnectionId, TransportChannel>>>,
    /// Event sender
    event_tx: mpsc::Sender<NetworkEvent>,
    /// Event receiver
    event_rx: mpsc::Receiver<NetworkEvent>,
    /// Network state broadcast channel
    state_tx: watch::Sender<StabilityMetrics>,
    /// Network shutdown signal
    shutdown: watch::Sender<bool>,
}

impl CrystalNetwork {
    /// Creates a new crystal network with the specified configuration
    pub async fn new(config: NetworkConfig) -> NetworkResult<Self> {
        let (event_tx, event_rx) = mpsc::channel(100);
        let (state_tx, _) = watch::channel(StabilityMetrics {
            harmony_level: 1.0,
            reality_anchor_strength: config.reality_anchor,
            quantum_stability: 1.0,
            stable_connections: 0,
        });
        let (shutdown, _) = watch::channel(false);

        Ok(Self {
            id: Uuid::new_v4(),
           config,
           nodes: Arc::new(RwLock::new(HashMap::new())),
           topology: Arc::new(RwLock::new(NetworkTopology::new())),
           harmony: Arc::new(RwLock::new(HarmonyMonitor::new(DEFAULT_QUANTUM_STABILITY))),
           connections: Arc::new(RwLock::new(HashMap::new())),
           event_tx,
           event_rx,
           state_tx,
           shutdown,
        })
    }

    /// Adds a new node to the network
    pub async fn add_node(&self, config: NodeConfig) -> NetworkResult<NodeId> {
        let nodes = self.nodes.read().await;
        if nodes.len() >= self.config.max_nodes {
            return Err(NetworkError::CapacityExceeded {
                current: nodes.len(),
                       maximum: self.config.max_nodes,
            });
        }
        drop(nodes);

        let node = CrystalNode::new(config)?;
        let node_id = node.id();

        let mut nodes = self.nodes.write().await;
        nodes.insert(node_id, node);

        self.event_tx.send(NetworkEvent::NodeJoined {
            node_id,
            quantum_state: QuantumState::new(),
        }).await.map_err(|_| NetworkError::ConfigurationError("Event channel closed".into()))?;

        Ok(node_id)
    }

    /// Removes a node from the network
    pub async fn remove_node(&self, node_id: NodeId) -> NetworkResult<()> {
        let mut nodes = self.nodes.write().await;
        if nodes.remove(&node_id).is_some() {
            self.event_tx.send(NetworkEvent::NodeLeft(node_id))
            .await
            .map_err(|_| NetworkError::ConfigurationError("Event channel closed".into()))?;
        }
        Ok(())
    }

    /// Establishes a quantum bridge between two nodes
    pub async fn create_bridge(
        &self,
        source: NodeId,
        target: NodeId,
    ) -> NetworkResult<ConnectionId> {
        let nodes = self.nodes.read().await;
        let source_node = nodes.get(&source)
        .ok_or_else(|| NetworkError::ConfigurationError("Source node not found".into()))?;
        let target_node = nodes.get(&target)
        .ok_or_else(|| NetworkError::ConfigurationError("Target node not found".into()))?;

        let connection_id = ConnectionId::new();
        let channel = TransportChannel::create_quantum_bridge(
            source_node.quantum_state(),
                                                              target_node.quantum_state(),
        )?;

        let mut connections = self.connections.write().await;
        connections.insert(connection_id, channel);

        Ok(connection_id)
    }

    /// Updates the network topology
    pub async fn update_topology(&self, new_type: TopologyType) -> NetworkResult<()> {
        let mut topology = self.topology.write().await;
        let old_type = topology.topology_type();
        topology.change_topology(new_type)?;

        self.event_tx.send(NetworkEvent::TopologyChanged {
            old_type,
            new_type,
        }).await.map_err(|_| NetworkError::ConfigurationError("Event channel closed".into()))?;

        Ok(())
    }

    /// Stabilizes the network's harmony state
    pub async fn stabilize(&self) -> NetworkResult<()> {
        let mut harmony = self.harmony.write().await;
        harmony.stabilize()?;

        let metrics = harmony.get_metrics();
        self.state_tx.send(metrics)
        .map_err(|_| NetworkError::ConfigurationError("State channel closed".into()))?;

        Ok(())
    }

    /// Returns the current stability metrics
    pub async fn get_metrics(&self) -> NetworkResult<StabilityMetrics> {
        let harmony = self.harmony.read().await;
        Ok(harmony.get_metrics())
    }

    /// Starts listening for network events
    pub async fn listen<F>(&mut self, mut callback: F) -> NetworkResult<()>
    where
    F: FnMut(NetworkEvent) -> NetworkResult<()> + Send + 'static,
    {
        while let Some(event) = self.event_rx.recv().await {
            callback(event)?;
        }
        Ok(())
    }

    /// Shuts down the network
    pub async fn shutdown(&self) -> NetworkResult<()> {
        self.shutdown.send(true)
        .map_err(|_| NetworkError::ConfigurationError("Shutdown channel closed".into()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_creation() {
        let config = NetworkConfig::default();
        let network = CrystalNetwork::new(config).await.unwrap();
        let metrics = network.get_metrics().await.unwrap();
        assert!(metrics.harmony_level >= DEFAULT_QUANTUM_STABILITY);
    }

    #[tokio::test]
    async fn test_node_addition() {
        let config = NetworkConfig::default();
        let network = CrystalNetwork::new(config).await.unwrap();

        let node_config = NodeConfig {
            harmony_threshold: DEFAULT_QUANTUM_STABILITY,
            reality_anchor: DEFAULT_REALITY_ANCHOR,
            ..Default::default()
        };

        let node_id = network.add_node(node_config).await.unwrap();
        let nodes = network.nodes.read().await;
        assert!(nodes.contains_key(&node_id));
    }

    #[tokio::test]
    async fn test_network_capacity() {
        let config = NetworkConfig {
            max_nodes: 1,
            ..Default::default()
        };
        let network = CrystalNetwork::new(config).await.unwrap();

        let node_config = NodeConfig::default();
        network.add_node(node_config.clone()).await.unwrap();

        let result = network.add_node(node_config).await;
        assert!(matches!(result, Err(NetworkError::CapacityExceeded { .. })));
    }

    #[tokio::test]
    async fn test_topology_change() {
        let config = NetworkConfig::default();
        let network = CrystalNetwork::new(config).await.unwrap();

        let result = network.update_topology(TopologyType::Ring).await;
        assert!(result.is_ok());

        let topology = network.topology.read().await;
        assert_eq!(topology.topology_type(), TopologyType::Ring);
    }
}
