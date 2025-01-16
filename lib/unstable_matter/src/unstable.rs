/// Quantum State Descriptor for 3D Space
/// Last Updated: 2025-01-16 02:22:39 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    vector::Vector3D,
    phantom::QuantumCell,
    quantum::Quantum,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantumState {
    Stable,
    Entangled,
    Decoherent,
    Superposed,
}

/// Three-dimensional quantum state descriptor
#[derive(Debug, Clone)]
pub struct UnstableDescriptor {
    /// Spatial position in quantum space
    position: QuantumCell<Vector3D<f64>>,

    /// Quantum momentum vector
    momentum: QuantumCell<Vector3D<f64>>,

    /// Quantum phase angle
    phase: QuantumCell<f64>,

    /// Quantum coherence value
    coherence: QuantumCell<f64>,

    /// Current quantum state
    state: QuantumCell<QuantumState>,

    /// Spatial uncertainty
    uncertainty: QuantumCell<Vector3D<f64>>,
}

impl UnstableDescriptor {
    pub fn new() -> Self {
        Self {
            position: QuantumCell::new(Vector3D::new(0.0, 0.0, 0.0)),
            momentum: QuantumCell::new(Vector3D::new(0.0, 0.0, 0.0)),
            phase: QuantumCell::new(0.0),
            coherence: QuantumCell::new(1.0),
            state: QuantumCell::new(QuantumState::Stable),
            uncertainty: QuantumCell::new(Vector3D::new(PLANCK_LENGTH, PLANCK_LENGTH, PLANCK_LENGTH)),
        }
    }

    /// Get current 3D position
    pub fn position(&self) -> Vector3D<f64> {
        self.position.get().clone()
    }

    /// Set new 3D position with quantum effects
    pub fn set_position(&self, pos: Vector3D<f64>) {
        self.position.set(pos);
        self.increase_uncertainty();
        self.decay_coherence();
    }

    /// Get quantum momentum vector
    pub fn momentum(&self) -> Vector3D<f64> {
        self.momentum.get().clone()
    }

    /// Update momentum with quantum effects
    pub fn apply_momentum(&self, force: Vector3D<f64>) {
        let current = self.momentum.get().clone();
        self.momentum.set(current + force);
        self.increase_uncertainty();
    }

    /// Get current quantum phase
    pub fn phase(&self) -> f64 {
        self.phase.get()
    }

    /// Rotate quantum phase
    pub fn rotate_phase(&self, angle: f64) {
        let current = self.phase.get();
        self.phase.set(current + angle);
        self.decay_coherence();
    }

    /// Get current coherence value
    pub fn coherence(&self) -> f64 {
        self.coherence.get()
    }

    /// Check if quantum state is stable
    pub fn is_stable(&self) -> bool {
        self.coherence() > QUANTUM_STABILITY_THRESHOLD &&
        self.state.get() == QuantumState::Stable
    }

    /// Get current uncertainty vector
    pub fn uncertainty(&self) -> Vector3D<f64> {
        self.uncertainty.get().clone()
    }

    /// Increase spatial uncertainty
    fn increase_uncertainty(&self) {
        let current = self.uncertainty.get().clone();
        let factor = 1.0 + (1.0 - self.coherence());
        self.uncertainty.set(current * factor);
    }

    /// Decay quantum coherence
    fn decay_coherence(&self) {
        let current = self.coherence.get();
        let new_coherence = current * COHERENCE_DECAY_FACTOR;
        self.coherence.set(new_coherence);

        // Update quantum state based on coherence
        let new_state = match new_coherence {
            c if c > 0.9 => QuantumState::Stable,
            c if c > 0.7 => QuantumState::Superposed,
            c if c > QUANTUM_STABILITY_THRESHOLD => QuantumState::Entangled,
            _ => QuantumState::Decoherent,
        };

        self.state.set(new_state);
    }

