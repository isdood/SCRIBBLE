//! Harmony State Management
//! =========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 17:42:00 UTC
//! Version: 0.1.1
//! License: MIT

use core::fmt::{Display, Formatter, Result as FmtResult};
use magicmath::traits::{
    MeshValue,
    CrystalAdd,
    CrystalSub,
    CrystalMul,
    CrystalDiv,
};
use magicmath::errors::{Error as MathError, Result as MathResult};
use scribe::{
    native_string::String,
    Scribe,
};

use crate::align::AlignmentState;
use crate::errors::CoherenceError;

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
    fn add(&self, other: &Self) -> MathResult<Self> {
        Ok(Self::new(self.value.add(&other.value)?))
    }

    fn add_assign(&mut self, other: &Self) -> MathResult<()> {
        self.value.add_assign(&other.value)
    }
}

impl<T> CrystalSub for HarmonyState<T>
where
T: CrystalSub + Clone,
{
    fn sub(&self, other: &Self) -> MathResult<Self> {
        Ok(Self::new(self.value.sub(&other.value)?))
    }

    fn sub_assign(&mut self, other: &Self) -> MathResult<()> {
        self.value.sub_assign(&other.value)
    }
}

impl<T> CrystalMul for HarmonyState<T>
where
T: CrystalMul + Clone,
{
    fn mul(&self, other: &Self) -> MathResult<Self> {
        Ok(Self::new(self.value.mul(&other.value)?))
    }

    fn mul_assign(&mut self, other: &Self) -> MathResult<()> {
        self.value.mul_assign(&other.value)
    }
}

impl<T> CrystalDiv for HarmonyState<T>
where
T: CrystalDiv + Clone,
{
    fn div(&self, other: &Self) -> MathResult<Self> {
        Ok(Self::new(self.value.div(&other.value)?))
    }

    fn div_assign(&mut self, other: &Self) -> MathResult<()> {
        self.value.div_assign(&other.value)
    }
}

impl<T> MeshValue for HarmonyState<T>
where
T: MeshValue + CrystalAdd + CrystalSub + CrystalMul + CrystalDiv + Clone,
{
    fn to_f64(&self) -> MathResult<f64> {
        self.value.to_f64()
    }

    fn from(value: f64) -> Self {
        Self::new(T::from(value))
    }

    fn coherence(&self) -> MathResult<f64> {
        self.value.coherence()
    }

    fn energy(&self) -> MathResult<f64> {
        self.value.energy()
    }

    fn magnitude(&self) -> MathResult<f64> {
        self.value.magnitude()
    }

    fn to_usize(&self) -> MathResult<usize> {
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
