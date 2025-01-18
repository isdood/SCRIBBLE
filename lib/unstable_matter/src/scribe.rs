/// Quantum Space-Time Scribing Module
/// Last Updated: 2025-01-16 02:36:23 UTC
/// Author: isdood
/// Current User: isdood

use std::fmt::Write;
use crate::constants::*;

/// Native quantum-safe string type
#[derive(Debug, Default, Clone)]
pub struct QuantumString {
    buffer: Vec<u8>,
}

impl QuantumString {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { buffer: Vec::with_capacity(capacity) }
    }

    pub fn push_str(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes());
    }

    pub fn push_char(&mut self, c: char) {
        let mut buf = [0; 4];
        self.buffer.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
    }

    pub fn push_f64(&mut self, value: f64, precision: usize) {
        if value.is_nan() {
            self.push_str("NaN");
            return;
        }
        if value.is_infinite() {
            self.push_str(if value.is_sign_positive() { "∞" } else { "-∞" });
            return;
        }

        let mut buffer = String::with_capacity(20);
        write!(&mut buffer, "{:.*}", precision, value)
        .expect("Failed to format f64");
        self.push_str(&buffer);
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buffer).unwrap_or("ERROR")
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

/// Represents formatting precision for quantum values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScribePrecision {
    /// High precision (10⁻³⁵)
    Planck,
    /// Medium precision (10⁻¹⁰)
    Quantum,
    /// Standard precision (10⁻⁶)
    Standard,
}

impl Default for ScribePrecision {
    fn default() -> Self {
        Self::Standard
    }
}

impl ScribePrecision {
    pub fn decimal_places(&self) -> usize {
        match self {
            Self::Planck => 35,
            Self::Quantum => 10,
            Self::Standard => 6,
        }
    }

    pub fn epsilon(&self) -> f64 {
        match self {
            Self::Planck => PLANCK_LENGTH,
            Self::Quantum => QUANTUM_THRESHOLD,
            Self::Standard => 1e-6,
        }
    }

    pub fn format_value(&self, value: f64) -> f64 {
        if value.abs() < self.epsilon() {
            0.0
        } else {
            value
        }
    }
}

/// Native quantum-safe scribing trait
pub trait Scribe {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString);

    fn to_string(&self) -> String {
        let mut qs = QuantumString::new();
        self.scribe(ScribePrecision::Standard, &mut qs);
        ToString::to_string(qs.as_str())
    }
}

impl Scribe for f64 {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_f64(precision.format_value(*self), precision.decimal_places());
    }
}

impl Scribe for bool {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str(if *self { "true" } else { "false" });
    }
}

impl Scribe for usize {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        let mut buffer = String::with_capacity(20);
        write!(&mut buffer, "{}", self)
        .expect("Failed to format usize");
        output.push_str(&buffer);
    }
}

impl Scribe for str {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str(self);
    }
}

impl Scribe for isize {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str(&self.to_string());
    }
}

impl Scribe for usize {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str(&self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_string() {
        let mut qs = QuantumString::new();
        qs.push_str("test");
        qs.push_char('!');
        assert_eq!(qs.as_str(), "test!");
    }

    #[test]
    fn test_quantum_string_with_capacity() {
        let mut qs = QuantumString::with_capacity(100);
        qs.push_str("test");
        assert_eq!(qs.as_str(), "test");
    }

    #[test]
    fn test_float_special_values() {
        let mut qs = QuantumString::new();
        f64::NAN.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "NaN");

        qs.clear();
        f64::INFINITY.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "∞");

        qs.clear();
        f64::NEG_INFINITY.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "-∞");
    }

    #[test]
    fn test_small_values() {
        let mut qs = QuantumString::new();
        let small = 1e-36;
        small.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "0.000000");
    }

    #[test]
    fn test_precision_formatting() {
        let mut qs = QuantumString::new();
        let x = 123.456789;
        x.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "123.456789");

        qs.clear();
        x.scribe(ScribePrecision::Quantum, &mut qs);
        assert_eq!(qs.as_str(), "123.4567890000");
    }

    #[test]
    fn test_string_scribing() {
        let mut qs = QuantumString::new();
        "test string".scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "test string");
    }
}
