//! Crystal resonance module for Murmur type

/// Crystal resonance state
#[derive(Debug, Clone, Copy)]
pub struct CrystalResonance {
    energy: f64,
    coherence: f64,
}

impl CrystalResonance {
    /// Creates a new crystal resonance
    pub fn new(energy: f64, coherence: f64) -> Self {
        Self {
            energy: energy.abs(),
            coherence: coherence.clamp(0.0, 1.0),
        }
    }

    /// Gets the resonance energy
    pub fn energy(&self) -> f64 {
        self.energy
    }

    /// Gets the resonance coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Combines with another resonance
    pub fn combine(&self, other: &Self) -> Self {
        Self::new(
            (self.energy + other.energy) / 2.0,
            (self.coherence * other.coherence).sqrt(),
        )
    }

    /// Amplifies with another resonance
    pub fn amplify(&self, other: &Self) -> Self {
        Self::new(
            self.energy * other.energy,
            self.coherence * other.coherence,
        )
    }

    /// Attenuates with another resonance
    pub fn attenuate(&self, other: &Self) -> Self {
        Self::new(
            self.energy / other.energy.max(1.0),
            self.coherence / other.coherence.max(1.0),
        )
    }

    /// Check if resonances are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.energy - other.energy).abs() < f64::EPSILON &&
        (self.coherence - other.coherence).abs() < f64::EPSILON
    }
}

impl Default for CrystalResonance {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}
