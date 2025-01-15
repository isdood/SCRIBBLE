/// Quantum Cell Implementation
/// Last Updated: 2025-01-15 05:44:49 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    quantum::Quantum,
    scribe::{Scribe, ScribePrecision, QuantumString},
    helium::Helium,
    constants::*,
};

#[derive(Debug)]
pub struct QCell<T> {
    value: Helium<T>,
    coherence: Helium<f64>,
    state: Helium<bool>,
}

impl<T> QCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Helium::new(value),
            coherence: Helium::new(1.0),
            state: Helium::new(true),
        }
    }

    pub fn get(&self) -> T
    where T: Clone {
        self.value.quantum_load()
    }

    pub fn set(&self, value: T) {
        self.value.quantum_store(value);
        self.decay_coherence();
    }

    pub fn map<F, U>(&self, f: F) -> QCell<U>
    where
    F: Fn(&T) -> U,
    T: Clone,
    U: Clone,
    {
        let value = f(&self.get());
        let mut cell = QCell::new(value);
        cell.coherence.quantum_store(self.coherence.quantum_load());
        cell.state.quantum_store(self.state.quantum_load());
        cell
    }

    pub fn zip<U>(&self, other: &QCell<U>) -> QCell<(T, U)>
    where
    T: Clone,
    U: Clone,
    {
        let value = (self.get(), other.get());
        let mut cell = QCell::new(value);
        let coherence = self.coherence.quantum_load().min(other.coherence.quantum_load());
        cell.coherence.quantum_store(coherence);
        cell.state.quantum_store(self.state.quantum_load() && other.state.quantum_load());
        cell
    }
}

impl<T: Clone> Clone for QCell<T> {
    fn clone(&self) -> Self {
        Self {
            value: Helium::new(self.get()),
            coherence: Helium::new(self.coherence.quantum_load()),
            state: Helium::new(self.state.quantum_load()),
        }
    }
}

impl<T: PartialEq> PartialEq for QCell<T>
where T: Clone {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get() &&
        (self.coherence.quantum_load() - other.coherence.quantum_load()).abs() < QUANTUM_EPSILON
    }
}

impl<T: Scribe> Scribe for QCell<T>
where T: Clone {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("QCell{");
        self.get().scribe(precision, output);
        output.push_str(", coherence=");
        output.push_f64(self.coherence.quantum_load(), precision.decimal_places());
        output.push_str(", stable=");
        output.push_str(if self.state.quantum_load() { "true" } else { "false" });
        output.push_char('}');
    }
}

impl<T> Quantum for QCell<T> {
    fn get_coherence(&self) -> f64 {
        self.coherence.quantum_load()
    }

    fn is_quantum_stable(&self) -> bool {
        self.state.quantum_load() &&
        self.coherence.quantum_load() > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        let current = self.coherence.quantum_load();
        let new = current * 0.9;
        self.coherence.quantum_store(new);
        self.state.quantum_store(new > QUANTUM_STABILITY_THRESHOLD);
    }

    fn reset_coherence(&self) {
        self.coherence.quantum_store(1.0);
        self.state.quantum_store(true);
    }
}

/// Vector space operations
impl<T> QCell<T>
where T: Clone {
    pub fn quantum_distance(&self, other: &Self) -> f64
    where T: QuantumMetric {
        let d = T::quantum_distance(&self.get(), &other.get());
        d * (self.get_coherence().min(other.get_coherence()))
    }
}

/// Trait for types that support quantum distance metrics
pub trait QuantumMetric {
    fn quantum_distance(&self, other: &Self) -> f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qcell_basic() {
        let cell = QCell::new(42);
        assert_eq!(cell.get(), 42);
        assert!(cell.is_quantum_stable());
    }

    #[test]
    fn test_qcell_map() {
        let cell = QCell::new(42);
        let mapped = cell.map(|x| x * 2);
        assert_eq!(mapped.get(), 84);
        assert_eq!(mapped.get_coherence(), cell.get_coherence());
    }

    #[test]
    fn test_qcell_zip() {
        let a = QCell::new(1);
        let b = QCell::new("test");
        let zipped = a.zip(&b);
        assert_eq!(zipped.get(), (1, "test"));
    }

    #[test]
    fn test_quantum_mechanics() {
        let cell = QCell::new(42);
        let initial = cell.get_coherence();
        cell.decay_coherence();
        assert!(cell.get_coherence() < initial);
    }

    #[test]
    fn test_vector_operations() {
        #[derive(Clone, PartialEq, Debug)]
        struct TestVector(f64);

        impl QuantumMetric for TestVector {
            fn quantum_distance(&self, other: &Self) -> f64 {
                (self.0 - other.0).abs()
            }
        }

        let a = QCell::new(TestVector(1.0));
        let b = QCell::new(TestVector(2.0));
        let distance = a.quantum_distance(&b);
        assert!(distance <= 1.0);
    }
}
