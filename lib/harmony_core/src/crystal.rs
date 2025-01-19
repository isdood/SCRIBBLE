//! Crystal Computing Core Operations
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:51:41 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::Index;
use meshmath::{floor, sqrt};
use crate::{
    vector::Vector3D,
    errors::{QuantumError, CoherenceError},
    constants::{
        MAX_QUANTUM_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        CRYSTAL_RESONANCE_THRESHOLD
    },
    align::{Alignment, AlignmentState},
    idk::ShardUninit,
};

/// Core crystal node for quantum operations
#[derive(Debug)]
pub struct CrystalNode {
    /// Position in crystal lattice
    position: Vector3D<f64>,
    /// Phase coherence value
    coherence: f64,
    /// Node alignment
    alignment: Alignment,
}

/// Crystal lattice structure
#[derive(Debug)]
pub struct CrystalLattice {
    /// Lattice nodes storage
    nodes: [[ShardUninit<CrystalNode>; MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE],
    /// Lattice size
    size: usize,
    /// Lattice alignment
    alignment: Alignment,
}

impl CrystalNode {
    /// Create a new crystal node
    pub fn new(position: Vector3D<f64>) -> Self {
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
    pub fn set_phase_coherence(&mut self, value: f64) -> Result<(), CoherenceError> {
        if value < 0.0 || value > 1.0 {
            return Err(CoherenceError::InvalidValue);
        }
        self.coherence = value;
        Ok(())
    }

    /// Get node's position
    pub fn position(&self) -> &Vector3D<f64> {
        &self.position
    }

    /// Get node's alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        self.alignment.get_state()
    }
}

impl CrystalLattice {
    /// Create a new crystal lattice
    pub fn new(size: usize) -> Self {
        let size = size.min(MAX_QUANTUM_SIZE);
        let nodes = [[ShardUninit::new(); MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE];
        let origin = Vector3D::new(0.0, 0.0, 0.0);

        Self {
            nodes,
            size,
            alignment: Alignment::new(origin),
        }
    }

    /// Get node at position
    pub fn get_node(&self, pos: &Vector3D<f64>) -> Result<&CrystalNode, QuantumError> {
        let x = floor(pos.x) as usize;
        let y = floor(pos.y) as usize;
        let z = floor(pos.z) as usize;

        if x >= self.size || y >= self.size || z >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        unsafe {
            self.nodes[x][y].get_ref()
            .ok_or(QuantumError::InvalidState)
        }
    }

    /// Set node at position
    pub fn set_node(&mut self, pos: &Vector3D<f64>, node: CrystalNode) -> Result<(), QuantumError> {
        let x = floor(pos.x) as usize;
        let y = floor(pos.y) as usize;
        let z = floor(pos.z) as usize;

        if x >= self.size || y >= self.size || z >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        unsafe {
            self.nodes[x][y].set(node);
        }
        Ok(())
    }

    /// Calculate resonance at position
    pub fn calculate_resonance(&self, pos: &Vector3D<f64>) -> Result<f64, QuantumError> {
        let node = self.get_node(pos)?;
        let coherence = node.get_phase_coherence();

        if coherence < CRYSTAL_RESONANCE_THRESHOLD {
            return Err(QuantumError::ResonanceFailure);
        }

        Ok(sqrt(coherence * QUANTUM_STABILITY_THRESHOLD))
    }

    /// Get lattice size
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get current alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        self.alignment.get_state()
    }
}

impl Index<usize> for CrystalLattice {
    type Output = [ShardUninit<CrystalNode>; MAX_QUANTUM_SIZE];

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
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
