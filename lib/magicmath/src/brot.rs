//! Mandelbrot Set Implementation for Crystal Lattice Systems
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 14:37:51 UTC
//! Version: 0.1.0
//! License: MIT

use crate::constants::{
    HARMONY_MANDELBROT_THRESHOLD,
    RESONANCE_FACTOR,
    CONVERGENCE_EPSILON,
    COMPLEX_ITERATION_LIMIT,
    HARMONY_STABILITY_THRESHOLD,
};
use errors::MathError;
use scribe::native_string::String; // Import the correct String type

/// Parameters for Mandelbrot set calculation
#[derive(Debug, Clone, Copy)]
pub struct MandelbrotParams {
    pub max_iterations: usize,
    pub escape_radius: f64,
    pub stability_threshold: f64,
}

impl Default for MandelbrotParams {
    fn default() -> Self {
        Self {
            max_iterations: COMPLEX_ITERATION_LIMIT,
            escape_radius: 2.0,
            stability_threshold: HARMONY_STABILITY_THRESHOLD,
        }
    }
}

/// State for Mandelbrot set calculation
#[derive(Debug, Clone, Copy)]
pub struct MandelbrotState {
    pub x: f64,
    pub y: f64,
    pub iterations: usize,
    pub stability: f64,
    pub escape_time: Option<usize>,
}

impl MandelbrotState {
    /// Create new Mandelbrot state
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
        self.stability >= HARMONY_MANDELBROT_THRESHOLD
    }
}

/// Mandelbrot set calculation variants
#[derive(Debug, Clone, Copy)]
pub enum MandelbrotVariant {
    Standard,
    Optimized,
    HighPrecision,
}

/// Calculate Mandelbrot set membership
#[inline]
pub fn iterate_mandelbrot(
    mut state: MandelbrotState,
    params: &MandelbrotParams,
    variant: MandelbrotVariant,
) -> Result<MandelbrotState, MathError> {
    let mut z_real = 0.0;
    let mut z_imag = 0.0;
    let c_real = state.x;
    let c_imag = state.y;

    for i in 0..params.max_iterations {
        // z = z^2 + c
        let next_z_real = z_real * z_real - z_imag * z_imag + c_real;
        let next_z_imag = 2.0 * z_real * z_imag + c_imag;

        z_real = next_z_real;
        z_imag = next_z_imag;

        let magnitude = z_real * z_real + z_imag * z_imag;

        // Update harmony stability
        state.stability *= RESONANCE_FACTOR;
        state.iterations = i + 1;

        // Check for escape
        if magnitude > params.escape_radius * params.escape_radius {
            state.escape_time = Some(i);
            return Ok(state);
        }

        // Check for stability loss
        if state.stability < HARMONY_MANDELBROT_THRESHOLD {
            return Err(MathError::MandelbrotStabilityLoss(
                String::from("Harmony stability lost during iteration")
            ));
        }

        // Check for convergence
        if magnitude < CONVERGENCE_EPSILON {
            match variant {
                MandelbrotVariant::Standard => break,
                MandelbrotVariant::Optimized => {
                    if i > COMPLEX_ITERATION_LIMIT / 2 {
                        break;
                    }
                }
                MandelbrotVariant::HighPrecision => {
                    if magnitude < CONVERGENCE_EPSILON / 10.0 {
                        break;
                    }
                }
            }
        }
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandelbrot_default_params() {
        let params = MandelbrotParams::default();
        assert_eq!(params.max_iterations, COMPLEX_ITERATION_LIMIT);
        assert_eq!(params.escape_radius, 2.0);
        assert_eq!(params.stability_threshold, HARMONY_STABILITY_THRESHOLD);
    }

    #[test]
    fn test_mandelbrot_state_new() {
        let state = MandelbrotState::new(0.0, 0.0);
        assert_eq!(state.x, 0.0);
        assert_eq!(state.y, 0.0);
        assert_eq!(state.iterations, 0);
        assert_eq!(state.stability, 1.0);
        assert_eq!(state.escape_time, None);
    }

    #[test]
    fn test_mandelbrot_escape_time() {
        let state = MandelbrotState::new(0.0, 0.0);
        assert_eq!(state.escape_time(), None);
    }

    #[test]
    fn test_mandelbrot_is_stable() {
        let state = MandelbrotState::new(0.0, 0.0);
        assert!(state.is_stable());
    }

    #[test]
    fn test_iterate_mandelbrot_standard() {
        let params = MandelbrotParams::default();
        let state = MandelbrotState::new(0.0, 0.0);
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Standard).unwrap();
        assert!(result.is_stable());
    }

    #[test]
    fn test_iterate_mandelbrot_optimized() {
        let params = MandelbrotParams::default();
        let state = MandelbrotState::new(0.0, 0.0);
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::Optimized).unwrap();
        assert!(result.is_stable());
    }

    #[test]
    fn test_iterate_mandelbrot_high_precision() {
        let params = MandelbrotParams::default();
        let state = MandelbrotState::new(0.0, 0.0);
        let result = iterate_mandelbrot(state, &params, MandelbrotVariant::HighPrecision).unwrap();
        assert!(result.is_stable());
    }
}
