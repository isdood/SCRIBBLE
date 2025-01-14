/// Quantum Space-Time Scribing Module
/// Last Updated: 2025-01-14 23:47:10 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    Vector3D,
    phantom::QuantumCell,
};

/// Native quantum-safe string type
pub struct QuantumString {
    buffer: Vec<u8>,
}

impl QuantumString {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn push_str(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes());
    }

    pub fn push_char(&mut self, c: char) {
        let mut buf = [0; 4];
        self.buffer.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
    }

    pub fn push_f64(&mut self, value: f64, precision: usize) {
        let mut int_part = value.abs() as i64;
        let mut frac_part = ((value.abs() - int_part as f64) * 10f64.powi(precision as i32)) as i64;

        if value < 0.0 {
            self.push_char('-');
        }

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

        self.push_char('.');
        for _ in 0..precision {
            let digit = frac_part % 10;
            frac_part /= 10;
            self.push_char((b'0' + digit as u8) as char);
        }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buffer).unwrap_or("ERROR")
    }
}

/// Represents formatting precision for quantum values
#[derive(Clone, Copy)]
pub enum ScribePrecision {
    /// High precision (10⁻³⁵)
    Planck,
    /// Medium precision (10⁻¹⁰)
    Quantum,
    /// Standard precision (10⁻⁶)
    Standard,
}

impl ScribePrecision {
    fn decimal_places(&self) -> usize {
        match self {
            Self::Planck => 35,
            Self::Quantum => 10,
            Self::Standard => 6,
        }
    }

    fn epsilon(&self) -> f64 {
        match self {
            Self::Planck => PLANCK_LENGTH,
            Self::Quantum => QUANTUM_THRESHOLD,
            Self::Standard => 1e-6,
        }
    }
}

/// Native quantum-safe scribing trait
pub trait Scribe {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString);

    fn to_string(&self, precision: ScribePrecision) -> String {
        let mut qs = QuantumString::new();
        self.scribe(precision, &mut qs);
        qs.as_str().to_string()
    }
}

impl<T: Copy> Scribe for Vector3D<T> where T: Into<f64> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_char('⟨');
        output.push_f64(self.x().into(), precision.decimal_places());
        output.push_str(", ");
        output.push_f64(self.y().into(), precision.decimal_places());
        output.push_str(", ");
        output.push_f64(self.z().into(), precision.decimal_places());
        output.push_char('⟩');
    }
}

impl Scribe for f64 {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        if self.abs() < precision.epsilon() {
            output.push_f64(0.0, precision.decimal_places());
        } else {
            output.push_f64(*self, precision.decimal_places());
        }
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
    fn test_vector_scribe() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let mut qs = QuantumString::new();
        v.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "⟨1.000000, 2.000000, 3.000000⟩");
    }

    #[test]
    fn test_small_values() {
        let v = Vector3D::new(1e-36, 1e-11, 1.0);
        let mut qs = QuantumString::new();
        v.scribe(ScribePrecision::Standard, &mut qs);
        assert_eq!(qs.as_str(), "⟨0.000000, 0.000000, 1.000000⟩");
    }
}
