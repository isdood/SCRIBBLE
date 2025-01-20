//! Integration tests for crystal network topology management
//!
//! Tests the topology management and node organization in the
//! Mycelium crystal computing network.
//!
//! Author: isdood
//! Last Updated: 2025-01-20 02:07:33 UTC

use std::time::Duration;
use tokio::time::sleep;

use crate::topology::{NetworkTopology, TopologyType};
use crate::node::NodeId;
use crate::error::{NetworkError, NetworkResult};
use crate::harmony::Harmonizable;

const TEST_TIMEOUT: Duration = Duration::from_secs(5);
const STABILIZATION_DELAY: Duration = Duration::from_millis(100);

/// Helper function to create a set of test nodes
async fn create_test_nodes(count: usize) -> Vec<NodeId> {
    (0..count).map(|_| NodeId::new()).collect()
}

#[tokio::test]
async fn test_topology_creation() {
    let topology = NetworkTopology::new();
    assert_eq!(topology.topology_type(), TopologyType::Mesh);
}

#[tokio::test]
async fn test_mesh_topology() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(4).await;

    // Add nodes to mesh topology
    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Verify full connectivity
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            assert!(topology.are_connected(nodes[i], nodes[j]).await);
        }
    }
}

#[tokio::test]
async fn test_ring_topology() {
    let mut topology = NetworkTopology::new();
    topology.change_topology(TopologyType::Ring).await.unwrap();

    let nodes = create_test_nodes(4).await;
    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Verify ring connections
    for i in 0..nodes.len() {
        let next = (i + 1) % nodes.len();
        assert!(topology.are_connected(nodes[i], nodes[next]).await);
    }
}

#[tokio::test]
async fn test_star_topology() {
    let mut topology = NetworkTopology::new();
    topology.change_topology(TopologyType::Star).await.unwrap();

    let nodes = create_test_nodes(5).await;
    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Verify star connections
    let center = nodes[0];
    for &node in nodes.iter().skip(1) {
        assert!(topology.are_connected(center, node).await);

        // Verify non-center nodes aren't connected to each other
        for &other in nodes.iter().skip(1) {
            if node != other {
                assert!(!topology.are_connected(node, other).await);
            }
        }
    }
}

#[tokio::test]
async fn test_crystal_lattice_topology() {
    let mut topology = NetworkTopology::new();
    topology.change_topology(TopologyType::CrystalLattice).await.unwrap();

    let nodes = create_test_nodes(8).await;
    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Verify crystal lattice properties
    for node in &nodes {
        let coordinates = topology.get_crystal_coordinates(*node).await.unwrap();
        assert_eq!(coordinates.len(), 3); // 3D coordinates
    }
}

#[tokio::test]
async fn test_topology_transition() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(4).await;

    // Add nodes in mesh topology
    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Transition to ring topology
    topology.change_topology(TopologyType::Ring).await.unwrap();

    // Verify ring properties
    for i in 0..nodes.len() {
        let next = (i + 1) % nodes.len();
        assert!(topology.are_connected(nodes[i], nodes[next]).await);
    }
}

#[tokio::test]
async fn test_dynamic_topology_adaptation() {
    let mut topology = NetworkTopology::new();
    topology.change_topology(TopologyType::DynamicAdaptive).await.unwrap();

    let nodes = create_test_nodes(6).await;
    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Simulate network load
    topology.simulate_load(nodes[0], 0.8).await.unwrap();

    // Verify topology adaptation
    sleep(STABILIZATION_DELAY).await;
    assert!(topology.is_balanced().await);
}

#[tokio::test]
async fn test_topology_harmony() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(4).await;

    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    assert!(topology.harmony_level() >= 0.87);
    topology.harmonize().unwrap();
    assert!(topology.is_harmonized());
}

#[tokio::test]
async fn test_topology_capacity() {
    let topology = NetworkTopology::new();
    let nodes = create_test_nodes(33).await; // More than MAX_NODES

    for (i, node) in nodes.iter().enumerate() {
        let result = topology.add_node(*node).await;
        if i >= 32 {
            assert!(matches!(result, Err(NetworkError::CapacityExceeded { .. })));
        } else {
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn test_node_removal() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(4).await;

    // Add nodes
    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Remove a node
    topology.remove_node(nodes[0]).await.unwrap();

    // Verify topology remains valid
    assert!(topology.verify_integrity().await.unwrap());
}

#[tokio::test]
async fn test_topology_recovery() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(4).await;

    // Add nodes
    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Simulate failure
    topology.simulate_node_failure(nodes[0]).await.unwrap();

    // Verify recovery
    sleep(STABILIZATION_DELAY).await;
    assert!(topology.verify_integrity().await.unwrap());
}

#[tokio::test]
async fn test_concurrent_topology_operations() {
    let topology = NetworkTopology::new();
    let nodes = create_test_nodes(10).await;

    // Perform concurrent operations
    let mut handles = vec![];
    for node in nodes {
        let topology = topology.clone();
        handles.push(tokio::spawn(async move {
            topology.add_node(node).await.unwrap();
            sleep(Duration::from_millis(10)).await;
            topology.remove_node(node).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert!(topology.verify_integrity().await.unwrap());
}

#[tokio::test]
async fn test_quantum_path_calculation() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(4).await;

    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    let paths = topology.calculate_quantum_paths().await.unwrap();
    assert!(!paths.is_empty());

    // Verify path validity
    for path in paths {
        assert!(topology.verify_path(&path).await.unwrap());
    }
}

#[tokio::test]
async fn test_topology_optimization() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(8).await;

    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    let initial_efficiency = topology.calculate_efficiency().await;
    topology.optimize().await.unwrap();
    let final_efficiency = topology.calculate_efficiency().await;

    assert!(final_efficiency >= initial_efficiency);
}

#[tokio::test]
#[should_panic]
async fn test_invalid_topology_transition() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(3).await;

    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    // Attempt invalid topology change
    topology.force_invalid_topology().await;
}

#[tokio::test]
async fn test_topology_stress() {
    let topology = NetworkTopology::new();
    let nodes = create_test_nodes(16).await;

    // Perform rapid topology changes
    let mut handles = vec![];
    for i in 0..100 {
        let topology = topology.clone();
        let nodes = nodes.clone();
        handles.push(tokio::spawn(async move {
            match i % 4 {
                0 => topology.add_node(nodes[i % 16]).await.unwrap(),
                                  1 => topology.remove_node(nodes[i % 16]).await.unwrap(),
                                  2 => topology.optimize().await.unwrap(),
                                  _ => topology.calculate_quantum_paths().await.unwrap(),
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert!(topology.verify_integrity().await.unwrap());
}

#[tokio::test]
async fn test_topology_metrics() {
    let mut topology = NetworkTopology::new();
    let nodes = create_test_nodes(4).await;

    for node in &nodes {
        topology.add_node(*node).await.unwrap();
    }

    let metrics = topology.get_metrics().await.unwrap();
    assert!(metrics.connectivity_ratio > 0.0);
    assert!(metrics.average_path_length > 0.0);
    assert!(metrics.clustering_coefficient >= 0.0);
}
