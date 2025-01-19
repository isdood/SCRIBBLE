//! Addition Operations for MeshMath
//! ==========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 10:01:28 UTC
//! Last Updated: 2025-01-19 10:01:28 UTC
//! Version: 0.1.0
//! License: MIT

use core::ops::Add;
use crate::errors::MathError;

/// Add two values with overflow checking
pub fn checked_add<T: Add<Output = T> + PartialOrd + Copy>(a: T, b: T) -> Result<T, MathError> {
    let result = a + b;
    if result < a {
        return Err(MathError::Overflow);
    }
    Ok(result)
}

/// Add array of values with overflow checking
pub fn sum<T: Add<Output = T> + PartialOrd + Copy + Default>(values: &[T]) -> Result<T, MathError> {
    let mut result = T::default();
    for &value in values {
        result = checked_add(result, value)?;
    }
    Ok(result)
}

/// Add values with saturation at bounds
pub fn saturating_add<T: Add<Output = T> + PartialOrd + Copy>(a: T, b: T, max: T) -> T {
    let result = a + b;
    if result > max {
        max
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add() {
        assert!(checked_add(1.0, 2.0).is_ok());
        assert_eq!(checked_add(1.0, 2.0).unwrap(), 3.0);
    }

    #[test]
    fn test_sum() {
        let values = vec![1.0, 2.0, 3.0];
        assert_eq!(sum(&values).unwrap(), 6.0);
    }

    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(1.0, 2.0, 2.0), 2.0);
        assert_eq!(saturating_add(1.0, 0.5, 2.0), 1.5);
    }
}
