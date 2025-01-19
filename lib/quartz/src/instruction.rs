//! Crystal Instruction Implementation
//! Last Updated: 2025-01-19 13:46:44 UTC
//! Author: isdood
//! Current User: isdood

use crate::BLEND_COHERENCE_THRESHOLD;
use harmony_core::aether::AetherCell;
use shard::core::{ShardInstruction, Vector4D};

/// Represents a blended instruction in the crystal-mesh
#[derive(Clone)]
pub struct CrystalInstruction {
    /// Base instruction from Shard architecture
    pub base: ShardInstruction,
    /// Quantum phase component
    pub quantum_phase: f64,
    /// Position in the crystal mesh
    pub mesh_position: Vector4D,
    /// Coherence factor for quantum stability
    pub coherence_factor: f64,
    /// Blend mask for instruction superposition
    pub blend_mask: AetherCell<u64>,
}

impl CrystalInstruction {
    /// Create a new crystal instruction
    pub fn new(base: ShardInstruction, position: Vector4D) -> Self {
        Self {
            base,
            quantum_phase: 1.0,
            mesh_position: position,
            coherence_factor: BLEND_COHERENCE_THRESHOLD,
            blend_mask: AetherCell::new(0),
        }
    }

    /// Check if instruction maintains quantum coherence
    pub fn is_coherent(&self) -> bool {
        self.coherence_factor >= BLEND_COHERENCE_THRESHOLD
    }

    /// Apply quantum phase adjustment
    pub fn adjust_phase(&mut self, adjustment: f64) {
        self.quantum_phase *= adjustment;
        self.coherence_factor *= adjustment;
    }
}
