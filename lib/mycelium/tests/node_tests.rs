//! Integration tests for crystal node functionality
//!
//! Tests the behavior and interactions of crystal nodes in the
//! Mycelium quantum computing network.
//!
//! Author: isdood
//! Last Updated: 2025-01-20 02:06:16 UTC

use std::time::Duration;
use tokio::time::sleep;

use crate::node::{CrystalNode, NodeConfig, NodeId, NodeState, NodeCapabilities};
use crate::transport::{TransportChannel, QuantumState};
use crate::topology::TopologyType;
use crate::error::NetworkError;
use crate::harmony::Harmonizable;

const TEST_TIMEOUT: Duration = Duration::from_secs(5);
const STABILIZATION_DELAY: Duration = Duration::from_millis(100);

/// Creates a default test configuration for nodes
fn create_test_config() -> NodeConfig {
    NodeConfig {
        harmony_threshold: 0.87,
        reality_anchor: 0.93,
        topology_preference: TopologyType::Mesh,
        max_connections: 16,
        auto_stabilize: true,
        capabilities: NodeCapabilities {
            quantum_bridging: true,
            reality_anchoring: true,
            harmony_stabilization: true,
            max_quantum_channels: 8,
            supported_topologies: vec![
                TopologyType::Mesh,
                TopologyType::Ring,
                TopologyType::Star,
            ],
        },
    }
}

#[tokio::test]
async fn test_node_creation() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    assert_eq!(node.quantum_state().stability(), 1.0);
    assert!(matches!(node.get_stats().await.harmony_level, 1.0));
}

#[tokio::test]
async fn test_quantum_state_update() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    let new_state = QuantumState::new();
    node.update_quantum_state(new_state.clone()).await.unwrap();

    assert_eq!(node.quantum_state(), new_state);
}

#[tokio::test]
async fn test_node_connections() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();
    let target_id = NodeId::new();

    // Test connection establishment
    node.connect(target_id, TransportChannel::new_quantum())
    .await
    .unwrap();

    let stats = node.get_stats().await;
    assert_eq!(stats.active_connections, 1);
}

#[tokio::test]
async fn test_connection_capacity() {
    let config = NodeConfig {
        max_connections: 1,
        ..create_test_config()
    };
    let node = CrystalNode::new(config).unwrap();

    // Add first connection
    node.connect(NodeId::new(), TransportChannel::new_quantum())
    .await
    .unwrap();

    // Try to exceed capacity
    let result = node.connect(NodeId::new(), TransportChannel::new_quantum()).await;
    assert!(matches!(result, Err(NetworkError::CapacityExceeded { .. })));
}

#[tokio::test]
async fn test_node_stabilization() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    // Artificially destabilize the node
    node.update_quantum_state(QuantumState::new()).await.unwrap();

    // Attempt stabilization
    node.stabilize().await.unwrap();

    let stats = node.get_stats().await;
    assert!(stats.harmony_level >= 0.87);
}

#[tokio::test]
async fn test_reality_anchor_adjustment() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    let initial_stats = node.get_stats().await;
    let new_anchor = 0.95;

    node.adjust_reality_anchor(new_anchor).await.unwrap();

    let updated_stats = node.get_stats().await;
    assert!(updated_stats.reality_anchor > initial_stats.reality_anchor);
}

#[tokio::test]
async fn test_node_state_transitions() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    assert!(matches!(node.current_state().await, NodeState::Initializing));

    node.transition_to(NodeState::Operating).await.unwrap();
    assert!(matches!(node.current_state().await, NodeState::Operating));
}

#[tokio::test]
async fn test_quantum_channel_creation() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    let channel = node.create_quantum_channel().await.unwrap();
    assert!(channel.stability().await >= 0.87);
}

#[tokio::test]
async fn test_node_harmonization() {
    let config = create_test_config();
    let mut node = CrystalNode::new(config).unwrap();

    assert!(node.is_harmonized());
    node.harmonize().unwrap();
    assert!(node.harmony_level() >= 0.87);
}

#[tokio::test]
async fn test_node_shutdown() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    node.shutdown().await.unwrap();
    assert!(matches!(node.current_state().await, NodeState::ShuttingDown));
}

#[tokio::test]
async fn test_concurrent_state_updates() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    let mut handles = vec![];
    for _ in 0..5 {
        let node_clone = node.clone();
        handles.push(tokio::spawn(async move {
            node_clone.update_quantum_state(QuantumState::new()).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert!(node.quantum_state().stability() > 0.0);
}

#[tokio::test]
async fn test_node_stress() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    // Perform multiple operations concurrently
    let mut handles = vec![];
    for i in 0..100 {
        let node_clone = node.clone();
        handles.push(tokio::spawn(async move {
            match i % 3 {
                0 => node_clone.update_quantum_state(QuantumState::new()).await.unwrap(),
                                  1 => node_clone.connect(NodeId::new(), TransportChannel::new_quantum()).await.unwrap(),
                                  _ => node_clone.stabilize().await.unwrap(),
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let stats = node.get_stats().await;
    assert!(stats.harmony_level >= 0.87);
}

#[tokio::test]
async fn test_node_recovery() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    // Simulate destabilization
    node.disturb_harmony(0.5).await.unwrap();

    // Wait for auto-stabilization
    sleep(STABILIZATION_DELAY).await;

    let stats = node.get_stats().await;
    assert!(stats.harmony_level >= 0.87);
}

#[tokio::test]
async fn test_quantum_state_coherence() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    // Create multiple quantum states
    let states: Vec<QuantumState> = (0..5)
    .map(|_| QuantumState::new())
    .collect();

    // Apply states sequentially
    for state in states {
        node.update_quantum_state(state).await.unwrap();
        assert!(node.quantum_state().stability() >= 0.87);
    }
}

#[tokio::test]
async fn test_node_capabilities() {
    let mut config = create_test_config();
    config.capabilities.quantum_bridging = false;

    let node = CrystalNode::new(config).unwrap();
    let result = node.create_quantum_bridge(NodeId::new()).await;

    assert!(matches!(
        result,
        Err(NetworkError::ConfigurationError(_))
    ));
}

#[tokio::test]
#[should_panic]
async fn test_critical_failure_handling() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    node.simulate_critical_failure().await;
}

#[tokio::test]
async fn test_node_statistics() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    // Perform various operations
    node.update_quantum_state(QuantumState::new()).await.unwrap();
    node.connect(NodeId::new(), TransportChannel::new_quantum())
    .await
    .unwrap();

    let stats = node.get_stats().await;
    assert_eq!(stats.state_transitions, 1);
    assert_eq!(stats.active_connections, 1);
    assert!(stats.uptime > 0);
}

#[tokio::test]
async fn test_node_topology_support() {
    let config = create_test_config();
    let node = CrystalNode::new(config).unwrap();

    assert!(node.supports_topology(TopologyType::Mesh));
    assert!(node.supports_topology(TopologyType::Ring));
    assert!(node.supports_topology(TopologyType::Star));
    assert!(!node.supports_topology(TopologyType::Hypercube));
}
