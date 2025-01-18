//! Crystalline Alignment Module
//! =========================
//!
//! Provides quantum-safe alignment operations through crystalline
//! lattice structures and harmonic resonance.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:44:37 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::QUANTUM_STABILITY_THRESHOLD,
    cube::CrystalCube,
    harmony::{Quantum, MeshValue},
    vector::Vector3D,
    zeronaut::Zeronaut
};

/// Crystalline alignment state
#[derive(Clone, Copy)]
pub enum AlignState {
    /// Perfect crystalline alignment
    Perfect,
    /// Stable but imperfect alignment
    Stable,
    /// Unstable alignment
    Unstable,
    /// Complete misalignment
    Misaligned,
}

/// Crystalline alignment manager
pub struct CrystalAlign {
    /// Base alignment point
    origin: Vector3D<f64>,
    /// Crystalline coherence value
    coherence: f64,
    /// Current alignment state
    state: AlignState,
}

impl CrystalAlign {
    /// Creates a new crystalline alignment manager
    pub fn new() -> Self {
        Self {
            origin: Vector3D::zero(),
            coherence: 1.0,
            state: AlignState::Perfect,
        }
    }

    /// Creates a new alignment at specific coordinates
    pub fn new_positioned(x: f64, y: f64, z: f64) -> Self {
        Self {
            origin: Vector3D::new(x, y, z),
            coherence: 1.0,
            state: AlignState::Perfect,
        }
    }

    /// Gets the current alignment state
    pub fn state(&self) -> AlignState {
        self.state
    }

    /// Gets the crystalline coherence value
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Aligns a Zeronaut in crystalline space
    pub fn align_zeronaut(&mut self, zeronaut: &mut Zeronaut<u8>, x: f64, y: f64, z: f64) -> Result<(), &'static str> {
        if self.coherence < QUANTUM_STABILITY_THRESHOLD {
            return Err("Insufficient crystalline coherence for alignment");
        }

        let target = Vector3D::new(x, y, z);
        let distance = target.mesh_sub(&self.origin);

        // Check alignment bounds
        if distance.length_squared() > 100.0 {
            self.state = AlignState::Misaligned;
            return Err("Target position out of crystalline alignment range");
        }

        // Perform alignment
        zeronaut.set_position(target);
        self.decohere();

        Ok(())
    }

    /// Creates a grid of aligned Zeronauts
    pub fn create_alignment_grid(&mut self, size: usize) -> Result<Vec<Zeronaut<u8>>, &'static str> {
        if size > 16 {
            return Err("Grid size exceeds crystalline stability limits");
        }

        let mut grid = Vec::new();
        let zero = Zeronaut::new();

        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let mut zeronaut = zero.clone();
                    self.align_zeronaut(&mut zeronaut, x as f64, y as f64, z as f64)?;
                    grid.push(zeronaut);
                }
            }
        }

        Ok(grid)
    }

    /// Applies quantum decoherence
    fn decohere(&mut self) {
        self.coherence *= 0.9;
        if self.coherence < QUANTUM_STABILITY_THRESHOLD {
            self.state = AlignState::Unstable;
        } else if self.coherence < 0.8 {
            self.state = AlignState::Stable;
        }
    }

    /// Restores crystalline coherence
    pub fn restore_coherence(&mut self) {
        self.coherence = 1.0;
        self.state = AlignState::Perfect;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_basics() {
        let mut align = CrystalAlign::new();
        assert_eq!(align.coherence(), 1.0);
        assert!(matches!(align.state(), AlignState::Perfect));
    }

    #[test]
    fn test_zeronaut_alignment() {
        let mut align = CrystalAlign::new();
        let mut zeronaut = Zeronaut::new();

        assert!(align.align_zeronaut(&mut zeronaut, 1.0, 1.0, 1.0).is_ok());
        assert!(align.coherence() < 1.0);
    }

    #[test]
    fn test_grid_creation() {
        let mut align = CrystalAlign::new();
        let grid = align.create_alignment_grid(2);
        assert!(grid.is_ok());
        assert_eq!(grid.unwrap().len(), 8); // 2x2x2 grid
    }

    #[test]
    fn test_alignment_limits() {
        let mut align = CrystalAlign::new();
        let mut zeronaut = Zeronaut::new();

        assert!(align.align_zeronaut(&mut zeronaut, 100.0, 100.0, 100.0).is_err());
        assert!(matches!(align.state(), AlignState::Misaligned));
    }
}
