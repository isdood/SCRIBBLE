/// Quantum Helium Module
/// Last Updated: 2025-01-15 04:31:39 UTC
/// Author: isdood
/// Current User: isdood

use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicUsize, Ordering};
use crate::{
    constants::*,
    phantom::{Quantum, QuantumCell},
    scribe::{Scribe, ScribePrecision, QuantumString},
};

#[derive(Debug, Clone, Copy)]
pub enum HeliumOrdering {
    Relaxed,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
    Quantum,
}

/// Core quantum-safe memory type
#[derive(Debug)]
pub struct Helium<T: 'static> {
    value: AtomicPtr<T>,
    coherence: AtomicU64,
    timestamp: AtomicUsize,
}

impl<T: 'static> Helium<T> {
    pub fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));
        Self {
            value: AtomicPtr::new(ptr),
            coherence: AtomicU64::new(f64::to_bits(1.0)),
            timestamp: AtomicUsize::new(CURRENT_TIMESTAMP),
        }
    }

    pub fn load(&self) -> T where T: Copy {
        unsafe {
            *self.value.load(Ordering::Acquire)
        }
    }

    pub fn store(&self, value: T) {
        let new_ptr = Box::into_raw(Box::new(value));
        let old_ptr = self.value.swap(new_ptr, Ordering::AcqRel);
        unsafe {
            drop(Box::from_raw(old_ptr));
        }
    }

    pub fn get_coherence(&self) -> f64 {
        f64::from_bits(self.coherence.load(Ordering::Relaxed))
    }

    pub fn set_coherence(&self, value: f64) {
        self.coherence.store(f64::to_bits(value), Ordering::Relaxed);
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }
}

impl<T: 'static> Quantum for Helium<T> {
    fn get_coherence(&self) -> f64 {
        self.get_coherence()
    }

    fn is_quantum_stable(&self) -> bool {
        self.is_quantum_stable()
    }

    fn decay_coherence(&self) {
        let current = self.get_coherence();
        self.set_coherence(current * COHERENCE_DECAY_FACTOR);
    }

    fn reset_coherence(&self) {
        self.set_coherence(1.0);
    }
}

impl<T: 'static> Scribe for Helium<T> {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("Helium[coherence=");
        output.push_f64(self.get_coherence(), 6);
        output.push_char(']');
    }
}

/// Specialized size-based Helium implementation
#[derive(Debug)]
pub struct HeliumSize {
    value: Helium<usize>,
    coherence: QuantumCell<f64>,
}

impl HeliumSize {
    fn new(value: usize) -> Self {
        Self {
            value: Helium::new(value),
            coherence: QuantumCell::new(1.0),
        }
    }

    fn quantum_load(&self) -> Result<usize, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }
        Ok(self.value.load())
    }

    fn quantum_store(&self, value: usize) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }
        self.value.store(value);
        Ok(())
    }

    fn fetch_add(&self, value: usize, _order: &HeliumOrdering) -> Result<usize, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }
        let current = self.value.load();
        self.value.store(current + value);
        Ok(current)
    }

    fn fetch_sub(&self, value: usize, _order: &HeliumOrdering) -> Result<usize, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }
        let current = self.value.load();
        self.value.store(current - value);
        Ok(current)
    }
}

impl Quantum for HeliumSize {
    fn get_coherence(&self) -> f64 {
        self.coherence.get_coherence()
    }

    fn is_quantum_stable(&self) -> bool {
        self.coherence.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        self.coherence.decay_coherence();
    }

    fn reset_coherence(&self) {
        self.coherence.reset_coherence();
    }
}

impl Scribe for HeliumSize {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("HeliumSize[");
        self.coherence.scribe(precision, output);
        output.push_char(']');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helium_creation() {
        let h = Helium::new(42);
        assert_eq!(h.load(), 42);
        assert!(h.is_quantum_stable());
    }

    #[test]
    fn test_helium_store() {
        let h = Helium::new(42);
        h.store(84);
        assert_eq!(h.load(), 84);
    }

    #[test]
    fn test_helium_coherence() {
        let h = Helium::<i32>::new(42);
        assert!(h.get_coherence() > 0.0);
        h.decay_coherence();
        assert!(h.get_coherence() < 1.0);
    }

    #[test]
    fn test_helium_size() {
        let hs = HeliumSize::new(42);
        assert!(hs.quantum_load().is_ok());
        assert!(hs.quantum_store(84).is_ok());
    }
}
