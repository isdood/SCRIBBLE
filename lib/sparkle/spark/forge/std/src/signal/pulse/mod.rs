//! Quantum pulse module for Signal type

/// Quantum pulse state
#[derive(Debug, Clone)]
pub struct Pulse {
    /// Quantum phase
    phase: f64,
    /// Coherence level
    coherence: f64,
    /// Entanglement factor
    entanglement: f64,
}

impl Pulse {
    /// Creates a new pulse state
    pub fn new(phase: f64, coherence: f64, entanglement: f64) -> Self {
        Self {
            phase: phase % (2.0 * std::f64::consts::PI),
            coherence: coherence.clamp(0.0, 1.0),
            entanglement: entanglement.clamp(0.0, 1.0),
        }
    }

    /// Creates a pulse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let phase = bytes.iter().fold(0.0, |acc, &b| acc + b as f64) % (2.0 * std::f64::consts::PI);
        let coherence = bytes.iter().fold(0.0, |acc, &b| acc + (b as f64 / 255.0)) / bytes.len() as f64;
        let entanglement = bytes.windows(2).fold(0.0, |acc, w| acc + (w[0] as f64 - w[1] as f64).abs()) / 255.0;

        Self::new(phase, coherence, entanglement)
    }

    /// Gets the quantum phase
    pub fn phase(&self) -> f64 {
        self.phase
    }

    /// Gets the coherence level
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Gets the entanglement factor
    pub fn entanglement(&self) -> f64 {
        self.entanglement
    }

    /// Updates the pulse with new bytes
    pub fn update(&mut self, bytes: &[u8]) {
        let new = Self::from_bytes(bytes);
        self.phase = (self.phase + new.phase) / 2.0;
        self.coherence = (self.coherence + new.coherence) / 2.0;
        self.entanglement = (self.entanglement + new.entanglement) / 2.0;
    }

    /// Resonates with another pulse
    pub fn resonate(&mut self, other: &Self) {
        self.phase = (self.phase + other.phase) % (2.0 * std::f64::consts::PI);
        self.coherence = (self.coherence * other.coherence).sqrt();
        self.entanglement = (self.entanglement + other.entanglement) / 2.0;
    }

    /// Amplifies the pulse
    pub fn amplify(&mut self, gain: f64) {
        self.phase = (self.phase * gain) % (2.0 * std::f64::consts::PI);
        self.coherence = (self.coherence * gain).min(1.0);
        self.entanglement = self.entanglement.powf(1.0 / gain).min(1.0);
    }
}

impl Default for Pulse {
    fn default() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
}

impl std::ops::BitAnd for Pulse {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(
            (self.phase + rhs.phase) / 2.0,
            self.coherence * rhs.coherence,
            (self.entanglement + rhs.entanglement) / 2.0,
        )
    }
}

impl std::ops::BitOr for Pulse {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(
            self.phase.max(rhs.phase),
            self.coherence.max(rhs.coherence),
            self.entanglement.max(rhs.entanglement),
        )
    }
}

impl std::ops::BitXor for Pulse {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(
            (self.phase - rhs.phase).abs(),
            (self.coherence + rhs.coherence) / 2.0,
            (self.entanglement * rhs.entanglement).sqrt(),
        )
    }
}

impl std::ops::Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(
            (std::f64::consts::PI - self.phase) % (2.0 * std::f64::consts::PI),
            1.0 - self.coherence,
            1.0 - self.entanglement,
        )
    }
}
