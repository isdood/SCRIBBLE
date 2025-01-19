//! Fractal Generation for Crystal Lattice Systems
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 18:51:07 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::{
        HARMONY_FRACTAL_THRESHOLD,
        RESONANCE_FACTOR,
        CONVERGENCE_EPSILON,
        COMPLEX_ITERATION_LIMIT,
        HARMONY_STABILITY_THRESHOLD,
    },
    brot::{self, MandelbrotParams, MandelbrotState, MandelbrotVariant},
    julia::{self, JuliaParams, JuliaState, JuliaVariant},
};
use errors::core::MathError;
use scribe::native_string::String;

/// Parameters for fractal generation
#[derive(Debug, Clone, Copy)]
pub struct FractalParams {
    max_iterations: usize,
    escape_radius: f64,
    stability_threshold: f64,
    phase_shift: f64,
    stability_factor: f64,
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            max_iterations: COMPLEX_ITERATION_LIMIT,
            escape_radius: 2.0,
            stability_threshold: HARMONY_STABILITY_THRESHOLD,
            phase_shift: 0.0,
            stability_factor: RESONANCE_FACTOR,
        }
    }
}

/// Custom fractal state
#[derive(Debug, Clone, Copy)]
pub struct CustomState {
    pub z_real: f64,
    pub z_imag: f64,
    pub iterations: usize,
    pub stability: f64,
    pub phase: f64,
    pub escape_time: Option<usize>,
}

/// Fractal state variants
#[derive(Debug, Clone, Copy)]
pub enum FractalState {
    Julia(JuliaState),
    Mandelbrot(MandelbrotState),
    Custom(CustomState),
}

impl FractalState {
    /// Get current escape time
    #[inline]
    pub fn escape_time(&self) -> Option<usize> {
        match self {
            FractalState::Julia(state) => state.escape_time(),
            FractalState::Mandelbrot(state) => state.escape_time(),
            FractalState::Custom(state) => state.escape_time,
        }
    }

    /// Check if state is stable
    #[inline]
    pub fn is_stable(&self) -> bool {
        match self {
            FractalState::Julia(state) => state.is_stable(),
            FractalState::Mandelbrot(state) => state.is_stable(),
            FractalState::Custom(state) => state.stability >= HARMONY_FRACTAL_THRESHOLD,
        }
    }
}

/// Fractal type variants
#[derive(Debug, Clone, Copy)]
pub enum FractalType {
    Julia,
    Mandelbrot,
    Custom,
}

/// Generate fractal based on state and parameters
#[inline]
pub fn generate_fractal(
    state: FractalState,
    params: &FractalParams,
) -> Result<FractalState, MathError> {
    match state {
        FractalState::Julia(julia_state) => {
            let julia_params = JuliaParams {
                max_iterations: params.max_iterations,
                escape_radius: params.escape_radius,
                stability_threshold: params.stability_threshold,
                c_real: -0.4,
                c_imag: 0.6,
            };
            let result = julia::iterate_julia(julia_state, &julia_params, JuliaVariant::Standard)?;
            Ok(FractalState::Julia(result))
        }
        FractalState::Mandelbrot(mandel_state) => {
            let mandel_params = MandelbrotParams {
                max_iterations: params.max_iterations,
                escape_radius: params.escape_radius,
                stability_threshold: params.stability_threshold,
            };
            let result = brot::iterate_mandelbrot(mandel_state, &mandel_params, MandelbrotVariant::Standard)?;
            Ok(FractalState::Mandelbrot(result))
        }
        FractalState::Custom(mut custom_state) => {
            for i in 0..params.max_iterations {
                let z_real = custom_state.z_real;
                let z_imag = custom_state.z_imag;

                // Custom iteration formula
                let next_z_real = z_real * z_real - z_imag * z_imag + params.phase_shift;
                let next_z_imag = 2.0 * z_real * z_imag;

                custom_state.z_real = next_z_real;
                custom_state.z_imag = next_z_imag;
                custom_state.iterations = i + 1;
                custom_state.stability *= params.stability_factor;
                custom_state.phase += params.phase_shift;

                let magnitude = next_z_real * next_z_real + next_z_imag * next_z_imag;

                if magnitude > params.escape_radius * params.escape_radius {
                    custom_state.escape_time = Some(i);
                    break;
                }

                if custom_state.stability < HARMONY_FRACTAL_THRESHOLD {
                    return Err(MathError::FractalStabilityLoss(
                        String::from("Harmony stability lost during custom fractal iteration")
                    ));
                }

                if magnitude < CONVERGENCE_EPSILON {
                    break;
                }
            }
            Ok(FractalState::Custom(custom_state))
        }
    }
}
