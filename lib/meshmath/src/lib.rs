//! MeshMath - Core Mathematical Operations Module
//! ==========================================
//!
//! Native implementation of core mathematical operations for
//! mesh-based quantum computations.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 08:52:21 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]

/// Implementation of fundamental mathematical constants
pub mod constants {
    /// π (pi)
    pub const PI: f64 = 3.141592653589793238462643383279502884;

    /// e (Euler's number)
    pub const E: f64 = 2.718281828459045235360287471352662498;

    /// √2 (square root of 2)
    pub const SQRT_2: f64 = 1.414213562373095048801688724209698079;

    /// Machine epsilon for f64
    pub const EPSILON: f64 = 2.220446049250313e-16;
}

/// Core mathematical operations for mesh-based computations
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
        Self::abs(a - b) < constants::EPSILON
    }

    /// Calculate absolute value
    #[inline(always)]
    pub fn abs(x: f64) -> f64 {
        if x < 0.0 { -x } else { x }
    }

    /// Calculate square root using Newton's method
    #[inline]
    pub fn sqrt(x: f64) -> f64 {
        if x < 0.0 { return 0.0; }
        if x == 0.0 || x == 1.0 { return x; }

        let mut guess = x / 2.0;
        for _ in 0..10 {  // Usually converges in <10 iterations
            let next = (guess + x / guess) / 2.0;
            if Self::eq_f64(next, guess) {
                return guess;
            }
            guess = next;
        }
        guess
    }

    /// Floor function using bit manipulation
    #[inline]
    pub fn floor(x: f64) -> f64 {
        let mut int_part = x as i64 as f64;
        if x < 0.0 && !Self::eq_f64(x, int_part) {
            int_part -= 1.0;
        }
        int_part
    }

    /// Ceiling function using floor
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

    /// Power function using exponential series
    #[inline]
    pub fn pow(x: f64, y: f64) -> f64 {
        if y == 0.0 { return 1.0; }
        if x == 0.0 { return 0.0; }
        if y == 1.0 { return x; }
        if y == -1.0 { return 1.0 / x; }

        let abs_y = Self::abs(y);
        let int_y = Self::floor(abs_y);
        let fract_y = abs_y - int_y;

        let mut result = 1.0;
        let mut current = x;
        let mut n = int_y;

        // Handle integer part
        while n > 0.0 {
            if n - 2.0 * Self::floor(n / 2.0) > 0.0 {
                result *= current;
            }
            current *= current;
            n = Self::floor(n / 2.0);
        }

        // Handle fractional part using exp(ln(x) * y)
        if fract_y > 0.0 {
            result *= Self::exp(Self::ln(x) * fract_y);
        }

        if y < 0.0 { 1.0 / result } else { result }
    }

    /// Natural logarithm using Taylor series
    #[inline]
    pub fn ln(x: f64) -> f64 {
        if x <= 0.0 { return 0.0; } // Should be NaN, but keeping no_std compatible
        if x == 1.0 { return 0.0; }

        // Scale input for better convergence
        let mut n = 0;
        let mut x_scaled = x;
        while x_scaled >= 2.0 {
            x_scaled /= 2.0;
            n += 1;
        }
        while x_scaled < 1.0 {
            x_scaled *= 2.0;
            n -= 1;
        }

        // Use Taylor series for ln(1 + x)
        let y = x_scaled - 1.0;
        let mut term = y;
        let mut sum = term;
        let mut k = 2;

        while !Self::eq_f64(term, 0.0) && k < 30 {
            term = -term * y * (k - 1) as f64 / k as f64;
            sum += term;
            k += 1;
        }

        sum + n as f64 * constants::LN_2
    }

    /// Exponential function using Taylor series
    #[inline]
    pub fn exp(x: f64) -> f64 {
        if x == 0.0 { return 1.0; }

        let mut sum = 1.0;
        let mut term = 1.0;
        let mut n = 1;

        while !Self::eq_f64(term, 0.0) && n < 30 {
            term *= x / n as f64;
            sum += term;
            n += 1;
        }

        sum
    }
}

/// Trait for mesh-compatible numeric types
pub trait MeshValue: Copy + Clone + core::fmt::Debug {
    fn mesh_add(self, other: Self) -> Self;
    fn mesh_sub(self, other: Self) -> Self;
    fn mesh_mul(self, other: Self) -> Self;
    fn mesh_div(self, other: Self) -> Self;
    fn mesh_neg(self) -> Self;
    fn mesh_magnitude(self) -> f64;
    fn mesh_normalize(self) -> Self;
    fn mesh_zero() -> Self;
    fn mesh_one() -> Self;
    fn as_f64(self) -> f64;
    fn from_f64(value: f64) -> Self;
}

