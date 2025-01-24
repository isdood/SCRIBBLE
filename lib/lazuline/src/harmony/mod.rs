//! Harmony field manipulation module

use std::sync::Arc;
use parking_lot::RwLock;

/// Represents a field of harmony that can be enhanced and manipulated
pub struct HarmonyField {
    intensity: f64,
    resonance: Vec<f64>,
    stability: Arc<RwLock<f64>>,
}

impl HarmonyField {
    /// Creates a new HarmonyField with the given intensity
    pub fn new(intensity: f64) -> Self {
        Self {
            intensity,
            resonance: Vec::new(),
            stability: Arc::new(RwLock::new(1.0)),
        }
    }

    /// Enhances the harmony field by adjusting its stability
    pub fn enhance_harmony(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stability = self.stability.write();
        *stability *= self.intensity;
        Ok(())
    }

    /// Gets the current resonance values
    pub fn get_resonance(&self) -> &[f64] {
        &self.resonance
    }

    /// Adds a new resonance value
    pub fn add_resonance(&mut self, value: f64) {
        self.resonance.push(value);
    }
}
