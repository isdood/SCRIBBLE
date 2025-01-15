/// Quantum Glitch Module
/// Last Updated: 2025-01-15 22:40:01 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    vector::Vector3D,
    quantum::Quantum,
    scribe::{Scribe, ScribePrecision, QuantumString},
    phantom::QuantumCell,
    constants::*,
};

#[derive(Debug, Clone, Copy)]
pub enum WormholeError {
    QuantumStateCompromised,
    StabilityFailure,
    TunnellingFailed,
    InvalidDestination,
}

#[derive(Debug, Clone)]
pub struct WormholeGlitch {
    position: QuantumCell<Vector3D<f64>>,
    severity: QuantumCell<f64>,
    coherence: QuantumCell<f64>,
}

impl WormholeGlitch {
    pub fn new(position: Vector3D<f64>, severity: f64) -> Self {
        Self {
            position: QuantumCell::new(position),
            severity: QuantumCell::new(severity),
            coherence: QuantumCell::new(1.0),
        }
    }

    pub fn get_position(&self) -> Vector3D<f64> {
        self.position.get()
    }

    pub fn get_severity(&self) -> f64 {
        self.severity.get()
    }

    // Error constructors
    pub fn quantum_state_compromised() -> Self {
        Self::new(Vector3D::new(0.0, 0.0, 0.0), 0.9)
    }

    pub fn stability_failure() -> Self {
        Self::new(Vector3D::new(0.0, 0.0, 0.0), 0.8)
    }

    pub fn tunnelling_failed() -> Self {
        Self::new(Vector3D::new(0.0, 0.0, 0.0), 0.7)
    }

    pub fn invalid_destination() -> Self {
        Self::new(Vector3D::new(0.0, 0.0, 0.0), 0.6)
    }
}

impl From<WormholeError> for WormholeGlitch {
    fn from(error: WormholeError) -> Self {
        match error {
            WormholeError::QuantumStateCompromised => Self::quantum_state_compromised(),
            WormholeError::StabilityFailure => Self::stability_failure(),
            WormholeError::TunnellingFailed => Self::tunnelling_failed(),
            WormholeError::InvalidDestination => Self::invalid_destination(),
        }
    }
}

impl Quantum for WormholeGlitch {
    fn get_coherence(&self) -> f64 {
        self.coherence.get()
    }

    fn is_quantum_stable(&self) -> bool {
        self.coherence.get() > QUANTUM_STABILITY_THRESHOLD
    }

    fn decay_coherence(&self) {
        let current = self.coherence.get();
        let new_coherence = current * COHERENCE_DECAY_FACTOR;
        self.coherence.set(new_coherence.max(0.0).min(1.0));
    }

    fn reset_coherence(&self) {
        self.coherence.set(1.0);
    }
}

impl Scribe for WormholeGlitch {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("WormholeGlitch{pos=");
        output.push_str(&format!("{:?}", self.get_position()));
        output.push_str(", severity=");
        output.push_f64(self.get_severity(), precision.decimal_places());
        output.push_str(", coherence=");
        output.push_f64(self.get_coherence(), precision.decimal_places());
        output.push_char('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glitch_creation() {
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        let glitch = WormholeGlitch::new(pos.clone(), 0.5);
        assert!(glitch.is_quantum_stable());
        assert_eq!(glitch.get_severity(), 0.5);
        assert_eq!(glitch.get_position(), pos);
    }

    #[test]
    fn test_error_conversion() {
        let glitch = WormholeGlitch::from(WormholeError::QuantumStateCompromised);
        assert_eq!(glitch.get_severity(), 0.9);
        assert!(glitch.is_quantum_stable());

        let glitch = WormholeGlitch::from(WormholeError::StabilityFailure);
        assert_eq!(glitch.get_severity(), 0.8);
        assert!(glitch.is_quantum_stable());
    }

    #[test]
    fn test_quantum_scribing() {
        let glitch = WormholeGlitch::new(Vector3D::new(1.0, 2.0, 3.0), 0.5);
        let mut output = QuantumString::new();
        glitch.scribe(ScribePrecision::Standard, &mut output);
        assert!(output.as_str().starts_with("WormholeGlitch{"));
        assert!(output.as_str().contains("severity=0.5"));
    }

    #[test]
    fn test_coherence_decay() {
        let glitch = WormholeGlitch::new(Vector3D::new(1.0, 2.0, 3.0), 0.5);
        let initial = glitch.get_coherence();
        glitch.decay_coherence();
        assert!(glitch.get_coherence() < initial);
    }
}
