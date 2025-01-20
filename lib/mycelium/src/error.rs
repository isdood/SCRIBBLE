//! Error types and handling for the Mycelium networking system
//!
//! This module provides specialized error types for crystal-based networking
//! operations, harmony state management, and quantum transport issues.
//!
//! Author: isdood
//! Last Updated: 2025-01-20 01:56:11 UTC

use std::fmt;
use std::error::Error;
use std::result::Result;
use uuid::Uuid;

/// Type alias for Network operation results
pub type NetworkResult<T> = Result<T, NetworkError>;

/// Unique identifier for network connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(Uuid);

impl ConnectionId {
    /// Creates a new random connection ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Gets the underlying UUID
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Primary error type for network operations
#[derive(Debug)]
pub enum NetworkError {
    /// Loss of harmony state with given current harmony level
    HarmonyLoss(f64),
    /// Connection failure with specific connection
    ConnectionFailure(ConnectionId),
    /// Quantum state mismatch between nodes
    QuantumStateMismatch {
        expected: f64,
        actual: f64,
    },
    /// Reality anchor failure in the crystal network
    RealityAnchorFailure {
        node_id: String,
        anchor_strength: f64,
    },
    /// Violation of network topology rules
    TopologyViolation {
        message: String,
        severity: TopologyViolationSeverity,
    },
    /// Harmony state became unstable
    HarmonyStateUnstable {
        current_level: f64,
        threshold: f64,
    },
    /// Transport layer error
    TransportError(TransportErrorKind),
    /// Node initialization error
    NodeInitializationError {
        reason: String,
        severity: InitializationErrorSeverity,
    },
    /// Protocol version mismatch
    ProtocolMismatch {
        local_version: String,
        remote_version: String,
    },
    /// Invalid quantum bridge state
    InvalidBridgeState {
        bridge_id: String,
        state: String,
    },
    /// Network capacity exceeded
    CapacityExceeded {
        current: usize,
        maximum: usize,
    },
    /// Configuration error
    ConfigurationError(String),
}

/// Severity levels for topology violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyViolationSeverity {
    /// Warning level violation - network can continue
    Warning,
    /// Critical violation - requires immediate attention
    Critical,
    /// Fatal violation - network cannot continue
    Fatal,
}

/// Types of transport layer errors
#[derive(Debug)]
pub enum TransportErrorKind {
    /// Timeout during transport operation
    Timeout {
        operation: String,
        duration_ms: u64,
    },
    /// Data corruption during transport
    Corruption {
        message: String,
        integrity_check: String,
    },
    /// Connection dropped unexpectedly
    ConnectionDropped {
        connection_id: ConnectionId,
        reason: String,
    },
    /// Buffer overflow in transport layer
    BufferOverflow {
        capacity: usize,
        attempted: usize,
    },
}

/// Severity levels for initialization errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitializationErrorSeverity {
    /// Recoverable error
    Recoverable,
    /// Non-recoverable error
    Fatal,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HarmonyLoss(level) =>
            write!(f, "Harmony loss detected (current level: {:.3})", level),
            Self::ConnectionFailure(id) =>
            write!(f, "Connection failure on {}", id.0),
            Self::QuantumStateMismatch { expected, actual } =>
            write!(f, "Quantum state mismatch - expected: {:.3}, actual: {:.3}", expected, actual),
            Self::RealityAnchorFailure { node_id, anchor_strength } =>
            write!(f, "Reality anchor failure on node {} (strength: {:.3})", node_id, anchor_strength),
            Self::TopologyViolation { message, severity } =>
            write!(f, "Topology violation ({:?}): {}", severity, message),
            Self::HarmonyStateUnstable { current_level, threshold } =>
            write!(f, "Harmony state unstable - current: {:.3}, threshold: {:.3}", current_level, threshold),
            Self::TransportError(kind) =>
            write!(f, "Transport error: {:?}", kind),
            Self::NodeInitializationError { reason, severity } =>
            write!(f, "Node initialization error ({:?}): {}", severity, reason),
            Self::ProtocolMismatch { local_version, remote_version } =>
            write!(f, "Protocol version mismatch - local: {}, remote: {}", local_version, remote_version),
            Self::InvalidBridgeState { bridge_id, state } =>
            write!(f, "Invalid bridge state for {} - state: {}", bridge_id, state),
            Self::CapacityExceeded { current, maximum } =>
            write!(f, "Network capacity exceeded - current: {}, maximum: {}", current, maximum),
            Self::ConfigurationError(msg) =>
            write!(f, "Configuration error: {}", msg),
        }
    }
}

impl Error for NetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Trait for types that can be converted into a NetworkError
pub trait IntoNetworkError {
    fn into_network_error(self) -> NetworkError;
}

/// Extension methods for NetworkResult
pub trait NetworkResultExt<T> {
    /// Adds context to an error result
    fn with_context<C>(self, ctx: C) -> NetworkResult<T>
    where
    C: FnOnce() -> String;
}

impl<T> NetworkResultExt<T> for NetworkResult<T> {
    fn with_context<C>(self, ctx: C) -> NetworkResult<T>
    where
    C: FnOnce() -> String,
    {
        self.map_err(|e| match e {
            NetworkError::ConfigurationError(msg) =>
            NetworkError::ConfigurationError(format!("{}: {}", ctx(), msg)),
                     e => e,
        })
    }
}

/// Helper functions for creating common error types
impl NetworkError {
    /// Creates a new harmony loss error
    pub fn harmony_loss(level: f64) -> Self {
        Self::HarmonyLoss(level)
    }

    /// Creates a new connection failure error
    pub fn connection_failure(id: ConnectionId) -> Self {
        Self::ConnectionFailure(id)
    }

    /// Creates a new quantum state mismatch error
    pub fn quantum_mismatch(expected: f64, actual: f64) -> Self {
        Self::QuantumStateMismatch { expected, actual }
    }

    /// Creates a new reality anchor failure error
    pub fn reality_anchor_failure(node_id: String, anchor_strength: f64) -> Self {
        Self::RealityAnchorFailure {
            node_id,
            anchor_strength,
        }
    }

    /// Creates a new topology violation error
    pub fn topology_violation(message: String, severity: TopologyViolationSeverity) -> Self {
        Self::TopologyViolation { message, severity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_id_creation() {
        let id = ConnectionId::new();
        assert_ne!(id.as_uuid(), Uuid::nil());
    }

    #[test]
    fn test_error_display() {
        let error = NetworkError::HarmonyLoss(0.5);
        assert!(error.to_string().contains("0.5"));

        let error = NetworkError::QuantumStateMismatch {
            expected: 1.0,
            actual: 0.7,
        };
        let error_string = error.to_string();
        assert!(error_string.contains("1.000"));
        assert!(error_string.contains("0.700"));
    }

    #[test]
    fn test_error_context() {
        let result: NetworkResult<()> = Err(NetworkError::ConfigurationError("invalid value".into()));
        let result_with_context = result.with_context(|| "parsing config".into());
        assert!(result_with_context.unwrap_err().to_string().contains("parsing config"));
    }

    #[test]
    fn test_error_creation_helpers() {
        let harmony_error = NetworkError::harmony_loss(0.5);
        match harmony_error {
            NetworkError::HarmonyLoss(level) => assert_eq!(level, 0.5),
            _ => panic!("Wrong error variant"),
        }

        let connection_id = ConnectionId::new();
        let connection_error = NetworkError::connection_failure(connection_id);
        match connection_error {
            NetworkError::ConnectionFailure(id) => assert_eq!(id, connection_id),
            _ => panic!("Wrong error variant"),
        }
    }
}
