//! Wave propagation module for Murmur type

use std::f64::consts::PI;

/// Wave propagation state
#[derive(Debug, Clone, Copy)]
pub struct WaveState {
    amplitude: f64,
    frequency: f64,
    phase: f64,
}

impl WaveState {
    /// Creates a new wave state
    pub fn new(amplitude: f64, frequency: f64, phase: f64) -> Self {
        Self {
            amplitude: amplitude.abs(),
            frequency: frequency.abs(),
            phase: phase % (2.0 * PI),
        }
    }

    /// Gets the wave amplitude
    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    /// Gets the wave frequency
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Gets the wave phase
    pub fn phase(&self) -> f64 {
        self.phase
    }

    /// Propagates with another wave
    pub fn propagate(&self, other: &Self) -> Self {
        Self::new(
            self.amplitude + other.amplitude,
            (self.frequency + other.frequency) / 2.0,
            self.phase + other.phase,
        )
    }

    /// Interferes with another wave
    pub fn interfere(&self, other: &Self) -> Self {
        Self::new(
            (self.amplitude.powi(2) + other.amplitude.powi(2)).sqrt(),
            (self.frequency * other.frequency).sqrt(),
            (self.phase - other.phase).abs(),
        )
    }

    /// Amplifies with another wave
    pub fn amplify(&self, other: &Self) -> Self {
        Self::new(
            self.amplitude * other.amplitude,
            self.frequency * other.frequency,
            self.phase * other.phase,
        )
    }

    /// Attenuates with another wave
    pub fn attenuate(&self, other: &Self) -> Self {
        Self::new(
            self.amplitude / other.amplitude.max(1.0),
            self.frequency / other.frequency.max(1.0),
            self.phase / other.phase.max(1.0),
        )
    }

    /// Check if wave states are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.amplitude - other.amplitude).abs() < f64::EPSILON &&
        (self.frequency - other.frequency).abs() < f64::EPSILON &&
        (self.phase - other.phase).abs() < f64::EPSILON
    }
}

impl Default for WaveState {
    fn default() -> Self {
        Self::new(1.0, 1.0, 0.0)
    }
}
