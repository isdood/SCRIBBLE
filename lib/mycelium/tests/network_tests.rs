//! Integration tests for crystal-based networking
//!
//! Tests the interaction between nodes, quantum transport, and topology
//! in the Mycelium crystal computing network.
//!
//! Author: isdood
//! Last Updated: 2025-01-20 02:05:02 UTC

use std::time::Duration;
use tokio::time::sleep;

use crate::network::{CrystalNetwork, NetworkConfig, NetworkEvent};
use crate::node::{CrystalNode, NodeConfig, NodeId};
use crate::transport::{TransportChannel, QuantumState};
use crate::topology::TopologyType;
use crate::error::NetworkError;
use crate::harmony::Harmonizable;

const TEST_TIMEOUT: Duration = Duration::from_secs(5);
const STABILIZATION_DELAY: Duration = Duration::from_millis(100);

/// Test helper to create a basic network configuration
fn create_test_config() -> NetworkConfig {
    NetworkConfig {
        topology: TopologyType::Mesh,
        min_harmony: 0.87,
        reality_anchor: 0.93,
        max_nodes: 16,
        auto_stabilize: true,
    }
}

/// Test helper to create a node configuration
fn create_test_node_config() -> NodeConfig {
    NodeConfig {
        harmony_threshold: 0.87,
        reality_anchor: 0.93,
        topology_preference: TopologyType::Mesh,
        max_connections: 8,
        auto_stabilize: true,
        capabilities: Default::default(),
    }
}

#[tokio::test]
async fn test_network_creation() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();
    let metrics = network.get_metrics().await.unwrap();
    assert!(metrics.harmony_level >= 0.87);
    assert!(metrics.reality_anchor_strength >= 0.93);
}

#[tokio::test]
async fn test_node_addition_and_removal() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Add node
    let node_config = create_test_node_config();
    let node_id = network.add_node(node_config).await.unwrap();

    // Verify node addition
    let metrics = network.get_metrics().await.unwrap();
    assert_eq!(metrics.stable_connections, 0);

    // Remove node
    network.remove_node(node_id).await.unwrap();

    // Verify node removal
    let metrics = network.get_metrics().await.unwrap();
    assert_eq!(metrics.stable_connections, 0);
}

#[tokio::test]
async fn test_quantum_bridge_creation() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Add two nodes
    let node1_id = network.add_node(create_test_node_config()).await.unwrap();
    let node2_id = network.add_node(create_test_node_config()).await.unwrap();

    // Create quantum bridge
    let bridge_id = network.create_bridge(node1_id, node2_id).await.unwrap();

    // Verify bridge creation
    let metrics = network.get_metrics().await.unwrap();
    assert_eq!(metrics.stable_connections, 1);
}

#[tokio::test]
async fn test_network_topology_change() {
    let config = create_test_config();
    let mut network = CrystalNetwork::new(config).await.unwrap();

    // Add multiple nodes
    for _ in 0..4 {
        network.add_node(create_test_node_config()).await.unwrap();
    }

    // Change topology
    network.update_topology(TopologyType::Ring).await.unwrap();
    sleep(STABILIZATION_DELAY).await;

    // Verify topology change
    let metrics = network.get_metrics().await.unwrap();
    assert!(metrics.harmony_level >= 0.87);
}

#[tokio::test]
async fn test_network_stability_under_load() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Add maximum nodes
    for _ in 0..16 {
        network.add_node(create_test_node_config()).await.unwrap();
    }

    // Verify network stability
    let metrics = network.get_metrics().await.unwrap();
    assert!(metrics.harmony_level >= 0.87);
    assert!(metrics.quantum_stability >= 0.87);
}

