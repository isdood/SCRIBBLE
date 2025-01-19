//! Multiplication Operations for MeshMath
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 10:01:28 UTC
//! Last Updated: 2025-01-19 10:01:28 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::Mul;
use crate::errors::MathError;

/// Multiply two values with overflow checking
pub fn checked_mul<T: Mul<Output = T> + PartialOrd + Copy>(a: T, b: T, max: T) -> Result<T, MathError> {
    let result = a * b;
    if result > max {
        return Err(MathError::Overflow);
    }
    Ok(result)
}

/// Multiply array of values with overflow checking
pub fn product<T: Mul<Output = T> + PartialOrd + Copy + From<u8>>(values: &[T], max: T) -> Result<T, MathError> {
    let mut result = T::from(1);
    for &value in values {
        result = checked_mul(result, value, max)?;
    }
    Ok(result)
}

/// Multiply by power of two
pub fn mul_pow2<T: Mul<Output = T> + Copy>(value: T, power: u32) -> T {
    let mut result = value;
    for _ in 0..power {
        result = result * (T::from(2));
    }
    result
}

impl From<u8> for T {
    fn from(value: u8) -> Self {
        // Implementation depends on type T
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_mul() {
        assert!(checked_mul(2.0, 3.0, 10.0).is_ok());
        assert_eq!(checked_mul(2.0, 3.0, 10.0).unwrap(), 6.0);
        assert!(checked_mul(2.0, 3.0, 5.0).is_err());
    }

    #[test]
    fn test_product() {
        let values = vec![2.0, 3.0, 4.0];
        assert_eq!(product(&values, 100.0).unwrap(), 24.0);
    }

    #[test]
    fn test_mul_pow2() {
        assert_eq!(mul_pow2(2.0, 2), 8.0);
    }
}
