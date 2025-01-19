//! Core Math Implementation Module
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 08:55:47 UTC
//! Last Updated: 2025-01-19 08:55:47 UTC
//! Version: 0.1.0
//! License: MIT

use super::constants::*;
use super::errors::{MathError, MathResult};

/// Core mathematical operations implementation
#[derive(Debug, Clone, Copy)]
pub struct MeshMath;

impl MeshMath {
    /// Convert isize to f64
    #[inline(always)]
    pub fn isize_to_f64(value: isize) -> f64 {
        value as f64
    }

    /// Compare two f64 values for equality with epsilon
    #[inline(always)]
    pub fn eq_f64(a: f64, b: f64) -> bool {
        Self::abs(a - b) < EPSILON
    }

    /// Calculate absolute value
    #[inline(always)]
    pub fn abs(x: f64) -> f64 {
        if x < 0.0 { -x } else { x }
    }

    /// Calculate square root using Newton's method
    #[inline]
    pub fn sqrt(x: f64) -> MathResult<f64> {
        if x < 0.0 {
            return Err(MathError::DomainError);
        }
        if x == 0.0 || x == 1.0 {
            return Ok(x);
        }

        let mut guess = x / 2.0;
        for _ in 0..MAX_ITERATIONS {
            let next = (guess + x / guess) / 2.0;
            if Self::eq_f64(next, guess) {
                return Ok(guess);
            }
            guess = next;
        }
        Err(MathError::ConvergenceFailure)
    }

    /// Floor function
    #[inline]
    pub fn floor(x: f64) -> f64 {
        let int_part = x as i64 as f64;
        if x < 0.0 && !Self::eq_f64(x, int_part) {
            int_part - 1.0
        } else {
            int_part
        }
    }

    /// Ceiling function
    #[inline]
    pub fn ceil(x: f64) -> f64 {
        let floor = Self::floor(x);
        if Self::eq_f64(floor, x) { floor } else { floor + 1.0 }
    }

    /// Round function
    #[inline]
    pub fn round(x: f64) -> f64 {
        Self::floor(x + 0.5)
    }

    /// Natural logarithm using improved algorithm
    #[inline]
    pub fn ln(x: f64) -> MathResult<f64> {
        if x <= 0.0 {
            return Err(MathError::DomainError);
        }
        if x == 1.0 {
            return Ok(0.0);
        }

        let mut n = 0;
        let mut x_scaled = x;

        // Scale x into [1/√2, √2]
        while x_scaled >= SQRT_2 {
            x_scaled *= 0.5;
            n += 1;
        }
        while x_scaled < 1.0/SQRT_2 {
            x_scaled *= 2.0;
            n -= 1;
        }

        // Use optimized series for ln(1 + y)
        let y = x_scaled - 1.0;
        let y2 = y * y;
        let mut sum = y;
        let mut term = y;

        for i in 1..MAX_ITERATIONS {
            term = -term * y * (2 * i - 1) as f64 / (2 * i) as f64;
            let next = sum + term;
            if Self::eq_f64(next, sum) {
                return Ok(sum + n as f64 * LN_2);
            }
            sum = next;
        }

        Err(MathError::ConvergenceFailure)
    }

    /// Exponential function using improved series
    #[inline]
    pub fn exp(x: f64) -> MathResult<f64> {
        if x == 0.0 {
            return Ok(1.0);
        }

        // Handle large values
        if x > 709.0 {
            return Err(MathError::Overflow);
        }
        if x < -745.0 {
            return Err(MathError::Underflow);
        }

        let mut sum = 1.0;
        let mut term = 1.0;

        for i in 1..MAX_ITERATIONS {
            term *= x / i as f64;
            let next = sum + term;
            if Self::eq_f64(next, sum) {
                return Ok(sum);
            }
            sum = next;
        }

        Err(MathError::ConvergenceFailure)
    }

    /// Power function using exp(ln(x) * y)
    #[inline]
    pub fn pow(x: f64, y: f64) -> MathResult<f64> {
        if y == 0.0 {
            return Ok(1.0);
        }
        if x == 0.0 {
            return if y > 0.0 { Ok(0.0) } else { Err(MathError::DomainError) };
        }

        let ln_x = Self::ln(x)?;
        Self::exp(ln_x * y)
    }
}
