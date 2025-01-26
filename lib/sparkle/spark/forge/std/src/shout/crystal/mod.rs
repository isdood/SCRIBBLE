//! Crystal amplifier module for Shout type

/// Crystal amplifier state
#[derive(Debug, Clone, Copy)]
pub struct Amplifier {
    /// Gain level
    gain: f64,
    /// Crystal purity
    purity: f64,
    /// Quantum efficiency
    efficiency: f64,
    /// Resonance modes
    modes: u32,
}

impl Amplifier {
    /// Creates a new amplifier state
    pub fn new(gain: f64, purity: f64, efficiency: f64, modes: u32) -> Self {
        Self {
            gain: gain.abs(),
            purity: purity.clamp(0.0, 1.0),
            efficiency: efficiency.clamp(0.0, 1.0),
            modes: modes.min(16),
        }
    }

    /// Gets the gain level
    pub fn gain(&self) -> f64 {
        self.gain
    }

    /// Gets the crystal purity
    pub fn purity(&self) -> f64 {
        self.purity
    }

    /// Gets the quantum efficiency
    pub fn efficiency(&self) -> f64 {
        self.efficiency
    }

    /// Gets the resonance modes
    pub fn modes(&self) -> u32 {
        self.modes
    }

    /// Combines with another amplifier
    pub fn combine(&self, other: &Self) -> Self {
        Self::new(
            (self.gain + other.gain) / 2.0,
            (self.purity * other.purity).sqrt(),
            (self.efficiency + other.efficiency) / 2.0,
            self.modes.max(other.modes),
        )
    }

    /// Amplifies with another amplifier
    pub fn amplify(&self, other: &Self) -> Self {
        Self::new(
            self.gain * other.gain,
            self.purity * other.purity,
            self.efficiency * other.efficiency,
            self.modes + other.modes,
        )
    }

    /// Attenuates with another amplifier
    pub fn attenuate(&self, other: &Self) -> Self {
        Self::new(
            self.gain / other.gain.max(1.0),
            self.purity / other.purity.max(0.1),
            self.efficiency / other.efficiency.max(0.1),
            (self.modes as f64 / other.modes as f64).round() as u32,
        )
    }

    /// Synchronizes with another amplifier
    pub fn synchronize(&self, other: &Self) -> Self {
        Self::new(
            (self.gain * other.gain).sqrt(),
            (self.purity + other.purity) / 2.0,
            (self.efficiency * other.efficiency).sqrt(),
            (self.modes + other.modes) / 2,
        )
    }

    /// Check if amplifiers are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.gain - other.gain).abs() < f64::EPSILON &&
        (self.purity - other.purity).abs() < f64::EPSILON &&
        (self.efficiency - other.efficiency).abs() < f64::EPSILON &&
        self.modes == other.modes
    }
}

impl Default for Amplifier {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0, 1)
    }
}
