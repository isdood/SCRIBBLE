//! Mandelbrot Set Operations for Crystal Lattice HPC Systems
//! ==============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:32:35 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_MANDELBROT_FACTOR,
        QUANTUM_MANDELBROT_THRESHOLD,
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

/// Mandelbrot set parameters with quantum properties
#[derive(Debug, Clone)]
pub struct MandelbrotParams {
    max_iterations: usize,
    escape_radius: f64,
    quantum_threshold: f64,
    phase_shift: f64,
    stability_factor: f64,
    perturbation_strength: f64,
    bulb_detection: bool,
    cardioid_detection: bool,
}

impl Default for MandelbrotParams {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            escape_radius: 2.0,
            quantum_threshold: QUANTUM_STABILITY_THRESHOLD,
            phase_shift: 0.0,
            stability_factor: RESONANCE_FACTOR,
            perturbation_strength: 0.1,
            bulb_detection: true,
            cardioid_detection: true,
        }
    }
}

/// Mandelbrot set variants
#[derive(Debug, Clone, Copy)]
pub enum MandelbrotVariant {
    Standard,
    Quantum,
    Golden,
    Fibonacci,
    Burning,
}

/// Mandelbrot set point state
#[derive(Debug, Clone)]
pub struct MandelbrotState {
    c_real: f64,
    c_imag: f64,
    z_real: f64,
    z_imag: f64,
    iterations: usize,
    stability: f64,
    phase: f64,
    escape_time: Option<usize>,
    orbit_trap: Vec<(f64, f64)>,
}

impl MandelbrotState {
    /// Create new Mandelbrot state
    pub fn new(c_real: f64, c_imag: f64) -> Self {
        Self {
            c_real,
            c_imag,
            z_real: 0.0,
            z_imag: 0.0,
            iterations: 0,
            stability: 1.0,
            phase: 0.0,
            escape_time: None,
            orbit_trap: Vec::new(),
        }
    }

    /// Check if point is stable
    pub fn is_stable(&self) -> bool {
        self.stability >= QUANTUM_MANDELBROT_THRESHOLD
    }

    /// Get escape iteration
    pub fn escape_time(&self) -> Option<usize> {
        self.escape_time
    }

    /// Get orbit trap points
    pub fn orbit_trap(&self) -> &[(f64, f64)] {
        &self.orbit_trap
    }
}

/// Calculate Mandelbrot set iteration with quantum stability
pub fn iterate_mandelbrot(
    mut state: MandelbrotState,
    params: &MandelbrotParams,
    variant: MandelbrotVariant
) -> Result<MandelbrotState, MathError> {
    // Early detection optimizations
    if params.cardioid_detection && is_in_cardioid(state.c_real, state.c_imag)? {
        return Ok(state);
    }
    if params.bulb_detection && is_in_period_2_bulb(state.c_real, state.c_imag)? {
        return Ok(state);
    }

    for i in 0..params.max_iterations {
        if !state.is_stable() {
            return Err(MathError::MandelbrotStabilityLoss);
        }

        state = match variant {
            MandelbrotVariant::Standard => {
                iterate_standard_mandelbrot(state, params)?
            },
            MandelbrotVariant::Quantum => {
                iterate_quantum_mandelbrot(state, params)?
            },
            MandelbrotVariant::Golden => {
                iterate_golden_mandelbrot(state, params)?
            },
            MandelbrotVariant::Fibonacci => {
                iterate_fibonacci_mandelbrot(state, params)?
            },
            MandelbrotVariant::Burning => {
                iterate_burning_mandelbrot(state, params)?
            },
        };

        // Store orbit trap points
        state.orbit_trap.push((state.z_real, state.z_imag));

        if has_escaped(&state, params.escape_radius)? {
            state.escape_time = Some(i);
            break;
        }
    }

    Ok(state)
}

/// Standard Mandelbrot iteration
fn iterate_standard_mandelbrot(
    state: MandelbrotState,
    params: &MandelbrotParams
) -> Result<MandelbrotState, MathError> {
    let new_real = state.z_real * state.z_real - state.z_imag * state.z_imag + state.c_real;
    let new_imag = 2.0 * state.z_real * state.z_imag + state.c_imag;

    Ok(MandelbrotState {
        c_real: state.c_real,
       c_imag: state.c_imag,
       z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * params.stability_factor,
       phase: (state.phase + params.phase_shift * PHASE_MANDELBROT_FACTOR) % TAU,
       escape_time: None,
       orbit_trap: state.orbit_trap,
    })
}

