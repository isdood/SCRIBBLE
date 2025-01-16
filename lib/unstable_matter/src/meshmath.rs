// src/meshmath.rs
/// MeshSpace Mathematics Implementation
/// Last Updated: 2025-01-16 04:14:07 UTC
/// Author: isdood
/// Current User: isdood

#[derive(Debug, Clone, PartialEq)]
pub struct MeshMath;

impl MeshMath {

    /// Normalizes an angle to be within the range [-π, π]
    pub fn normalize_angle(angle: f64) -> f64 {
        let two_pi = 2.0 * std::f64::consts::PI;
        let mut normalized = angle % two_pi;
        if normalized > std::f64::consts::PI {
            normalized -= two_pi;
        } else if normalized < -std::f64::consts::PI {
            normalized += two_pi;
        }
        normalized
    }

    /// Returns the absolute value of a number
    pub fn abs(x: f64) -> f64 {
        if x < 0.0 {
            -x
        } else {
            x
        }
    }

    /// Custom square root implementation for mesh calculations
    pub fn sqrt(x: f64) -> f64 {
        if x < 0.0 {
            panic!("Cannot calculate square root of negative number in meshspace");
        }
        if x == 0.0 {
            return 0.0;
        }

        let mut guess = x / 2.0;
        let mut prev_guess = 0.0;

        while (guess - prev_guess).abs() > 1e-15 {
            prev_guess = guess;
            guess = (guess + x / guess) / 2.0;
        }

        guess
    }

    /// Custom sine implementation using Taylor series
    pub fn sin(x: f64) -> f64 {
        // Normalize angle to -2π to 2π range
        let x = x % (2.0 * std::f64::consts::PI);

        let mut result = 0.0;
        let mut term = x;
        let mut n = 1;

        for i in 0..12 { // 12 terms for good precision
            if i > 0 {
                term = -term * x * x / ((2 * n) * (2 * n + 1)) as f64;
                n += 1;
            }
            result += term;
        }

        result
    }

    /// Custom cosine implementation using sin(x + π/2)
    pub fn cos(x: f64) -> f64 {
        Self::sin(x + std::f64::consts::PI / 2.0)
    }

    /// Custom exponential function implementation
    pub fn exp(x: f64) -> f64 {
        let mut result = 1.0f64;
        let mut term: f64 = 1.0;
        let mut n = 1;

        while term.abs() > 1e-15 && n < 100 {
            term *= x / n as f64;
            result += term;
            n += 1;
        }

        result
    }

    /// Custom natural logarithm implementation
    pub fn ln(x: f64) -> f64 {
        if x <= 0.0 {
            panic!("Cannot calculate natural logarithm of non-positive number in meshspace");
        }

        let mut result = 0.0f64;
        let y = (x - 1.0) / (x + 1.0);  // Removed mut as it's not needed
        let mut power = y;
        let mut n = 1;

        while power.abs() > 1e-15 && n < 100 {
            result += power / n as f64;
            power *= y * y;
            n += 2;
        }

        2.0 * result
    }

    /// Custom power function implementation
    pub fn pow(x: f64, y: f64) -> f64 {
        Self::exp(y * Self::ln(x))
    }

    /// Calculate hyperbolic sine
    pub fn sinh(x: f64) -> f64 {
        (Self::exp(x) - Self::exp(-x)) / 2.0
    }

    /// Calculate hyperbolic cosine
    pub fn cosh(x: f64) -> f64 {
        (Self::exp(x) + Self::exp(-x)) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_sqrt() {
        assert!((MeshMath::sqrt(4.0) - 2.0).abs() < 1e-10);
        assert!((MeshMath::sqrt(2.0) - std::f64::consts::SQRT_2).abs() < 1e-10);
    }

    #[test]
    fn test_sin() {
        assert!((MeshMath::sin(PI/2.0) - 1.0).abs() < 1e-10);
        assert!((MeshMath::sin(PI) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_cos() {
        assert!((MeshMath::cos(0.0) - 1.0).abs() < 1e-10);
        assert!((MeshMath::cos(PI) + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_exp() {
        assert!((MeshMath::exp(1.0) - std::f64::consts::E).abs() < 1e-10);
        assert!((MeshMath::exp(0.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln() {
        assert!((MeshMath::ln(std::f64::consts::E) - 1.0).abs() < 1e-10);
        assert!((MeshMath::ln(1.0) - 0.0).abs() < 1e-10);
    }
}
