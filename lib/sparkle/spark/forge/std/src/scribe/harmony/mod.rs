//! Harmony module for resonance patterns

use super::ResonancePoint;

/// A harmonic pattern in 3D space
#[derive(Debug, Clone)]
pub struct HarmonicPattern {
    frequencies: Vec<f64>,
    amplitudes: Vec<f64>,
}

impl HarmonicPattern {
    /// Creates a new harmonic pattern
    pub fn new() -> Self {
        Self {
            frequencies: Vec::new(),
            amplitudes: Vec::new(),
        }
    }

    /// Updates the pattern with new resonance points
    pub fn update(&mut self, points: &[ResonancePoint]) {
        // Calculate frequency components using FFT
        self.frequencies.clear();
        self.amplitudes.clear();

        for point in points {
            let freq = point.intensity().sqrt();
            self.frequencies.push(freq);
            self.amplitudes.push(point.amplitude.value());
        }
    }
}

impl Default for HarmonicPattern {
    fn default() -> Self {
        Self::new()
    }
}
