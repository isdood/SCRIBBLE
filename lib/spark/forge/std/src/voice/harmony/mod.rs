//! Harmonic oscillation module for Voice type

/// Harmonic oscillation state
#[derive(Debug, Clone, Copy)]
pub struct Harmonic {
    /// Fundamental frequency
    frequency: f64,
    /// Harmonic amplitudes
    amplitudes: [f64; 4],
    /// Phase offset
    phase: f64,
}

impl Harmonic {
    /// Creates a new harmonic state
    pub fn new(frequency: f64, amplitudes: [f64; 4], phase: f64) -> Self {
        Self {
            frequency: frequency.abs(),
            amplitudes: amplitudes.map(|a| a.abs()),
            phase: phase % (2.0 * std::f64::consts::PI),
        }
    }

    /// Gets the fundamental frequency
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Gets the harmonic amplitudes
    pub fn amplitudes(&self) -> &[f64; 4] {
        &self.amplitudes
    }

    /// Gets the phase offset
    pub fn phase(&self) -> f64 {
        self.phase
    }

    /// Combines with another harmonic
    pub fn combine(&self, other: &Self) -> Self {
        let mut new_amplitudes = [0.0; 4];
        for i in 0..4 {
            new_amplitudes[i] = (self.amplitudes[i] + other.amplitudes[i]) / 2.0;
        }

        Self::new(
            (self.frequency + other.frequency) / 2.0,
            new_amplitudes,
            self.phase + other.phase,
        )
    }

    /// Interferes with another harmonic
    pub fn interfere(&self, other: &Self) -> Self {
        let mut new_amplitudes = [0.0; 4];
        for i in 0..4 {
            new_amplitudes[i] = (self.amplitudes[i].powi(2) + other.amplitudes[i].powi(2)).sqrt();
        }

        Self::new(
            (self.frequency * other.frequency).sqrt(),
            new_amplitudes,
            (self.phase - other.phase).abs(),
        )
    }

    /// Amplifies with another harmonic
    pub fn amplify(&self, other: &Self) -> Self {
        let mut new_amplitudes = [0.0; 4];
        for i in 0..4 {
            new_amplitudes[i] = self.amplitudes[i] * other.amplitudes[i];
        }

        Self::new(
            self.frequency * other.frequency,
            new_amplitudes,
            self.phase * other.phase,
        )
    }

    /// Attenuates with another harmonic
    pub fn attenuate(&self, other: &Self) -> Self {
        let mut new_amplitudes = [0.0; 4];
        for i in 0..4 {
            new_amplitudes[i] = self.amplitudes[i] / other.amplitudes[i].max(1.0);
        }

        Self::new(
            self.frequency / other.frequency.max(1.0),
            new_amplitudes,
            self.phase / other.phase.max(1.0),
        )
    }

    /// Harmonizes with another harmonic
    pub fn harmonize(&self, other: &Self) -> Self {
        let mut new_amplitudes = [0.0; 4];
        for i in 0..4 {
            new_amplitudes[i] = (self.amplitudes[i] * other.amplitudes[i]).sqrt();
        }

        Self::new(
            (self.frequency * other.frequency).sqrt(),
            new_amplitudes,
            (self.phase + other.phase) / 2.0,
        )
    }

    /// Check if harmonics are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.frequency - other.frequency).abs() < f64::EPSILON &&
        self.amplitudes.iter().zip(other.amplitudes.iter())
            .all(|(a, b)| (a - b).abs() < f64::EPSILON) &&
        (self.phase - other.phase).abs() < f64::EPSILON
    }
}

impl Default for Harmonic {
    fn default() -> Self {
        Self::new(440.0, [1.0, 0.5, 0.25, 0.125], 0.0)
    }
}
