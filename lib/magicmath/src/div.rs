//! Crystal-Aware Division Operations
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 23:56:38 UTC
//! Version: 0.1.0
//! License: MIT

use crate::traits::{MeshValue, CrystalDiv};
use crate::constants::HARMONY_STABILITY_THRESHOLD;
use errors::MathError;

impl<T: MeshValue> CrystalDiv for T {
    fn div(&self, other: &Self) -> MathResult<Self> {
        if !self.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable);
        }

        if !other.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable);
        }

        let other_val = other.to_f64()?;
        if other_val == 0.0 {
            return Err(MathError::DivisionByZero);
        }

        let result = self.raw_div(other)?;

        if !result.check_harmony_state() {
            return Err(MathError::HarmonyStateUnstable);
        }

        Ok(result)
    }

    fn div_assign(&mut self, other: &Self) -> MathResult<()> {
        *self = self.div(other)?;
        Ok(())
    }
}

trait RawDiv {
    fn raw_div(&self, other: &Self) -> MathResult<Self> where Self: Sized;
}

impl<T: MeshValue> RawDiv for T {
    fn raw_div(&self, other: &Self) -> MathResult<Self> {
        let self_val = self.to_f64()?;
        let other_val = other.to_f64()?;
        if other_val == 0.0 {
            return Err(MathError::DivisionByZero);
        }
        Ok(T::from(self_val / other_val))
    }
}
