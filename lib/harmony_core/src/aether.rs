//! Crystalline Aether Implementation
//! ===========================
//!
//! Core quantum aether operations through crystalline
//! structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 21:17:24 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::{AETHER_RESONANCE_FACTOR, QUANTUM_STABILITY_THRESHOLD, MAX_QUANTUM_SIZE},
    harmony::Quantum,
    vector::Vector3D,
    idk::ShardUninit,
};

/// A crystalline aether node for quantum operations
#[derive(Clone)]
pub struct AetherNode<T: Clone + 'static> {
    /// Node data
    data: T,
    /// Node position
    position: Vector3D<f64>,
    /// Quantum coherence
    coherence: f64,
}

impl<T: Clone + Default + 'static> AetherNode<T> {
    /// Creates a new aether node
    pub fn new(data: T, x: f64, y: f64, z: f64) -> Self {
        Self {
            data,
            position: Vector3D::new(x, y, z),
            coherence: 1.0,
        }
    }

    /// Gets a reference to the node data
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Gets a mutable reference to the node data
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Gets the node position
    pub fn position(&self) -> &Vector3D<f64> {
        &self.position
    }

    /// Sets the node position
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position = Vector3D::new(x, y, z);
        self.decohere();
    }
}

impl<T: Clone + 'static> Quantum for AetherNode<T> {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    fn decohere(&mut self) {
        self.coherence *= AETHER_RESONANCE_FACTOR;
        if self.coherence < QUANTUM_STABILITY_THRESHOLD {
            self.coherence = QUANTUM_STABILITY_THRESHOLD;
        }
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
    }
}

/// A crystalline aether grid for quantum operations
#[derive(Clone)]
pub struct AetherGrid<T: Clone + 'static> {
    /// Grid nodes
    nodes: [[[ShardUninit<Option<AetherNode<T>>>; MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE],
    /// Grid dimensions
    dimensions: Vector3D<u64>,
    /// Grid coherence
    coherence: f64,
}

impl<T: Clone + Default + 'static> AetherGrid<T> {
    /// Creates a new aether grid
    pub fn new(x: u64, y: u64, z: u64) -> Option<Self> {
        if x as usize > MAX_QUANTUM_SIZE || y as usize > MAX_QUANTUM_SIZE || z as usize > MAX_QUANTUM_SIZE {
            return None;
        }

        let mut grid = Self {
            nodes: [[[ShardUninit::uninit(); MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE]; MAX_QUANTUM_SIZE],
            dimensions: Vector3D::new(x, y, z),
            coherence: 1.0,
        };

        // Initialize nodes
        for i in 0..x as usize {
            for j in 0..y as usize {
                for k in 0..z as usize {
                    grid.nodes[i][j][k] = ShardUninit::new(Some(AetherNode::new(
                        T::default(),
                                                                                i as f64,
                                                                                j as f64,
                                                                                k as f64
                    )));
                }
            }
        }

        Some(grid)
    }

    /// Gets the grid dimensions
    pub fn dimensions(&self) -> &Vector3D<u64> {
        &self.dimensions
    }

    /// Gets a reference to a node
    pub fn get_node(&self, x: usize, y: usize, z: usize) -> Option<&AetherNode<T>> {
        if x >= self.dimensions.x as usize ||
            y >= self.dimensions.y as usize ||
            z >= self.dimensions.z as usize {
                return None;
            }
            unsafe { self.nodes[x][y][z].assume_init_ref().as_ref() }
    }

    /// Gets a mutable reference to a node
    pub fn get_node_mut(&mut self, x: usize, y: usize, z: usize) -> Option<&mut AetherNode<T>> {
        if x >= self.dimensions.x as usize ||
            y >= self.dimensions.y as usize ||
            z >= self.dimensions.z as usize {
                return None;
            }
            unsafe { self.nodes[x][y][z].assume_init_mut().as_mut() }
    }

    /// Sets a node's data
    pub fn set_node_data(&mut self, x: usize, y: usize, z: usize, data: T) -> Result<(), &'static str> {
        if let Some(node) = self.get_node_mut(x, y, z) {
            node.data = data;
            node.decohere();
            self.decohere();
            Ok(())
        } else {
            Err("Node not found")
        }
    }
}

impl<T: Clone + 'static> Quantum for AetherGrid<T> {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    fn decohere(&mut self) {
        self.coherence *= AETHER_RESONANCE_FACTOR;
        if self.coherence < QUANTUM_STABILITY_THRESHOLD {
            self.coherence = QUANTUM_STABILITY_THRESHOLD;
        }
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aether_node_basics() {
        let node = AetherNode::new(42u8, 1.0, 2.0, 3.0);
        assert_eq!(*node.data(), 42);
        assert_eq!(node.position().x, 1.0);
        assert_eq!(node.position().y, 2.0);
        assert_eq!(node.position().z, 3.0);
        assert!(node.is_stable());
    }

    #[test]
    fn test_aether_grid_creation() {
        let grid = AetherGrid::<u8>::new(2, 2, 2);
        assert!(grid.is_some());

        let grid = grid.unwrap();
        assert_eq!(grid.dimensions(), &Vector3D::new(2, 2, 2));
        assert!(grid.is_stable());
    }

    #[test]
    fn test_aether_grid_access() {
        let mut grid = AetherGrid::<u8>::new(2, 2, 2).unwrap();
        assert!(grid.get_node(0, 0, 0).is_some());
        assert!(grid.get_node(2, 2, 2).is_none());

        assert!(grid.set_node_data(0, 0, 0, 42).is_ok());
        assert!(grid.set_node_data(2, 2, 2, 42).is_err());

        assert_eq!(*grid.get_node(0, 0, 0).unwrap().data(), 42);
    }
}
