//! Crystal-optimized rune primitive type
//!
//! A rune is a 32-bit Unicode scalar value optimized for crystal-space operations.

use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

mod error;
pub use error::RuneError;

/// A crystal-space optimized Unicode scalar value
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Rune(u32);

impl Rune {
    /// The replacement character '�' (U+FFFD)
    pub const REPLACEMENT: Rune = Rune(0xFFFD);

    /// The maximum value of a Unicode scalar value (0x10FFFF)
    pub const MAX: Rune = Rune(0x10FFFF);

    /// Creates a new rune from a raw u32 value
    ///
    /// Returns None if the value is not a valid Unicode scalar value
    pub fn new(value: u32) -> Option<Self> {
        if Self::is_valid_unicode(value) {
            Some(Rune(value))
        } else {
            None
        }
    }

    /// Creates a new rune from a raw u32 value without checking
    ///
    /// # Safety
    /// The value must be a valid Unicode scalar value
    pub const unsafe fn from_u32_unchecked(value: u32) -> Self {
        Rune(value)
    }

    /// Returns true if this is an ASCII character
    pub fn is_ascii(&self) -> bool {
        self.0 <= 0x7F
    }

    /// Returns true if this is a valid Unicode scalar value
    pub fn is_valid(&self) -> bool {
        Self::is_valid_unicode(self.0)
    }

    /// Returns the underlying u32 value
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Returns true if this rune is in a given range
    pub fn is_in_range(&self, start: u32, end: u32) -> bool {
        self.0 >= start && self.0 <= end
    }

    /// Returns the number of UTF-8 bytes needed to encode this rune
    pub fn len_utf8(&self) -> usize {
        match self.0 {
            0..=0x7F => 1,
            0x80..=0x7FF => 2,
            0x800..=0xFFFF => 3,
            _ => 4,
        }
    }

    /// Returns true if this rune is a valid Unicode scalar value
    fn is_valid_unicode(value: u32) -> bool {
        value <= 0x10FFFF && !Self::is_surrogate(value)
    }

    /// Returns true if this value is a UTF-16 surrogate
    fn is_surrogate(value: u32) -> bool {
        value >= 0xD800 && value <= 0xDFFF
    }

    /// Encodes this rune as UTF-8 bytes
    pub fn encode_utf8(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4);
        match self.0 {
            0..=0x7F => {
                buf.push(self.0 as u8);
            }
            0x80..=0x7FF => {
                buf.push((0xC0 | (self.0 >> 6)) as u8);
                buf.push((0x80 | (self.0 & 0x3F)) as u8);
            }
            0x800..=0xFFFF => {
                buf.push((0xE0 | (self.0 >> 12)) as u8);
                buf.push((0x80 | ((self.0 >> 6) & 0x3F)) as u8);
                buf.push((0x80 | (self.0 & 0x3F)) as u8);
            }
            _ => {
                buf.push((0xF0 | (self.0 >> 18)) as u8);
                buf.push((0x80 | ((self.0 >> 12) & 0x3F)) as u8);
                buf.push((0x80 | ((self.0 >> 6) & 0x3F)) as u8);
                buf.push((0x80 | (self.0 & 0x3F)) as u8);
            }
        }
        buf
    }

    /// Returns true if this rune is alphabetic
    pub fn is_alphabetic(&self) -> bool {
        // Basic Latin + Latin-1 Supplement
        (self.is_in_range(0x41, 0x5A) || // A-Z
         self.is_in_range(0x61, 0x7A) || // a-z
         self.is_in_range(0xC0, 0xFF)) && // Latin-1 Supplement
        self.0 != 0xD7 && self.0 != 0xF7  // Exclude × and ÷
    }

    /// Returns true if this rune is numeric
    pub fn is_numeric(&self) -> bool {
        self.is_in_range(0x30, 0x39)  // 0-9
    }

    /// Returns true if this rune is alphanumeric
    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_numeric()
    }

    /// Returns true if this rune is whitespace
    pub fn is_whitespace(&self) -> bool {
        matches!(self.0,
            0x20 | 0x9 | 0xA | 0xB | 0xC | 0xD |  // ASCII whitespace
            0x85 | 0x2000..=0x200A |              // Other whitespace
            0x2028 | 0x2029                        // Line/para separators
        )
    }

    /// Returns true if this rune is a control character
    pub fn is_control(&self) -> bool {
        self.is_in_range(0x00, 0x1F) || self.is_in_range(0x7F, 0x9F)
    }
}

impl From<char> for Rune {
    fn from(c: char) -> Self {
        Rune(c as u32)
    }
}

impl TryFrom<u32> for Rune {
    type Error = RuneError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(RuneError::InvalidScalar(value))
    }
}

impl FromStr for Rune {
    type Err = RuneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(Rune::from(c)),
            _ => Err(RuneError::InvalidString(s.to_owned())),
        }
    }
}

impl fmt::Debug for Rune {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rune('{}')", self)
    }
}

impl fmt::Display for Rune {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(c) = std::char::from_u32(self.0) {
            write!(f, "{}", c)
        } else {
            write!(f, "{}", Rune::REPLACEMENT)
        }
    }
}

impl Add<u32> for Rune {
    type Output = Option<Rune>;

    fn add(self, rhs: u32) -> Self::Output {
        Rune::new(self.0.saturating_add(rhs))
    }
}

impl Sub<u32> for Rune {
    type Output = Option<Rune>;

    fn sub(self, rhs: u32) -> Self::Output {
        Rune::new(self.0.saturating_sub(rhs))
    }
}
