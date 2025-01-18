//! Crystalline Phantom Implementation
//! ============================
//!
//! Core quantum phantom operations through crystalline
//! structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 21:20:36 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::QUANTUM_STABILITY_THRESHOLD,
    harmony::Quantum,
    vector::Vector4D,
    cube::CrystalCube,
    idk::ShardUninit,
};

/// A quantum phantom state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PhantomState {
    /// Fully materialized
    Materialized,
    /// Partially materialized
    Partial,
    /// Fully dematerialized
    Dematerialized,
}

/// A quantum phantom for 4D operations
#[derive(Clone)]
pub struct Phantom<T: Clone + Default + 'static> {
    /// Phantom data
    data: ShardUninit<T>,
    /// 4D position including quantum time
    position: Vector4D<f64>,
    /// Current state
    state: PhantomState,
    /// Quantum coherence
    coherence: f64,
}

impl<T: Clone + Default + 'static> Phantom<T> {
    /// Creates a new phantom with positioned data
    pub fn new_positioned(data: T, x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            data: ShardUninit::new(data),
            position: Vector4D::new(x, y, z, w),
            state: PhantomState::Materialized,
            coherence: 1.0,
        }
    }

    /// Gets a reference to the phantom data
    pub fn data(&self) -> &T {
        unsafe { self.data.assume_init_ref() }
    }

    /// Gets a mutable reference to the phantom data
    pub fn data_mut(&mut self) -> &mut T {
        unsafe { self.data.assume_init_mut() }
    }

    /// Gets the current 4D position
    pub fn position(&self) -> &Vector4D<f64> {
        &self.position
    }

    /// Gets the current state
    pub fn state(&self) -> PhantomState {
        self.state
    }

    /// Sets the 4D position and updates quantum state
    pub fn set_position(&mut self, x: f64, y: f64, z: f64, w: f64) {
        self.position = Vector4D::new(x, y, z, w);
        self.decohere();
    }

    /// Attempts to materialize into a crystal cube
    pub fn materialize(&mut self, cube: &mut CrystalCube<T>) -> Result<(), &'static str> {
        if self.state == PhantomState::Dematerialized {
            return Err("Cannot materialize while dematerialized");
        }

        let pos = self.position();
        let x = meshmath::floor(pos.x) as usize;
        let y = meshmath::floor(pos.y) as usize;
        let z = meshmath::floor(pos.z) as usize;

        if let Some(cell) = cube.get_mut(x, y, z) {
            unsafe {
                let mut temp = ShardUninit::new(T::default());
                core::ptr::swap(self.data.as_mut_ptr(), temp.as_mut_ptr());
                *cell = temp.assume_init();
            }
            self.state = PhantomState::Materialized;
            self.decohere();
            cube.decohere();
            Ok(())
        } else {
            Err("Invalid cube coordinates")
        }
    }

    /// Attempts to dematerialize from current position
    pub fn dematerialize(&mut self) -> Result<(), &'static str> {
        if self.state == PhantomState::Dematerialized {
            return Err("Already dematerialized");
        }

        self.state = PhantomState::Dematerialized;
        self.decohere();
        Ok(())
    }
}

impl<T: Clone + Default + 'static> Quantum for Phantom<T> {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    fn decohere(&mut self) {
        self.coherence *= 0.9;
        if self.coherence < QUANTUM_STABILITY_THRESHOLD {
            self.coherence = QUANTUM_STABILITY_THRESHOLD;
            self.state = PhantomState::Partial;
        }
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
        self.state = PhantomState::Materialized;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phantom_basics() {
        let phantom = Phantom::new_positioned(42u8, 1.0, 2.0, 3.0, 0.0);
        assert_eq!(*phantom.data(), 42);
        assert_eq!(phantom.position().x, 1.0);
        assert_eq!(phantom.position().y, 2.0);
        assert_eq!(phantom.position().z, 3.0);
        assert_eq!(phantom.position().w, 0.0);
        assert_eq!(phantom.state(), PhantomState::Materialized);
        assert!(phantom.is_stable());
    }

    #[test]
    fn test_phantom_state_changes() {
        let mut phantom = Phantom::new_positioned(42u8, 0.0, 0.0, 0.0, 0.0);
        assert!(phantom.dematerialize().is_ok());
        assert_eq!(phantom.state(), PhantomState::Dematerialized);
        assert!(phantom.dematerialize().is_err());
    }

    #[test]
    fn test_phantom_materialization() {
        let mut phantom = Phantom::new_positioned(42u8, 0.0, 0.0, 0.0, 0.0);
        let mut cube = CrystalCube::<u8>::new(2, 2, 2).unwrap();

        assert!(phantom.materialize(&mut cube).is_ok());
        assert_eq!(*cube.get(0, 0, 0).unwrap(), 42);

        phantom.dematerialize().unwrap();
        assert!(phantom.materialize(&mut cube).is_err());
    }
}