    /// Reset quantum state to initial conditions
    pub fn reset(&mut self) {
        self.position.set(Vector3D::new(0.0, 0.0, 0.0));
        self.momentum.set(Vector3D::new(0.0, 0.0, 0.0));
        self.phase.set(0.0);
        self.coherence.set(1.0);
        self.state.set(QuantumState::Stable);
        self.uncertainty.set(Vector3D::new(PLANCK_LENGTH, PLANCK_LENGTH, PLANCK_LENGTH));
    }

    /// Get current state type
    pub fn get_state(&self) -> QuantumState {
        self.state.get()
    }
}

impl Quantum for UnstableDescriptor {
    fn get_coherence(&self) -> f64 {
        self.coherence()
    }

    fn is_quantum_stable(&self) -> bool {
        self.is_stable()
    }

    fn decay_coherence(&self) {
        let current = self.coherence.get();
        let new_coherence = current * COHERENCE_DECAY_FACTOR;
        self.coherence.set(new_coherence);

        let new_state = match new_coherence {
            c if c > 0.9 => QuantumState::Stable,
            c if c > 0.7 => QuantumState::Superposed,
            c if c > QUANTUM_STABILITY_THRESHOLD => QuantumState::Entangled,
            _ => QuantumState::Decoherent,
        };

        self.state.set(new_state);
    }

    fn reset_coherence(&self) {
        self.coherence.set(1.0);
        self.state.set(QuantumState::Stable);
    }
}

impl Scribe for UnstableDescriptor {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("Quantum[pos=");
        self.position.get().scribe(precision, output);
        output.push_str(", p=");
        self.momentum.get().scribe(precision, output);
        output.push_str(", φ=");
        output.push_f64(self.phase(), precision.decimal_places());
        output.push_str(", c=");
        output.push_f64(self.coherence(), 6);
        output.push_str(", s=");
        match self.get_state() {
            QuantumState::Stable => output.push_str("stable"),
            QuantumState::Entangled => output.push_str("entangled"),
            QuantumState::Decoherent => output.push_str("decoherent"),
            QuantumState::Superposed => output.push_str("superposed"),
        }
        output.push_str(", Δx=");
        self.uncertainty.get().scribe(precision, output);
        output.push_char(']');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptor_creation() {
        let desc = UnstableDescriptor::new();
        assert!(desc.is_stable());
        assert_eq!(desc.coherence(), 1.0);
        assert_eq!(desc.get_state(), QuantumState::Stable);
    }

    #[test]
    fn test_position_update() {
        let desc = UnstableDescriptor::new();
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        desc.set_position(pos.clone());
        assert_eq!(desc.position(), pos);
        assert!(desc.coherence() < 1.0);
    }

    #[test]
    fn test_momentum() {
        let desc = UnstableDescriptor::new();
        let force = Vector3D::new(0.1, 0.2, 0.3);
        desc.apply_momentum(force.clone());
        assert_eq!(desc.momentum(), force);
    }

    #[test]
    fn test_uncertainty_growth() {
        let desc = UnstableDescriptor::new();
        let initial = desc.uncertainty();

        // Force multiple updates
        for _ in 0..10 {
            desc.set_position(Vector3D::new(1.0, 1.0, 1.0));
        }

        assert!(desc.uncertainty().magnitude() > initial.magnitude());
    }

    #[test]
    fn test_state_transitions() {
        let desc = UnstableDescriptor::new();
        assert_eq!(desc.get_state(), QuantumState::Stable);

        // Force state changes through multiple position updates
        for _ in 0..20 {
            desc.set_position(Vector3D::new(1.0, 1.0, 1.0));
        }

        assert_ne!(desc.get_state(), QuantumState::Stable);
    }

    #[test]
    fn test_quantum_scribing() {
        let desc = UnstableDescriptor::new();
        let mut output = QuantumString::new();
        desc.scribe(ScribePrecision::Standard, &mut output);
        assert!(output.as_str().starts_with("Quantum[pos="));
        assert!(output.as_str().contains("s=stable"));
    }
}
