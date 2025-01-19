//! Harmony State Management
//! =========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 21:14:31 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::traits::{MeshValue, Scribe};
use magicmath::errors::MathError;
use crate::{
    phantom::PhantomCore,
    errors::CoherenceError,
    align::AlignmentState,
    cube::Cube,
    scribe::native_string::String,
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
#[derive(Debug)]
pub struct HarmonyState<T> {
    /// Inner value pointer
    value: T,
    /// Phantom data for variance
    _phantom: PhantomCore<T>,
}

impl<T> ShardUninit<T> {
    /// Create a new uninitialized shard
    #[inline]
    pub const fn new() -> Self {
        Self {
            value: None,
        }
    }

    /// Create a new initialized shard
    #[inline]
    pub fn new_init(value: T) -> Self {
        Self {
            value: Some(value),
        }
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
        Self {
            value,
            _phantom: PhantomCore::new(),
        }
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

impl<T> Scribe for HarmonyState<T> where T: Scribe {
    fn scribe(&self) -> String {
        self.value.scribe()
    }
}

impl<T> MeshValue for HarmonyState<T> where T: MeshValue {
    fn to_f64(&self) -> Result<f64, MathError> {
        self.value.to_f64()
    }

    fn from(value: f64) -> Self {
        Self {
            value: T::from(value),
            _phantom: PhantomCore::new(),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestValue {
        value: f64,
        energy_factor: f64,
    }

    impl TestValue {
        fn new(value: f64) -> Self {
            Self {
                value,
                energy_factor: 1.0,
            }
        }
    }

    impl Scribe for TestValue {
        fn scribe(&self) -> String {
            format!("{}", self.value)
        }
    }

    impl MeshValue for TestValue {
        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.value)
        }

        fn from(value: f64) -> Self {
            Self::new(value)
        }

        fn coherence(&self) -> Result<f64, MathError> {
            Ok(self.value.abs().min(1.0))
        }

        fn energy(&self) -> Result<f64, MathError> {
            Ok(self.value * self.energy_factor)
        }

        fn magnitude(&self) -> Result<f64, MathError> {
            Ok(self.value.abs())
        }

        fn to_usize(&self) -> Result<usize, MathError> {
            if self.value < 0.0 || self.value.fract() != 0.0 {
                Err(MathError::InvalidConversion)
            } else {
                Ok(self.value as usize)
            }
        }
    }

    #[test]
    fn test_mesh_value_implementation() {
        let state = HarmonyState::new(TestValue::new(42.0));

        assert_eq!(state.to_f64().unwrap(), 42.0);
        assert_eq!(state.coherence().unwrap(), 1.0);
        assert_eq!(state.energy().unwrap(), 42.0);
        assert_eq!(state.magnitude().unwrap(), 42.0);
        assert_eq!(state.to_usize().unwrap(), 42);
    }

    #[test]
    fn test_invalid_conversions() {
        let negative = HarmonyState::new(TestValue::new(-1.0));
        assert!(negative.to_usize().is_err());

        let fractional = HarmonyState::new(TestValue::new(1.5));
        assert!(fractional.to_usize().is_err());
    }

    #[test]
    fn test_coherence_limits() {
        let large = HarmonyState::new(TestValue::new(2.0));
        assert_eq!(large.coherence().unwrap(), 1.0);

        let small = HarmonyState::new(TestValue::new(0.5));
        assert_eq!(small.coherence().unwrap(), 0.5);
    }
}
