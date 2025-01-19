//! Division Operations for MeshMath
//! =========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 10:01:28 UTC
//! Last Updated: 2025-01-19 10:01:28 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::Div;
use crate::errors::MathError;

/// Divide two values with error checking
pub fn checked_div<T: Div<Output = T> + PartialOrd + PartialEq + Copy>(a: T, b: T, zero: T) -> Result<T, MathError> {
    if b == zero {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

/// Safe division with default value
pub fn safe_div<T: Div<Output = T> + PartialEq + Copy>(a: T, b: T, zero: T, default: T) -> T {
    if b == zero {
        default
    } else {
        a / b
    }
}

/// Divide value by power of two
pub fn div_pow2<T: Div<Output = T> + Copy>(value: T, power: u32) -> T {
    let mut result = value;
    for _ in 0..power {
        result = result / (T::from(2));
    }
    result
}

impl From<u32> for T {
    fn from(value: u32) -> Self {
        // Implementation depends on type T
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_div() {
        assert!(checked_div(4.0, 2.0, 0.0).is_ok());
        assert_eq!(checked_div(4.0, 2.0, 0.0).unwrap(), 2.0);
        assert!(checked_div(1.0, 0.0, 0.0).is_err());
    }

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(4.0, 2.0, 0.0, 1.0), 2.0);
        assert_eq!(safe_div(1.0, 0.0, 0.0, 1.0), 1.0);
    }

    #[test]
    fn test_div_pow2() {
        assert_eq!(div_pow2(8.0, 2), 2.0);
    }
}
