/// Quantum Trait Module
/// Last Updated: 2025-01-15 05:38:07 UTC
/// Author: isdood
/// Current User: isdood

use crate::scribe::Scribe;

pub trait Quantum: Scribe {
    fn get_coherence(&self) -> f64;
    fn is_quantum_stable(&self) -> bool;
    fn decay_coherence(&self);
    fn reset_coherence(&self);
}
