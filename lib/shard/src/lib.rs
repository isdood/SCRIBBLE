//! Shard Architecture Library
//! Last Updated: 2025-01-18 19:19:36 UTC
//! Author: isdood
//!
//! A quantum-crystal hybrid architecture implementation integrating with
//! unstable_matter memory management for enhanced quantum stability.

use crate::hashbrown::{QuantumHashMap, HashBrownConfig};
use crate::vector4d::{Vector4D, HyperRotation, QuatTransform};
use crate::unstable_matter::{
    UnstableMatter,
    QuantumState,
    CrystalLattice,
    RealityAnchor,
    WaveFunction,
};
use crate::cereal::{Cereal, QuantumBuffer, CerealResult};
use crate::scribe::{Scribe, ScribePrecision, QuantumString};

// Re-export core components with unstable matter integration
pub mod core;
pub use core::{
    ShardRegisterFile,
    ShardMemory,
    ShardInstruction,
    ShardOpcode,
    QUANTUM_COHERENCE_THRESHOLD,
    FAIRY_DUST_COEFFICIENT,
    CACHE_MAX_ENTRIES,
};

// Re-export ISA with quantum state management
pub mod isa;
pub use isa::{
    without_quantum_decoherence,
    quantum_barrier,
    crystal_sync,
    matter_stabilize,
};

// Re-export emulator with unstable matter support
pub mod emulator;
pub use emulator::{
    ShardEmulator,
    QUANTUM_DECOHERENCE_FACTOR,
    MAX_DREAM_DEPTH,
    MATTER_STABILITY_THRESHOLD,
};

// Re-export memory management using unstable_matter
pub mod memory;
pub use memory::{
    ShardMemoryPattern,
    MAX_CRYSTAL_SIZE,
    GROWTH_DAMPENING,
    WAVE_COLLAPSE_THRESHOLD,
};

/// Common type aliases for Shard architecture with unstable matter
pub mod types {
    use super::*;

    pub type QResult<T> = Result<T, UnstableMatter>;
    pub type CrystalStructure = CrystalLattice<Vector4D>;
    pub type QuantumStateVector = WaveFunction<f64>;
    pub type RealityAnchorPoint = RealityAnchor<Vector4D>;
}

/// Constants updated for unstable matter integration
pub mod constants {
    use super::*;

    pub const VERSION: &str = "0.1.0";
    pub const ARCH_NAME: &str = "Shard";
    pub const CREATOR: &[u8] = b"isdood";
    pub const BUILD_TIMESTAMP: &str = "2025-01-18 19:19:36";
    pub const MATTER_STABILITY: f64 = 0.918033988749895; // φ² for enhanced stability
}

/// Feature flags for unstable matter optimization
#[cfg(feature = "quantum_acceleration")]
pub mod quantum_accel {
    pub use super::isa::quantum_accelerated_ops;
    pub use super::unstable_matter::accelerated_wave_collapse;
}

#[cfg(feature = "crystal_optimization")]
pub mod crystal_opt {
    pub use super::isa::crystal_optimized_ops;
    pub use super::unstable_matter::lattice_optimization;
}

/// Prelude module with unstable matter integration
pub mod prelude {
    pub use super::core::{ShardRegisterFile, ShardMemory, ShardInstruction, ShardOpcode};
    pub use super::emulator::ShardEmulator;
    pub use super::memory::ShardMemoryPattern;
    pub use super::types::{QResult, CrystalStructure, QuantumStateVector, RealityAnchorPoint};
    pub use super::isa::{without_quantum_decoherence, quantum_barrier, crystal_sync, matter_stabilize};
    pub use super::unstable_matter::{UnstableMatter, QuantumState, CrystalLattice, WaveFunction};
}

/// Initialize the Shard architecture with unstable matter support
///
/// # Returns
/// * `QResult<ShardEmulator>` - Initialized emulator or unstable matter error
pub fn init() -> types::QResult<ShardEmulator> {
    let emulator = ShardEmulator::new();

    // Verify quantum coherence and matter stability
    if !emulator.check_quantum_stability() {
        return Err(UnstableMatter::new("Quantum state unstable"));
    }

    if !emulator.check_matter_stability() {
        return Err(UnstableMatter::new("Matter stability below threshold"));
    }

    Ok(emulator)
}

/// Version information with matter stability metrics
pub fn version_info() -> String {
    format!(
        "Shard Architecture v{}\nBuilt: {}\nCreator: {}\nMatter Stability: {:.6}\n",
        constants::VERSION,
        constants::BUILD_TIMESTAMP,
        String::from_utf8_lossy(constants::CREATOR),
            constants::MATTER_STABILITY
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let result = init();
        assert!(result.is_ok(), "Shard initialization should succeed");
    }

    #[test]
    fn test_matter_stability() {
        let emulator = ShardEmulator::new();
        assert!(emulator.check_matter_stability(), "Matter should be stable");
    }

    #[test]
    fn test_version_info() {
        let info = version_info();
        assert!(info.contains("Matter Stability"));
        assert!(info.contains(format!("{:.6}", constants::MATTER_STABILITY).as_str()));
    }
}
