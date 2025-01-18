/// Quantum Space-Time Scribing Module
/// Last Updated: 2025-01-18 17:09:18 UTC
/// Author: isdood
/// Current User: isdood

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

        // Native f64 to string conversion
        let mut int_part = value.abs().trunc() as i64;
        let mut frac_part = value.abs().fract();

        // Handle negative numbers
        if value < 0.0 {
            self.push_char('-');
        }

        // Convert integer part
        if int_part == 0 {
            self.push_char('0');
        } else {
            let mut digits = Vec::new();
            while int_part > 0 {
                digits.push((b'0' + (int_part % 10) as u8) as char);
                int_part /= 10;
            }
            for digit in digits.iter().rev() {
                self.push_char(*digit);
            }
        }

        // Handle fractional part
        if precision > 0 {
            self.push_char('.');
            for _ in 0..precision {
                frac_part *= 10.0;
                let digit = frac_part.trunc() as u8;
                self.push_char((b'0' + digit) as char);
                frac_part -= digit as f64;
            }
        }
    }

    pub fn push_isize(&mut self, value: isize) {
        if value < 0 {
            self.push_char('-');
        }
        let mut num = value.abs();
        if num == 0 {
            self.push_char('0');
            return;
        }
        let mut digits = Vec::new();
        while num > 0 {
            digits.push((b'0' + (num % 10) as u8) as char);
            num /= 10;
        }
        for digit in digits.iter().rev() {
            self.push_char(*digit);
        }
    }

    pub fn push_usize(&mut self, value: usize) {
        if value == 0 {
            self.push_char('0');
            return;
        }
        let mut num = value;
        let mut digits = Vec::new();
        while num > 0 {
            digits.push((b'0' + (num % 10) as u8) as char);
            num /= 10;
        }
        for digit in digits.iter().rev() {
            self.push_char(*digit);
        }
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
}

// Implementation for primitive types
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

impl Scribe for str {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str(self);
    }
}

impl Scribe for isize {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_isize(*self);
    }
}

impl Scribe for usize {
    fn scribe(&self, _precision: ScribePrecision, output: &mut QuantumString) {
        output.push_usize(*self);
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

    #[test]
    fn test_integer_scribing() {
        let mut qs = QuantumString::new();

        let i: isize = -42;
        i.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "-42");

        qs.clear();
        let u: usize = 42;
        u.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "42");
    }

    #[test]
    fn test_large_numbers() {
        let mut qs = QuantumString::new();
        let large = 123456789.123456789;
        large.scribe(ScribePrecision::Quantum, &mut qs);
        assert_eq!(qs.as_str(), "123456789.1234567890");
    }
}
