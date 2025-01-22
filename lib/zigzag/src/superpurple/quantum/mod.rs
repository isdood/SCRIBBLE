//! Quantum operations module
//! Created: 2025-01-22 00:35:31
//! Author: isdood

use std::sync::Arc;
use num_traits::Float;
use crate::superpurple::core::SIMDValue;

mod operations;
pub use operations::*;

#[derive(Debug, Clone)]
pub struct QuantumState {
    coherence: f64,
    phase: f64,
}

impl QuantumState {
    pub fn new(coherence: f64) -> Self {
        Self {
            coherence,
            phase: 0.0,
        }
    }

    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    pub fn phase(&self) -> f64 {
        self.phase
    }
}

pub trait QuantumOp<T: SIMDValue> {
    fn apply(&self, state: &QuantumState, data: &[T]) -> Vec<T>;
    fn is_unitary(&self) -> bool;
}
