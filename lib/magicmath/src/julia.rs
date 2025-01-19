//! Julia Set Implementation for Crystal Lattice Systems
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 14:39:17 UTC
//! Version: 0.1.0
//! License: MIT

use crate::constants::{
    QUANTUM_JULIA_THRESHOLD,
    RESONANCE_FACTOR,
    CONVERGENCE_EPSILON,
    COMPLEX_ITERATION_LIMIT,
    QUANTUM_STABILITY_THRESHOLD,
};
use errors::core::MathError;

/// Parameters for Julia set calculation
#[derive(Debug, Clone, Copy)]
pub struct JuliaParams {
    pub max_iterations: usize,
    pub escape_radius: f64,
    pub stability_threshold: f64,
    pub c_real: f64,
    pub c_imag: f64,
}

impl Default for JuliaParams {
    fn default() -> Self {
        Self {
            max_iterations: COMPLEX_ITERATION_LIMIT,
            escape_radius: 2.0,
            stability_threshold: QUANTUM_STABILITY_THRESHOLD,
            c_real: -0.4,
            c_imag: 0.6,
        }
    }
}

/// State for Julia set calculation
#[derive(Debug, Clone, Copy)]
pub struct JuliaState {
    pub x: f64,
    pub y: f64,
    pub iterations: usize,
    pub stability: f64,
    pub escape_time: Option<usize>,
}

impl JuliaState {
    /// Create new Julia state
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            iterations: 0,
            stability: 1.0,
            escape_time: None,
        }
    }

    /// Get current escape time
    #[inline]
    pub fn escape_time(&self) -> Option<usize> {
        self.escape_time
    }

    /// Check if state is stable
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.stability >= QUANTUM_JULIA_THRESHOLD
    }
}

/// Julia set calculation variants
#[derive(Debug, Clone, Copy)]
pub enum JuliaVariant {
    Standard,
    Optimized,
    HighPrecision,
}

/// Calculate Julia set membership
#[inline]
pub fn iterate_julia(
    mut state: JuliaState,
    params: &JuliaParams,
    variant: JuliaVariant,
) -> Result<JuliaState, MathError> {
    let mut z_real = state.x;
    let mut z_imag = state.y;
    let c_real = params.c_real;
    let c_imag = params.c_imag;

    for i in 0..params.max_iterations {
        // z = z^2 + c
        let next_z_real = z_real * z_real - z_imag * z_imag + c_real;
        let next_z_imag = 2.0 * z_real * z_imag + c_imag;

        z_real = next_z_real;
        z_imag = next_z_imag;

        let magnitude = z_real * z_real + z_imag * z_imag;

        // Update quantum stability
        state.stability *= RESONANCE_FACTOR;
        state.iterations = i + 1;

        // Check for escape
        if magnitude > params.escape_radius * params.escape_radius {
            state.escape_time = Some(i);
            return Ok(state);
        }

        // Check for stability loss
        if state.stability < QUANTUM_JULIA_THRESHOLD {
            return Err(MathError::JuliaStabilityLoss(
                "Quantum stability lost during iteration".to_string()
            ));
        }

        // Check for convergence
        if magnitude < CONVERGENCE_EPSILON {
            match variant {
                JuliaVariant::Standard => break,
                JuliaVariant::Optimized => {
                    if i > COMPLEX_ITERATION_LIMIT / 2 {
                        break;
                    }
                }
                JuliaVariant::HighPrecision => {
                    if magnitude < CONVERGENCE_EPSILON / 10.0 {
                        break;
                    }
                }
            }
        }
    }

    Ok(state)
}
