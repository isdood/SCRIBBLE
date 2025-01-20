//! Harmony State Management
//! =========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 17:04:28 UTC
//! Version: 0.1.1
//! License: MIT

use core::fmt::{self, Display, Formatter, Result as FmtResult};
use magicmath::{
    traits::{MeshValue, Scribe, CrystalAdd, CrystalSub, CrystalMul, CrystalDiv},
    errors::MathError,
};
use crate::{
    errors::CoherenceError,
    align::AlignmentState,
};

/// Result type for coherence operations
pub type CoherenceResult<T> = Result<T, CoherenceError>;

/// A shard that may or may not be initialized
#[derive(Debug)]
#[repr(transparent)]
pub struct ShardUninit<T> {
    /// Inner value storage
    value: Option<T>,
}

/// Core harmony state container
#[derive(Debug, Clone)]
pub struct HarmonyState<T> {
    /// Inner value pointer
    value: T,
}

impl<T> ShardUninit<T> {
    /// Create a new uninitialized shard
    #[inline]
    pub const fn new() -> Self {
        Self { value: None }
    }

    /// Create a new initialized shard
    #[inline]
    pub fn new_init(value: T) -> Self {
        Self { value: Some(value) }
    }

    /// Get a reference to the inner value if initialized
    #[inline]
    pub fn get_ref(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Get a mutable reference to the inner value if initialized
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    /// Set the inner value
    #[inline]
    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }

    /// Check if the shard is initialized
    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.value.is_some()
    }
}

impl<T> HarmonyState<T> {
    /// Create a new harmony state
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Get the inner value
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Check coherence of the state
    pub fn check_coherence(&self) -> CoherenceResult<AlignmentState> {
        Ok(AlignmentState::Perfect)
    }
}

impl<T> CrystalAdd for HarmonyState<T>
where
T: CrystalAdd + Clone,
{
    fn add(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self::new(self.value.add(&other.value)?))
    }

    fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
        self.value.add_assign(&other.value)
    }
}

impl<T> CrystalSub for HarmonyState<T>
where
T: CrystalSub + Clone,
{
    fn sub(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self::new(self.value.sub(&other.value)?))
    }

    fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
        self.value.sub_assign(&other.value)
    }
}

impl<T> CrystalMul for HarmonyState<T>
where
T: CrystalMul + Clone,
{
    fn mul(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self::new(self.value.mul(&other.value)?))
    }

    fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
        self.value.mul_assign(&other.value)
    }
}

impl<T> CrystalDiv for HarmonyState<T>
where
T: CrystalDiv + Clone,
{
    fn div(&self, other: &Self) -> Result<Self, MathError> {
        Ok(Self::new(self.value.div(&other.value)?))
    }

    fn div_assign(&mut self, other: &Self) -> Result<(), MathError> {
        self.value.div_assign(&other.value)
    }
}

impl<T> MeshValue for HarmonyState<T>
where
T: MeshValue + CrystalAdd + CrystalSub + CrystalMul + CrystalDiv + Clone,
{
    fn to_f64(&self) -> Result<f64, MathError> {
        self.value.to_f64()
    }

    fn from(value: f64) -> Self {
        Self::new(T::from(value))
    }

    fn coherence(&self) -> Result<f64, MathError> {
        self.value.coherence()
    }

    fn energy(&self) -> Result<f64, MathError> {
        self.value.energy()
    }

    fn magnitude(&self) -> Result<f64, MathError> {
        self.value.magnitude()
    }

    fn to_usize(&self) -> Result<usize, MathError> {
        self.value.to_usize()
    }

    fn check_harmony_state(&self) -> bool {
        self.value.check_harmony_state()
    }
}

impl<T: Scribe> Scribe for HarmonyState<T> {
    fn scribe(&self) -> String {
        self.value.scribe()
    }
}

impl<T: Display> Display for HarmonyState<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestValue {
        value: f64,
    }

    impl CrystalAdd for TestValue {
        fn add(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value + other.value })
        }

        fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value += other.value;
            Ok(())
        }
    }

    impl CrystalSub for TestValue {
        fn sub(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value - other.value })
        }

        fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value -= other.value;
            Ok(())
        }
    }

    impl CrystalMul for TestValue {
        fn mul(&self, other: &Self) -> Result<Self, MathError> {
            Ok(Self { value: self.value * other.value })
        }

        fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.value *= other.value;
            Ok(())
        }
    }

    impl CrystalDiv for TestValue {
        fn div(&self, other: &Self) -> Result<Self, MathError> {
            if other.value == 0.0 {
                return Err(MathError::DivisionByZero);
            }
            Ok(Self { value: self.value / other.value })
        }

        fn div_assign(&mut self, other: &Self) -> Result<(), MathError> {
            if other.value == 0.0 {
                return Err(MathError::DivisionByZero);
            }
            self.value /= other.value;
            Ok(())
        }
    }

    impl MeshValue for TestValue {
        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.value)
        }

        fn from(value: f64) -> Self {
            Self { value }
        }

        fn coherence(&self) -> Result<f64, MathError> {
            Ok(1.0)
        }

        fn energy(&self) -> Result<f64, MathError> {
            Ok(self.value.abs())
        }

        fn magnitude(&self) -> Result<f64, MathError> {
            Ok(self.value.abs())
        }

        fn to_usize(&self) -> Result<usize, MathError> {
            Ok(self.value as usize)
        }

        fn check_harmony_state(&self) -> bool {
            true
        }
    }

    impl Scribe for TestValue {
        fn scribe(&self) -> String {
            self.value.to_string()
        }
    }

    impl Display for TestValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "{}", self.value)
        }
    }

    #[test]
    fn test_harmony_state() {
        let state1 = HarmonyState::new(TestValue { value: 42.0 });
        let state2 = HarmonyState::new(TestValue { value: 8.0 });

        assert!(state1.add(&state2).is_ok());
        assert!(state1.sub(&state2).is_ok());
        assert!(state1.mul(&state2).is_ok());
        assert!(state1.div(&state2).is_ok());
        assert!(state1.check_harmony_state());
    }

    #[test]
    fn test_shard_uninit() {
        let mut shard = ShardUninit::<TestValue>::new();
        assert!(!shard.is_initialized());

        shard.set(TestValue { value: 42.0 });
        assert!(shard.is_initialized());
        assert_eq!(shard.get_ref().unwrap().value, 42.0);
    }
}
