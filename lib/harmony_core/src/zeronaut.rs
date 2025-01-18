//! Crystalline Zeronaut Implementation
//! ===============================
//!
//! Core zero-point quantum navigation through crystalline
//! lattice structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:48:56 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::QUANTUM_STABILITY_THRESHOLD,
    harmony::{Quantum, MeshValue},
    vector::Vector3D,
    CrystalArray,
    CrystalCube,
};

/// Quantum navigation states
#[derive(Clone, Copy, PartialEq)]
pub enum ZeronautState {
    /// Perfect quantum alignment
    Aligned,
    /// Stable but imperfect alignment
    Partial,
    /// Complete misalignment
    Unaligned,
}

/// Zero-point quantum navigator
#[derive(Clone)]
pub struct Zeronaut<T: Clone + 'static> {
    /// Quantum essence data
    essence: CrystalCube<T>,
    /// Current position in quantum space
    position: Vector3D<f64>,
    /// Navigation state
    state: ZeronautState,
    /// Quantum coherence value
    coherence: f64,
}

impl<T: Clone + 'static> Zeronaut<T> {
    /// Creates a new zeronaut at quantum origin
    pub fn new() -> Self where T: Default {
        Self {
            essence: CrystalCube::new(T::default()),
            position: Vector3D::zero(),
            state: ZeronautState::Aligned,
            coherence: 1.0,
        }
    }

    /// Creates a new zeronaut at specific coordinates
    pub fn new_positioned(value: T, x: f64, y: f64, z: f64) -> Self {
        Self {
            essence: CrystalCube::new_positioned(value, x, y, z),
            position: Vector3D::new(x, y, z),
            state: ZeronautState::Aligned,
            coherence: 1.0,
        }
    }

    /// Gets the current position in quantum space
    pub fn position(&self) -> &Vector3D<f64> {
        &self.position
    }

    /// Sets the position in quantum space
    pub fn set_position(&mut self, pos: Vector3D<f64>) {
        self.position = pos;
        self.essence.set_position(pos);
        self.decohere();
    }

    /// Gets the current navigation state
    pub fn state(&self) -> ZeronautState {
        self.state
    }

    /// Gets a reference to the quantum essence
    pub fn essence(&self) -> &T {
        self.essence.get()
    }

    /// Gets a mutable reference to the quantum essence
    pub fn essence_mut(&mut self) -> &mut T {
        self.essence.get_mut()
    }

    /// Sets the quantum essence
    pub fn set_essence(&mut self, value: T) {
        self.essence.set(value);
        self.decohere();
    }

    /// Updates the navigation state based on coherence
    fn update_state(&mut self) {
        self.state = if self.coherence >= 0.9 {
            ZeronautState::Aligned
        } else if self.coherence >= QUANTUM_STABILITY_THRESHOLD {
            ZeronautState::Partial
        } else {
            ZeronautState::Unaligned
        };
    }
}

impl<T: Clone + 'static> Quantum for Zeronaut<T> {
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
        }
        self.update_state();
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
        self.state = ZeronautState::Aligned;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeronaut_basics() {
        let mut zeronaut = Zeronaut::<u8>::new();
        assert_eq!(zeronaut.state(), ZeronautState::Aligned);
        assert!(zeronaut.is_stable());
    }

    #[test]
    fn test_zeronaut_positioning() {
        let mut zeronaut = Zeronaut::new_positioned(42u8, 1.0, 2.0, 3.0);
        assert_eq!(zeronaut.position().x, 1.0);
        assert_eq!(zeronaut.position().y, 2.0);
        assert_eq!(zeronaut.position().z, 3.0);

        zeronaut.set_position(Vector3D::zero());
        assert_eq!(*zeronaut.position(), Vector3D::zero());
        assert!(zeronaut.coherence() < 1.0);
    }

    #[test]
    fn test_essence_management() {
        let mut zeronaut = Zeronaut::new_positioned(42u8, 0.0, 0.0, 0.0);
        assert_eq!(*zeronaut.essence(), 42u8);

        zeronaut.set_essence(43u8);
        assert_eq!(*zeronaut.essence(), 43u8);
        assert!(zeronaut.coherence() < 1.0);
    }

    #[test]
    fn test_state_transitions() {
        let mut zeronaut = Zeronaut::<u8>::new();
        assert_eq!(zeronaut.state(), ZeronautState::Aligned);

        // Force decoherence
        for _ in 0..5 {
            zeronaut.decohere();
        }

        assert_eq!(zeronaut.state(), ZeronautState::Unaligned);

        zeronaut.recohere();
        assert_eq!(zeronaut.state(), ZeronautState::Aligned);
    }
}
