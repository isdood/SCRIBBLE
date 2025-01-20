//! Integration tests for quantum transport functionality
//!
//! Tests the quantum transport channels and state management in the
//! Mycelium crystal computing network.
//!
//! Author: isdood
//! Last Updated: 2025-01-20 02:09:26 UTC

use std::time::Duration;
use tokio::time::sleep;

use crate::transport::{
    TransportChannel,
    QuantumState,
    QuantumTransform,
    AnchorState,
    TunnelState,
    Complex,
};
use crate::error::{NetworkError, NetworkResult};
use crate::harmony::Harmonizable;

const TEST_TIMEOUT: Duration = Duration::from_secs(5);
const STABILIZATION_DELAY: Duration = Duration::from_millis(100);

#[tokio::test]
async fn test_quantum_state_creation() {
    let state = QuantumState::new();
    assert_eq!(state.stability(), 1.0);
    assert_eq!(state.reality_anchor(), 1.0);
}

#[tokio::test]
async fn test_quantum_transform() {
    let mut state = QuantumState::new();
    let transform = QuantumTransform::identity();

    state.apply_transform(&transform).unwrap();
    assert_eq!(state.stability(), 1.0);
}

#[tokio::test]
async fn test_quantum_bridge_creation() {
    let source = QuantumState::new();
    let target = QuantumState::new();

    let bridge = TransportChannel::create_quantum_bridge(source, target).unwrap();
    assert!(bridge.stability().await >= 0.87);
}

#[tokio::test]
async fn test_reality_anchor_channel() {
    let channel = TransportChannel::new_reality_anchor();
    let state = QuantumState::new();

    let transmitted = channel.transmit(state).await.unwrap();
    assert!(transmitted.reality_anchor() >= 0.93);
}

#[tokio::test]
async fn test_hyperspace_tunnel() {
    let channel = TransportChannel::new_hyperspace_tunnel();
    let state = QuantumState::new();

    let transmitted = channel.transmit(state).await.unwrap();
    assert!(transmitted.stability() > 0.0);
}

#[tokio::test]
async fn test_channel_capacity() {
    let channel = TransportChannel::new_hyperspace_tunnel();
    let state = QuantumState::new();

    // Fill to capacity
    for _ in 0..100 {
        channel.transmit(state.clone()).await.unwrap();
    }

    // Should fail on capacity exceeded
    let result = channel.transmit(state).await;
    assert!(matches!(result, Err(NetworkError::CapacityExceeded { .. })));
}

#[tokio::test]
async fn test_quantum_state_stability() {
    let mut state = QuantumState::new();
    let transform = QuantumTransform::new(
        vec![Complex::new(0.7071, 0.7071)],
                                          0.95,
    );

    state.apply_transform(&transform).unwrap();
    assert!(state.stability() >= 0.87);
}

