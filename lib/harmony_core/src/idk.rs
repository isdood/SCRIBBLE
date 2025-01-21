//! IDK - Internal Data Kernel
//! ======================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 23:57:11 UTC
//! Version: 0.1.1
//! License: MIT

use core::{
    fmt::{self, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    mem::MaybeUninit,
    result::Result,
};

use magicmath::{
    MeshValue,
    Vector3D,
    CrystalAdd,
    CrystalSub,
    CrystalMul,
    CrystalDiv,
};

use errors::MathError;

/// Uninitialized shard type
#[derive(Debug)]
pub struct ShardUninit<T> {
    /// Uninitialized data
    data: MaybeUninit<T>,
    /// Initialization flag
    initialized: bool,
    /// Type marker
    _marker: PhantomData<T>,
}

impl<T: Default> ShardUninit<T> {
    /// Create new uninitialized shard
    pub fn new() -> Self {
        Self {
            data: MaybeUninit::uninit(),
            initialized: false,
            _marker: PhantomData,
        }
    }

    /// Check if data is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get reference to data if initialized
    pub fn get(&self) -> Option<&T> {
        if self.initialized {
            // SAFETY: We check initialized flag before accessing
            unsafe { Some(&*self.data.as_ptr()) }
        } else {
            None
        }
    }

    /// Initialize with value
    pub fn init(&mut self, value: T) {
        if self.initialized {
            // SAFETY: We check initialized flag before dropping
            unsafe { self.data.assume_init_drop(); }
        }
        self.data.write(value);
        self.initialized = true;
    }
}

impl<T> Drop for ShardUninit<T> {
    fn drop(&mut self) {
        if self.initialized {
            // SAFETY: We check initialized flag before dropping
            unsafe { self.data.assume_init_drop(); }
        }
    }
}

/// A quantum harmony state
#[derive(Debug)]
pub struct HarmonyState<T> {
    /// State value
    value: T,
    /// Position in 3D space
    position: Vector3D,
}

impl<T: Default + MeshValue> Default for HarmonyState<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            position: Vector3D::new(0.0, 0.0, 0.0),
        }
    }
}

impl<T: MeshValue + Display> HarmonyState<T> {
    /// Create new harmony state
    pub fn new(value: T, position: Vector3D) -> Self {
        Self { value, position }
    }

    /// Get state value
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Set position
    pub fn set_position(&mut self, position: Vector3D) {
        self.position = position;
    }

    /// Calculate state energy
    pub fn energy(&self) -> Result<f64, MathError> {
        self.value.energy()
    }

    /// Calculate state coherence
    pub fn coherence(&self) -> Result<f64, MathError> {
        self.value.coherence()
    }
}

impl<T: Display> Display for HarmonyState<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "State({}) at {:?}", self.value, self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, Clone)]
    struct TestValue(f64);

    impl Display for TestValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "{}", self.0)
        }
    }

    impl CrystalAdd for TestValue {
        fn add(&self, other: &Self) -> Result<Self, MathError> {
            Ok(TestValue(self.0 + other.0))
        }

        fn add_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.0 += other.0;
            Ok(())
        }
    }

    impl CrystalSub for TestValue {
        fn sub(&self, other: &Self) -> Result<Self, MathError> {
            Ok(TestValue(self.0 - other.0))
        }

        fn sub_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.0 -= other.0;
            Ok(())
        }
    }

    impl CrystalMul for TestValue {
        fn mul(&self, other: &Self) -> Result<Self, MathError> {
            Ok(TestValue(self.0 * other.0))
        }

        fn mul_assign(&mut self, other: &Self) -> Result<(), MathError> {
            self.0 *= other.0;
            Ok(())
        }
    }

    impl CrystalDiv for TestValue {
        fn div(&self, other: &Self) -> Result<Self, MathError> {
            if other.0 == 0.0 {
                return Err(MathError::DivisionByZero);
            }
            Ok(TestValue(self.0 / other.0))
        }

        fn div_assign(&mut self, other: &Self) -> Result<(), MathError> {
            if other.0 == 0.0 {
                return Err(MathError::DivisionByZero);
            }
            self.0 /= other.0;
            Ok(())
        }
    }

    impl MeshValue for TestValue {
        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.0)
        }

        fn from(value: f64) -> Self {
            Self(value)
        }

        fn magnitude(&self) -> Result<f64, MathError> {
            Ok(self.0.abs())
        }

        fn coherence(&self) -> Result<f64, MathError> {
            Ok(1.0)
        }

        fn energy(&self) -> Result<f64, MathError> {
            Ok(self.0 * self.0)
        }

        fn to_usize(&self) -> Result<usize, MathError> {
            Ok(self.0 as usize)
        }

        fn check_harmony_state(&self) -> bool {
            self.0 >= 0.0
        }
    }

    #[test]
    fn test_shard_uninit() {
        let mut shard: ShardUninit<i32> = ShardUninit::new();
        assert!(!shard.is_initialized());
        assert!(shard.get().is_none());

        shard.init(42);
        assert!(shard.is_initialized());
        assert_eq!(shard.get(), Some(&42));
    }

    #[test]
    fn test_harmony_state() {
        let value = TestValue(42.0);
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        let state = HarmonyState::new(value, pos.clone());

        assert_eq!(state.value().0, 42.0);
        assert_eq!(format!("{:?}", state.position()), format!("{:?}", &pos));
    }

    #[test]
    fn test_state_energy() {
        let state = HarmonyState::new(TestValue(2.0), Vector3D::new(0.0, 0.0, 0.0));
        assert_eq!(state.energy().unwrap(), 4.0);
    }
}
