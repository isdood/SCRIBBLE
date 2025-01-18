//! Crystalline Scribe Implementation
//! ==============================
//!
//! Provides quantum-safe string operations through crystalline
//! data structures and harmonic resonance.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:39:21 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    aether::Aether,
    harmony::Quantum,
    cube::Box
};

/// Maximum length for quantum strings
const MAX_QUANTUM_STRING_LENGTH: usize = 1024;

/// Crystalline precision levels for quantum-safe string operations
#[derive(Debug, Clone, Copy)]
pub enum ScribePrecision {
    /// Low precision, high stability
    Coarse = 2,
    /// Medium precision, balanced stability
    Standard = 4,
    /// High precision, requires strong coherence
    Fine = 8,
    /// Maximum precision, perfect crystalline alignment required
    Ultra = 16,
}

impl ScribePrecision {
    /// Gets the number of decimal places for this precision level
    pub fn decimal_places(&self) -> usize {
        *self as usize
    }
}

/// A quantum-safe string implementation
#[derive(Debug)]
pub struct QuantumString {
    /// Crystalline data buffer
    buffer: Box<[u8]>,
    /// Current length of coherent data
    length: usize,
}

impl QuantumString {
    /// Creates a new empty quantum string
    pub fn new() -> Self {
        Self {
            buffer: Box::new([0; MAX_QUANTUM_STRING_LENGTH]),
            length: 0,
        }
    }

    /// Appends a string slice while maintaining quantum coherence
    pub fn push_str(&mut self, s: &str) {
        let new_len = self.length.saturating_add(s.len());
        if new_len <= MAX_QUANTUM_STRING_LENGTH {
            self.buffer[self.length..new_len].copy_from_slice(s.as_bytes());
            self.length = new_len;
        }
    }

    /// Appends a floating-point number with quantum-safe precision
    pub fn push_f64(&mut self, value: f64, precision: usize) {
        // Convert to fixed-point for quantum stability
        let scaled = (value * 10f64.powi(precision as i32)) as i64;
        let mut buf = [0u8; 32];
        let mut pos = 0;

        // Handle negative values
        if scaled < 0 {
            buf[pos] = b'-';
            pos += 1;
        }

        // Convert absolute value to string
        let mut abs = scaled.abs();
        let whole = abs / 10i64.pow(precision as u32);
        abs %= 10i64.pow(precision as u32);

        // Write whole part
        let whole_str = whole.to_string();
        buf[pos..pos + whole_str.len()].copy_from_slice(whole_str.as_bytes());
        pos += whole_str.len();

        // Write fractional part if needed
        if precision > 0 {
            buf[pos] = b'.';
            pos += 1;

            let frac_str = abs.to_string();
            let padding = precision.saturating_sub(frac_str.len());

            // Add leading zeros
            for _ in 0..padding {
                buf[pos] = b'0';
                pos += 1;
            }

            buf[pos..pos + frac_str.len()].copy_from_slice(frac_str.as_bytes());
            pos += frac_str.len();
        }

        // Append to quantum string
        self.push_str(core::str::from_utf8(&buf[..pos]).unwrap_or("ERROR"));
    }

    /// Gets the current length of coherent data
    pub fn len(&self) -> usize {
        self.length
    }

    /// Checks if the string is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Clears the quantum string
    pub fn clear(&mut self) {
        self.length = 0;
    }
}

/// Trait for types that can be written to a quantum string
pub trait Scribe {
    /// Writes the value to a quantum string with the given precision
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString);
}

impl Scribe for str {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str(self);
    }
}

impl Scribe for f64 {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_f64(*self, precision.decimal_places());
    }
}

impl Scribe for i64 {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str(&self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_string_basic() {
        let mut qs = QuantumString::new();
        assert!(qs.is_empty());

        qs.push_str("Hello");
        assert_eq!(qs.len(), 5);

        qs.clear();
        assert!(qs.is_empty());
    }

    #[test]
    fn test_quantum_string_numbers() {
        let mut qs = QuantumString::new();

        let value = 42.123456789;
        qs.push_f64(value, ScribePrecision::Standard.decimal_places());
        assert!(!qs.is_empty());

        qs.clear();
        qs.push_f64(-123.45, ScribePrecision::Fine.decimal_places());
        assert!(!qs.is_empty());
    }

    #[test]
    fn test_scribe_implementations() {
        let mut qs = QuantumString::new();

        "test".scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.len(), 4);

        qs.clear();
        42.0f64.scribe(ScribePrecision::Fine, &mut qs);
        assert!(!qs.is_empty());

        qs.clear();
        42i64.scribe(ScribePrecision::Standard, &mut qs);
        assert!(!qs.is_empty());
    }
}
