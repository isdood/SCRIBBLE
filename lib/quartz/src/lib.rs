//! Crystal Threading Implementation
//! Last Updated: 2025-01-19 13:46:44 UTC
//! Author: isdood
//! Current User: isdood
//!
//! Crystal-based threading system for near-native workload distribution
//! through quantum-coherent instruction blending.

pub mod executor;
pub mod instruction;
pub mod thread;
pub mod fabric;

// Re-exports
pub use executor::CrystalMeshExecutor;
pub use instruction::CrystalInstruction;
pub use thread::CrystalMeshThread;
pub use fabric::InstructionFabric;

// Constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub use harmony_core::constants::QUANTUM_STABILITY_THRESHOLD;
pub use shard::core::FAIRY_DUST_COEFFICIENT;

/// Quantum phase coherence threshold for instruction blending
pub const BLEND_COHERENCE_THRESHOLD: f64 = 0.95;
/// Maximum blend depth for instruction superposition
pub const MAX_BLEND_DEPTH: usize = 64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(BLEND_COHERENCE_THRESHOLD > QUANTUM_STABILITY_THRESHOLD);
        assert!(MAX_BLEND_DEPTH > 0);
    }
}
