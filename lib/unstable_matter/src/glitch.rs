/// Quantum Glitch Types
/// Last Updated: 2025-01-15 01:49:27 UTC
/// Author: isdood
/// Current User: isdood

use crate::scribe::{Scribe, ScribePrecision, QuantumString};

#[derive(Debug)]
pub enum WormholeGlitch {
    StabilityFailure,
    QuantumStateCompromised,
    TunnellingFailed,
    InvalidDestination,
}

impl Scribe for WormholeGlitch {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        let message = match self {
            WormholeGlitch::StabilityFailure => "Wormhole stability compromised",
            WormholeGlitch::QuantumStateCompromised => "Quantum state is unstable",
            WormholeGlitch::TunnellingFailed => "Tunnelling operation failed",
            WormholeGlitch::InvalidDestination => "Invalid destination coordinates",
        };
        output.push_str(message);
    }
}
