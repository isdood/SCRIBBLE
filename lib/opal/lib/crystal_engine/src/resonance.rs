use harmony_core::Vector3D;
use magicmath::constants::*;

pub struct ResonanceHandler {
    base_frequency: f64,
    harmonic_factor: f64,
}

impl ResonanceHandler {
    pub fn new() -> Self {
        Self {
            base_frequency: RESONANCE_FACTOR,
            harmonic_factor: PHI,
        }
    }

    pub fn calculate_resonance(&self, value: f64) -> f64 {
        value * self.harmonic_factor * self.base_frequency
    }
}
