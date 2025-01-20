//! Qubit - Quantum Bit Operations
//! ==========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Version: 0.1.0
//! License: MIT

use core::fmt::{self, Display, Formatter};
use errors::MathError;

/// A quantum bit (qubit)
#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    /// Probability amplitude for state |0>
    alpha: f64,
    /// Probability amplitude for state |1>
    beta: f64,
}

impl Qubit {
    /// Create a new qubit with given probability amplitudes
    pub fn new(alpha: f64, beta: f64) -> Result<Self, MathError> {
        if alpha.powi(2) + beta.powi(2) > 1.0 {
            return Err(MathError::InvalidRange);
        }
        Ok(Self { alpha, beta })
    }

    /// Get probability amplitude for state |0>
    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    /// Get probability amplitude for state |1>
    pub fn beta(&self) -> f64 {
        self.beta
    }

    /// Measure the qubit, collapsing it to |0> or |1>
    pub fn measure(&self) -> Result<u8, MathError> {
        let prob_0 = self.alpha.powi(2);
        let random_value = rand::random::<f64>();
        if random_value < prob_0 {
            Ok(0) // Collapsed to state |0>
        } else {
            Ok(1) // Collapsed to state |1>
        }
    }
}

impl Display for Qubit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Qubit(|0>: {:.2}, |1>: {:.2})", self.alpha, self.beta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qubit_creation() {
        let qubit = Qubit::new(0.6, 0.8).unwrap();
        assert_eq!(qubit.alpha(), 0.6);
        assert_eq!(qubit.beta(), 0.8);
    }

    #[test]
    fn test_invalid_qubit_creation() {
        let result = Qubit::new(1.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_qubit_measurement() {
        let qubit = Qubit::new(0.6, 0.8).unwrap();
        let result = qubit.measure();
        assert!(result.is_ok());
        let state = result.unwrap();
        assert!(state == 0 || state == 1);
    }
}
