//! Crystalline Alignment Implementation
//! ==============================
//!
//! Core quantum alignment operations through crystalline
//! structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 21:12:37 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::Deref;
use libm::floor;
use crate::{
    constants::{
        QUANTUM_GOLDEN_RATIO,
        QUANTUM_STABILITY_THRESHOLD,
        MAX_QUANTUM_SIZE
    },
    harmony::Quantum,
    vector::Vector3D,
    zeronaut::Zeronaut
};

/// Alignment status for quantum grid
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignmentStatus {
    /// Perfectly aligned
    Perfect,
    /// Partially aligned
    Partial,
    /// Misaligned
    Misaligned,
}

/// A quantum alignment grid
#[derive(Clone)]
pub struct AlignmentGrid {
    /// Grid dimensions
    dimensions: Vector3D<u64>,
    /// Alignment values
    values: Vec<Vec<Vec<f64>>>,
    /// Current status
    status: AlignmentStatus,
    /// Quantum coherence
    coherence: f64,
}

impl AlignmentGrid {
    /// Creates a new alignment grid
    pub fn new(x: u64, y: u64, z: u64) -> Option<Self> {
        if x as usize > MAX_QUANTUM_SIZE ||
            y as usize > MAX_QUANTUM_SIZE ||
            z as usize > MAX_QUANTUM_SIZE {
                return None;
            }

            let mut values = Vec::with_capacity(x as usize);
        for _ in 0..x as usize {
            let mut yvec = Vec::with_capacity(y as usize);
            for _ in 0..y as usize {
                let zvec = vec![QUANTUM_GOLDEN_RATIO; z as usize];
                yvec.push(zvec);
            }
            values.push(yvec);
        }

        let mut grid = Self {
            dimensions: Vector3D::new(x, y, z),
            values,
            status: AlignmentStatus::Perfect,
            coherence: 1.0,
        };

        grid.update_status();
        Some(grid)
    }

    /// Gets the grid dimensions
    pub fn dimensions(&self) -> &Vector3D<u64> {
        &self.dimensions
    }

    /// Gets the current alignment status
    pub fn status(&self) -> AlignmentStatus {
        self.status
    }

    /// Gets an alignment value
    pub fn get_value(&self, x: usize, y: usize, z: usize) -> Option<f64> {
        if x >= self.dimensions.x as usize ||
            y >= self.dimensions.y as usize ||
            z >= self.dimensions.z as usize {
                return None;
            }
            Some(self.values[x][y][z])
    }

    /// Sets an alignment value
    pub fn set_value(&mut self, x: usize, y: usize, z: usize, value: f64) -> Result<(), &'static str> {
        if x >= self.dimensions.x as usize ||
            y >= self.dimensions.y as usize ||
            z >= self.dimensions.z as usize {
                return Err("Coordinates out of bounds");
            }

            self.values[x][y][z] = value;
        self.update_status();
        self.decohere();
        Ok(())
    }

    /// Updates alignment status based on current values
    fn update_status(&mut self) {
        let mut perfect_count = 0;
        let mut misaligned_count = 0;
        let total_cells = (self.dimensions.x * self.dimensions.y * self.dimensions.z) as usize;

        for x in 0..self.dimensions.x as usize {
            for y in 0..self.dimensions.y as usize {
                for z in 0..self.dimensions.z as usize {
                    let value = self.values[x][y][z];
                    if libm::fabs(value - QUANTUM_GOLDEN_RATIO) < 1e-10 {
                        perfect_count += 1;
                    } else if value < QUANTUM_STABILITY_THRESHOLD {
                        misaligned_count += 1;
                    }
                }
            }
        }

        self.status = if perfect_count == total_cells {
            AlignmentStatus::Perfect
        } else if misaligned_count > 0 {
            AlignmentStatus::Misaligned
        } else {
            AlignmentStatus::Partial
        };
    }

    /// Aligns a zeronaut with the grid
    pub fn align_zeronaut(&mut self, zeronaut: &mut Zeronaut<f64>) -> Result<(), &'static str> {
        let pos = zeronaut.position();
        let x = floor(pos.x) as usize;
        let y = floor(pos.y) as usize;
        let z = floor(pos.z) as usize;

        if let Some(value) = self.get_value(x, y, z) {
            *zeronaut.data_mut() = value;
            zeronaut.decohere();
            self.decohere();
            Ok(())
        } else {
            Err("Zeronaut position out of grid bounds")
        }
    }
}

impl Quantum for AlignmentGrid {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    fn decohere(&mut self) {
        self.coherence *= 0.9;
        if self.coherence < QUANTUM_STABILITY_THRESHOLD {
            self.coherence = QUANTUM_STABILITY_THRESHOLD;
            self.status = AlignmentStatus::Misaligned;
        }
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
        self.update_status();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_grid_creation() {
        let grid = AlignmentGrid::new(2, 2, 2);
        assert!(grid.is_some());

        let grid = grid.unwrap();
        assert_eq!(grid.dimensions(), &Vector3D::new(2, 2, 2));
        assert_eq!(grid.status(), AlignmentStatus::Perfect);
        assert!(grid.is_stable());
    }

    #[test]
    fn test_alignment_grid_bounds() {
        let grid = AlignmentGrid::new(MAX_QUANTUM_SIZE as u64 + 1, 2, 2);
        assert!(grid.is_none());
    }

    #[test]
    fn test_alignment_values() {
        let mut grid = AlignmentGrid::new(2, 2, 2).unwrap();
        assert!(grid.set_value(0, 0, 0, 1.0).is_ok());
        assert!(grid.set_value(2, 2, 2, 1.0).is_err());

        assert_eq!(grid.get_value(0, 0, 0), Some(1.0));
        assert_eq!(grid.status(), AlignmentStatus::Partial);
    }

    #[test]
    fn test_zeronaut_alignment() {
        let mut grid = AlignmentGrid::new(2, 2, 2).unwrap();
        let mut zeronaut = Zeronaut::new_positioned(0.0, 0.0, 0.0, 0.0);

        assert!(grid.align_zeronaut(&mut zeronaut).is_ok());
        assert_eq!(*zeronaut.data(), QUANTUM_GOLDEN_RATIO);
    }
}
