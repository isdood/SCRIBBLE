/// Quantum Helium Module - Atomic operations with quantum coherence
/// Last Updated: 2025-01-14 21:13:09 UTC
/// Author: isdood
/// Current User: isdood

use crate::phantom::QuantumCell;
use crate::UFO;
use crate::constants::CURRENT_TIMESTAMP;

/// Native quantum memory ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeliumOrdering {
    /// No ordering or coherence guarantees
    Relaxed,
    /// Acquire quantum state
    Acquire,
    /// Release quantum state
    Release,
    /// Acquire and Release combined
    AcquireRelease,
    /// Full quantum consistency
    Quantum,
}

#[derive(Debug)]
pub struct Helium<T> {
    inner: QuantumCell<T>,
    timestamp: QuantumCell<usize>,
}

unsafe impl<T: Send> Send for Helium<T> {}
unsafe impl<T: Send> Sync for Helium<T> {}

impl<T: Copy> Helium<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: QuantumCell::new(value),
            timestamp: QuantumCell::new(CURRENT_TIMESTAMP),
        }
    }

    pub fn load(&self, order: HeliumOrdering) -> T {
        match order {
            HeliumOrdering::Quantum => {
                self.inner.decay_coherence();
                self.timestamp.set(CURRENT_TIMESTAMP);
            }
            HeliumOrdering::AcquireRelease | HeliumOrdering::Acquire => {
                self.inner.decay_coherence();
            }
            _ => {}
        }
        *self.inner.get()
    }

    pub fn store(&self, value: T, order: HeliumOrdering) {
        self.inner.set(value);
        match order {
            HeliumOrdering::Quantum => {
                self.timestamp.set(CURRENT_TIMESTAMP);
            }
            HeliumOrdering::AcquireRelease | HeliumOrdering::Release => {
                self.inner.decay_coherence();
                self.timestamp.set(CURRENT_TIMESTAMP);
            }
            _ => {}
        }
    }

    pub fn fetch_add(&self, value: T, order: HeliumOrdering) -> T
    where T: std::ops::Add<Output = T> {
        let old = *self.inner.get();
        let new = old + value;
        self.store(new, order);
        old
    }

    pub fn fetch_sub(&self, value: T, order: HeliumOrdering) -> T
    where T: std::ops::Sub<Output = T> {
        let old = *self.inner.get();
        let new = old - value;
        self.store(new, order);
        old
    }

    pub fn get_coherence(&self) -> f64 {
        self.inner.get_coherence()
    }

    pub fn reset_coherence(&self) {
        self.inner.set(*self.inner.get());
        self.timestamp.set(CURRENT_TIMESTAMP);
    }

    pub fn decay_coherence(&self) {
        self.inner.decay_coherence();
        self.timestamp.set(CURRENT_TIMESTAMP);
    }

    pub fn get_timestamp(&self) -> usize {
        *self.timestamp.get()
    }
}

/// Specialized size type with quantum coherence
#[derive(Debug)]
pub struct HeliumSize {
    value: Helium<usize>,
    ufo: UFO<usize>,
}

impl HeliumSize {
    pub fn new(value: usize) -> Self {
        Self {
            value: Helium::new(value),
            ufo: UFO::new(),
        }
    }

    pub const fn const_new(value: usize) -> Self {
        Self {
            value: Helium::new(value),
            ufo: UFO::const_default(),
        }
    }

    pub fn load(&self, order: HeliumOrdering) -> usize {
        self.ufo.protect();
        self.value.load(order)
    }

    pub fn store(&self, value: usize, order: HeliumOrdering) {
        self.value.store(value, order);
        self.ufo.protect();
    }

    pub fn fetch_add(&self, value: usize, order: HeliumOrdering) -> usize {
        self.ufo.protect();
        self.value.fetch_add(value, order)
    }

    pub fn fetch_sub(&self, value: usize, order: HeliumOrdering) -> usize {
        self.ufo.protect();
        self.value.fetch_sub(value, order)
    }

    pub fn get_coherence(&self) -> f64 {
        self.value.get_coherence()
    }

    pub fn is_protected(&self) -> bool {
        self.ufo.is_protected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helium_operations() {
        let h = Helium::new(42usize);

        assert_eq!(h.load(HeliumOrdering::Relaxed), 42);
        assert_eq!(h.load(HeliumOrdering::Quantum), 42);

        h.store(100, HeliumOrdering::Quantum);
        assert_eq!(h.load(HeliumOrdering::Quantum), 100);

        let old = h.fetch_add(50, HeliumOrdering::Quantum);
        assert_eq!(old, 100);
        assert_eq!(h.load(HeliumOrdering::Quantum), 150);
    }

    #[test]
    fn test_helium_coherence() {
        let h = Helium::new(1usize);
        assert_eq!(h.get_coherence(), 1.0);

        h.load(HeliumOrdering::Quantum);
        assert!(h.get_coherence() < 1.0);

        h.reset_coherence();
        assert_eq!(h.get_coherence(), 1.0);
    }

    #[test]
    fn test_helium_size() {
        let hs = HeliumSize::new(100);
        assert_eq!(hs.load(HeliumOrdering::Quantum), 100);
        assert!(hs.is_protected());

        let old = hs.fetch_add(50, HeliumOrdering::Quantum);
        assert_eq!(old, 100);
        assert_eq!(hs.load(HeliumOrdering::Quantum), 150);
        assert!(hs.get_coherence() < 1.0);
    }
}