#[tokio::test]
async fn test_network_event_handling() {
    let config = create_test_config();
    let mut network = CrystalNetwork::new(config).await.unwrap();

    let mut event_count = 0;
    let node_id = network.add_node(create_test_node_config()).await.unwrap();

    // Start event listener
    let handle = tokio::spawn(async move {
        network.listen(|event| {
            match event {
                NetworkEvent::NodeJoined { .. } => event_count += 1,
                NetworkEvent::NodeLeft(_) => event_count += 1,
                       _ => {},
            }
            Ok(())
        }).await.unwrap();
    });

    // Wait for events
    sleep(STABILIZATION_DELAY).await;
    assert!(handle.await.is_ok());
}

#[tokio::test]
async fn test_network_capacity_limits() {
    let config = NetworkConfig {
        max_nodes: 2,
        ..create_test_config()
    };
    let network = CrystalNetwork::new(config).await.unwrap();

    // Add maximum nodes
    network.add_node(create_test_node_config()).await.unwrap();
    network.add_node(create_test_node_config()).await.unwrap();

    // Try to add one more node
    let result = network.add_node(create_test_node_config()).await;
    assert!(matches!(result, Err(NetworkError::CapacityExceeded { .. })));
}

#[tokio::test]
async fn test_quantum_state_propagation() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Add two nodes
    let node1_id = network.add_node(create_test_node_config()).await.unwrap();
    let node2_id = network.add_node(create_test_node_config()).await.unwrap();

    // Create quantum bridge
    let bridge_id = network.create_bridge(node1_id, node2_id).await.unwrap();

    // Test state propagation
    let state = QuantumState::new();
    let result = network.propagate_quantum_state(node1_id, state).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_network_harmonization() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Add nodes and create disturbance
    let node_id = network.add_node(create_test_node_config()).await.unwrap();
    network.disturb_harmony(node_id, 0.5).await.unwrap();

    // Test auto-stabilization
    sleep(STABILIZATION_DELAY).await;
    let metrics = network.get_metrics().await.unwrap();
    assert!(metrics.harmony_level >= 0.87);
}

#[tokio::test]
async fn test_reality_anchor_adjustment() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Add node and adjust reality anchor
    let node_id = network.add_node(create_test_node_config()).await.unwrap();
    network.adjust_reality_anchor(node_id, 0.95).await.unwrap();

    // Verify adjustment
    let metrics = network.get_metrics().await.unwrap();
    assert!(metrics.reality_anchor_strength >= 0.95);
}

#[tokio::test]
async fn test_network_shutdown() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Add some nodes
    for _ in 0..3 {
        network.add_node(create_test_node_config()).await.unwrap();
    }

    // Shutdown network
    let result = network.shutdown().await;
    assert!(result.is_ok());

    // Verify shutdown
    let metrics = network.get_metrics().await;
    assert!(metrics.is_err());
}

#[tokio::test]
async fn test_concurrent_operations() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Spawn multiple concurrent operations
    let mut handles = vec![];
    for _ in 0..5 {
        let network_clone = network.clone();
        handles.push(tokio::spawn(async move {
            let node_id = network_clone.add_node(create_test_node_config()).await.unwrap();
            network_clone.remove_node(node_id).await.unwrap();
        }));
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify network stability
    let metrics = network.get_metrics().await.unwrap();
    assert!(metrics.harmony_level >= 0.87);
}

#[tokio::test]
async fn test_network_recovery() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Create network disturbance
    for _ in 0..3 {
        let node_id = network.add_node(create_test_node_config()).await.unwrap();
        network.disturb_harmony(node_id, 0.5).await.unwrap();
    }

    // Wait for recovery
    sleep(Duration::from_secs(1)).await;

    // Verify recovery
    let metrics = network.get_metrics().await.unwrap();
    assert!(metrics.harmony_level >= 0.87);
    assert!(metrics.quantum_stability >= 0.87);
}

#[tokio::test]
#[should_panic]
async fn test_network_critical_failure() {
    let config = create_test_config();
    let network = CrystalNetwork::new(config).await.unwrap();

    // Trigger critical failure
    network.simulate_critical_failure().await;
}
