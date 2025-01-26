//! Crystal-optimized 32-bit integer type.
//!
//! This module provides a high-performance 32-bit integer type optimized for
//! crystal-space harmonic resonance and quantum vibrations.

pub mod harmony;
pub mod crystal;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use harmony::Harmonic;
use crystal::Resonator;

/// Result type for voice operations
pub type VoiceResult<T> = Result<T, VoiceError>;

/// Error type for voice operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoiceError {
    /// Overflow error
    Overflow,
    /// Underflow error
    Underflow,
    /// Division by zero
    DivisionByZero,
    /// Harmonic error
    HarmonicError(String),
}

/// Crystal-optimized 32-bit integer
#[derive(Debug, Clone, Copy)]
pub struct Voice {
    /// The raw 32-bit value
    value: i32,
    /// Harmonic state
    harmonic: Harmonic,
    /// Crystal resonator
    resonator: Resonator,
}

impl PartialEq for Voice {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value &&
        self.harmonic.approx_eq(&other.harmonic) &&
        self.resonator.approx_eq(&other.resonator)
    }
}

impl Eq for Voice {}

impl PartialOrd for Voice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Voice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Voice {
    /// Creates a new Voice value
    pub fn new(value: i32) -> Self {
        Self {
            value,
            harmonic: Harmonic::default(),
            resonator: Resonator::default(),
        }
    }

    /// Gets the raw value
    pub fn get(&self) -> i32 {
        self.value
    }

    /// Gets the harmonic state
    pub fn harmonic(&self) -> &Harmonic {
        &self.harmonic
    }

    /// Gets the resonator
    pub fn resonator(&self) -> &Resonator {
        &self.resonator
    }

    /// Performs a checked addition with harmonic combination
    pub fn checked_add(self, rhs: Self) -> VoiceResult<Self> {
        let (value, overflow) = self.value.overflowing_add(rhs.value);
        if overflow {
            Err(VoiceError::Overflow)
        } else {
            Ok(Self {
                value,
                harmonic: self.harmonic.combine(&rhs.harmonic),
                resonator: self.resonator.resonate(&rhs.resonator),
            })
        }
    }

    /// Performs a checked subtraction with harmonic interference
    pub fn checked_sub(self, rhs: Self) -> VoiceResult<Self> {
        let (value, overflow) = self.value.overflowing_sub(rhs.value);
        if overflow {
            Err(VoiceError::Underflow)
        } else {
            Ok(Self {
                value,
                harmonic: self.harmonic.interfere(&rhs.harmonic),
                resonator: self.resonator.resonate(&rhs.resonator),
            })
        }
    }

    /// Performs a checked multiplication with harmonic amplification
    pub fn checked_mul(self, rhs: Self) -> VoiceResult<Self> {
        let (value, overflow) = self.value.overflowing_mul(rhs.value);
        if overflow {
            Err(VoiceError::Overflow)
        } else {
            Ok(Self {
                value,
                harmonic: self.harmonic.amplify(&rhs.harmonic),
                resonator: self.resonator.amplify(&rhs.resonator),
            })
        }
    }

    /// Performs a checked division with harmonic attenuation
    pub fn checked_div(self, rhs: Self) -> VoiceResult<Self> {
        if rhs.value == 0 {
            Err(VoiceError::DivisionByZero)
        } else {
            let (value, overflow) = self.value.overflowing_div(rhs.value);
            if overflow {
                Err(VoiceError::Overflow)
            } else {
                Ok(Self {
                    value,
                    harmonic: self.harmonic.attenuate(&rhs.harmonic),
                    resonator: self.resonator.attenuate(&rhs.resonator),
                })
            }
        }
    }

    /// Harmonizes with another Voice value
    pub fn harmonize(&self, other: &Self) -> VoiceResult<Self> {
        Ok(Self {
            value: self.value,
            harmonic: self.harmonic.harmonize(&other.harmonic),
            resonator: self.resonator.synchronize(&other.resonator),
        })
    }
}

impl Default for Voice {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<i32> for Voice {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl Add for Voice {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap_or_else(|_| self)
    }
}

impl Sub for Voice {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).unwrap_or_else(|_| self)
    }
}

impl Mul for Voice {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs).unwrap_or_else(|_| self)
    }
}

impl Div for Voice {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).unwrap_or_else(|_| self)
    }
}

impl fmt::Display for Voice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
