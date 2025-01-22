//! Core module providing fundamental types and traits

use num_traits::{Float, Zero, One};

pub trait SIMDValue: Float + Zero + One + Copy + Send + Sync {
    fn to_f32(self) -> Option<f32>;
    fn from_f32(v: f32) -> Option<Self>;
}

impl SIMDValue for f32 {
    fn to_f32(self) -> Option<f32> {
        Some(self)
    }

    fn from_f32(v: f32) -> Option<Self> {
        Some(v)
    }
}

impl SIMDValue for f64 {
    fn to_f32(self) -> Option<f32> {
        if self.is_finite() {
            Some(self as f32)
        } else {
            None
        }
    }

    fn from_f32(v: f32) -> Option<Self> {
        if v.is_finite() {
            Some(v as f64)
        } else {
            None
        }
    }
}