/// Quantum Mandelbrot iteration
fn iterate_quantum_mandelbrot(
    state: MandelbrotState,
    params: &MandelbrotParams
) -> Result<MandelbrotState, MathError> {
    let magnitude = quantum_sqrt(state.z_real * state.z_real + state.z_imag * state.z_imag)?;
    let phase = quantum_pi(state.phase)?;

    let new_real = magnitude * phase.cos() + state.c_real;
    let new_imag = magnitude * phase.sin() + state.c_imag;

    Ok(MandelbrotState {
        c_real: state.c_real,
       c_imag: state.c_imag,
       z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * quantum_resonance(params.stability_factor)?,
       phase: (state.phase + params.phase_shift * PHASE_MANDELBROT_FACTOR) % TAU,
       escape_time: None,
       orbit_trap: state.orbit_trap,
    })
}

/// Golden ratio Mandelbrot iteration
fn iterate_golden_mandelbrot(
    state: MandelbrotState,
    params: &MandelbrotParams
) -> Result<MandelbrotState, MathError> {
    let phi = quantum_phi(1.0)?;
    let new_real = state.z_real * phi + state.c_real;
    let new_imag = state.z_imag * phi + state.c_imag;

    Ok(MandelbrotState {
        c_real: state.c_real,
       c_imag: state.c_imag,
       z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * golden_resonance(params.stability_factor)?,
       phase: (state.phase + params.phase_shift * PHI) % TAU,
       escape_time: None,
       orbit_trap: state.orbit_trap,
    })
}

/// Fibonacci Mandelbrot iteration
fn iterate_fibonacci_mandelbrot(
    state: MandelbrotState,
    params: &MandelbrotParams
) -> Result<MandelbrotState, MathError> {
    let fib_n = quantum_fibonacci(state.iterations + 2)?;
    let new_real = state.z_real * fib_n + state.c_real;
    let new_imag = state.z_imag * fib_n + state.c_imag;

    Ok(MandelbrotState {
        c_real: state.c_real,
       c_imag: state.c_imag,
       z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * fibonacci_resonance(params.stability_factor)?,
       phase: (state.phase + params.phase_shift * fib_n) % TAU,
       escape_time: None,
       orbit_trap: state.orbit_trap,
    })
}

/// Burning Ship Mandelbrot iteration
fn iterate_burning_mandelbrot(
    state: MandelbrotState,
    params: &MandelbrotParams
) -> Result<MandelbrotState, MathError> {
    let new_real = state.z_real.abs() * state.z_real.abs() -
    state.z_imag.abs() * state.z_imag.abs() + state.c_real;
    let new_imag = 2.0 * state.z_real.abs() * state.z_imag.abs() + state.c_imag;

    Ok(MandelbrotState {
        c_real: state.c_real,
       c_imag: state.c_imag,
       z_real: new_real,
       z_imag: new_imag,
       iterations: state.iterations + 1,
       stability: state.stability * params.stability_factor,
       phase: (state.phase + params.phase_shift * PHASE_MANDELBROT_FACTOR) % TAU,
       escape_time: None,
       orbit_trap: state.orbit_trap,
    })
}

/// Check if point is in main cardioid
fn is_in_cardioid(x: f64, y: f64) -> Result<bool, MathError> {
    let q = (x - 0.25).powi(2) + y.powi(2);
    Ok(q * (q + (x - 0.25)) <= 0.25 * y.powi(2))
}

/// Check if point is in period-2 bulb
fn is_in_period_2_bulb(x: f64, y: f64) -> Result<bool, MathError> {
    Ok((x + 1.0).powi(2) + y.powi(2) <= 0.0625)
}

/// Check if point has escaped
fn has_escaped(state: &MandelbrotState, radius: f64) -> Result<bool, MathError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_mandelbrot() {
        let state = MandelbrotState::new(0.0, 0.0);
        let params = MandelbrotParams::default();
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Standard);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quantum_mandelbrot() {
        let state = MandelbrotState::new(0.25, 0.0);
        let params = MandelbrotParams::default();
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Quantum);
        assert!(result.is_ok());
    }

    #[test]
    fn test_golden_mandelbrot() {
        let state = MandelbrotState::new(PHI - 2.0, 0.0);
        let params = MandelbrotParams::default();
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Golden);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fibonacci_mandelbrot() {
        let state = MandelbrotState::new(-1.0, 0.0);
        let params = MandelbrotParams::default();
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Fibonacci);
        assert!(result.is_ok());
    }

    #[test]
    fn test_burning_mandelbrot() {
        let state = MandelbrotState::new(-1.0, -1.0);
        let params = MandelbrotParams::default();
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Burning);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cardioid_detection() {
        assert!(is_in_cardioid(0.0, 0.0).unwrap());
        assert!(!is_in_cardioid(2.0, 2.0).unwrap());
    }

    #[test]
    fn test_period_2_bulb() {
        assert!(is_in_period_2_bulb(-1.0, 0.0).unwrap());
        assert!(!is_in_period_2_bulb(1.0, 1.0).unwrap());
    }

    #[test]
    fn test_orbit_trap() {
        let state = MandelbrotState::new(0.0, 0.0);
        let params = MandelbrotParams::default();
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Standard).unwrap();
        assert!(!result.orbit_trap.is_empty());
    }
}
