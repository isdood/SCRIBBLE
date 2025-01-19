//! Fibonacci Operations for Crystal Lattice HPC Systems
//! ==========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:21:02 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_SEQUENCE_FACTOR,
        QUANTUM_SEQUENCE_THRESHOLD,
        CONVERGENCE_THRESHOLD,
        PHI,
        FIBONACCI_GROWTH_FACTOR
    },
    traits::MeshValue,
};

/// Quantum-aware Fibonacci calculation for crystal lattice values
/// Handles sequence energy progression and phase alignment
pub fn quantum_fibonacci<T: MeshValue>(n: usize) -> Result<T, MathError> {
    let coherence = calculate_sequence_coherence(n)?;
    if coherence < QUANTUM_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_fibonacci(n)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Calculate Fibonacci with quantum sequence preservation
pub fn sequential_fibonacci<T: MeshValue>(n: usize) -> Result<T, MathError> {
    let sequence = check_sequence_state(n)?;
    if !sequence.is_stable() {
        return Err(MathError::SequenceLoss);
    }

    quantum_fibonacci(n)
}

/// Matrix-based Fibonacci calculation with harmonic progression
pub fn matrix_fibonacci<T: MeshValue>(n: usize) -> Result<T, MathError> {
    let harmonics = calculate_sequence_harmonics(n)?;
    let progression = apply_sequence_progression(n, harmonics)?;

    quantum_fibonacci(progression)
}

/// Calculate Fibonacci with phase progression
pub fn phase_fibonacci<T: MeshValue>(n: usize, phase: f64) -> Result<T, MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_progressed = apply_phase_progression(n, phase)?;
    quantum_fibonacci(phase_progressed)
}

// Internal helper functions

#[inline]
fn calculate_sequence_coherence(n: usize) -> Result<f64, MathError> {
    let base_coherence = (PHI.powi(n as i32) / 5.0f64.sqrt()).abs();
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_fibonacci<T: MeshValue>(n: usize) -> Result<T, MathError> {
    if n <= 1 {
        return Ok(T::from(n as f64));
    }

    let mut a = T::zero();
    let mut b = T::unit();
    let mut sequence_state = SequenceState::new(n);

    for _ in 1..n {
        sequence_state.progress()?;
        let temp = b.clone();
        b = a.raw_add(b)?;
        a = temp;
    }

    Ok(b)
}

#[inline]
fn validate_lattice_bounds<T: MeshValue>(value: T) -> Result<(), MathError> {
    let mag = value.magnitude()?;
    if mag < MIN_LATTICE_SIZE as f64 || mag > MAX_LATTICE_SIZE as f64 {
        return Err(MathError::LatticeBoundsError);
    }
    Ok(())
}

#[inline]
fn check_sequence_state(n: usize) -> Result<SequenceState, MathError> {
    let state = SequenceState::new(n);
    Ok(state)
}

#[inline]
fn calculate_sequence_harmonics(n: usize) -> Result<f64, MathError> {
    let growth = FIBONACCI_GROWTH_FACTOR.powi(n as i32);
    Ok(growth * RESONANCE_FACTOR)
}

#[inline]
fn apply_sequence_progression(n: usize, harmonics: f64) -> Result<usize, MathError> {
    Ok((n as f64 * harmonics) as usize)
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= 2.0 * core::f64::consts::PI
}

#[inline]
fn apply_phase_progression(n: usize, phase: f64) -> Result<usize, MathError> {
    let progressed_phase = phase * PHASE_SEQUENCE_FACTOR;
    Ok((n as f64 * progressed_phase.cos()) as usize)
}

/// Matrix-based Fibonacci calculation
#[inline]
fn matrix_power<T: MeshValue>(n: usize) -> Result<[[T; 2]; 2], MathError> {
    if n == 0 {
        return Ok([[T::unit(), T::zero()],
                  [T::zero(), T::unit()]]);
    }

    let mut result = [[T::unit(), T::unit()],
    [T::unit(), T::zero()]];
    let mut power = n - 1;

    while power > 0 {
        if power % 2 == 1 {
            result = matrix_multiply(result, [[T::unit(), T::unit()],
                                     [T::unit(), T::zero()]])?;
        }
        power /= 2;
    }

    Ok(result)
}

#[inline]
fn matrix_multiply<T: MeshValue>(a: [[T; 2]; 2], b: [[T; 2]; 2]) -> Result<[[T; 2]; 2], MathError> {
    Ok([[a[0][0].raw_mul(b[0][0])?.raw_add(a[0][1].raw_mul(b[1][0])?)?,
       a[0][0].raw_mul(b[0][1])?.raw_add(a[0][1].raw_mul(b[1][1])?)?],
       [a[1][0].raw_mul(b[0][0])?.raw_add(a[1][1].raw_mul(b[1][0])?)?,
       a[1][0].raw_mul(b[0][1])?.raw_add(a[1][1].raw_mul(b[1][1])?)?]])
}

/// Quantum sequence state for Fibonacci calculations
#[derive(Debug, Clone)]
pub struct SequenceState {
    position: usize,
    coherence: f64,
    phase: f64,
    energy: f64,
    sequence_stability: f64,
}

impl SequenceState {
    #[inline]
    pub fn new(n: usize) -> Self {
        Self {
            position: n,
            coherence: 1.0,
            phase: 0.0,
            energy: PHI.powi(n as i32),
            sequence_stability: 1.0,
        }
    }

    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD &&
        self.sequence_stability >= QUANTUM_SEQUENCE_THRESHOLD
    }

    #[inline]
    pub fn progress(&mut self) -> Result<(), MathError> {
        self.coherence *= RESONANCE_FACTOR;
        self.phase += PHASE_SEQUENCE_FACTOR;
        self.energy *= FIBONACCI_GROWTH_FACTOR;
        self.sequence_stability *= RESONANCE_FACTOR;
        Ok(())
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
        fn raw_add(&self, other: Self) -> Result<Self, MathError> { Ok(self + other) }
        fn raw_mul(&self, other: Self) -> Result<Self, MathError> { Ok(self * other) }
        fn from(value: f64) -> Self { value }
        fn zero() -> Self { 0.0 }
        fn unit() -> Self { 1.0 }
        fn clone(&self) -> Self { *self }
    }

    #[test]
    fn test_quantum_fibonacci() {
        let result = quantum_fibonacci::<f64>(10).unwrap();
        assert_eq!(result, 55.0);
    }

    #[test]
    fn test_sequential_fibonacci() {
        let result = sequential_fibonacci::<f64>(10).unwrap();
        assert_eq!(result, 55.0);
    }

    #[test]
    fn test_matrix_fibonacci() {
        let result = matrix_fibonacci::<f64>(10).unwrap();
        assert_eq!(result, 55.0);
    }

    #[test]
    fn test_phase_fibonacci() {
        let result = phase_fibonacci::<f64>(10, 0.0).unwrap();
        assert_eq!(result, 55.0);

        assert!(phase_fibonacci::<f64>(10, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_sequence_state() {
        let mut state = SequenceState::new(10);
        assert!(state.is_stable());

        state.progress().unwrap();
        assert!(state.sequence_stability > 1.0);
    }

    #[test]
    fn test_matrix_power() {
        let result = matrix_power::<f64>(5).unwrap();
        assert_eq!(result[0][0], 8.0);
        assert_eq!(result[0][1], 5.0);
        assert_eq!(result[1][0], 5.0);
        assert_eq!(result[1][1], 3.0);
    }
}
