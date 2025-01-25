//! Wave module for resonance patterns

/// Wave amplitude
#[derive(Debug, Clone, Copy)]
pub struct Amplitude(f64);

impl Amplitude {
    /// Creates a new amplitude
    pub fn new(value: f64) -> Self {
        Self(value.abs())
    }

    /// Gets the amplitude value
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Wave phase
#[derive(Debug, Clone, Copy)]
pub struct Phase(f64);

impl Phase {
    /// Creates a new phase
    pub fn new(value: f64) -> Self {
        Self(value % (2.0 * std::f64::consts::PI))
    }

    /// Gets the phase value
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A wave in 3D space
#[derive(Debug, Clone)]
pub struct Wave {
    amplitude: Amplitude,
    phase: Phase,
    frequency: f64,
}

impl Wave {
    /// Creates a new wave
    pub fn new(amplitude: f64, phase: f64, frequency: f64) -> Self {
        Self {
            amplitude: Amplitude::new(amplitude),
            phase: Phase::new(phase),
            frequency,
        }
    }

    /// Gets the wave value at a given time
    pub fn value(&self, t: f64) -> f64 {
        self.amplitude.value() * (self.frequency * t + self.phase.value()).sin()
    }
}

impl Default for Wave {
    fn default() -> Self {
        Self::new(1.0, 0.0, 1.0)
    }
}
