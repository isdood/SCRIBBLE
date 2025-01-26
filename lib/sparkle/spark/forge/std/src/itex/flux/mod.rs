//! Quantum flux module for Itex type

use std::sync::atomic::{AtomicBool, Ordering};

/// Quantum flux state
#[derive(Debug)]
pub struct Flux {
    /// Flux intensity
    intensity: f64,
    /// Parallelization state
    parallel: bool,
    /// Stability flag
    stable: AtomicBool,
}

impl Clone for Flux {
    fn clone(&self) -> Self {
        Self {
            intensity: self.intensity,
            parallel: self.parallel,
            stable: AtomicBool::new(self.stable.load(Ordering::Relaxed)),
        }
    }
}

impl Flux {
    /// Creates a new flux state
    pub fn new(intensity: f64) -> Self {
        Self {
            intensity: intensity.abs(),
            parallel: false,
            stable: AtomicBool::new(true),
        }
    }

    /// Gets the flux intensity
    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    /// Checks if flux is parallel
    pub fn is_parallel(&self) -> bool {
        self.parallel
    }

    /// Checks if flux is stable
    pub fn is_stable(&self) -> bool {
        self.stable.load(Ordering::Relaxed)
    }

    /// Propagates quantum flux
    pub fn propagate(&self) {
        if self.intensity > 10.0 {
            self.stable.store(false, Ordering::Relaxed);
        }
    }

    /// Stabilizes quantum flux
    pub fn stabilize(&mut self) -> Result<(), String> {
        if !self.is_stable() {
            Err("Flux instability detected".to_string())
        } else {
            self.intensity = self.intensity.min(10.0);
            Ok(())
        }
    }

    /// Parallelizes quantum flux
    pub fn parallelize(&mut self, chunks: usize) -> Result<(), String> {
        if chunks == 0 {
            Err("Invalid chunk count for parallelization".to_string())
        } else {
            self.parallel = true;
            self.intensity /= chunks as f64;
            Ok(())
        }
    }
}

impl Default for Flux {
    fn default() -> Self {
        Self::new(1.0)
    }
}
