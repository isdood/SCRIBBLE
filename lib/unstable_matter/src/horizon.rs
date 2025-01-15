/// Quantum Horizon System
/// Last Updated: 2025-01-15 04:53:33 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::{QUANTUM_COHERENCE_THRESHOLD, QUANTUM_STABILITY_THRESHOLD},
    scribe::{Scribe, ScribePrecision, QuantumString},
};

const CURRENT_TIMESTAMP: usize = 1705287213; // 2025-01-15 04:53:33 UTC
const HORIZON_DECAY_RATE: f64 = 0.99;

/// Quantum shared state management system
#[derive(Debug, Clone)]
pub struct Horizon<T> {
    state: *mut T,
    coherence: f64,
    timestamp: usize,
    observers: usize,
}

unsafe impl<T: Send> Send for Horizon<T> {}
unsafe impl<T: Send> Sync for Horizon<T> {}

impl<T> Horizon<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: Box::into_raw(Box::new(value)),
            coherence: 1.0,
            timestamp: CURRENT_TIMESTAMP,
            observers: 0,
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
        unsafe {
            let coherence_ptr = &self.coherence as *const f64 as *mut f64;
            *coherence_ptr *= HORIZON_DECAY_RATE;

            let timestamp_ptr = &self.timestamp as *const usize as *mut usize;
            *timestamp_ptr = CURRENT_TIMESTAMP;
        }
    }

    fn reset_coherence(&self) {
        unsafe {
            let coherence_ptr = &self.coherence as *const f64 as *mut f64;
            *coherence_ptr = 1.0;

            let timestamp_ptr = &self.timestamp as *const usize as *mut usize;
            *timestamp_ptr = CURRENT_TIMESTAMP;

            let observers_ptr = &self.observers as *const usize as *mut usize;
            *observers_ptr = 0;
        }
    }

    fn increment_observers(&self) {
        unsafe {
            let observers_ptr = &self.observers as *const usize as *mut usize;
            *observers_ptr = observers_ptr.wrapping_add(1);
        }
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp
    }

    pub fn get_observer_count(&self) -> usize {
        self.observers
    }

    pub fn is_coherent(&self) -> bool {
        self.coherence > QUANTUM_COHERENCE_THRESHOLD
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.coherence > QUANTUM_STABILITY_THRESHOLD
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
        output.push_f64(self.coherence, precision.decimal_places());
        output.push_str(", observers=");
        output.push_usize(self.observers);
        output.push_char('}');
    }
}

// Integration with QuantumCell
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
