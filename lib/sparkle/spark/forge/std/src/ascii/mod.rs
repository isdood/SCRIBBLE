//! Crystal-optimized ASCII string handling
//!
//! This module provides SIMD-accelerated ASCII string operations with proper crystal alignment.

use crate::align::Alignment;
use crate::array::CrystalArray;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::borrow::Cow;

mod set;
pub use set::AsciiSet;

/// A crystal-space aligned ASCII string
#[derive(Clone, Debug)]
pub struct CrystalStr {
    inner: CrystalArray<u8>,
}

impl CrystalStr {
    /// Creates a new empty ASCII string
    pub fn new() -> Self {
        Self {
            inner: CrystalArray::new(Alignment::Crystal16)
        }
    }

    /// Creates a new ASCII string from a byte slice
    /// Returns None if the slice contains non-ASCII bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.iter().all(|&b| b.is_ascii()) {
            let mut inner = CrystalArray::new(Alignment::Crystal16);
            inner.extend_from_slice(bytes);
            Some(Self { inner })
        } else {
            None
        }
    }

    /// Returns true if the string contains only ASCII alphabetic characters
    pub fn is_alphabetic(&self) -> bool {
        self.inner.iter().all(|&b| b.is_ascii_alphabetic())
    }

    /// Returns true if the string contains only ASCII alphanumeric characters
    pub fn is_alphanumeric(&self) -> bool {
        self.inner.iter().all(|&b| b.is_ascii_alphanumeric())
    }

    /// Returns true if the string contains only ASCII decimal digits
    pub fn is_ascii_digit(&self) -> bool {
        self.inner.iter().all(|&b| b.is_ascii_digit())
    }

    /// Returns true if the string contains only ASCII hexadecimal digits
    pub fn is_ascii_hexdigit(&self) -> bool {
        self.inner.iter().all(|&b| b.is_ascii_hexdigit())
    }

    /// Returns true if the string contains only ASCII lowercase characters
    pub fn is_ascii_lowercase(&self) -> bool {
        self.inner.iter().all(|&b| b.is_ascii_lowercase())
    }

    /// Returns true if the string contains only ASCII uppercase characters
    pub fn is_ascii_uppercase(&self) -> bool {
        self.inner.iter().all(|&b| b.is_ascii_uppercase())
    }

    /// Converts the string to ASCII lowercase in-place
    pub fn make_ascii_lowercase(&mut self) {
        for byte in self.inner.iter_mut() {
            byte.make_ascii_lowercase();
        }
    }

    /// Converts the string to ASCII uppercase in-place
    pub fn make_ascii_uppercase(&mut self) {
        for byte in self.inner.iter_mut() {
            byte.make_ascii_uppercase();
        }
    }

    /// Returns true if the string contains the given ASCII set
    pub fn contains_set(&self, set: &AsciiSet) -> bool {
        self.inner.iter().any(|&b| set.contains(b))
    }

    /// Returns true if the string matches the given ASCII set exactly
    pub fn matches_set(&self, set: &AsciiSet) -> bool {
        self.inner.iter().all(|&b| set.contains(b))
    }

    /// Returns the string as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    /// Returns the string as a mutable byte slice
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    /// Returns the length of the string
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the string is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Default for CrystalStr {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CrystalStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Safe because we ensure all bytes are ASCII
        let s = unsafe { std::str::from_utf8_unchecked(self.as_bytes()) };
        f.write_str(s)
    }
}

impl Deref for CrystalStr {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl DerefMut for CrystalStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_bytes_mut()
    }
}

impl From<&str> for CrystalStr {
    fn from(s: &str) -> Self {
        Self::from_bytes(s.as_bytes()).expect("String contains non-ASCII characters")
    }
}

impl<'a> From<CrystalStr> for Cow<'a, str> {
    fn from(s: CrystalStr) -> Self {
        // Safe because we ensure all bytes are ASCII
        unsafe {
            Cow::Owned(String::from_utf8_unchecked(s.inner.to_vec()))
        }
    }
}
