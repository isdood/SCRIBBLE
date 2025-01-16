/// Quantum Horizon System
/// Last Updated: 2025-01-16 03:34:40 UTC
/// Author: isdood
/// Current User: isdood

use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use crate::{
    constants::{QUANTUM_COHERENCE_THRESHOLD, QUANTUM_STABILITY_THRESHOLD},
    scribe::{Scribe, ScribePrecision, QuantumString},
};

const CURRENT_TIMESTAMP: usize = 1705374880; // 2025-01-16 03:34:40 UTC
const HORIZON_DECAY_RATE: f64 = 0.99;

/// Quantum shared state management system
#[derive(Debug)]
pub struct Horizon<T> {
    state: *mut T,
    coherence: AtomicU64,   // Changed to AtomicU64
    timestamp: AtomicUsize,
    observers: AtomicUsize,
}

unsafe impl<T: Send> Send for Horizon<T> {}
unsafe impl<T: Send> Sync for Horizon<T> {}

impl<T> Horizon<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: Box::into_raw(Box::new(value)),
            coherence: AtomicU64::new(f64::to_bits(1.0)),  // Initialize using to_bits
            timestamp: AtomicUsize::new(CURRENT_TIMESTAMP),
            observers: AtomicUsize::new(0),
        }
    }

    pub fn observe(&self) -> &T {
        unsafe {
            self.decay_coherence();
            self.increment_observers();
            &*self.state
        }
    }

    pub fn observe_mut(&self) -> &mut T {
        unsafe {
            self.decay_coherence();
            self.increment_observers();
            &mut *self.state
        }
    }

    pub fn collapse(&self, value: T) {
        unsafe {
            *self.state = value;
            self.reset_coherence();
        }
    }

    fn decay_coherence(&self) {
        let current = f64::from_bits(self.coherence.load(Ordering::Relaxed));
        self.coherence.store(f64::to_bits(current * HORIZON_DECAY_RATE), Ordering::Relaxed);
        self.timestamp.store(CURRENT_TIMESTAMP, Ordering::SeqCst);
    }

    fn reset_coherence(&self) {
        self.coherence.store(f64::to_bits(1.0), Ordering::Relaxed);
        self.timestamp.store(CURRENT_TIMESTAMP, Ordering::SeqCst);
        self.observers.store(0, Ordering::SeqCst);
    }

    fn increment_observers(&self) {
        self.observers.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_coherence(&self) -> f64 {
        f64::from_bits(self.coherence.load(Ordering::Relaxed))
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    pub fn get_observer_count(&self) -> usize {
        self.observers.load(Ordering::SeqCst)
    }

    pub fn is_coherent(&self) -> bool {
        self.get_coherence() > QUANTUM_COHERENCE_THRESHOLD
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD
    }
}

impl<T> Clone for Horizon<T> where T: Clone {
    fn clone(&self) -> Self {
        Self {
            state: Box::into_raw(Box::new(unsafe { (*self.state).clone() })),
            coherence: AtomicU64::new(self.coherence.load(Ordering::Relaxed)),
            timestamp: AtomicUsize::new(self.timestamp.load(Ordering::SeqCst)),
            observers: AtomicUsize::new(self.observers.load(Ordering::SeqCst)),
        }
    }
}

impl<T> Drop for Horizon<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.state));
        }
    }
}

impl<T: Scribe> Scribe for Horizon<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("Horizon{value=");
        unsafe { (*self.state).scribe(precision, output); }
        output.push_str(", coherence=");
        output.push_f64(self.get_coherence(), precision.decimal_places());
        output.push_str(", observers=");
        output.push_str(&ToString::to_string(&self.observers.load(Ordering::SeqCst)));
        output.push_char('}');
    }
}

/// Integration with QuantumCell
#[derive(Debug)]
pub struct HorizonCell<T> {
    value: Horizon<T>,
    quantum_state: Horizon<bool>,
}

impl<T> HorizonCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Horizon::new(value),
            quantum_state: Horizon::new(true),
        }
    }

    pub fn get(&self) -> &T {
        self.value.observe()
    }

    pub fn get_mut(&self) -> &mut T {
        self.value.observe_mut()
    }

    pub fn set(&self, value: T) {
        self.value.collapse(value);
        self.quantum_state.collapse(self.value.is_coherent());
    }

    pub fn is_quantum_stable(&self) -> bool {
        *self.quantum_state.observe() && self.value.is_coherent()
    }

    pub fn get_coherence(&self) -> f64 {
        self.value.get_coherence()
    }
}

impl<T: Clone> Clone for HorizonCell<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            quantum_state: self.quantum_state.clone(),
        }
    }
}

impl<T: Scribe> Scribe for HorizonCell<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("HorizonCell{");
        self.value.scribe(precision, output);
        output.push_str(", quantum_state=");
        self.quantum_state.scribe(precision, output);
        output.push_char('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizon_basic() {
        let horizon = Horizon::new(42);
        assert_eq!(*horizon.observe(), 42);
        assert!(horizon.is_coherent());
    }

    #[test]
    fn test_horizon_decay() {
        let horizon = Horizon::new(1.0f64);
        let initial = horizon.get_coherence();

        for _ in 0..10 {
            horizon.observe();
        }

        assert!(horizon.get_coherence() < initial);
    }

    #[test]
    fn test_horizon_cell() {
        let cell = HorizonCell::new(42);
        assert_eq!(*cell.get(), 42);
        assert!(cell.is_quantum_stable());

        cell.set(84);
        assert_eq!(*cell.get(), 84);
    }

    #[test]
    fn test_observer_count() {
        let horizon = Horizon::new(1);
        assert_eq!(horizon.get_observer_count(), 0);

        horizon.observe();
        assert_eq!(horizon.get_observer_count(), 1);
    }

    #[test]
    fn test_scribe() {
        let horizon = Horizon::new(42);
        let mut output = QuantumString::new();
        horizon.scribe(ScribePrecision::Standard, &mut output);
        assert!(output.as_str().contains("coherence=1.000000"));
    }
}
