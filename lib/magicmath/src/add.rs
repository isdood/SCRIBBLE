// lib/magicmath/src/add.rs

use crate::traits::MeshValue;
use errors::MathError;

impl<T: MeshValue> Add for T {
    type Output = Result<T, MathError>;

    fn add(self, rhs: T) -> Self::Output {
        if !self.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable); // Fixed: Changed from UnstableState
        }

        let result = self.raw_add(rhs)?;

        // Verify the result maintains harmony
        if !result.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable); // Fixed: Changed from UnstableState
        }

        Ok(result)
    }
}

impl<T: MeshValue> AddAssign for T {
    fn add_assign(&mut self, rhs: T) {
        match self.add(rhs) {
            Ok(result) => *self = result,
            Err(e) => panic!("Addition operation failed: {}", e.scribe()),
        }
    }
}

// Helper trait for raw addition
trait RawAdd {
    fn raw_add(&self, other: Self) -> Result<Self, MathError> where Self: Sized;
}

impl<T: MeshValue> RawAdd for T {
    fn raw_add(&self, rhs: T) -> Result<Self, MathError> {
        self.add(&rhs)
    }
}

// Remove unused variable warning by using _rhs
fn check_harmony(&self, _rhs: &Self) -> bool {
    self.coherence() >= HARMONY_STABILITY_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = TestValue::new(5.0);
        let b = TestValue::new(3.0);
        let result = a.add(b).unwrap();
        assert_eq!(result.value(), 8.0);
    }

    #[test]
    fn test_addition_assign() {
        let mut a = TestValue::new(5.0);
        let b = TestValue::new(3.0);
        a += b;
        assert_eq!(a.value(), 8.0);
    }

    #[test]
    fn test_harmony_violation() {
        let mut a = TestValue::new(5.0);
        a.destabilize();
        let b = TestValue::new(3.0);
        assert!(matches!(a.add(b), Err(MathError::HarmonyStateUnstable)));
    }
}
