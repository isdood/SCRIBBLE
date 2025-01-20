//! Crystal-Aware Subtraction Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 23:56:38 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::CrystalSub;
use crate::constants::HARMONY_STABILITY_THRESHOLD;
use errors::{MathError, MathResult};

impl<T: MeshValue> CrystalSub for T {
    fn sub(&self, other: &Self) -> MathResult<Self> {
        if !self.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable);
        }

        if !other.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable);
        }

        let result = self.raw_sub(other)?;

        if !result.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable);
        }

        Ok(result)
    }

    fn sub_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.sub(other)?;
        Ok(())
    }
}

trait RawSub {
    fn raw_sub(&self, other: &Self) -> MathResult<Self> where Self: Sized;
}

impl<T: MeshValue> RawSub for T {
    fn raw_sub(&self, other: &Self) -> MathResult<Self> {
        let self_val = self.to_f64()?;
        let other_val = other.to_f64()?;
        Ok(T::from(self_val - other_val))
    }
}
