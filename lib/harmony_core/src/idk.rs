//! IDK - Internal Data Kernel
//! ======================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 20:29:06 UTC
//! Version: 0.1.1
//! License: MIT

use core::{
    fmt::{self, Display},
    marker::PhantomData,
    mem::MaybeUninit,
};

use magicmath::{
    MeshValue,
    Vector3D,
};

use errors::MathError;

/// Uninitialized shard type
#[derive(Debug)]
pub struct ShardUninit<T> {
    data: MaybeUninit<T>,
    initialized: bool,
    _marker: PhantomData<T>,
}

impl<T> ShardUninit<T> {
    /// Create new uninitialized shard
    pub const fn new() -> Self {
        Self {
            data: MaybeUninit::uninit(),
            initialized: false,
            _marker: PhantomData,
        }
    }

    /// Get reference to initialized data
    /// # Safety
    /// Caller must ensure data is initialized
    pub unsafe fn get_ref(&self) -> Option<&T> {
        if self.initialized {
            Some(&*self.data.as_ptr())
        } else {
            None
        }
    }

    /// Set data value
    /// # Safety
    /// Previous value is dropped if initialized
    pub unsafe fn set(&mut self, value: T) {
        if self.initialized {
            self.data.assume_init_drop();
        }
        self.data.write(value);
        self.initialized = true;
    }

    /// Check if data is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Quantum harmony state
#[derive(Debug, Clone)]
pub struct HarmonyState<T> {
    /// State value
    value: T,
    /// Position in lattice
    position: Vector3D,
}

impl<T: Default> Default for HarmonyState<T> {
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
    pub fn set_position(&mut self, pos: Vector3D) {
        self.position = pos;
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State({}) at {}", self.value, self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_uninit() {
        let mut shard: ShardUninit<i32> = ShardUninit::new();
        assert!(!shard.is_initialized());

        unsafe {
            shard.set(42);
            assert!(shard.is_initialized());
            assert_eq!(*shard.get_ref().unwrap(), 42);
        }
    }

    #[test]
    fn test_harmony_state() {
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        let state = HarmonyState::new(42.0, pos.clone());
        assert_eq!(*state.value(), 42.0);
        assert_eq!(*state.position(), pos);
    }

    #[test]
    fn test_harmony_state_default() {
        let state: HarmonyState<f64> = HarmonyState::default();
        assert_eq!(*state.value(), 0.0);
        assert_eq!(*state.position(), Vector3D::new(0.0, 0.0, 0.0));
    }
}
