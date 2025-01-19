//! Julia Set Operations for Crystal Lattice HPC Systems
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:30:52 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_JULIA_FACTOR,
        QUANTUM_JULIA_THRESHOLD,
        CONVERGENCE_THRESHOLD,
        PHI,
        PI,
        TAU
    },
    traits::MeshValue,
    core::{
        gold::quantum_phi,
        pi::quantum_pi,
        fibb::quantum_fibonacci,
        sqrt::quantum_sqrt,
        log::quantum_ln
    }
};

/// Julia set parameters with quantum properties
#[derive(Debug, Clone)]
pub struct JuliaParams {
    c_real: f64,
    c_imag: f64,
    max_iterations: usize,
    escape_radius: f64,
    quantum_threshold: f64,
    phase_shift: f64,
    stability_factor: f64,
    harmonic_resonance: bool,
}

impl Default for JuliaParams {
    fn default() -> Self {
        Self {
            c_real: -0.4,
            c_imag: 0.6,
            max_iterations: 1000,
            escape_radius: 2.0,
            quantum_threshold: QUANTUM_STABILITY_THRESHOLD,
            phase_shift: 0.0,
            stability_factor: RESONANCE_FACTOR,
            harmonic_resonance: true,
        }
    }
}

/// Julia set variants
#[derive(Debug, Clone, Copy)]
pub enum JuliaVariant {
    Standard,
    Quantum,
    Golden,
    Fibonacci,
    Harmonic,
}

/// Julia set point state
#[derive(Debug, Clone)]
pub struct JuliaState {
    z_real: f64,
    z_imag: f64,
    iterations: usize,
    stability: f64,
    phase: f64,
    escape_time: Option<usize>,
}

impl JuliaState {
    /// Create new Julia state
    pub fn new(z_real: f64, z_imag: f64) -> Self {
        Self {
            z_real,
            z_imag,
            iterations: 0,
            stability: 1.0,
            phase: 0.0,
            escape_time: None,
        }
    }

    /// Check if point is stable
    pub fn is_stable(&self) -> bool {
        self.stability >= QUANTUM_JULIA_THRESHOLD
    }

    /// Get escape iteration
    pub fn escape_time(&self) -> Option<usize> {
        self.escape_time
    }
}

/// Calculate Julia set iteration with quantum stability
pub fn iterate_julia(
    mut state: JuliaState,
    params: &JuliaParams,
    variant: JuliaVariant
) -> Result<JuliaState, MathError> {
    for i in 0..params.max_iterations {
        if !state.is_stable() {
            return Err(MathError::JuliaStabilityLoss);
        }

        state = match variant {
            JuliaVariant::Standard => {
                iterate_standard_julia(state, params)?
            },
            JuliaVariant::Quantum => {
                iterate_quantum_julia(state, params)?
            },
            JuliaVariant::Golden => {
                iterate_golden_julia(state, params)?
            },
            JuliaVariant::Fibonacci => {
                iterate_fibonacci_julia(state, params)?
            },
            JuliaVariant::Harmonic => {
                iterate_harmonic_julia(state, params)?
            },
        };

        if has_escaped(&state, params.escape_radius)? {
            state.escape_time = Some(i);
            break;
        }
    }

    Ok(state)
}

/// Standard Julia iteration
fn iterate_standard_julia(
    state: JuliaState,
    params: &JuliaParams
) -> Result<JuliaState, MathError> {
    let new_real = state.z_real * state.z_real - state.z_imag * state.z_imag + params.c_real;
    let new_imag = 2.0 * state.z_real * state.z_imag + params.c_imag;

    Ok(JuliaState {
        z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * params.stability_factor,
       phase: (state.phase + params.phase_shift * PHASE_JULIA_FACTOR) % TAU,
       escape_time: None,
    })
}

/// Quantum Julia iteration
fn iterate_quantum_julia(
    state: JuliaState,
    params: &JuliaParams
) -> Result<JuliaState, MathError> {
    let magnitude = quantum_sqrt(state.z_real * state.z_real + state.z_imag * state.z_imag)?;
    let phase = quantum_pi(state.phase)?;

    let new_real = magnitude * phase.cos() + params.c_real;
    let new_imag = magnitude * phase.sin() + params.c_imag;

    Ok(JuliaState {
        z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * quantum_resonance(params.stability_factor)?,
       phase: (state.phase + params.phase_shift * PHASE_JULIA_FACTOR) % TAU,
       escape_time: None,
    })
}