// Implementation for f64
impl MeshValue for f64 {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self { self + other }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self { self - other }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self { self * other }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        if MeshMath::eq_f64(other, 0.0) { 0.0 } else { self / other }
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self { -self }

    #[inline(always)]
    fn mesh_magnitude(self) -> f64 { MeshMath::abs(self) }

    #[inline(always)]
    fn mesh_normalize(self) -> Self {
        if MeshMath::eq_f64(self, 0.0) { 0.0 } else { self / MeshMath::abs(self) }
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0.0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1.0 }

    #[inline(always)]
    fn as_f64(self) -> f64 { self }

    #[inline(always)]
    fn from_f64(value: f64) -> Self { value }
}

// Implementation for isize
impl MeshValue for isize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self { self + other }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self { self - other }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self { self * other }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        if other == 0 { 0 } else { self / other }
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self { -self }

    #[inline(always)]
    fn mesh_magnitude(self) -> f64 { self.abs() as f64 }

    #[inline(always)]
    fn mesh_normalize(self) -> Self {
        if self == 0 { 0 } else { self / self.abs() }
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1 }

    #[inline(always)]
    fn as_f64(self) -> f64 { self as f64 }

    #[inline(always)]
    fn from_f64(value: f64) -> Self { value as isize }
}

// Implementation for usize
impl MeshValue for usize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self { self + other }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        if other > self { 0 } else { self - other }
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self { self * other }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        if other == 0 { 0 } else { self / other }
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self { 0 }

    #[inline(always)]
    fn mesh_magnitude(self) -> f64 { self as f64 }

    #[inline(always)]
    fn mesh_normalize(self) -> Self {
        if self == 0 { 0 } else { 1 }
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1 }

    #[inline(always)]
    fn as_f64(self) -> f64 { self as f64 }

    #[inline(always)]
    fn from_f64(value: f64) -> Self { value as usize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        assert_eq!(MeshMath::isize_to_f64(5), 5.0);
        assert!(MeshMath::eq_f64(5.0, 5.0));
        assert!(MeshMath::eq_f64(MeshMath::sqrt(25.0), 5.0));
        assert_eq!(MeshMath::floor(3.7), 3.0);
        assert_eq!(MeshMath::ceil(3.2), 4.0);
        assert_eq!(MeshMath::round(3.5), 4.0);
        assert_eq!(MeshMath::abs(-2.5), 2.5);
    }

    #[test]
    fn test_advanced_math() {
        // Test sqrt with various inputs
        assert!(MeshMath::eq_f64(MeshMath::sqrt(2.0), constants::SQRT_2));
        assert!(MeshMath::eq_f64(MeshMath::sqrt(4.0), 2.0));
        assert_eq!(MeshMath::sqrt(-1.0), 0.0);

        // Test power function
        assert!(MeshMath::eq_f64(MeshMath::pow(2.0, 3.0), 8.0));
        assert!(MeshMath::eq_f64(MeshMath::pow(2.0, -1.0), 0.5));
        assert!(MeshMath::eq_f64(MeshMath::pow(2.0, 0.5), constants::SQRT_2));

        // Test natural log
        assert!(MeshMath::eq_f64(MeshMath::ln(constants::E), 1.0));
        assert!(MeshMath::eq_f64(MeshMath::ln(1.0), 0.0));

        // Test exp
        assert!(MeshMath::eq_f64(MeshMath::exp(1.0), constants::E));
        assert!(MeshMath::eq_f64(MeshMath::exp(0.0), 1.0));
    }

    #[test]
    fn test_mesh_value_implementations() {
        // Test f64
        assert_eq!(5.0_f64.mesh_add(3.0), 8.0);
        assert_eq!(5.0_f64.mesh_sub(3.0), 2.0);
        assert_eq!(5.0_f64.mesh_mul(3.0), 15.0);
        assert_eq!(15.0_f64.mesh_div(3.0), 5.0);

        // Test isize
        assert_eq!(5_isize.mesh_add(3), 8);
        assert_eq!(5_isize.mesh_sub(3), 2);
        assert_eq!(5_isize.mesh_mul(3), 15);
        assert_eq!(15_isize.mesh_div(3), 5);

        // Test usize
        assert_eq!(5_usize.mesh_add(3), 8);
        assert_eq!(5_usize.mesh_sub(3), 2);
        assert_eq!(5_usize.mesh_mul(3), 15);
        assert_eq!(15_usize.mesh_div(3), 5);
    }
}
