/// Quantum Helium - Coherent State Management System
/// Last Updated: 2025-01-15 22:22:48 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    vector::Vector3D,
    phantom::QuantumCell,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

/// Quantum ordering for Helium operations
#[derive(Debug, Clone, Copy)]
pub enum HeliumOrdering {
    /// Full quantum coherence maintained
    Quantum,
    /// Partial quantum coherence allowed
    Relaxed,
    /// Strict causality enforcement
    Strict,
}

/// Helium quantum state container
#[derive(Debug, Clone)]
pub struct Helium<T: Clone + 'static> {
    /// Quantum-protected value
    value: QuantumCell<T>,
    /// Spatial coherence vector
    coherence_vector: QuantumCell<Vector3D<f64>>,
    /// Phase alignment
    phase: QuantumCell<f64>,
    /// Position in quantum space
    position: QuantumCell<Vector3D<f64>>,
}

impl<T: Clone + 'static> Helium<T> {
    /// Create new Helium container with quantum protection
    pub fn new(value: T) -> Self {
        Self {
            value: QuantumCell::new(value),
            coherence_vector: QuantumCell::new(Vector3D::new(1.0, 1.0, 1.0)),
            phase: QuantumCell::new(0.0),
            position: QuantumCell::new(Vector3D::new(0.0, 0.0, 0.0)),
        }
    }

    pub fn get(&self) -> T {
        self.value.quantum_load()
    }

    pub fn set(&self, value: T) {
        self.value.quantum_store(value, &HeliumOrdering::Quantum);
    }

    /// Quantum-safe load operation
    pub fn quantum_load(&self) -> T {
        let val = self.value.get();
        self.decay_coherence();
        val
    }

    /// Quantum-safe store operation
    pub fn quantum_store(&self, value: T) {
        self.value.set(value);
        self.decay_coherence();
        self.rotate_phase(QUANTUM_PHASE_ROTATION);
    }

    /// Load with specific quantum ordering
    pub fn load(&self, ordering: &HeliumOrdering) -> Result<T, &'static str> {
        match ordering {
            HeliumOrdering::Quantum => {
                if !self.is_quantum_stable() {
                    return Err("Quantum coherence lost");
                }
                Ok(self.value.get())
            },
            HeliumOrdering::Relaxed => Ok(self.value.get()),
            HeliumOrdering::Strict => {
                if self.get_coherence() < CAUSALITY_PROTECTION_THRESHOLD {
                    return Err("Causality violation detected");
                }
                Ok(self.value.get())
            }
        }
    }

    /// Store with specific quantum ordering
    pub fn store(&self, value: T, ordering: &HeliumOrdering) -> Result<(), &'static str> {
        match ordering {
            HeliumOrdering::Quantum => {
                if !self.is_quantum_stable() {
                    return Err("Quantum coherence lost");
                }
                self.quantum_store(value);
                Ok(())
            },
            HeliumOrdering::Relaxed => {
                self.value.set(value);
                Ok(())
            },
            HeliumOrdering::Strict => {
                if self.get_coherence() < CAUSALITY_PROTECTION_THRESHOLD {
                    return Err("Causality violation detected");
                }
                self.quantum_store(value);
                Ok(())
            }
        }
    }

    /// Get current quantum coherence
    pub fn get_coherence(&self) -> f64 {
        let vec = self.coherence_vector.get();
        (vec.x() + vec.y() + vec.z()) / 3.0
    }

    /// Check quantum stability
    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }

    /// Decay quantum coherence - public for quantum trait implementations
    pub fn decay_coherence(&self) {
        let current = self.coherence_vector.get();
        let decay = Vector3D::new(
            current.x() * COHERENCE_DECAY_FACTOR,
                                  current.y() * COHERENCE_DECAY_FACTOR,
                                  current.z() * COHERENCE_DECAY_FACTOR,
        );
        self.coherence_vector.set(decay);
    }

    /// Rotate quantum phase
    fn rotate_phase(&self, angle: f64) {
        let current = self.phase.get();
        self.phase.set(current + angle);
    }

    /// Reset quantum state
    pub fn reset_coherence(&self) {
        self.coherence_vector.set(Vector3D::new(1.0, 1.0, 1.0));
        self.phase.set(0.0);
    }

    /// Get position in quantum space
    pub fn get_position(&self) -> Vector3D<f64> {
        self.position.get()
    }

    /// Set position in quantum space
    pub fn set_position(&self, pos: Vector3D<f64>) {
        self.position.set(pos);
        self.decay_coherence();
    }
}

impl<T: Scribe + Clone + 'static> Scribe for Helium<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("He⟨");
        self.value.get().scribe(precision, output);
        output.push_str(", c=");
        output.push_f64(self.get_coherence(), 6);
        output.push_str(", φ=");
        output.push_f64(self.phase.get(), precision.decimal_places());
        output.push_str(", pos=");
        self.position.get().scribe(precision, output);
        output.push_char('⟩');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helium_creation() {
        let helium = Helium::new(42);
        assert!(helium.is_quantum_stable());
        assert_eq!(helium.quantum_load(), 42);
    }

    #[test]
    fn test_quantum_operations() {
        let helium = Helium::new(Vector3D::new(1.0, 2.0, 3.0));
        assert!(helium.is_quantum_stable());

        // Perform multiple quantum operations
        for _ in 0..5 {
            let val = helium.quantum_load();
            helium.quantum_store(val);
        }

        assert!(helium.get_coherence() < 1.0);
    }

    #[test]
    fn test_ordering_constraints() {
        let helium = Helium::new(42);

        // Quantum ordering should work initially
        assert!(helium.load(&HeliumOrdering::Quantum).is_ok());

        // Force coherence decay
        for _ in 0..20 {
            helium.decay_coherence();
        }

        // Quantum ordering should fail after coherence loss
        assert!(helium.load(&HeliumOrdering::Quantum).is_err());
        // Relaxed ordering should still work
        assert!(helium.load(&HeliumOrdering::Relaxed).is_ok());
    }

    #[test]
    fn test_position_tracking() {
        let helium = Helium::new(42);
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        helium.set_position(pos.clone());
        assert_eq!(helium.get_position(), pos);
        assert!(helium.get_coherence() < 1.0);
    }

    #[test]
    fn test_quantum_scribing() {
        let helium = Helium::new(42);
        let mut output = QuantumString::new();
        helium.scribe(ScribePrecision::Standard, &mut output);
        assert!(output.as_str().starts_with("He⟨"));
        assert!(output.as_str().contains("c="));
    }
}
