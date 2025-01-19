//! Native String Type for Scribe
//! =============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 18:02:27 UTC
//! Version: 0.1.0
//! License: MIT

#[derive(Debug, Clone)] // Derive Debug and Clone traits
pub struct String {
    inner: std::vec::Vec<u8>,
}

impl String {
    pub fn new() -> Self {
        String { inner: std::vec::Vec::new() }
    }

    pub fn push_str(&mut self, s: &str) {
        self.inner.extend_from_slice(s.as_bytes());
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.inner).unwrap()
    }

    pub fn to_str(&self) -> &str {
        self.as_str()
    }
}
