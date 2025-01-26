//! Crystal resonator module for Voice type

/// Crystal resonator state
#[derive(Debug, Clone, Copy)]
pub struct Resonator {
    /// Energy level
    energy: f64,
    /// Coherence factor
    coherence: f64,
    /// Resonance quality
    quality: f64,
}

impl Resonator {
    /// Creates a new resonator state
    pub fn new(energy: f64, coherence: f64, quality: f64) -> Self {
        Self {
            energy: energy.abs(),
            coherence: coherence.clamp(0.0, 1.0),
            quality: quality.clamp(0.0, 1.0),
        }
    }

    /// Gets the energy level
    pub fn energy(&self) -> f64 {
        self.energy
    }

    /// Gets the coherence factor
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Gets the resonance quality
    pub fn quality(&self) -> f64 {
        self.quality
    }

    /// Resonates with another resonator
    pub fn resonate(&self, other: &Self) -> Self {
        Self::new(
            (self.energy + other.energy) / 2.0,
            (self.coherence * other.coherence).sqrt(),
            (self.quality + other.quality) / 2.0,
        )
    }

    /// Amplifies with another resonator
    pub fn amplify(&self, other: &Self) -> Self {
        Self::new(
            self.energy * other.energy,
            self.coherence * other.coherence,
            (self.quality * other.quality).sqrt(),
        )
    }

    /// Attenuates with another resonator
    pub fn attenuate(&self, other: &Self) -> Self {
        Self::new(
            self.energy / other.energy.max(1.0),
            self.coherence / other.coherence.max(1.0),
            self.quality,
        )
    }

    /// Synchronizes with another resonator
    pub fn synchronize(&self, other: &Self) -> Self {
        Self::new(
            (self.energy * other.energy).sqrt(),
            (self.coherence + other.coherence) / 2.0,
            (self.quality * other.quality).sqrt(),
        )
    }

    /// Check if resonators are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.energy - other.energy).abs() < f64::EPSILON &&
        (self.coherence - other.coherence).abs() < f64::EPSILON &&
        (self.quality - other.quality).abs() < f64::EPSILON
    }
}

impl Default for Resonator {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}
