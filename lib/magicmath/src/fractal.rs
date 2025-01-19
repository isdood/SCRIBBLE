//! Fractal Operations for Crystal Lattice HPC Systems
//! ========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:34:31 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        QUANTUM_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_FRACTAL_FACTOR,
        QUANTUM_FRACTAL_THRESHOLD,
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
    },
    julia::{self, JuliaParams, JuliaState, JuliaVariant},
    brot::{self, MandelbrotParams, MandelbrotState, MandelbrotVariant}
};

/// Fractal types supported by the system
#[derive(Debug, Clone, Copy)]
pub enum FractalType {
    Julia(JuliaVariant),
    Mandelbrot(MandelbrotVariant),
    Custom,
}

/// Fractal generation parameters
#[derive(Debug, Clone)]
pub struct FractalParams {
    fractal_type: FractalType,
    max_iterations: usize,
    escape_radius: f64,
    quantum_threshold: f64,
    phase_shift: f64,
    stability_factor: f64,
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            fractal_type: FractalType::Mandelbrot(MandelbrotVariant::Standard),
            max_iterations: 1000,
            escape_radius: 2.0,
            quantum_threshold: QUANTUM_STABILITY_THRESHOLD,
            phase_shift: 0.0,
            stability_factor: RESONANCE_FACTOR,
        }
    }
}

/// Fractal generation state
#[derive(Debug, Clone)]
pub enum FractalState {
    Julia(JuliaState),
    Mandelbrot(MandelbrotState),
    Custom(CustomState),
}

/// Custom fractal state
#[derive(Debug, Clone)]
pub struct CustomState {
    z_real: f64,
    z_imag: f64,
    iterations: usize,
    stability: f64,
    phase: f64,
    escape_time: Option<usize>,
}

impl FractalState {
    /// Check if state is stable
    pub fn is_stable(&self) -> bool {
        match self {
            FractalState::Julia(state) => state.is_stable(),
            FractalState::Mandelbrot(state) => state.is_stable(),
            FractalState::Custom(state) => state.stability >= QUANTUM_FRACTAL_THRESHOLD,
        }
    }

    /// Get escape time
    pub fn escape_time(&self) -> Option<usize> {
        match self {
            FractalState::Julia(state) => state.escape_time(),
            FractalState::Mandelbrot(state) => state.escape_time(),
            FractalState::Custom(state) => state.escape_time,
        }
    }
}

/// Generate fractal with specified parameters
pub fn generate_fractal(
    mut state: FractalState,
    params: &FractalParams
) -> Result<FractalState, MathError> {
    match (&mut state, &params.fractal_type) {
        (FractalState::Julia(julia_state), FractalType::Julia(variant)) => {
            let julia_params = JuliaParams {
                max_iterations: params.max_iterations,
                escape_radius: params.escape_radius,
                quantum_threshold: params.quantum_threshold,
                phase_shift: params.phase_shift,
                stability_factor: params.stability_factor,
                ..JuliaParams::default()
            };

            let new_state = julia::iterate_julia(
                julia_state.clone(),
                                                 &julia_params,
                                                 *variant
            )?;
            Ok(FractalState::Julia(new_state))
        },

        (FractalState::Mandelbrot(mandel_state), FractalType::Mandelbrot(variant)) => {
            let mandel_params = MandelbrotParams {
                max_iterations: params.max_iterations,
                escape_radius: params.escape_radius,
                quantum_threshold: params.quantum_threshold,
                phase_shift: params.phase_shift,
                stability_factor: params.stability_factor,
                ..MandelbrotParams::default()
            };

            let new_state = brot::iterate_mandelbrot(
                mandel_state.clone(),
                                                     &mandel_params,
                                                     *variant
            )?;
            Ok(FractalState::Mandelbrot(new_state))
        },

        (FractalState::Custom(custom_state), FractalType::Custom) => {
            iterate_custom_fractal(custom_state.clone(), params)
        },

        _ => Err(MathError::FractalTypeMismatch),
    }
}

/// Custom fractal iteration
fn iterate_custom_fractal(
    state: CustomState,
    params: &FractalParams
) -> Result<FractalState, MathError> {
    // Custom fractal implementation
    let phi = quantum_phi(1.0)?;
    let pi = quantum_pi(1.0)?;
    let fib = quantum_fibonacci(state.iterations + 2)?;

    let new_real = state.z_real * phi * pi + fib;
    let new_imag = state.z_imag * phi * pi + fib;

    let new_state = CustomState {
        z_real: new_real,
        z_imag: new_imag,
        iterations: state.iterations + 1,
        stability: state.stability * params.stability_factor,
        phase: (state.phase + params.phase_shift * PHASE_FRACTAL_FACTOR) % TAU,
        escape_time: if new_real * new_real + new_imag * new_imag > params.escape_radius * params.escape_radius {
            Some(state.iterations)
        } else {
            None
        },
    };

    Ok(FractalState::Custom(new_state))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julia_fractal() {
        let state = FractalState::Julia(JuliaState::new(0.0, 0.0));
        let mut params = FractalParams::default();
        params.fractal_type = FractalType::Julia(JuliaVariant::Standard);

        let result = generate_fractal(state, &params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mandelbrot_fractal() {
        let state = FractalState::Mandelbrot(MandelbrotState::new(0.0, 0.0));
        let mut params = FractalParams::default();
        params.fractal_type = FractalType::Mandelbrot(MandelbrotVariant::Standard);

        let result = generate_fractal(state, &params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_custom_fractal() {
        let state = FractalState::Custom(CustomState {
            z_real: 0.0,
            z_imag: 0.0,
            iterations: 0,
            stability: 1.0,
            phase: 0.0,
            escape_time: None,
        });
        let mut params = FractalParams::default();
        params.fractal_type = FractalType::Custom;

        let result = generate_fractal(state, &params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_mismatch() {
        let state = FractalState::Julia(JuliaState::new(0.0, 0.0));
        let mut params = FractalParams::default();
        params.fractal_type = FractalType::Mandelbrot(MandelbrotVariant::Standard);

        let result = generate_fractal(state, &params);
        assert!(matches!(result, Err(MathError::FractalTypeMismatch)));
    }
}
