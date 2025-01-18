//! Harmony Core Library
//! ===================
//!
//! A quantum-harmonic memory management system for the Scribble kernel.
//!
//! This module provides core functionality for managing quantum-harmonic memory cells,
//! including coherence tracking and harmonic memory protection.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:14:23 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]

// Core modules
mod aether;
mod cube;
mod phantom;
mod constants;
mod vector;
mod scribe;
mod zeronaut;
mod align;
mod harmony;

// Re-export commonly used types and traits
pub use {
    aether::{Aether, AetherHarmony},
    cube::Box,
    vector::Vector3D,
    scribe::{Scribe, ScribePrecision, QuantumString},
    harmony::Quantum,
    align::{Alignment, AlignedSpace},
    constants::{
        CURRENT_TIMESTAMP,
        QUANTUM_STABILITY_THRESHOLD,
        COHERENCE_DECAY_FACTOR,
        PLANCK_LENGTH,
        VECTOR_ALIGN,
        VECTOR_QUANTUM_STATE,
    },
};

/// A thread-safe container for harmonic memory operations
#[derive(Debug)]
pub struct HarmonicCell<T: Clone + 'static> {
    /// Aether-managed value
    essence: Aether<T>,
    /// Memory cube for aligned storage
    cube: Box<T>,
    /// Quantum coherence tracking
    harmony: Aether<f64>,
}

impl<T: Clone + 'static> HarmonicCell<T> {
    /// Creates a new HarmonicCell with the given value
    pub fn new(value: T) -> Self {
        Self {
            essence: Aether::crystallize(value.clone()),
            cube: Box::new(value),
            harmony: Aether::crystallize(1.0),
        }
    }

    /// Retrieves the current value
    pub fn get(&self) -> T {
        self.essence.glimpse().unwrap_or_else(|_| self.cube.as_ref().clone())
    }

    /// Updates the stored value
    pub fn set(&self, value: T) {
        let _ = self.essence.encode(value.clone());
        *self.cube.as_mut() = value;
    }

    /// Gets the current coherence value
    pub fn get_coherence(&self) -> f64 {
        let aether_coherence = self.essence.get_resonance();
        let cube_coherence = self.cube.get_coherence();
        let harmony_coherence = self.harmony.glimpse().unwrap_or(1.0);
        (aether_coherence + cube_coherence + harmony_coherence) / 3.0
    }

    /// Checks if the cell is harmonically stable
    pub fn is_harmonically_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD &&
        self.cube.is_quantum_stable()
    }

    /// Causes natural coherence decay
    pub fn decay_coherence(&mut self) {
        self.essence.diminish_resonance();
        self.cube.decay_coherence();
        if let Ok(current) = self.harmony.glimpse() {
            let _ = self.harmony.encode(current * COHERENCE_DECAY_FACTOR);
        }
    }

    /// Restores harmonic coherence
    pub fn restore_harmony(&mut self) {
        self.essence.restore_harmony();
        self.cube.reset_coherence();
        let _ = self.harmony.encode(1.0);
    }
}

/// Trait for protected memory operations
pub trait Protected {
    /// Enables memory protection
    fn protect(&self) -> bool;

    /// Disables memory protection
    fn unprotect(&self) -> bool;

    /// Gets the current coherence value
    fn get_coherence(&self) -> f64;

    /// Checks if the memory is harmonically stable
    fn is_harmonically_stable(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_cell() {
        let cell = HarmonicCell::new(42);
        assert_eq!(cell.get(), 42);
        assert!(cell.get_coherence() > 0.0);
        assert!(cell.is_harmonically_stable());
    }

    #[test]
    fn test_coherence_decay() {
        let mut cell = HarmonicCell::new(42);
        let initial_coherence = cell.get_coherence();

        for _ in 0..5 {
            cell.decay_coherence();
        }

        assert!(cell.get_coherence() < initial_coherence);
    }

    #[test]
    fn test_harmony_restoration() {
        let mut cell = HarmonicCell::new(42);

        for _ in 0..5 {
            cell.decay_coherence();
        }

        let decayed_coherence = cell.get_coherence();
        cell.restore_harmony();
        assert!(cell.get_coherence() > decayed_coherence);
    }
}
