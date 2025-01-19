//! MeshMath - Core Mathematical Operations Library
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 08:59:17 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]

pub mod constants;
pub mod core;
pub mod errors;
pub mod traits;

// Re-export core functionality at the module level
pub use core::MeshMath;

// Re-export commonly used functions at the top level
pub use core::MeshMath as Math;

// Export core math functions at the module level
pub fn sqrt(x: f64) -> f64 {
    Math::sqrt(x).unwrap_or(0.0)
}

pub fn floor(x: f64) -> f64 {
    Math::floor(x)
}

pub fn ceil(x: f64) -> f64 {
    Math::ceil(x)
}

pub fn round(x: f64) -> f64 {
    Math::round(x)
}

pub fn abs(x: f64) -> f64 {
    Math::abs(x)
}

pub fn exp(x: f64) -> f64 {
    Math::exp(x).unwrap_or(0.0)
}

pub fn ln(x: f64) -> f64 {
    Math::ln(x).unwrap_or(0.0)
}

pub fn pow(x: f64, y: f64) -> f64 {
    Math::pow(x, y).unwrap_or(0.0)
}

// Re-export other items
pub use constants::*;
pub use errors::{MathError, MathResult};
pub use traits::MeshValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        assert!(Math::eq_f64(5.0, 5.0));
        assert_eq!(Math::sqrt(25.0).unwrap(), 5.0);
        assert_eq!(Math::floor(3.7), 3.0);
        assert_eq!(Math::ceil(3.2), 4.0);
        assert_eq!(Math::round(3.5), 4.0);
        assert_eq!(Math::abs(-2.5), 2.5);
    }

    #[test]
    fn test_advanced_math() {
        assert!(Math::eq_f64(Math::sqrt(2.0).unwrap(), SQRT_2));
        assert!(Math::eq_f64(Math::exp(1.0).unwrap(), E));
        assert!(Math::eq_f64(Math::ln(E).unwrap(), 1.0));
        assert!(Math::eq_f64(Math::pow(2.0, 3.0).unwrap(), 8.0));
    }

    #[test]
    fn test_error_handling() {
        assert!(matches!(Math::sqrt(-1.0), Err(MathError::DomainError)));
        assert!(matches!(Math::ln(0.0), Err(MathError::DomainError)));
    }
}
