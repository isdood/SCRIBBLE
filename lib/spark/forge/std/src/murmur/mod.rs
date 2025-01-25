//! Crystal-optimized 16-bit integer type.
//!
//! This module provides a high-performance 16-bit integer type optimized for
//! crystal-space wave propagation and harmonic resonance.

pub mod wave;
pub mod crystal;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use wave::WaveState;
use crystal::CrystalResonance;

/// Result type for murmur operations
pub type MurmurResult<T> = Result<T, MurmurError>;

/// Error type for murmur operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MurmurError {
    /// Overflow error
    Overflow,
    /// Underflow error
    Underflow,
    /// Division by zero
    DivisionByZero,
    /// Wave resonance error
    WaveError(String),
}

/// Crystal-optimized 16-bit integer
#[derive(Debug, Clone, Copy)]
pub struct Murmur {
    /// The raw 16-bit value
    value: i16,
    /// Wave propagation state
    wave: WaveState,
    /// Crystal resonance state
    resonance: CrystalResonance,
}

impl PartialEq for Murmur {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value &&
        self.wave.approx_eq(&other.wave) &&
        self.resonance.approx_eq(&other.resonance)
    }
}

impl Eq for Murmur {}

impl Murmur {
    /// Creates a new Murmur value
    pub fn new(value: i16) -> Self {
        Self {
            value,
            wave: WaveState::default(),
            resonance: CrystalResonance::default(),
        }
    }

    /// Gets the raw value
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Gets the wave state
    pub fn wave(&self) -> &WaveState {
        &self.wave
    }

    /// Gets the crystal resonance
    pub fn resonance(&self) -> &CrystalResonance {
        &self.resonance
    }

    /// Performs a checked addition with wave propagation
    pub fn checked_add(self, rhs: Self) -> MurmurResult<Self> {
        let (value, overflow) = self.value.overflowing_add(rhs.value);
        if overflow {
            Err(MurmurError::Overflow)
        } else {
            Ok(Self {
                value,
                wave: self.wave.propagate(&rhs.wave),
                resonance: self.resonance.combine(&rhs.resonance),
            })
        }
    }

    /// Performs a checked subtraction with wave interference
    pub fn checked_sub(self, rhs: Self) -> MurmurResult<Self> {
        let (value, overflow) = self.value.overflowing_sub(rhs.value);
        if overflow {
            Err(MurmurError::Underflow)
        } else {
            Ok(Self {
                value,
                wave: self.wave.interfere(&rhs.wave),
                resonance: self.resonance.combine(&rhs.resonance),
            })
        }
    }

    /// Performs a checked multiplication with resonance amplification
    pub fn checked_mul(self, rhs: Self) -> MurmurResult<Self> {
        let (value, overflow) = self.value.overflowing_mul(rhs.value);
        if overflow {
            Err(MurmurError::Overflow)
        } else {
            Ok(Self {
                value,
                wave: self.wave.amplify(&rhs.wave),
                resonance: self.resonance.amplify(&rhs.resonance),
            })
        }
    }

    /// Performs a checked division with wave attenuation
    pub fn checked_div(self, rhs: Self) -> MurmurResult<Self> {
        if rhs.value == 0 {
            Err(MurmurError::DivisionByZero)
        } else {
            let (value, overflow) = self.value.overflowing_div(rhs.value);
            if overflow {
                Err(MurmurError::Overflow)
            } else {
                Ok(Self {
                    value,
                    wave: self.wave.attenuate(&rhs.wave),
                    resonance: self.resonance.attenuate(&rhs.resonance),
                })
            }
        }
    }
}

impl Default for Murmur {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<i16> for Murmur {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

impl Add for Murmur {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap_or_else(|_| self)
    }
}

impl Sub for Murmur {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).unwrap_or_else(|_| self)
    }
}

impl Mul for Murmur {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs).unwrap_or_else(|_| self)
    }
}

impl Div for Murmur {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).unwrap_or_else(|_| self)
    }
}

impl fmt::Display for Murmur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialOrd for Murmur {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Murmur {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
