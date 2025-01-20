//! Resonance Mathematics and Operations
//! ================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 17:44:14 UTC
//! Version: 0.1.0
//! License: MIT

use crate::vector3d::Vector3D;
use scribe::Scribe;
use scribe::native_string::String;
use errors::MathError;
use crate::constants::PI;

/// Newtype wrapper for `f64` to implement `Scribe`
#[derive(Debug, Clone, Copy)]
struct WrappedF64(f64);

impl From<f64> for WrappedF64 {
    fn from(value: f64) -> Self {
        WrappedF64(value)
    }
}

impl Scribe for WrappedF64 {
    fn scribe(&self) -> String {
        String::from(self.0.to_string().as_str())
    }
}

// Add String extension traits
trait StringExt {
    fn contains(&self, needle: &str) -> bool;
    fn push_str(&mut self, s: &str);
}

impl StringExt for String {
    fn contains(&self, needle: &str) -> bool {
        self.as_str().contains(needle)
    }

    fn push_str(&mut self, s: &str) {
        let mut new_str = self.as_str().to_owned();
        new_str.push_str(s);
        *self = String::from(new_str.as_str());
    }
}

/// Quantum state trait
pub trait Quantum {
    fn energy(&self) -> Result<f64, MathError>;
    fn phase(&self) -> Result<f64, MathError>;
}

/// Phase operations trait
pub trait Phase {
    fn phase_shift(&mut self, shift: f64) -> Result<(), MathError>;
}

/// Quantum resonance state handler
#[derive(Debug, Clone)]
pub struct Resonance {
    phase: f64,
    energy: f64,
    position: Vector3D,
}

impl Resonance {
    /// Create new resonance state
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            energy: 1.0,
            position: Vector3D::new(0.0, 0.0, 0.0),
        }
    }

    /// Get current energy level
    pub fn energy(&self) -> Result<f64, MathError> {
        if self.energy <= 0.0 {
            return Err(MathError::InvalidParameter(String::from("Energy must be positive")));
        }
        Ok(self.energy)
    }

    /// Set energy level
    pub fn set_energy(&mut self, energy: f64) -> Result<(), MathError> {
        if energy <= 0.0 {
            return Err(MathError::InvalidParameter(String::from("Energy must be positive")));
        }
        self.energy = energy;
        Ok(())
    }

    /// Get current phase
    pub fn phase(&self) -> Result<f64, MathError> {
        Ok(self.phase)
    }

    /// Apply phase shift
    pub fn phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        self.phase = (self.phase + shift) % (2.0 * PI);
        Ok(())
    }

    /// Get current position
    pub fn position(&self) -> Vector3D {
        self.position.clone()
    }

    /// Set new position
    pub fn set_position(&mut self, pos: Vector3D) {
        self.position = pos;
    }

    /// Get coherence level
    pub fn coherence(&self) -> Result<f64, MathError> {
        let mag = self.position.magnitude()?;
        if mag <= 0.0 {
            return Err(MathError::InvalidParameter(String::from("Magnitude must be positive")));
        }
        Ok(mag / HARMONY_STABILITY_THRESHOLD)
    }
}

impl Default for Resonance {
    fn default() -> Self {
        Self::new()
    }
}

impl Quantum for Resonance {
    fn energy(&self) -> Result<f64, MathError> {
        self.energy()
    }

    fn phase(&self) -> Result<f64, MathError> {
        self.phase()
    }
}

impl Phase for Resonance {
    fn phase_shift(&mut self, shift: f64) -> Result<(), MathError> {
        self.phase_shift(shift)
    }
}

impl Scribe for Resonance {
    fn scribe(&self) -> String {
        let mut result = String::new();
        result.push_str("Resonance(phase: ");
        result.push_str(&self.phase.to_string());
        result.push_str(", energy: ");
        result.push_str(&self.energy.to_string());
        result.push_str(", pos: (");
        result.push_str(&self.position.x.to_string());
        result.push_str(", ");
        result.push_str(&self.position.y.to_string());
        result.push_str(", ");
        result.push_str(&self.position.z.to_string());
        result.push_str("))");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_creation() {
        let res = Resonance::new();
        assert_eq!(res.phase().unwrap(), 0.0);
        assert_eq!(res.energy().unwrap(), 1.0);
    }

    #[test]
    fn test_energy_validation() {
        let mut res = Resonance::new();
        assert!(res.set_energy(2.0).is_ok());
        assert!(res.set_energy(0.0).is_err());
        assert!(res.set_energy(-1.0).is_err());
    }

    #[test]
    fn test_phase_shift() {
        let mut res = Resonance::new();
        res.phase_shift(PI).unwrap();
        assert_eq!(res.phase().unwrap(), PI);

        // Test wrapping around 2π
        res.phase_shift(2.0 * PI).unwrap();
        assert_eq!(res.phase().unwrap(), PI);
    }

    #[test]
    fn test_position() {
        let mut res = Resonance::new();
        let pos = Vector3D::new(1.0, 2.0, 3.0);
        res.set_position(pos.clone());
        let retrieved_pos = res.position();
        assert_eq!(retrieved_pos.x, 1.0);
        assert_eq!(retrieved_pos.y, 2.0);
        assert_eq!(retrieved_pos.z, 3.0);
    }

    #[test]
    fn test_quantum_trait() {
        let res = Resonance::new();
        assert_eq!(Quantum::energy(&res).unwrap(), 1.0);
        assert_eq!(Quantum::phase(&res).unwrap(), 0.0);
    }

    #[test]
    fn test_phase_trait() {
        let mut res = Resonance::new();
        assert!(Phase::phase_shift(&mut res, PI).is_ok());
        assert_eq!(res.phase().unwrap(), PI);
    }

    #[test]
    fn test_scribe() {
        let res = Resonance::new();
        let s = res.scribe();
        assert!(s.contains("phase: "));
        assert!(s.contains("energy: "));
        assert!(s.contains("pos: "));
    }
}
