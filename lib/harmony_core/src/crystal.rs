//! Crystal Computing Core Operations
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 17:59:18 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::{
    MeshValue,
    Vector3D,
    resonance::Resonance,
};

use errors::{
    MathError,
    QuantumError,
};

use crate::{
    constants::{
        MAX_QUANTUM_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        CRYSTAL_RESONANCE_THRESHOLD,
    },
    align::{Alignment, AlignmentState},
    idk::ShardUninit,
};

/// Core crystal node for quantum operations
#[derive(Debug)]
pub struct CrystalNode {
    /// Position in crystal lattice
    position: Vector3D,
    /// Phase coherence value
    coherence: f64,
    /// Node alignment
    alignment: Alignment,
}

impl CrystalNode {
    /// Create a new crystal node
    pub fn new(position: Vector3D) -> Self {
        Self {
            position: position.clone(),
            coherence: 1.0,
            alignment: Alignment::new(position),
        }
    }

    /// Get node's phase coherence
    pub fn get_phase_coherence(&self) -> f64 {
        self.coherence
    }

    /// Set node's phase coherence
    pub fn set_phase_coherence(&mut self, value: f64) -> Result<(), MathError> {
        if value < 0.0 || value > 1.0 {
            return Err(MathError::InvalidRange); // Fix: Correcting error variant
        }
        self.coherence = value;
        Ok(())
    }

    /// Get node's position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Get node's alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        self.alignment.state() // Fix: Correcting method call
    }
}

/// Crystal lattice structure
#[derive(Debug)]
pub struct CrystalLattice {
    /// Lattice nodes storage
    nodes: Vec<Vec<Option<CrystalNode>>>, // Fix: Using Vec instead of fixed-size array
    /// Lattice size
    size: usize,
    /// Lattice alignment
    alignment: Alignment,
}

impl CrystalLattice {
    /// Create a new crystal lattice
    pub fn new(size: usize) -> Self {
        let size = size.min(MAX_QUANTUM_SIZE);
        let nodes = vec![vec![None; size]; size]; // Fix: Using Vec instead of fixed-size array
        let origin = Vector3D::new(0.0, 0.0, 0.0);

        Self {
            nodes,
            size,
            alignment: Alignment::new(origin),
        }
    }

    /// Get node at position
    pub fn get_node(&self, pos: &Vector3D) -> Result<&CrystalNode, QuantumError> {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;

        if x >= self.size || y >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        self.nodes[x][y].as_ref().ok_or(QuantumError::InvalidState) // Fix: Using Option
    }

    /// Set node at position
    pub fn set_node(&mut self, pos: &Vector3D, node: CrystalNode) -> Result<(), QuantumError> {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;

        if x >= self.size || y >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        self.nodes[x][y] = Some(node); // Fix: Using Option
        Ok(())
    }

    /// Calculate resonance at position
    pub fn calculate_resonance(&self, pos: &Vector3D) -> Result<f64, QuantumError> {
        let node = self.get_node(pos)?;
        let coherence = node.get_phase_coherence();

        if coherence < CRYSTAL_RESONANCE_THRESHOLD {
            return Err(QuantumError::ResonanceFailure);
        }

        Ok((coherence * QUANTUM_STABILITY_THRESHOLD).sqrt())
    }

    /// Get lattice size
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get current alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        self.alignment.state() // Fix: Correcting method call
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_node_creation() {
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        let node = CrystalNode::new(pos);
        assert_eq!(node.get_phase_coherence(), 1.0);
    }

    #[test]
    fn test_crystal_lattice_creation() {
        let lattice = CrystalLattice::new(4);
        assert_eq!(lattice.size(), 4);
    }

    #[test]
    fn test_node_coherence() {
        let mut node = CrystalNode::new(Vector3D::new(0.0, 0.0, 0.0));
        assert!(node.set_phase_coherence(0.5).is_ok());
        assert_eq!(node.get_phase_coherence(), 0.5);
    }

    #[test]
    fn test_resonance_calculation() {
        let lattice = CrystalLattice::new(4);
        let pos = Vector3D::new(0.0, 0.0, 0.0);
        assert!(lattice.calculate_resonance(&pos).is_err()); // No node set yet
    }
}
