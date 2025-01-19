//! Crystal Mesh Thread Implementation
//! Last Updated: 2025-01-19 13:46:44 UTC
//! Author: isdood
//! Current User: isdood

use crate::instruction::CrystalInstruction;
use harmony_core::aether::AetherField;
use shard::core::{ShardRegisterFile, Vector4D};

/// Crystal-mesh thread state
pub struct CrystalMeshThread {
    /// Thread's register file
    pub register_file: ShardRegisterFile,
    /// Queue of instructions to be executed
    pub instruction_queue: Vec<CrystalInstruction>,
    /// Quantum state field
    pub quantum_state: AetherField,
    /// Position in the crystal mesh
    pub mesh_position: Vector4D,
    /// Current blend depth
    pub blend_depth: usize,
}

impl CrystalMeshThread {
    /// Create a new crystal mesh thread
    pub fn new(position: Vector4D) -> Self {
        Self {
            register_file: ShardRegisterFile::new(),
            instruction_queue: Vec::new(),
            quantum_state: AetherField::new(position.to_3d()),
            mesh_position: position,
            blend_depth: 0,
        }
    }

    /// Check if thread has pending instructions
    pub fn has_pending_instructions(&self) -> bool {
        !self.instruction_queue.is_empty()
    }

    /// Get next instruction while maintaining quantum coherence
    pub fn next_instruction(&mut self) -> Option<CrystalInstruction> {
        if self.quantum_state.get_coherence() >= crate::BLEND_COHERENCE_THRESHOLD {
            self.instruction_queue.pop()
        } else {
            None
        }
    }
}
