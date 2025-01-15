/// Quantum Glitch Module
/// Last Updated: 2025-01-15 05:25:49 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    vector::Vector3D,
    quantum::Quantum,
    scribe::{Scribe, ScribePrecision, QuantumString},
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
    position: Vector3D<f64>,
    severity: f64,
    coherence: f64,
}

impl WormholeGlitch {
    pub fn new(position: Vector3D<f64>, severity: f64) -> Self {
        Self {
            position,
            severity,
            coherence: 1.0,
        }
    }

    pub fn get_position(&self) -> &Vector3D<f64> {
        &self.position
    }

    pub fn get_severity(&self) -> f64 {
        self.severity
    }

    // Static error constructors
    pub const QuantumStateCompromised: WormholeError = WormholeError::QuantumStateCompromised;
    pub const StabilityFailure: WormholeError = WormholeError::StabilityFailure;
    pub const TunnellingFailed: WormholeError = WormholeError::TunnellingFailed;
    pub const InvalidDestination: WormholeError = WormholeError::InvalidDestination;
}

impl Quantum for WormholeGlitch {
    fn get_coherence(&self) -> f64 {
        self.coherence
    }

    fn is_quantum_stable(&self) -> bool {
        self.coherence > 0.5
    }

    fn decay_coherence(&self) {
        // Implementation here
    }

    fn reset_coherence(&self) {
        // Implementation here
    }
}

impl Scribe for WormholeGlitch {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("WormholeGlitch{pos=");
        self.position.scribe(precision, output);
        output.push_str(", severity=");
        output.push_f64(self.severity, precision.decimal_places());
        output.push_str(", coherence=");
        output.push_f64(self.coherence, precision.decimal_places());
        output.push_char('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glitch_creation() {
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        let glitch = WormholeGlitch::new(pos, 0.5);
        assert!(glitch.is_quantum_stable());
        assert_eq!(glitch.get_severity(), 0.5);
    }

    #[test]
    fn test_error_types() {
        assert!(matches!(WormholeGlitch::QuantumStateCompromised,
                         WormholeError::QuantumStateCompromised));
        assert!(matches!(WormholeGlitch::StabilityFailure,
                         WormholeError::StabilityFailure));
    }
}
