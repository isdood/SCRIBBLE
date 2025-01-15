/// Quantum Phantom Module
/// Last Updated: 2025-01-15 05:09:40 UTC
/// Author: isdood
/// Current User: isdood

use std::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use crate::{
    constants::*,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

pub trait Quantum: Scribe {
    fn get_coherence(&self) -> f64;
    fn is_quantum_stable(&self) -> bool;
    fn decay_coherence(&self);
    fn reset_coherence(&self);
}

#[derive(Debug)]
pub struct QuantumCell<T: 'static> {
    value: AtomicPtr<T>,
    coherence: AtomicU64,
    timestamp: AtomicPtr<usize>,
}

impl<T: 'static> QuantumCell<T> {
    pub fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));
        let timestamp_ptr = Box::into_raw(Box::new(CURRENT_TIMESTAMP));
        Self {
            value: AtomicPtr::new(ptr),
            coherence: AtomicU64::new(f64::to_bits(1.0)),
            timestamp: AtomicPtr::new(timestamp_ptr),
        }
    }

    pub fn get(&self) -> T
    where
    T: Clone,
    {
        unsafe {
            (*self.value.load(Ordering::Acquire)).clone()
        }
    }

    pub fn set(&self, value: T) {
        let new_ptr = Box::into_raw(Box::new(value));
        let old_ptr = self.value.swap(new_ptr, Ordering::AcqRel);
        unsafe {
            drop(Box::from_raw(old_ptr));
        }
        self.update_timestamp();
    }

    pub fn get_coherence(&self) -> f64 {
        f64::from_bits(self.coherence.load(Ordering::Relaxed))
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }

    pub fn decay_coherence(&self) {
        let current = self.get_coherence();
        let new_coherence = current * COHERENCE_DECAY_FACTOR;
        self.coherence.store(f64::to_bits(new_coherence), Ordering::Relaxed);
        self.update_timestamp();
    }

    pub fn reset_coherence(&self) {
        self.coherence.store(f64::to_bits(1.0), Ordering::Relaxed);
        self.update_timestamp();
    }

    fn update_timestamp(&self) {
        unsafe {
            *self.timestamp.load(Ordering::Acquire) = CURRENT_TIMESTAMP;
        }
    }
}

impl<T: 'static> Drop for QuantumCell<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.value.load(Ordering::Acquire)));
            drop(Box::from_raw(self.timestamp.load(Ordering::Acquire)));
        }
    }
}

impl<T: Scribe + Clone + 'static> Scribe for QuantumCell<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("Quantum(");
        self.get().scribe(precision, output);
        output.push_str(", coherence=");
        output.push_f64(self.get_coherence(), 6);
        output.push_char(')');
    }
}

impl<T: Clone + 'static> Quantum for QuantumCell<T> where T: Scribe {
    fn get_coherence(&self) -> f64 {
        self.get_coherence()
    }

    fn is_quantum_stable(&self) -> bool {
        self.is_quantum_stable()
    }

    fn decay_coherence(&self) {
        self.decay_coherence();
    }

    fn reset_coherence(&self) {
        self.reset_coherence();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_cell_basic() {
        let cell = QuantumCell::new(42);
        assert_eq!(cell.get(), 42);
        cell.set(84);
        assert_eq!(cell.get(), 84);
    }

    #[test]
    fn test_quantum_cell_coherence() {
        let cell = QuantumCell::<i32>::new(42);
        assert!(cell.is_quantum_stable());
        cell.decay_coherence();
        assert!(cell.get_coherence() < 1.0);
    }

    #[test]
    fn test_quantum_cell_timestamp() {
        let cell = QuantumCell::new(1);
        let initial_ts = unsafe { *cell.timestamp.load(Ordering::Acquire) };
        cell.set(2);
        let new_ts = unsafe { *cell.timestamp.load(Ordering::Acquire) };
        assert!(new_ts > initial_ts);
    }
}