/// Golden ratio Julia iteration
fn iterate_golden_julia(
    state: JuliaState,
    params: &JuliaParams
) -> Result<JuliaState, MathError> {
    let phi = quantum_phi(1.0)?;
    let new_real = state.z_real * phi + params.c_real;
    let new_imag = state.z_imag * phi + params.c_imag;

    Ok(JuliaState {
        z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * golden_resonance(params.stability_factor)?,
       phase: (state.phase + params.phase_shift * PHI) % TAU,
       escape_time: None,
    })
}

/// Fibonacci Julia iteration
fn iterate_fibonacci_julia(
    state: JuliaState,
    params: &JuliaParams
) -> Result<JuliaState, MathError> {
    let fib_n = quantum_fibonacci(state.iterations + 2)?;
    let new_real = state.z_real * fib_n + params.c_real;
    let new_imag = state.z_imag * fib_n + params.c_imag;

    Ok(JuliaState {
        z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * fibonacci_resonance(params.stability_factor)?,
       phase: (state.phase + params.phase_shift * fib_n) % TAU,
       escape_time: None,
    })
}

/// Harmonic Julia iteration
fn iterate_harmonic_julia(
    state: JuliaState,
    params: &JuliaParams
) -> Result<JuliaState, MathError> {
    let harmonic = quantum_ln(state.iterations as f64 + 1.0)?;
    let new_real = state.z_real * harmonic.cos() + params.c_real;
    let new_imag = state.z_imag * harmonic.sin() + params.c_imag;

    Ok(JuliaState {
        z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * harmonic_resonance(params.stability_factor)?,
       phase: (state.phase + params.phase_shift * harmonic) % TAU,
       escape_time: None,
    })
}

/// Check if point has escaped
fn has_escaped(state: &JuliaState, radius: f64) -> Result<bool, MathError> {
    Ok(state.z_real * state.z_real + state.z_imag * state.z_imag > radius * radius)
}

/// Calculate quantum resonance
fn quantum_resonance(factor: f64) -> Result<f64, MathError> {
    Ok(factor * (1.0 + quantum_pi(factor)?.sin()))
}

/// Calculate golden resonance
fn golden_resonance(factor: f64) -> Result<f64, MathError> {
    Ok(factor * (1.0 + quantum_phi(factor)?.fract()))
}

/// Calculate Fibonacci resonance
fn fibonacci_resonance(factor: f64) -> Result<f64, MathError> {
    let fib = quantum_fibonacci(factor as usize)?;
    Ok(factor * (1.0 + (fib % 1.0)))
}

/// Calculate harmonic resonance
fn harmonic_resonance(factor: f64) -> Result<f64, MathError> {
    Ok(factor * (1.0 + quantum_ln(factor)?.sin()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_julia() {
        let state = JuliaState::new(0.0, 0.0);
        let params = JuliaParams::default();
        let result = iterate_julia(state, &params, JuliaVariant::Standard);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quantum_julia() {
        let state = JuliaState::new(1.0, 1.0);
        let params = JuliaParams::default();
        let result = iterate_julia(state, &params, JuliaVariant::Quantum);
        assert!(result.is_ok());
    }

    #[test]
    fn test_golden_julia() {
        let state = JuliaState::new(PHI, 0.0);
        let params = JuliaParams::default();
        let result = iterate_julia(state, &params, JuliaVariant::Golden);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fibonacci_julia() {
        let state = JuliaState::new(1.0, 1.0);
        let params = JuliaParams::default();
        let result = iterate_julia(state, &params, JuliaVariant::Fibonacci);
        assert!(result.is_ok());
    }

    #[test]
    fn test_harmonic_julia() {
        let state = JuliaState::new(PI, PI);
        let params = JuliaParams::default();
        let result = iterate_julia(state, &params, JuliaVariant::Harmonic);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stability() {
        let mut state = JuliaState::new(0.0, 0.0);
        assert!(state.is_stable());

        state.stability = 0.0;
        assert!(!state.is_stable());
    }

    #[test]
    fn test_escape_detection() {
        let state = JuliaState::new(10.0, 10.0);
        assert!(has_escaped(&state, 2.0).unwrap());
    }
}
