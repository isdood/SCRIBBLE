/// Quantum Helium Implementation
/// Last Updated: 2025-01-14 23:09:52 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::{CURRENT_TIMESTAMP, QUANTUM_COHERENCE_THRESHOLD},
    phantom::{QuantumCell, Quantum},
    Vector3D,
    grav::GravityFieldRef,
    ufo::{UFO, Protected},
};

#[derive(Debug, Clone, Copy)]
pub enum HeliumOrdering {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
    Quantum,
}

#[derive(Debug)]
pub struct Helium<T> {
    inner: QuantumCell<T>,
    timestamp: QuantumCell<usize>,
    position: QuantumCell<Vector3D<f64>>,
    gravity_ref: Option<GravityFieldRef>,
    gravitational_coherence: QuantumCell<f64>,
    ufo: UFO<T>,
}

unsafe impl<T: Send> Send for Helium<T> {}
unsafe impl<T: Send> Sync for Helium<T> {}

impl<T: Copy> Helium<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: QuantumCell::new(value),
            timestamp: QuantumCell::new(CURRENT_TIMESTAMP),
            position: QuantumCell::new(Vector3D::new(0.0, 0.0, 0.0)),
            gravity_ref: None,
            gravitational_coherence: QuantumCell::new(1.0),
            ufo: UFO::new(),
        }
    }

    pub fn quantum_load(&self) -> T {
        if let Some(field) = &self.gravity_ref {
            let position = self.position.get().clone();
            let force = field.calculate_force_at(position, 1.0);
            let coherence = *self.gravitational_coherence.get();
            self.gravitational_coherence.set(coherence * 0.99);
        }
        *self.inner.get()
    }

    pub fn quantum_store(&self, value: T) -> Result<(), &'static str> {
        if !self.ufo.is_protected() {
            self.ufo.protect()?;
        }
        self.inner.set(value);
        self.timestamp.set(CURRENT_TIMESTAMP);
        self.decay_coherence();
        Ok(())
    }

    pub fn load(&self, order: &HeliumOrdering) -> Result<T, &'static str> {
        match order {
            HeliumOrdering::Quantum => Ok(self.quantum_load()),
            HeliumOrdering::AcquireRelease | HeliumOrdering::Acquire => {
                Ok(*self.inner.get())
            }
            _ => Ok(*self.inner.get())
        }
    }

    pub fn store(&self, value: T, order: &HeliumOrdering) -> Result<(), &'static str> {
        match order {
            HeliumOrdering::Quantum => self.quantum_store(value),
            HeliumOrdering::AcquireRelease | HeliumOrdering::Release => {
                self.inner.set(value);
                self.decay_coherence();
                Ok(())
            }
            _ => {
                self.inner.set(value);
                Ok(())
            }
        }
    }

    pub fn set_gravity_field(&mut self, field: GravityFieldRef) {
        self.gravity_ref = Some(field);
    }
}

impl<T> Quantum for Helium<T> {
    fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_COHERENCE_THRESHOLD && self.ufo.is_protected()
    }

    fn get_coherence(&self) -> f64 {
        let quantum_coherence = *self.gravitational_coherence.get();
        quantum_coherence * if self.ufo.is_protected() { 1.0 } else { 0.5 }
    }

    fn decay_coherence(&self) {
        let current = *self.gravitational_coherence.get();
        self.gravitational_coherence.set(current * 0.99);
    }

    fn reset_coherence(&self) {
        self.gravitational_coherence.set(1.0);
    }
}

/// Specialized size type with quantum coherence and gravitational awareness
#[derive(Debug)]
pub struct HeliumSize {
    value: Helium<usize>,
    coherence: QuantumCell<f64>,
    position: QuantumCell<Vector3D<f64>>,
}

impl HeliumSize {
    pub fn new(value: usize) -> Self {
        Self {
            value: Helium::new(value),
            coherence: QuantumCell::new(1.0),
            position: QuantumCell::new(Vector3D::new(0.0, 0.0, 0.0)),
        }
    }

    pub fn quantum_load(&self) -> Result<usize, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }
        Ok(self.value.quantum_load())
    }

    pub fn quantum_store(&self, value: usize) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }
        self.value.quantum_store(value)
    }

    pub fn fetch_add(&self, value: usize, order: &HeliumOrdering) -> Result<usize, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }
        let current = self.value.load(order)?;
        self.value.store(current + value, order)?;
        Ok(current)
    }

    pub fn fetch_sub(&self, value: usize, order: &HeliumOrdering) -> Result<usize, &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }
        let current = self.value.load(order)?;
        self.value.store(current.saturating_sub(value), order)?;
        Ok(current)
    }
}

impl Quantum for HeliumSize {
    fn is_quantum_stable(&self) -> bool {
        self.value.is_quantum_stable()
    }

    fn get_coherence(&self) -> f64 {
        self.value.get_coherence()
    }

    fn decay_coherence(&self) {
        self.value.decay_coherence()
    }

    fn reset_coherence(&self) {
        self.value.reset_coherence()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helium_quantum_operations() {
        let helium = Helium::new(42usize);
        assert!(helium.is_quantum_stable());

        helium.quantum_store(100).unwrap();
        assert_eq!(helium.quantum_load(), 100);
    }

    #[test]
    fn test_helium_gravity_interaction() {
        let mut helium = Helium::new(1.0f64);
        let grav_ref = GravityFieldRef::new(Vector3D::new(0.0, -9.81, 0.0));

        helium.set_gravity_field(grav_ref);
        helium.quantum_store(2.0).unwrap();

        assert!(helium.get_coherence() < 1.0);
    }

    #[test]
    fn test_helium_size_operations() {
        let hs = HeliumSize::new(100);
        assert!(hs.is_quantum_stable());

        assert!(hs.quantum_store(150).is_ok());
        assert_eq!(hs.quantum_load().unwrap(), 150);

        let order = HeliumOrdering::Quantum;
        assert!(hs.fetch_add(50, &order).is_ok());
        assert_eq!(hs.quantum_load().unwrap(), 200);
    }

    #[test]
    fn test_helium_ordering() {
        let helium = Helium::new(42usize);
        let order = HeliumOrdering::Quantum;

        assert!(helium.store(100, &order).is_ok());
        assert_eq!(helium.load(&order).unwrap(), 100);
    }
}
