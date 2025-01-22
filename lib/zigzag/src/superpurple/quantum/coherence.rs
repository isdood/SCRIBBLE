//! Quantum coherence management
//! Created: 2025-01-21 23:45:28 UTC
//! Author: isdood

use super::state::QuantumState;
use std::time::{Duration, Instant};

/// Coherence manager
pub struct CoherenceManager {
    /// Initial coherence
    initial_coherence: f64,
    /// Current coherence
    current_coherence: f64,
    /// Decoherence rate
    decoherence_rate: f64,
    /// Start time
    start_time: Instant,
    /// Coherence history
    history: Vec<(Instant, f64)>,
}

impl CoherenceManager {
    /// Create new coherence manager
    pub fn new(initial_coherence: f64, decoherence_rate: f64) -> Self {
        let now = Instant::now();
        Self {
            initial_coherence,
            current_coherence: initial_coherence,
            decoherence_rate,
            start_time: now,
            history: vec![(now, initial_coherence)],
        }
    }

    /// Update coherence
    pub fn update(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.start_time);
        self.current_coherence = self.calculate_coherence(elapsed);
        self.history.push((now, self.current_coherence));
    }

    /// Calculate coherence at given time
    fn calculate_coherence(&self, elapsed: Duration) -> f64 {
        let t = elapsed.as_secs_f64();
        self.initial_coherence * (-self.decoherence_rate * t).exp()
    }

    /// Get current coherence
    pub fn get_coherence(&self) -> f64 {
        self.current_coherence
    }

    /// Get coherence history
    pub fn get_history(&self) -> &[(Instant, f64)] {
        &self.history
    }

    /// Calculate coherence time
    pub fn coherence_time(&self) -> Duration {
        let threshold = 1.0 / std::f64::consts::E;
        Duration::from_secs_f64(-threshold.ln() / self.decoherence_rate)
    }
}
