//! Phantom Quantum Cell Implementation
//! =================================
//!
//! Provides quantum-safe memory operations through phantom references
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:15:32 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]

use core::{marker::PhantomData, ptr::NonNull};
use crate::{
    constants::*,
    vector::Vector3D,
    scribe::{Scribe, ScribePrecision, QuantumString},
    aether::{Aether, AetherHarmony},
};

/// A quantum-safe cell that uses phantom typing for memory safety
#[derive(Debug)]
pub struct QuantumCell<T: 'static> {
    /// Pointer to the managed value
    ptr: Aether<NonNull<T>>,
    /// Quantum coherence tracking
    coherence: Aether<f64>,
    /// Phantom data to maintain variance
    _phantom: PhantomData<T>,
}

impl<T: 'static> QuantumCell<T> {
    /// Creates a new QuantumCell containing the given value
    pub fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));
        Self {
            ptr: Aether::crystallize(unsafe { NonNull::new_unchecked(ptr) }),
            coherence: Aether::crystallize(1.0),
            _phantom: PhantomData,
        }
    }

    /// Quantum-safe load operation
    pub fn quantum_load(&self, harmony: &AetherHarmony) -> Result<T, &'static str> {
        match harmony {
            AetherHarmony::Prismatic => {
                if !self.is_coherent() {
                    return Err("Quantum decoherence detected");
                }
                self.load()
            },
            AetherHarmony::Amorphous => self.load(),
            AetherHarmony::Crystalline => {
                if self.get_coherence() < QUANTUM_STABILITY_THRESHOLD {
                    return Err("Quantum stability violation");
                }
                self.load()
            }
        }
    }

    /// Quantum-safe store operation
    pub fn quantum_store(&self, value: T, harmony: &AetherHarmony) -> Result<(), &'static str> {
        match harmony {
            AetherHarmony::Prismatic => {
                if !self.is_coherent() {
                    return Err("Quantum decoherence detected");
                }
                self.store(value)
            },
            AetherHarmony::Amorphous => self.store(value),
            AetherHarmony::Crystalline => {
                if self.get_coherence() < QUANTUM_STABILITY_THRESHOLD {
                    return Err("Quantum stability violation");
                }
                self.store(value)
            }
        }
    }

    /// Gets the current coherence value
    pub fn get_coherence(&self) -> f64 {
        self.coherence.glimpse().unwrap_or(0.0)
    }

    /// Checks if the cell is quantum coherent
    pub fn is_coherent(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }

    /// Load the current value
    fn load(&self) -> Result<T, &'static str> {
        if let Ok(ptr) = self.ptr.glimpse() {
            Ok(unsafe { ptr.as_ref() }.clone())
        } else {
            Err("Failed to load quantum state")
        }
    }

    /// Store a new value
    fn store(&self, value: T) -> Result<(), &'static str> {
        if let Ok(ptr) = self.ptr.glimpse() {
            unsafe {
                *ptr.as_ptr() = value;
            }
            self.decay_coherence();
            Ok(())
        } else {
            Err("Failed to store quantum state")
        }
    }

    /// Natural coherence decay
    fn decay_coherence(&self) {
        if let Ok(current) = self.coherence.glimpse() {
            let _ = self.coherence.encode(current * COHERENCE_DECAY_FACTOR);
        }
    }
}

impl<T: 'static> Drop for QuantumCell<T> {
    fn drop(&mut self) {
        if let Ok(ptr) = self.ptr.glimpse() {
            unsafe {
                Box::from_raw(ptr.as_ptr());
            }
        }
    }
}

impl<T: Clone + Scribe> Scribe for QuantumCell<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨");
        if let Ok(value) = self.quantum_load(&AetherHarmony::Amorphous) {
            value.scribe(precision, output);
        } else {
            output.push_str("∅");
        }
        output.push_str(", χ=");
        output.push_f64(self.get_coherence(), precision.decimal_places());
        output.push_str("⟩");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_cell() {
        let cell = QuantumCell::new(42);
        assert_eq!(cell.quantum_load(&AetherHarmony::Prismatic).unwrap(), 42);
        assert!(cell.is_coherent());
    }

    #[test]
    fn test_quantum_decoherence() {
        let cell = QuantumCell::new(Vector3D::new(1.0, 0.0, 0.0));

        // Force decoherence through multiple stores
        for i in 0..10 {
            let _ = cell.quantum_store(
                Vector3D::new(i as f64, 0.0, 0.0),
                                       &AetherHarmony::Amorphous
            );
        }

        assert!(cell.get_coherence() < 1.0);
    }

    #[test]
    fn test_quantum_stability() {
        let cell = QuantumCell::new(42);
        assert!(cell.quantum_store(43, &AetherHarmony::Crystalline).is_ok());
        assert!(cell.get_coherence() > QUANTUM_STABILITY_THRESHOLD);
    }
}
