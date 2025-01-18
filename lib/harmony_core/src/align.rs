//! Crystalline Alignment Implementation
//! ==============================
//!
//! Core quantum alignment operations through crystalline
//! lattice structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:55:08 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::{
        QUANTUM_STABILITY_THRESHOLD,
        CRYSTAL_LATTICE_SPACING,
        QUANTUM_MESH_RESOLUTION
    },
    harmony::{Quantum, MeshValue},
    vector::Vector3D,
    CrystalArray,
    Zeronaut,
};

/// Quantum alignment states
#[derive(Clone, Copy, PartialEq)]
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

/// Core quantum alignment implementation
#[derive(Clone)]
pub struct CrystalAlign<T: Clone + 'static> {
    /// Crystalline data grid
    grid: CrystalArray<Zeronaut<T>>,
    /// Current alignment state
    state: AlignState,
    /// Quantum coherence tracking
    coherence: f64,
    /// Grid dimensions
    dimensions: Vector3D<usize>,
}

impl<T: Clone + Default + 'static> CrystalAlign<T> {
    /// Creates a new crystal alignment grid
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        let dimensions = Vector3D::new(x, y, z);
        let capacity = x * y * z;
        let mut grid = CrystalArray::with_capacity(capacity);

        let zero = T::default();
        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                for z in 0..dimensions.z {
                    let pos = Vector3D::new(
                        (x as f64) * CRYSTAL_LATTICE_SPACING,
                                            (y as f64) * CRYSTAL_LATTICE_SPACING,
                                            (z as f64) * CRYSTAL_LATTICE_SPACING
                    );
                    let zeronaut = Zeronaut::new_positioned(zero.clone(), pos.x, pos.y, pos.z);
                }
            }
        }

        Self {
            grid,
            state: AlignState::Perfect,
            coherence: 1.0,
            dimensions,
        }
    }

    /// Gets the current alignment state
    pub fn state(&self) -> AlignState {
        self.state
    }

    /// Gets the current coherence value
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Gets the grid dimensions
    pub fn dimensions(&self) -> &Vector3D<usize> {
        &self.dimensions
    }

    /// Gets a reference to a zeronaut at specific coordinates
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&Zeronaut<T>> {
        if x >= self.dimensions.x || y >= self.dimensions.y || z >= self.dimensions.z {
            return None;
        }
        let index = x + y * self.dimensions.x + z * self.dimensions.x * self.dimensions.y;
        unsafe {
            Some(&*self.grid.as_ptr().add(index))
        }
    }

    /// Gets a mutable reference to a zeronaut at specific coordinates
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> Option<&mut Zeronaut<T>> {
        if x >= self.dimensions.x || y >= self.dimensions.y || z >= self.dimensions.z {
            return None;
        }
        let index = x + y * self.dimensions.x + z * self.dimensions.x * self.dimensions.y;
        unsafe {
            Some(&mut *self.grid.as_mut_ptr().add(index))
        }
    }

    /// Updates the alignment state based on coherence
    fn update_state(&mut self) {
        self.state = if self.coherence >= 0.9 {
            AlignState::Perfect
        } else if self.coherence >= QUANTUM_STABILITY_THRESHOLD {
            AlignState::Stable
        } else if self.coherence >= QUANTUM_STABILITY_THRESHOLD * 0.5 {
            AlignState::Unstable
        } else {
            AlignState::Misaligned
        };
    }
}

impl<T: Clone + Default + 'static> Quantum for CrystalAlign<T> {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    fn decohere(&mut self) {
        self.coherence *= 0.9;
        if self.coherence < QUANTUM_STABILITY_THRESHOLD * 0.5 {
            self.coherence = QUANTUM_STABILITY_THRESHOLD * 0.5;
        }
        self.update_state();
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
        self.state = AlignState::Perfect;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_align_basics() {
        let align = CrystalAlign::<u8>::new(2, 2, 2);
        assert_eq!(align.state(), AlignState::Perfect);
        assert!(align.is_stable());
        assert_eq!(align.dimensions(), &Vector3D::new(2, 2, 2));
    }

    #[test]
    fn test_crystal_align_access() {
        let mut align = CrystalAlign::<u8>::new(2, 2, 2);
        assert!(align.get(0, 0, 0).is_some());
        assert!(align.get(2, 2, 2).is_none());

        let zeronaut = align.get_mut(1, 1, 1);
        assert!(zeronaut.is_some());
    }

    #[test]
    fn test_crystal_align_coherence() {
        let mut align = CrystalAlign::<u8>::new(2, 2, 2);
        assert_eq!(align.coherence(), 1.0);

        align.decohere();
        assert!(align.coherence() < 1.0);
        assert_ne!(align.state(), AlignState::Perfect);

        align.recohere();
        assert_eq!(align.coherence(), 1.0);
        assert_eq!(align.state(), AlignState::Perfect);
    }
}
