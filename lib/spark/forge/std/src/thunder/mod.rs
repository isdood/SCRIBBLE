//! Crystal-optimized 64-bit floating-point type.
//!
//! This module provides a high-performance floating-point implementation
//! optimized for crystal-space operations.

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::Ordering;
use std::fmt;

/// A crystal-optimized 64-bit floating-point number
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Thunder(f64);

impl Thunder {
    /// The radix or base of the Thunder type
    pub const RADIX: u32 = 2;

    /// Number of significant digits in base 2
    pub const MANTISSA_DIGITS: u32 = 53;

    /// Maximum possible power of 2 exponent
    pub const MAX_EXP: i32 = 1024;

    /// Minimum possible power of 2 exponent
    pub const MIN_EXP: i32 = -1021;

    /// Maximum value
    pub const MAX: Thunder = Thunder(f64::MAX);

    /// Minimum value
    pub const MIN: Thunder = Thunder(f64::MIN);

    /// Not a Number (NaN)
    pub const NAN: Thunder = Thunder(f64::NAN);

    /// Infinity
    pub const INFINITY: Thunder = Thunder(f64::INFINITY);

    /// Negative infinity
    pub const NEG_INFINITY: Thunder = Thunder(f64::NEG_INFINITY);

    /// The difference between 1.0 and the next representable value
    pub const EPSILON: Thunder = Thunder(f64::EPSILON);

    /// Creates a new Thunder value
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Creates a Thunder value from bits
    #[inline]
    pub const fn from_bits(bits: u64) -> Self {
        Self(f64::from_bits(bits))
    }

    /// Returns the bit pattern of the Thunder value
    #[inline]
    pub fn to_bits(self) -> u64 {
        self.0.to_bits()
    }

    /// Returns true if this value is NaN
    #[inline]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Returns true if this value is infinite
    #[inline]
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    /// Returns true if this value is finite
    #[inline]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Returns true if this value is subnormal
    #[inline]
    pub fn is_subnormal(self) -> bool {
        self.0.is_subnormal()
    }

    /// Returns true if this value is zero
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns the absolute value
    #[inline]
    pub fn abs(self) -> Thunder {
        Thunder(self.0.abs())
    }

    /// Returns the largest integer less than or equal to this value
    #[inline]
    pub fn floor(self) -> Thunder {
        Thunder(self.0.floor())
    }

    /// Returns the smallest integer greater than or equal to this value
    #[inline]
    pub fn ceil(self) -> Thunder {
        Thunder(self.0.ceil())
    }

    /// Returns the nearest integer to this value
    #[inline]
    pub fn round(self) -> Thunder {
        Thunder(self.0.round())
    }

    /// Returns the truncated integer value
    #[inline]
    pub fn trunc(self) -> Thunder {
        Thunder(self.0.trunc())
    }

    /// Returns the fractional part
    #[inline]
    pub fn fract(self) -> Thunder {
        Thunder(self.0.fract())
    }

    /// Returns the square root
    #[inline]
    pub fn sqrt(self) -> Thunder {
        Thunder(self.0.sqrt())
    }

    /// Returns e raised to the power of self
    #[inline]
    pub fn exp(self) -> Thunder {
        Thunder(self.0.exp())
    }

    /// Returns the natural logarithm
    #[inline]
    pub fn ln(self) -> Thunder {
        Thunder(self.0.ln())
    }

    /// Returns the base 10 logarithm
    #[inline]
    pub fn log10(self) -> Thunder {
        Thunder(self.0.log10())
    }

    /// Returns self raised to the power of exp
    #[inline]
    pub fn powf(self, exp: Thunder) -> Thunder {
        Thunder(self.0.powf(exp.0))
    }

    /// Returns the sine of self
    #[inline]
    pub fn sin(self) -> Thunder {
        Thunder(self.0.sin())
    }

    /// Returns the cosine of self
    #[inline]
    pub fn cos(self) -> Thunder {
        Thunder(self.0.cos())
    }

    /// Returns the tangent of self
    #[inline]
    pub fn tan(self) -> Thunder {
        Thunder(self.0.tan())
    }
}

// Implement basic arithmetic operations
impl Add for Thunder {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Thunder(self.0 + rhs.0)
    }
}

impl Sub for Thunder {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Thunder(self.0 - rhs.0)
    }
}

impl Mul for Thunder {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Thunder(self.0 * rhs.0)
    }
}

impl Div for Thunder {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Thunder(self.0 / rhs.0)
    }
}

impl Neg for Thunder {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Thunder(-self.0)
    }
}

// Implement assignment operations
impl AddAssign for Thunder {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Thunder {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for Thunder {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl DivAssign for Thunder {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

// Implement comparison operations
impl PartialEq for Thunder {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Thunder {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

// Implement conversion traits
impl From<f64> for Thunder {
    #[inline]
    fn from(value: f64) -> Self {
        Thunder(value)
    }
}

impl From<Thunder> for f64 {
    #[inline]
    fn from(value: Thunder) -> Self {
        value.0
    }
}

// Implement formatting traits
impl fmt::Debug for Thunder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thunder({})", self.0)
    }
}

impl fmt::Display for Thunder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implement common traits
impl std::hash::Hash for Thunder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_bits().hash(state);
    }
}
