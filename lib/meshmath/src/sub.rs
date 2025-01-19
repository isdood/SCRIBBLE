//! Subtraction Operations for MeshMath
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 10:01:28 UTC
//! Last Updated: 2025-01-19 10:01:28 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::Sub;
use crate::errors::MathError;

/// Subtract two values with underflow checking
pub fn checked_sub<T: Sub<Output = T> + PartialOrd + Copy>(a: T, b: T, min: T) -> Result<T, MathError> {
    let result = a - b;
    if result < min {
        return Err(MathError::Underflow);
    }
    Ok(result)
}

/// Subtract with saturation at bounds
pub fn saturating_sub<T: Sub<Output = T> + PartialOrd + Copy>(a: T, b: T, min: T) -> T {
    let result = a - b;
    if result < min {
        min
    } else {
        result
    }
}

/// Calculate absolute difference
pub fn abs_diff<T: Sub<Output = T> + PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_sub() {
        assert!(checked_sub(3.0, 2.0, 0.0).is_ok());
        assert_eq!(checked_sub(3.0, 2.0, 0.0).unwrap(), 1.0);
        assert!(checked_sub(1.0, 2.0, 0.0).is_err());
    }

    #[test]
    fn test_saturating_sub() {
        assert_eq!(saturating_sub(3.0, 2.0, 0.0), 1.0);
        assert_eq!(saturating_sub(1.0, 2.0, 0.0), 0.0);
    }

    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(3.0, 1.0), 2.0);
        assert_eq!(abs_diff(1.0, 3.0), 2.0);
    }
}
