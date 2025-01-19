//! IDK - Core Quantum State Management
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 09:55:06 UTC
//! Version: 0.1.0
//! License: MIT

use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    marker::PhantomData,
};

use crate::{
    errors::{QuantumError, CoherenceError},
    align::AlignmentState,
};

/// Core error type for coherence operations
#[derive(Debug)]
pub enum CoherenceError {
    /// Invalid coherence value
    InvalidValue,
    /// Phase alignment failure
    PhaseAlignmentFailure,
    /// Invalid state detected
    InvalidState,
}

/// Result type for coherence operations
pub type CoherenceResult<T> = Result<T, CoherenceError>;

/// A quantum shard that may or may not be initialized
#[derive(Debug)]
#[repr(transparent)]
pub struct ShardUninit<T> {
    /// Inner value storage
    value: MaybeUninit<T>,
}

/// Core quantum state container
#[derive(Debug)]
pub struct QuantumState<T> {
    /// Inner value pointer
    ptr: NonNull<T>,
    /// Phantom data for variance
    _phantom: PhantomData<T>,
}

impl<T> ShardUninit<T> {
    /// Create a new uninitialized shard
    #[inline]
    pub const fn new() -> Self {
        Self {
            value: MaybeUninit::uninit(),
        }
    }

    /// Create a new initialized shard
    #[inline]
    pub fn new_init(value: T) -> Self {
        Self {
            value: MaybeUninit::new(value),
        }
    }

    /// Get a reference to the inner value if initialized
    #[inline]
    pub unsafe fn get_ref(&self) -> Option<&T> {
        if self.is_initialized() {
            Some(&*self.value.as_ptr())
        } else {
            None
        }
    }

    /// Get a mutable reference to the inner value if initialized
    #[inline]
    pub unsafe fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_initialized() {
            Some(&mut *self.value.as_mut_ptr())
        } else {
            None
        }
    }

    /// Set the inner value
    #[inline]
    pub unsafe fn set(&mut self, value: T) {
        self.value = MaybeUninit::new(value);
    }

    /// Check if the shard is initialized
    #[inline]
    pub fn is_initialized(&self) -> bool {
        // This is safe because we only set the value through `set` or `new_init`
        unsafe { !self.value.as_ptr().is_null() }
    }
}

impl<T> QuantumState<T> {
    /// Create a new quantum state
    pub fn new(value: T) -> Self {
        Self {
            ptr: NonNull::new(Box::into_raw(Box::new(value))).unwrap(),
            _phantom: PhantomData,
        }
    }

    /// Get the inner value
    pub fn into_inner(self) -> T {
        let value = unsafe { Box::from_raw(self.ptr.as_ptr()) };
        core::mem::forget(self);
        *value
    }

    /// Check coherence of the state
    pub fn check_coherence(&self) -> CoherenceResult<AlignmentState> {
        if self.ptr.is_null() {
            return Err(CoherenceError::InvalidState);
        }
        Ok(AlignmentState::Perfect)
    }
}

impl<T> Drop for QuantumState<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr.as_ptr());
        }
    }
}

impl<T> Deref for QuantumState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for QuantumState<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
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
    fn test_quantum_state() {
        let state = QuantumState::new(42);
        assert_eq!(*state, 42);

        let value = state.into_inner();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_coherence_check() {
        let state = QuantumState::new(42);
        assert!(matches!(state.check_coherence().unwrap(), AlignmentState::Perfect));
    }
}
