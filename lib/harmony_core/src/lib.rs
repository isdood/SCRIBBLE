//! Harmony Core Library
//! =====================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 21:08:54 UTC
//! Version: 0.1.1
//! License: MIT

mod align;
mod crystal;
mod idk;
mod aether;
mod cube;
mod growth;
mod harmony;
mod phantom;
mod zeronaut;
mod cell;

pub use align::*;
pub use crystal::*;
pub use idk::*;
pub use aether::*;
pub use cube::*;
pub use growth::*;
pub use harmony::*;
pub use phantom::*;
pub use zeronaut::*;
pub use cell::*;

use magicmath::constants::{
    HARMONY_RESONANCE_THRESHOLD,
    HARMONY_STABILITY_THRESHOLD,
    MAX_QUANTUM_SIZE,
};
use errors::{MathError, QuantumError};
use magicmath::{
    MeshValue,
    Vector3D,
    resonance::Resonance,
};

// Define the Protected trait at the crate root
pub trait Protected {
    fn protect(&self) -> bool;
    fn unprotect(&self) -> bool;
    fn get_coherence(&self) -> f64;
    fn is_harmonically_stable(&self) -> bool;
}

/// Core crystal node for quantum operations
#[derive(Debug, Clone)]
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
            return Err(MathError::InvalidParameter(String::from("Phase coherence value must be between 0 and 1"))); // Correcting error variant
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
        self.alignment.state() // Correcting method call
    }
}

/// Crystal lattice structure
#[derive(Debug)]
pub struct CrystalLattice {
    /// Lattice nodes storage
    nodes: Vec<Vec<Option<CrystalNode>>>, // Using Vec instead of fixed-size array
    /// Lattice size
    size: usize,
    /// Lattice alignment
    alignment: Alignment,
}

impl CrystalLattice {
    /// Create a new crystal lattice
    pub fn new(size: usize) -> Self {
        let size = size.min(MAX_QUANTUM_SIZE); // Adjusted to use a valid constant
        let nodes = vec![vec![None; size]; size]; // Using Vec instead of fixed-size array
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

        self.nodes[x][y].as_ref().ok_or(QuantumError::InvalidState) // Using Option
    }

    /// Set node at position
    pub fn set_node(&mut self, pos: &Vector3D, node: CrystalNode) -> Result<(), QuantumError> {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;

        if x >= self.size || y >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        self.nodes[x][y] = Some(node); // Using Option
        Ok(())
    }

    /// Calculate resonance at position
    pub fn calculate_resonance(&self, pos: &Vector3D) -> Result<f64, QuantumError> {
        let node = self.get_node(pos)?;
        let coherence = node.get_phase_coherence();

        if coherence < HARMONY_RESONANCE_THRESHOLD {
            return Err(QuantumError::ResonanceFailure);
        }

        Ok((coherence * HARMONY_STABILITY_THRESHOLD).sqrt())
    }

    /// Get lattice size
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get current alignment state
    pub fn alignment_state(&self) -> AlignmentState {
        self.alignment.state() // Correcting method call
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
