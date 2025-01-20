//! Crystal Computing Core Operations
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 23:38:27 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::Vector3D;

use errors::{MathError, QuantumError};

use magicmath::constants::{
    HARMONY_RESONANCE_THRESHOLD,
    HARMONY_STABILITY_THRESHOLD,
    MAX_QUANTUM_SIZE,  // Added this constant
};

use crate::align::{Alignment, AlignmentState};

/// Core crystal node for quantum operations
#[derive(Debug, Clone)]  // Added Debug derive
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
    ///
    /// # Parameters
    /// * `value` - Phase coherence value between 0.0 and 1.0
    ///
    /// # Returns
    /// * `Ok(())` if the value was set successfully
    /// * `Err(MathError::InvalidRange)` if value is outside [0.0, 1.0]
    pub fn set_phase_coherence(&mut self, value: f64) -> Result<(), MathError> {
        if value < 0.0 || value > 1.0 {
            return Err(MathError::InvalidRange);
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
        self.alignment.state()
    }
}

/// Crystal lattice structure
#[derive(Debug)]
pub struct CrystalLattice {
    /// Lattice nodes storage
    nodes: Vec<Vec<Option<CrystalNode>>>,
    /// Lattice size
    size: usize,
    /// Lattice alignment
    alignment: Alignment,
}

impl CrystalLattice {
    /// Create a new crystal lattice
    ///
    /// # Parameters
    /// * `size` - Size of the lattice (will be capped at MAX_QUANTUM_SIZE)
    pub fn new(size: usize) -> Self {
        let size = size.min(MAX_QUANTUM_SIZE);  // Use MAX_QUANTUM_SIZE instead
        let nodes = vec![vec![None; size]; size];
        let origin = Vector3D::new(0.0, 0.0, 0.0);

        Self {
            nodes,
            size,
            alignment: Alignment::new(origin),
        }
    }

    /// Get node at position
    ///
    /// # Returns
    /// * `Ok(&CrystalNode)` if node exists at position
    /// * `Err(QuantumError::BoundaryViolation)` if position is outside lattice bounds
    /// * `Err(QuantumError::InvalidState)` if no node exists at position
    pub fn get_node(&self, pos: &Vector3D) -> Result<&CrystalNode, QuantumError> {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;

        if x >= self.size || y >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        self.nodes[x][y].as_ref().ok_or(QuantumError::InvalidState)
    }

    /// Set node at position
    ///
    /// # Returns
    /// * `Ok(())` if node was set successfully
    /// * `Err(QuantumError::BoundaryViolation)` if position is outside lattice bounds
    pub fn set_node(&mut self, pos: &Vector3D, node: CrystalNode) -> Result<(), QuantumError> {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;

        if x >= self.size || y >= self.size {
            return Err(QuantumError::BoundaryViolation);
        }

        self.nodes[x][y] = Some(node);
        Ok(())
    }

    /// Calculate resonance at position
    ///
    /// # Returns
    /// * `Ok(f64)` containing the resonance value if calculation succeeds
    /// * `Err(QuantumError::ResonanceFailure)` if coherence is below threshold
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
        self.alignment.state()
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

    #[test]
    fn test_invalid_coherence() {
        let mut node = CrystalNode::new(Vector3D::new(0.0, 0.0, 0.0));
        assert!(node.set_phase_coherence(1.5).is_err());
        assert!(node.set_phase_coherence(-0.5).is_err());
    }

    #[test]
    fn test_boundary_violation() {
        let lattice = CrystalLattice::new(4);
        let pos = Vector3D::new(5.0, 5.0, 0.0);
        assert!(matches!(
            lattice.get_node(&pos),
                         Err(QuantumError::BoundaryViolation)
        ));
    }
}
