/// Quantum State Management Module
/// Last Updated: 2025-01-18 18:54:21 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    vector::Vector3D,
    aether::{AetherCell, AetherOrdering},
    meshmath::MeshValue,
};

/// Core trait for quantum-aware types
pub trait Quantum {
    /// Get quantum coherence level (0.0 to 1.0)
    fn get_coherence(&self) -> f64;

    /// Check quantum stability state
    fn is_quantum_stable(&self) -> bool;

    /// Apply quantum decoherence effects
    fn decay_coherence(&self);

    /// Reset quantum coherence to pristine state
    fn reset_coherence(&self);
}

/// Quantum particle state representation
#[derive(Debug)]
pub struct QuantumState {
    /// Spatial position in quantum mesh
    position: AetherCell<Vector3D<isize>>,

    /// Quantum coherence level
    coherence: AetherCell<f64>,

    /// Quantum stability indicator
    stability: AetherCell<bool>
}

impl QuantumState {
    /// Create new quantum state
    #[inline]
    pub fn new() -> Self {
        Self {
            position: AetherCell::new(Vector3D::new(0, 0, 0)),
            coherence: AetherCell::new(1.0),
            stability: AetherCell::new(true)
        }
    }

    /// Get current position
    #[inline]
    pub fn get_position(&self) -> Result<Vector3D<isize>, &'static str> {
        self.position.load(&AetherOrdering::Quantum)
    }

    /// Set new position
    #[inline]
    pub fn set_position(&mut self, pos: Vector3D<isize>) -> Result<(), &'static str> {
        self.position.store(pos, &AetherOrdering::Quantum)
    }

    /// Check if position matches target
    #[inline]
    pub fn is_at_position(&self, target: &Vector3D<isize>) -> Result<bool, &'static str> {
        Ok(self.get_position()? == *target)
    }

    /// Get distance to target
    #[inline]
    pub fn distance_to(&self, target: &Vector3D<isize>) -> Result<f64, &'static str> {
        Ok(self.get_position()?.quantum_distance(target))
    }

    /// Move towards target
    #[inline]
    pub fn move_towards(&mut self, target: &Vector3D<isize>) -> Result<(), &'static str> {
        let current = self.get_position()?;
        let diff = target.mesh_sub(&current);
        let new_pos = current.mesh_add(&diff);
        self.set_position(new_pos)
    }
}

impl Default for QuantumState {
    fn default() -> Self {
        Self::new()
    }
}

impl Quantum for QuantumState {
    #[inline]
    fn get_coherence(&self) -> f64 {
        self.coherence.load(&AetherOrdering::Quantum).unwrap_or(0.0)
    }

    #[inline]
    fn is_quantum_stable(&self) -> bool {
        self.stability.load(&AetherOrdering::Quantum).unwrap_or(false)
    }

    #[inline]
    fn decay_coherence(&self) {
        if let Ok(current) = self.coherence.load(&AetherOrdering::Quantum) {
            let _ = self.coherence.store(current * 0.99, &AetherOrdering::Quantum);
        }
    }

    #[inline]
    fn reset_coherence(&self) {
        let _ = self.coherence.store(1.0, &AetherOrdering::Quantum);
    }
}

/// Helper functions for quantum operations
pub mod quantum_ops {
    use super::*;

    /// Entangle two quantum states
    #[inline]
    pub fn entangle(a: &mut QuantumState, b: &mut QuantumState) -> Result<(), &'static str> {
        let coherence = (a.get_coherence() + b.get_coherence()) / 2.0;
        a.coherence.store(coherence, &AetherOrdering::Quantum)?;
        b.coherence.store(coherence, &AetherOrdering::Quantum)?;
        Ok(())
    }

    /// Measure quantum state (causes decoherence)
    #[inline]
    pub fn measure(state: &QuantumState) -> Result<f64, &'static str> {
        let coherence = state.get_coherence();
        state.coherence.store(coherence * 0.9, &AetherOrdering::Quantum)?;
        Ok(coherence)
    }

    /// Apply quantum tunneling effect
    #[inline]
    pub fn tunnel(state: &mut QuantumState, target: &Vector3D<isize>) -> Result<(), &'static str> {
        if state.get_coherence() > 0.5 {
            state.set_position(*target)?;
            state.coherence.store(state.get_coherence() * 0.8, &AetherOrdering::Quantum)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_state_creation() {
        let state = QuantumState::new();
        assert!(state.is_quantum_stable());
        assert!(state.get_coherence() > 0.99);
    }

    #[test]
    fn test_quantum_position() {
        let mut state = QuantumState::new();
        let pos = Vector3D::new(1, 2, 3);
        state.set_position(pos).unwrap();
        assert_eq!(state.get_position().unwrap(), pos);
    }

    #[test]
    fn test_quantum_decoherence() {
        let state = QuantumState::new();
        let initial = state.get_coherence();
        state.decay_coherence();
        assert!(state.get_coherence() < initial);
    }

    #[test]
    fn test_quantum_entanglement() {
        let mut a = QuantumState::new();
        let mut b = QuantumState::new();
        quantum_ops::entangle(&mut a, &mut b).unwrap();
        assert_eq!(a.get_coherence(), b.get_coherence());
    }
}
