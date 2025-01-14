/// Quantum UFO States
/// Last Updated: 2025-01-14 22:03:44 UTC
/// Author: isdood
/// Current User: isdood

use crate::constants::QUANTUM_COHERENCE_THRESHOLD;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UFOState {
    Flying,
    Landed,
    Hovering,
    Warping,
    Entangled,
    Decoherent,
    QuantumUncertain,
}

impl UFOState {
    pub fn transition_to_flying(self, coherence: f64) -> Result<Self, &'static str> {
        if coherence < QUANTUM_COHERENCE_THRESHOLD {
            return Err("Insufficient quantum coherence");
        }

        match self {
            UFOState::Landed | UFOState::Hovering => Ok(UFOState::Flying),
            UFOState::Decoherent | UFOState::QuantumUncertain => {
                Err("Quantum state too unstable")
            }
            UFOState::Warping | UFOState::Entangled => {
                Err("Cannot transition while in quantum special state")
            }
            UFOState::Flying => Err("Already in Flying state"),
        }
    }

    pub fn transition_to_landed(self, coherence: f64) -> Result<Self, &'static str> {
        if coherence < QUANTUM_COHERENCE_THRESHOLD {
            return Err("Insufficient quantum coherence");
        }

        match self {
            UFOState::Flying | UFOState::Hovering => Ok(UFOState::Landed),
            UFOState::Decoherent | UFOState::QuantumUncertain => {
                Err("Quantum state too unstable")
            }
            UFOState::Warping | UFOState::Entangled => {
                Err("Cannot transition while in quantum special state")
            }
            UFOState::Landed => Err("Already in Landed state"),
        }
    }

    pub fn transition_to_hovering(self, coherence: f64) -> Result<Self, &'static str> {
        if coherence < QUANTUM_COHERENCE_THRESHOLD {
            return Err("Insufficient quantum coherence");
        }

        match self {
            UFOState::Flying | UFOState::Landed => Ok(UFOState::Hovering),
            UFOState::Decoherent | UFOState::QuantumUncertain => {
                Err("Quantum state too unstable")
            }
            UFOState::Warping | UFOState::Entangled => {
                Err("Cannot transition while in quantum special state")
            }
            UFOState::Hovering => Err("Already in Hovering state"),
        }
    }

    pub fn transition_to_warping(self, coherence: f64) -> Result<Self, &'static str> {
        if coherence < QUANTUM_COHERENCE_THRESHOLD * 1.5 {
            return Err("Insufficient quantum coherence for warp");
        }

        match self {
            UFOState::Flying | UFOState::Hovering => Ok(UFOState::Warping),
            UFOState::Decoherent | UFOState::QuantumUncertain => {
                Err("Quantum state too unstable")
            }
            UFOState::Landed => Err("Cannot warp while landed"),
            UFOState::Entangled => Err("Cannot warp while entangled"),
            UFOState::Warping => Err("Already in Warping state"),
        }
    }

    pub fn handle_decoherence(self, coherence: f64) -> Self {
        if coherence < QUANTUM_COHERENCE_THRESHOLD {
            UFOState::Decoherent
        } else if coherence < QUANTUM_COHERENCE_THRESHOLD * 1.2 {
            UFOState::QuantumUncertain
        } else {
            self
        }
    }

    pub fn is_quantum_stable(self) -> bool {
        match self {
            UFOState::Decoherent | UFOState::QuantumUncertain => false,
            _ => true,
        }
    }

    pub fn requires_coherence(self) -> f64 {
        match self {
            UFOState::Warping => QUANTUM_COHERENCE_THRESHOLD * 1.5,
            UFOState::Entangled => QUANTUM_COHERENCE_THRESHOLD * 1.3,
            UFOState::Flying | UFOState::Hovering => QUANTUM_COHERENCE_THRESHOLD * 1.1,
            UFOState::Landed => QUANTUM_COHERENCE_THRESHOLD,
            UFOState::Decoherent | UFOState::QuantumUncertain => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        let state = UFOState::Landed;
        assert!(state.transition_to_flying(1.0).is_ok());
        assert!(state.transition_to_flying(0.4).is_err());
    }

    #[test]
    fn test_quantum_stability() {
        assert!(UFOState::Flying.is_quantum_stable());
        assert!(!UFOState::Decoherent.is_quantum_stable());
        assert!(!UFOState::QuantumUncertain.is_quantum_stable());
    }

    #[test]
    fn test_coherence_requirements() {
        assert!(UFOState::Warping.requires_coherence() >
        UFOState::Flying.requires_coherence());
        assert!(UFOState::Landed.requires_coherence() ==
        QUANTUM_COHERENCE_THRESHOLD);
    }

    #[test]
    fn test_decoherence_handling() {
        let state = UFOState::Flying;
        assert_eq!(state.handle_decoherence(0.4), UFOState::Decoherent);
        assert_eq!(state.handle_decoherence(0.55), UFOState::QuantumUncertain);
        assert_eq!(state.handle_decoherence(0.9), UFOState::Flying);
    }
}
