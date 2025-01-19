//! Crystal - Crystal Lattice Structures
//! ================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:24:52 UTC
//! Version: 0.1.0
//! License: MIT

use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use crate::vector::Vector3D;
use crate::idk::CoherenceError;

/// Crystal lattice structure
#[derive(Debug)]
pub struct CrystalLattice {
    size: usize,
    nodes: Vec<Vec<Vec<CrystalNode>>>,
}

/// Node in crystal lattice
#[derive(Debug, Clone)]
pub struct CrystalNode {
    phase: f64,
    resonance: f64,
    quantum_depth: u32,
}

impl CrystalLattice {
    pub fn new(size: usize) -> Self {
        let nodes = vec![vec![vec![CrystalNode::default(); size]; size]; size];
        Self { size, nodes }
    }

    pub fn is_valid_position(&self, pos: &Vector3D<f64>) -> bool {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let z = pos.z.floor() as usize;
        x < self.size && y < self.size && z < self.size
    }

    pub fn get_node(&self, x: usize, y: usize, z: usize) -> Result<&CrystalNode, CoherenceError> {
        self.nodes.get(x)
        .and_then(|yz| yz.get(y))
        .and_then(|z_nodes| z_nodes.get(z))
        .ok_or(CoherenceError::BoundaryViolation)
    }

    pub fn get_node_mut(&mut self, x: usize, y: usize, z: usize) -> Result<&mut CrystalNode, CoherenceError> {
        self.nodes.get_mut(x)
        .and_then(|yz| yz.get_mut(y))
        .and_then(|z_nodes| z_nodes.get_mut(z))
        .ok_or(CoherenceError::BoundaryViolation)
    }
}

impl CrystalNode {
    pub fn new(phase: f64, resonance: f64) -> Self {
        Self {
            phase,
            resonance,
            quantum_depth: 1,
        }
    }

    pub fn get_phase_coherence(&self) -> f64 {
        self.phase
    }

    pub fn get_resonance_factor(&self) -> f64 {
        self.resonance
    }

    pub fn get_quantum_depth(&self) -> u32 {
        self.quantum_depth
    }

    pub fn set_quantum_depth(&mut self, depth: u32) {
        self.quantum_depth = depth;
    }

    pub fn align_phase(&mut self, coherence: f64) -> Result<(), CoherenceError> {
        if coherence <= 0.0 || coherence > 1.0 {
            return Err(CoherenceError::PhaseAlignmentFailure);
        }
        self.phase = coherence;
        Ok(())
    }
}

impl Default for CrystalNode {
    fn default() -> Self {
        Self {
            phase: 1.0,
            resonance: 1.0,
            quantum_depth: 1,
        }
    }
}

/// Crystal cube structure
#[derive(Debug)]
pub struct CrystalCube<T> {
    size: usize,
    data: Vec<Vec<Vec<T>>>,
}

impl<T: Default + Clone> CrystalCube<T> {
    pub fn new(size: usize) -> Self {
        let data = vec![vec![vec![T::default(); size]; size]; size];
        Self { size, data }
    }

    pub fn get_state(&self, x: usize, y: usize, z: usize) -> Result<T, CoherenceError>
    where T: Clone {
        self.data.get(x)
        .and_then(|yz| yz.get(y))
        .and_then(|z_data| z_data.get(z))
        .cloned()
        .ok_or(CoherenceError::BoundaryViolation)
    }

    pub fn set_state(&mut self, x: usize, y: usize, z: usize, value: T) -> Result<(), CoherenceError> {
        if x >= self.size || y >= self.size || z >= self.size {
            return Err(CoherenceError::BoundaryViolation);
        }
        self.data[x][y][z] = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_lattice_creation() {
        let lattice = CrystalLattice::new(2);
        assert!(lattice.is_valid_position(&Vector3D::new(0.0, 0.0, 0.0)));
        assert!(!lattice.is_valid_position(&Vector3D::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_crystal_node_operations() {
        let mut node = CrystalNode::new(0.5, 0.8);
        assert_eq!(node.get_phase_coherence(), 0.5);
        assert_eq!(node.get_resonance_factor(), 0.8);
        assert!(node.align_phase(0.7).is_ok());
        assert!(node.align_phase(1.5).is_err());
    }
}
