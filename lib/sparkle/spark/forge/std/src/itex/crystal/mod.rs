//! Crystal resonance module for Itex type

use std::sync::atomic::{AtomicUsize, Ordering};

/// Crystal resonance state
#[derive(Debug)]
pub struct Crystal {
    /// Resonance frequency
    frequency: f64,
    /// Quantum state
    state: usize,
    /// Operation counter
    counter: AtomicUsize,
}

impl Clone for Crystal {
    fn clone(&self) -> Self {
        Self {
            frequency: self.frequency,
            state: self.state,
            counter: AtomicUsize::new(self.counter.load(Ordering::Relaxed)),
        }
    }
}

impl Crystal {
    /// Creates a new crystal state
    pub fn new(frequency: f64) -> Self {
        Self {
            frequency: frequency.abs(),
            state: 0,
            counter: AtomicUsize::new(0),
        }
    }

    /// Gets the resonance frequency
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Gets the quantum state
    pub fn state(&self) -> usize {
        self.state
    }

    /// Gets the operation count
    pub fn count(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    /// Resonates the crystal
    pub fn resonate(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }

    /// Accelerates crystal operations
    pub fn accelerate(&mut self) -> Result<(), String> {
        if self.frequency < 1.0 {
            Err("Crystal frequency too low for acceleration".to_string())
        } else {
            self.frequency *= 2.0;
            Ok(())
        }
    }

    /// Synchronizes with another crystal
    pub fn synchronize(&mut self, other: &Self) -> Result<(), String> {
        if (self.frequency - other.frequency).abs() > 1.0 {
            Err("Crystal frequency mismatch".to_string())
        } else {
            self.frequency = (self.frequency + other.frequency) / 2.0;
            Ok(())
        }
    }
}

impl Default for Crystal {
    fn default() -> Self {
        Self::new(1.0)
    }
}
