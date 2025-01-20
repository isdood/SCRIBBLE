//! Mycelium: Crystal-Based High-Performance Computing Network
//!
//! This library provides the networking infrastructure for crystal-based
//! distributed computing systems.

pub mod node;
pub mod network;
pub mod transport;
pub mod topology;
pub mod error;
pub mod coherence;

pub mod prelude {
    //! Convenient imports for common Mycelium types and traits
    pub use crate::node::{CrystalNode, NodeConfig, NodeId};
    pub use crate::network::{CrystalNetwork, NetworkEvent};
    pub use crate::transport::{TransportChannel, QuantumState};
    pub use crate::topology::TopologyType;
    pub use crate::error::{NetworkError, NetworkResult};
    pub use crate::coherence::{CoherenceMonitor, StabilityMetrics};
}

// Re-exports
pub use crate::node::CrystalNode;
pub use crate::network::CrystalNetwork;
pub use crate::error::NetworkResult;
