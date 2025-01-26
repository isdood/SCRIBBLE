//! Crystal-optimized 8-bit integer type.
//!
//! This module provides a high-performance 8-bit integer type optimized for
//! crystal-space operations and quantum resonance patterns.

pub mod ops;
pub mod crystal;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use crystal::{CrystalState, QuantumPhase};

/// Result type for whisper operations
pub type WhisperResult<T> = Result<T, WhisperError>;

/// Error type for whisper operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhisperError {
    /// Overflow error
    Overflow,
    /// Underflow error
    Underflow,
    /// Division by zero
    DivisionByZero,
    /// Quantum state error
    QuantumError(String),
}

/// Crystal-optimized 8-bit integer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Whisper {
    /// The raw 8-bit value
    value: i8,
    /// Quantum phase state
    phase: QuantumPhase,
    /// Crystal resonance state
    state: CrystalState,
}

impl Whisper {
    /// Creates a new Whisper value
    pub fn new(value: i8) -> Self {
        Self {
            value,
            phase: QuantumPhase::default(),
            state: CrystalState::default(),
        }
    }

    /// Gets the raw value
    pub fn get(&self) -> i8 {
        self.value
    }

    /// Gets the quantum phase
    pub fn phase(&self) -> &QuantumPhase {
        &self.phase
    }

    /// Gets the crystal state
    pub fn state(&self) -> &CrystalState {
        &self.state
    }

    /// Performs a checked addition
    pub fn checked_add(self, rhs: Self) -> WhisperResult<Self> {
        let (value, overflow) = self.value.overflowing_add(rhs.value);
        if overflow {
            Err(WhisperError::Overflow)
        } else {
            Ok(Self {
                value,
                phase: self.phase.combine(&rhs.phase),
                state: self.state.resonate(&rhs.state),
            })
        }
    }

    /// Performs a checked subtraction
    pub fn checked_sub(self, rhs: Self) -> WhisperResult<Self> {
        let (value, overflow) = self.value.overflowing_sub(rhs.value);
        if overflow {
            Err(WhisperError::Underflow)
        } else {
            Ok(Self {
                value,
                phase: self.phase.combine(&rhs.phase),
                state: self.state.resonate(&rhs.state),
            })
        }
    }

    /// Performs a checked multiplication
    pub fn checked_mul(self, rhs: Self) -> WhisperResult<Self> {
        let (value, overflow) = self.value.overflowing_mul(rhs.value);
        if overflow {
            Err(WhisperError::Overflow)
        } else {
            Ok(Self {
                value,
                phase: self.phase.combine(&rhs.phase),
                state: self.state.resonate(&rhs.state),
            })
        }
    }

    /// Performs a checked division
    pub fn checked_div(self, rhs: Self) -> WhisperResult<Self> {
        if rhs.value == 0 {
            Err(WhisperError::DivisionByZero)
        } else {
            let (value, overflow) = self.value.overflowing_div(rhs.value);
            if overflow {
                Err(WhisperError::Overflow)
            } else {
                Ok(Self {
                    value,
                    phase: self.phase.combine(&rhs.phase),
                    state: self.state.resonate(&rhs.state),
                })
            }
        }
    }
}

impl Default for Whisper {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<i8> for Whisper {
    fn from(value: i8) -> Self {
        Self::new(value)
    }
}

impl Add for Whisper {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap_or_else(|_| self)
    }
}

impl Sub for Whisper {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).unwrap_or_else(|_| self)
    }
}

impl Mul for Whisper {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs).unwrap_or_else(|_| self)
    }
}

impl Div for Whisper {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).unwrap_or_else(|_| self)
    }
}

impl fmt::Display for Whisper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialOrd for Whisper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Whisper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
