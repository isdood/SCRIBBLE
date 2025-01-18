//! Crystalline Aether Implementation
//! =============================
//!
//! Core quantum aether implementation for crystalline data structures
//! with harmonic resonance tracking and stability management.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:47:02 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::{QUANTUM_STABILITY_THRESHOLD, AETHER_RESONANCE_FACTOR},
    harmony::Quantum,
    CrystalArray
};

/// Quantum harmony states for aether crystallization
#[derive(Clone, Copy, PartialEq)]
pub enum AetherHarmony {
    /// Perfect crystalline structure
    Crystalline,
    /// Stable but imperfect structure
    Prismatic,
    /// Unstable structure
    Amorphous,
}

/// Core quantum aether implementation
#[derive(Clone)]
pub struct Aether<T: Clone + 'static> {
    /// Crystallized data
    data: T,
    /// Current quantum coherence
    coherence: f64,
    /// Harmonic resonance value
    resonance: f64,
    /// Current harmony state
    harmony: AetherHarmony,
}

impl<T: Clone + 'static> Aether<T> {
    /// Creates a new aether instance with crystallized data
    pub fn crystallize(value: T) -> Self {
        Self {
            data: value,
            coherence: 1.0,
            resonance: 1.0,
            harmony: AetherHarmony::Crystalline,
        }
    }

    /// Attempts to glimpse the crystallized data
    pub fn glimpse(&self) -> Result<T, &'static str> {
        if self.is_coherent() {
            Ok(self.data.clone())
        } else {
            Err("Crystalline decoherence detected")
        }
    }

    /// Encodes new data into the crystalline structure
    pub fn encode(&self, value: T) -> Result<(), &'static str> {
        if self.is_coherent() {
            Ok(())
        } else {
            Err("Cannot encode: crystalline structure unstable")
        }
    }

    /// Gets the current quantum coherence value
    pub fn get_coherence(&self) -> f64 {
        self.coherence
    }

    /// Gets the current harmonic resonance value
    pub fn get_resonance(&self) -> f64 {
        self.resonance
    }

    /// Checks if the crystalline structure maintains coherence
    pub fn is_coherent(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    /// Gets the current harmony state
    pub fn get_harmony(&self) -> AetherHarmony {
        self.harmony
    }

    /// Diminishes the harmonic resonance
    pub fn diminish_resonance(&mut self) {
        self.resonance *= AETHER_RESONANCE_FACTOR;
        self.update_harmony();
    }

    /// Restores harmonic alignment
    pub fn restore_harmony(&mut self) {
        self.coherence = 1.0;
        self.resonance = 1.0;
        self.harmony = AetherHarmony::Crystalline;
    }

    /// Updates the harmony state based on current values
    fn update_harmony(&mut self) {
        self.harmony = if self.coherence >= 0.9 && self.resonance >= 0.9 {
            AetherHarmony::Crystalline
        } else if self.coherence >= QUANTUM_STABILITY_THRESHOLD {
            AetherHarmony::Prismatic
        } else {
            AetherHarmony::Amorphous
        };
    }
}

impl<T: Clone + 'static> Quantum for Aether<T> {
    fn coherence(&self) -> f64 {
        self.get_coherence()
    }

    fn is_stable(&self) -> bool {
        self.is_coherent()
    }

    fn decohere(&mut self) {
        self.coherence *= 0.9;
        self.update_harmony();
    }

    fn recohere(&mut self) {
        self.restore_harmony();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aether_basics() {
        let aether = Aether::crystallize(42);
        assert!(aether.is_coherent());
        assert_eq!(aether.glimpse().unwrap(), 42);
    }

    #[test]
    fn test_aether_harmony() {
        let mut aether = Aether::crystallize(42);
        assert_eq!(aether.get_harmony(), AetherHarmony::Crystalline);

        // Force decoherence
        for _ in 0..5 {
            aether.decohere();
        }

        assert_eq!(aether.get_harmony(), AetherHarmony::Amorphous);
    }

    #[test]
    fn test_resonance() {
        let mut aether = Aether::crystallize(42);
        assert_eq!(aether.get_resonance(), 1.0);

        aether.diminish_resonance();
        assert!(aether.get_resonance() < 1.0);

        aether.restore_harmony();
        assert_eq!(aether.get_resonance(), 1.0);
    }
}