#[tokio::test]
async fn test_concurrent_transmissions() {
    let channel = TransportChannel::new_quantum();
    let state = QuantumState::new();

    let mut handles = vec![];
    for _ in 0..10 {
        let channel = channel.clone();
        let state = state.clone();
        handles.push(tokio::spawn(async move {
            channel.transmit(state).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_channel_harmonization() {
    let mut channel = TransportChannel::new_quantum();
    assert!(channel.is_harmonized());
    channel.harmonize().unwrap();
    assert!(channel.harmony_level() >= 0.87);
}

#[tokio::test]
async fn test_quantum_state_coherence() {
    let channel = TransportChannel::new_quantum();
    let initial_state = QuantumState::new();

    // Perform multiple transmissions
    let mut current_state = initial_state;
    for _ in 0..5 {
        current_state = channel.transmit(current_state).await.unwrap();
        assert!(current_state.stability() >= 0.87);
    }
}

#[tokio::test]
async fn test_reality_anchor_strength() {
    let channel = TransportChannel::new_reality_anchor();
    let state = QuantumState::new();

    let transmitted = channel.transmit(state).await.unwrap();
    assert!(transmitted.reality_anchor() >= 0.93);
}

#[tokio::test]
async fn test_channel_stress() {
    let channel = TransportChannel::new_quantum();
    let state = QuantumState::new();

    // Perform rapid transmissions
    let mut handles = vec![];
    for _ in 0..100 {
        let channel = channel.clone();
        let state = state.clone();
        handles.push(tokio::spawn(async move {
            channel.transmit(state).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_channel_recovery() {
    let channel = TransportChannel::new_quantum();

    // Simulate channel disturbance
    channel.disturb_stability(0.5).await.unwrap();

    // Wait for recovery
    sleep(STABILIZATION_DELAY).await;

    assert!(channel.stability().await >= 0.87);
}

#[tokio::test]
async fn test_quantum_state_transformation() {
    let mut state = QuantumState::new();

    // Apply series of transforms
    let transforms = vec![
        QuantumTransform::new(vec![Complex::new(0.866, 0.5)], 0.95),
        QuantumTransform::new(vec![Complex::new(0.7071, 0.7071)], 0.96),
        QuantumTransform::new(vec![Complex::new(0.9659, 0.2588)], 0.97),
    ];

    for transform in transforms {
        state.apply_transform(&transform).unwrap();
        assert!(state.stability() >= 0.87);
    }
}

#[tokio::test]
async fn test_channel_metrics() {
    let channel = TransportChannel::new_quantum();
    let state = QuantumState::new();

    // Perform some transmissions
    for _ in 0..5 {
        channel.transmit(state.clone()).await.unwrap();
    }

    let metrics = channel.get_metrics().await;
    assert!(metrics.total_transmissions > 0);
    assert!(metrics.average_stability > 0.0);
    assert!(metrics.quantum_coherence > 0.0);
}

#[tokio::test]
async fn test_hyperspace_tunnel_characteristics() {
    let tunnel = TransportChannel::new_hyperspace_tunnel();
    let state = QuantumState::new();

    let transmitted = tunnel.transmit(state).await.unwrap();
    let tunnel_state = tunnel.get_tunnel_state().await.unwrap();

    assert!(tunnel_state.stability > 0.0);
    assert!(tunnel_state.coherence > 0.0);
    assert_eq!(tunnel_state.connections, 1);
}

#[tokio::test]
#[should_panic]
async fn test_critical_channel_failure() {
    let channel = TransportChannel::new_quantum();
    channel.simulate_critical_failure().await;
}

#[tokio::test]
async fn test_quantum_bridge_entanglement() {
    let source = QuantumState::new();
    let target = QuantumState::new();

    let bridge = TransportChannel::create_quantum_bridge(source.clone(), target.clone()).unwrap();

    // Test entanglement preservation
    let transmitted = bridge.transmit(source).await.unwrap();
    assert!(transmitted.is_entangled_with(&target));
}

#[tokio::test]
async fn test_reality_anchor_adjustment() {
    let channel = TransportChannel::new_reality_anchor();
    let initial_strength = channel.get_anchor_strength().await;

    channel.adjust_anchor_strength(0.95).await.unwrap();
    let new_strength = channel.get_anchor_strength().await;

    assert!(new_strength > initial_strength);
}

#[tokio::test]
async fn test_transport_statistics() {
    let channel = TransportChannel::new_quantum();
    let state = QuantumState::new();

    // Perform various operations
    for _ in 0..5 {
        channel.transmit(state.clone()).await.unwrap();
    }

    let stats = channel.get_statistics().await;
    assert_eq!(stats.successful_transmissions, 5);
    assert_eq!(stats.failed_transmissions, 0);
    assert!(stats.average_transmission_time > 0.0);
}

#[tokio::test]
async fn test_complex_quantum_operations() {
    let channel = TransportChannel::new_quantum();
    let mut state = QuantumState::new();

    // Create a complex quantum circuit
    let operations = vec![
        QuantumTransform::hadamard(),
        QuantumTransform::phase(std::f64::consts::PI / 4.0),
        QuantumTransform::cnot(),
    ];

    // Apply operations
    for op in operations {
        state.apply_transform(&op).unwrap();
        let transmitted = channel.transmit(state.clone()).await.unwrap();
        state = transmitted;
    }

    assert!(state.stability() >= 0.87);
}

#[tokio::test]
async fn test_transport_error_handling() {
    let channel = TransportChannel::new_quantum();

    // Test various error conditions
    let results = vec![
        channel.transmit(QuantumState::new_unstable()).await,
        channel.transmit(QuantumState::new_decoherent()).await,
        channel.transmit(QuantumState::new_unanchored()).await,
    ];

    for result in results {
        assert!(result.is_err());
    }
}
