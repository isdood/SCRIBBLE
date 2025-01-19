//! Core Mathematical Operations for Crystal Lattice HPC Systems
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:22:28 UTC
//! Version: 0.1.0
//! License: MIT

pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod sqrt;
pub mod log;
pub mod pi;
pub mod gold;
pub mod py;
pub mod fibb;

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR
    },
    traits::MeshValue,
};

/// Core quantum state tracking for all mathematical operations
#[derive(Debug, Clone, Copy)]
pub struct QuantumState {
    coherence: f64,
    phase: f64,
    energy: f64,
    stability: f64,
    iterations: usize,
}

impl QuantumState {
    /// Create new quantum state with initial values
    #[inline]
    pub fn new() -> Self {
        Self {
            coherence: 1.0,
            phase: 0.0,
            energy: 1.0,
            stability: 1.0,
            iterations: 0,
        }
    }

    /// Check if quantum state is stable
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD &&
        self.stability >= QUANTUM_STABILITY_THRESHOLD
    }

    /// Update quantum state with new operation
    #[inline]
    pub fn update(&mut self, operation: Operation) -> Result<(), MathError> {
        self.coherence *= operation.coherence_factor();
        self.phase += operation.phase_shift();
        self.energy *= operation.energy_factor();
        self.stability *= RESONANCE_FACTOR;
        self.iterations += 1;

        if !self.is_stable() {
            return Err(MathError::QuantumStateUnstable);
        }
        Ok(())
    }
}

/// Mathematical operations with quantum properties
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    SquareRoot,
    Logarithm,
    Pi,
    Golden,
    Pythagorean,
    Fibonacci,
}

impl Operation {
    /// Get coherence factor for operation
    #[inline]
    fn coherence_factor(&self) -> f64 {
        match self {
            Operation::Add => 1.0,
            Operation::Subtract => 0.95,
            Operation::Multiply => 1.05,
            Operation::Divide => 0.90,
            Operation::SquareRoot => 0.85,
            Operation::Logarithm => 0.80,
            Operation::Pi => 1.10,
            Operation::Golden => 1.15,
            Operation::Pythagorean => 0.95,
            Operation::Fibonacci => 1.20,
        }
    }

    /// Get phase shift for operation
    #[inline]
    fn phase_shift(&self) -> f64 {
        match self {
            Operation::Add => 0.0,
            Operation::Subtract => core::f64::consts::PI,
            Operation::Multiply => core::f64::consts::PI / 2.0,
            Operation::Divide => -core::f64::consts::PI / 2.0,
            Operation::SquareRoot => core::f64::consts::PI / 4.0,
            Operation::Logarithm => -core::f64::consts::PI / 4.0,
            Operation::Pi => 2.0 * core::f64::consts::PI,
            Operation::Golden => (3.0 - 5.0f64.sqrt()) * core::f64::consts::PI,
            Operation::Pythagorean => core::f64::consts::PI / 3.0,
            Operation::Fibonacci => (1.0 - 5.0f64.sqrt()) * core::f64::consts::PI / 2.0,
        }
    }

    /// Get energy factor for operation
    #[inline]
    fn energy_factor(&self) -> f64 {
        match self {
            Operation::Add => 1.1,
            Operation::Subtract => 0.9,
            Operation::Multiply => 1.2,
            Operation::Divide => 0.8,
            Operation::SquareRoot => 0.7,
            Operation::Logarithm => 0.6,
            Operation::Pi => 1.3,
            Operation::Golden => 1.4,
            Operation::Pythagorean => 1.0,
            Operation::Fibonacci => 1.5,
        }
    }
}

/// Core mathematical functions with quantum stability
pub struct QuantumMath {
    state: QuantumState,
}

impl QuantumMath {
    /// Create new quantum math instance
    #[inline]
    pub fn new() -> Self {
        Self {
            state: QuantumState::new(),
        }
    }

    /// Perform operation with quantum stability check
    #[inline]
    pub fn operate<T: MeshValue>(&mut self, operation: Operation, value: T) -> Result<T, MathError> {
        self.state.update(operation)?;

        match operation {
            Operation::Add => add::quantum_add(value, value),
            Operation::Subtract => sub::quantum_sub(value, value),
            Operation::Multiply => mul::quantum_mul(value, value),
            Operation::Divide => div::quantum_div(value, value),
            Operation::SquareRoot => sqrt::quantum_sqrt(value),
            Operation::Logarithm => log::quantum_ln(value),
            Operation::Pi => pi::quantum_pi(value),
            Operation::Golden => gold::quantum_phi(value),
            Operation::Pythagorean => py::quantum_pythagoras(value, value),
            Operation::Fibonacci => fibb::quantum_fibonacci(value.to_usize()?),
        }
    }

    /// Get current quantum state
    #[inline]
    pub fn get_state(&self) -> QuantumState {
        self.state
    }

    /// Reset quantum state
    #[inline]
    pub fn reset_state(&mut self) {
        self.state = QuantumState::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test implementation of MeshValue for f64
    impl MeshValue for f64 {
        fn coherence(&self) -> Result<f64, MathError> { Ok(1.0) }
        fn energy(&self) -> Result<f64, MathError> { Ok(*self) }
        fn magnitude(&self) -> Result<f64, MathError> { Ok(self.abs()) }
        fn to_usize(&self) -> Result<usize, MathError> { Ok(*self as usize) }
    }

    #[test]
    fn test_quantum_state() {
        let mut state = QuantumState::new();
        assert!(state.is_stable());

        state.update(Operation::Add).unwrap();
        assert!(state.is_stable());
    }

    #[test]
    fn test_quantum_math() {
        let mut qmath = QuantumMath::new();

        let result = qmath.operate(Operation::Add, 2.0).unwrap();
        assert_eq!(result, 4.0);

        assert!(qmath.get_state().is_stable());
    }

    #[test]
    fn test_operation_factors() {
        assert!(Operation::Add.coherence_factor() > 0.0);
        assert!(Operation::Add.energy_factor() > 0.0);
    }

    #[test]
    fn test_state_reset() {
        let mut qmath = QuantumMath::new();
        qmath.operate(Operation::Add, 2.0).unwrap();
        qmath.reset_state();

        let state = qmath.get_state();
        assert_eq!(state.iterations, 0);
    }
}
