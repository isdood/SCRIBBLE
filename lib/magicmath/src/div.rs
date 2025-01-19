// lib/magicmath/src/div.rs

use crate::traits::MeshValue;
use errors::MathError;

impl<T: MeshValue> Div for T {
    type Output = Result<T, MathError>;

    fn div(self, rhs: T) -> Self::Output {
        if !self.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable); // Fixed: Changed from UnstableState
        }

        if rhs.is_zero() {
            return Err(MathError::DivisionByZero);
        }

        let result = self.raw_div(rhs)?;

        // Verify the result maintains harmony
        if !result.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable); // Fixed: Changed from UnstableState
        }

        Ok(result)
    }
}

impl<T: MeshValue> DivAssign for T {
    fn div_assign(&mut self, rhs: T) {
        match self.div(rhs) {
            Ok(result) => *self = result,
            Err(e) => panic!("Division operation failed: {}", e.scribe()),
        }
    }
}

// Helper trait for raw division
trait RawDiv {
    fn raw_div(&self, other: Self) -> Result<Self, MathError> where Self: Sized;
}

impl<T: MeshValue> RawDiv for T {
    fn raw_div(&self, rhs: T) -> Result<Self, MathError> {
        self.div(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division() {
        let a = TestValue::new(6.0);
        let b = TestValue::new(2.0);
        let result = a.div(b).unwrap();
        assert_eq!(result.value(), 3.0);
    }

    #[test]
    fn test_division_by_zero() {
        let a = TestValue::new(5.0);
        let b = TestValue::new(0.0);
        assert!(matches!(a.div(b), Err(MathError::DivisionByZero)));
    }

    #[test]
    fn test_division_assign() {
        let mut a = TestValue::new(6.0);
        let b = TestValue::new(2.0);
        a /= b;
        assert_eq!(a.value(), 3.0);
    }

    #[test]
    fn test_harmony_violation() {
        let mut a = TestValue::new(6.0);
        a.destabilize();
        let b = TestValue::new(2.0);
        assert!(matches!(a.div(b), Err(MathError::HarmonyStateUnstable)));
    }
}
