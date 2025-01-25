//! Echo propagation module for Shout type

/// Echo propagation state
#[derive(Debug, Clone, Copy)]
pub struct Echo {
    /// Echo intensity
    intensity: f64,
    /// Delay time
    delay: f64,
    /// Decay rate
    decay: f64,
    /// Reflection count
    reflections: u32,
}

impl Echo {
    /// Creates a new echo state
    pub fn new(intensity: f64, delay: f64, decay: f64, reflections: u32) -> Self {
        Self {
            intensity: intensity.abs(),
            delay: delay.abs(),
            decay: decay.clamp(0.0, 1.0),
            reflections: reflections.min(8),
        }
    }

    /// Gets the echo intensity
    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    /// Gets the delay time
    pub fn delay(&self) -> f64 {
        self.delay
    }

    /// Gets the decay rate
    pub fn decay(&self) -> f64 {
        self.decay
    }

    /// Gets the reflection count
    pub fn reflections(&self) -> u32 {
        self.reflections
    }

    /// Propagates with another echo
    pub fn propagate(&self, other: &Self) -> Self {
        Self::new(
            self.intensity + other.intensity,
            (self.delay + other.delay) / 2.0,
            (self.decay + other.decay) / 2.0,
            self.reflections.max(other.reflections),
        )
    }

    /// Interferes with another echo
    pub fn interfere(&self, other: &Self) -> Self {
        Self::new(
            (self.intensity.powi(2) + other.intensity.powi(2)).sqrt(),
            (self.delay * other.delay).sqrt(),
            (self.decay * other.decay).sqrt(),
            self.reflections.min(other.reflections),
        )
    }

    /// Amplifies with another echo
    pub fn amplify(&self, other: &Self) -> Self {
        Self::new(
            self.intensity * other.intensity,
            self.delay * other.delay,
            self.decay * other.decay,
            self.reflections + other.reflections,
        )
    }

    /// Attenuates with another echo
    pub fn attenuate(&self, other: &Self) -> Self {
        Self::new(
            self.intensity / other.intensity.max(1.0),
            self.delay / other.delay.max(1.0),
            self.decay / other.decay.max(0.1),
            (self.reflections as f64 / other.reflections as f64).round() as u32,
        )
    }

    /// Resonates with another echo
    pub fn resonate(&self, other: &Self) -> Self {
        Self::new(
            (self.intensity * other.intensity).sqrt(),
            (self.delay + other.delay) / 2.0,
            (self.decay * other.decay).sqrt(),
            (self.reflections + other.reflections) / 2,
        )
    }

    /// Check if echoes are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.intensity - other.intensity).abs() < f64::EPSILON &&
        (self.delay - other.delay).abs() < f64::EPSILON &&
        (self.decay - other.decay).abs() < f64::EPSILON &&
        self.reflections == other.reflections
    }
}

impl Default for Echo {
    fn default() -> Self {
        Self::new(1.0, 0.1, 0.5, 1)
    }
}
