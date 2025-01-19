// lib/magicmath/src/sub.rs

use crate::traits::MeshValue;
use errors::MathError;

impl<T: MeshValue> Sub for T {
    type Output = Result<T, MathError>;

    fn sub(self, rhs: T) -> Self::Output {
        if !self.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable); // Fixed: Changed from UnstableState
        }

        let result = self.raw_sub(rhs)?;

        // Verify the result maintains harmony
        if !result.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable); // Fixed: Changed from UnstableState
        }

        Ok(result)
    }
}

impl<T: MeshValue> SubAssign for T {
    fn sub_assign(&mut self, rhs: T) {
        match self.sub(rhs) {
            Ok(result) => *self = result,
            Err(e) => panic!("Subtraction operation failed: {}", e.scribe()),
        }
    }
}

// Helper trait for raw subtraction
trait RawSub {
    fn raw_sub(&self, other: Self) -> Result<Self, MathError> where Self: Sized;
}

impl<T: MeshValue> RawSub for T {
    fn raw_sub(&self, rhs: T) -> Result<Self, MathError> {
        self.sub(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtraction() {
        let a = TestValue::new(5.0);
        let b = TestValue::new(3.0);
        let result = a.sub(b).unwrap();
        assert_eq!(result.value(), 2.0);
    }

    #[test]
    fn test_subtraction_assign() {
        let mut a = TestValue::new(5.0);
        let b = TestValue::new(3.0);
        a -= b;
        assert_eq!(a.value(), 2.0);
    }

    #[test]
    fn test_harmony_violation() {
        let mut a = TestValue::new(5.0);
        a.destabilize();
        let b = TestValue::new(3.0);
        assert!(matches!(a.sub(b), Err(MathError::HarmonyStateUnstable)));
    }
}
