//! Crystal-optimized 64-bit integer type.
//!
//! This module provides a high-performance 64-bit integer type optimized for
//! crystal-space echo propagation and quantum amplification.

pub mod echo;
pub mod crystal;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use echo::Echo;
use crystal::Amplifier;

/// Result type for shout operations
pub type ShoutResult<T> = Result<T, ShoutError>;

/// Error type for shout operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShoutError {
    /// Overflow error
    Overflow,
    /// Underflow error
    Underflow,
    /// Division by zero
    DivisionByZero,
    /// Echo error
    EchoError(String),
}

/// Crystal-optimized 64-bit integer
#[derive(Debug, Clone, Copy)]
pub struct Shout {
    /// The raw 64-bit value
    value: i64,
    /// Echo propagation state
    echo: Echo,
    /// Crystal amplifier
    amplifier: Amplifier,
}

impl PartialEq for Shout {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value &&
        self.echo.approx_eq(&other.echo) &&
        self.amplifier.approx_eq(&other.amplifier)
    }
}

impl Eq for Shout {}

impl PartialOrd for Shout {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shout {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Shout {
    /// Creates a new Shout value
    pub fn new(value: i64) -> Self {
        Self {
            value,
            echo: Echo::default(),
            amplifier: Amplifier::default(),
        }
    }

    /// Gets the raw value
    pub fn get(&self) -> i64 {
        self.value
    }

    /// Gets the echo state
    pub fn echo(&self) -> &Echo {
        &self.echo
    }

    /// Gets the amplifier
    pub fn amplifier(&self) -> &Amplifier {
        &self.amplifier
    }

    /// Performs a checked addition with echo propagation
    pub fn checked_add(self, rhs: Self) -> ShoutResult<Self> {
        let (value, overflow) = self.value.overflowing_add(rhs.value);
        if overflow {
            Err(ShoutError::Overflow)
        } else {
            Ok(Self {
                value,
                echo: self.echo.propagate(&rhs.echo),
                amplifier: self.amplifier.combine(&rhs.amplifier),
            })
        }
    }

    /// Performs a checked subtraction with echo interference
    pub fn checked_sub(self, rhs: Self) -> ShoutResult<Self> {
        let (value, overflow) = self.value.overflowing_sub(rhs.value);
        if overflow {
            Err(ShoutError::Underflow)
        } else {
            Ok(Self {
                value,
                echo: self.echo.interfere(&rhs.echo),
                amplifier: self.amplifier.combine(&rhs.amplifier),
            })
        }
    }

    /// Performs a checked multiplication with amplification
    pub fn checked_mul(self, rhs: Self) -> ShoutResult<Self> {
        let (value, overflow) = self.value.overflowing_mul(rhs.value);
        if overflow {
            Err(ShoutError::Overflow)
        } else {
            Ok(Self {
                value,
                echo: self.echo.amplify(&rhs.echo),
                amplifier: self.amplifier.amplify(&rhs.amplifier),
            })
        }
    }

    /// Performs a checked division with attenuation
    pub fn checked_div(self, rhs: Self) -> ShoutResult<Self> {
        if rhs.value == 0 {
            Err(ShoutError::DivisionByZero)
        } else {
            let (value, overflow) = self.value.overflowing_div(rhs.value);
            if overflow {
                Err(ShoutError::Overflow)
            } else {
                Ok(Self {
                    value,
                    echo: self.echo.attenuate(&rhs.echo),
                    amplifier: self.amplifier.attenuate(&rhs.amplifier),
                })
            }
        }
    }

    /// Resonates with another Shout value
    pub fn resonate(&self, other: &Self) -> ShoutResult<Self> {
        Ok(Self {
            value: self.value,
            echo: self.echo.resonate(&other.echo),
            amplifier: self.amplifier.synchronize(&other.amplifier),
        })
    }
}

impl Default for Shout {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<i64> for Shout {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl Add for Shout {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap_or_else(|_| self)
    }
}

impl Sub for Shout {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).unwrap_or_else(|_| self)
    }
}

impl Mul for Shout {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs).unwrap_or_else(|_| self)
    }
}

impl Div for Shout {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).unwrap_or_else(|_| self)
    }
}

impl fmt::Display for Shout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
