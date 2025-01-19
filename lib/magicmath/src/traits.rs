//! Traits for Crystal Lattice HPC Systems
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:37:06 UTC
//! Version: 0.1.0
//! License: MIT

use crate::errors::MathError;

/// Core trait for values that can be used in crystal lattice calculations
pub trait MeshValue: Sized {
    /// Get coherence value of the type
    fn coherence(&self) -> Result<f64, MathError>;

    /// Get energy value of the type
    fn energy(&self) -> Result<f64, MathError>;

    /// Get magnitude of the value
    fn magnitude(&self) -> Result<f64, MathError>;

    /// Convert value to usize
    fn to_usize(&self) -> Result<usize, MathError>;

    /// Convert value to f64
    fn to_f64(&self) -> Result<f64, MathError>;

    /// Create value from f64
    fn from(value: f64) -> Self;
}

/// Trait for complex quantum values
pub trait ComplexQuantum: MeshValue {
    /// Get real part of complex number
    fn real(&self) -> Result<f64, MathError>;

    /// Get imaginary part of complex number
    fn imag(&self) -> Result<f64, MathError>;

    /// Get phase angle of complex number
    fn phase(&self) -> Result<f64, MathError>;

    /// Create new complex quantum value
    fn new_complex(real: f64, imag: f64) -> Self;

    /// Get complex conjugate
    fn conjugate(&self) -> Result<Self, MathError>;
}

/// Trait for fractal iteration values
pub trait FractalValue: ComplexQuantum {
    /// Check if point is in set
    fn in_set(&self) -> Result<bool, MathError>;

    /// Get escape time
    fn escape_time(&self) -> Result<Option<usize>, MathError>;

    /// Get orbit of point
    fn orbit(&self) -> Result<Vec<(f64, f64)>, MathError>;

    /// Check stability of point
    fn is_stable(&self) -> Result<bool, MathError>;

    /// Get current iteration count
    fn iterations(&self) -> Result<usize, MathError>;
}

/// Trait for quantum state tracking
pub trait QuantumState {
    /// Get current coherence
    fn get_coherence(&self) -> Result<f64, MathError>;

    /// Get current phase
    fn get_phase(&self) -> Result<f64, MathError>;

    /// Get current energy
    fn get_energy(&self) -> Result<f64, MathError>;

    /// Get current stability
    fn get_stability(&self) -> Result<f64, MathError>;

    /// Update quantum state
    fn update_state(&mut self, coherence: f64, phase: f64, energy: f64) -> Result<(), MathError>;

    /// Check if state is stable
    fn is_stable(&self) -> Result<bool, MathError>;
}

/// Trait for quantum operations
pub trait QuantumOperation {
    /// Get operation coherence factor
    fn coherence_factor(&self) -> f64;

    /// Get operation phase shift
    fn phase_shift(&self) -> f64;

    /// Get operation energy factor
    fn energy_factor(&self) -> f64;

    /// Get operation stability impact
    fn stability_impact(&self) -> f64;
}

/// Trait for resonance calculations
pub trait Resonance {
    /// Calculate quantum resonance
    fn quantum_resonance(&self) -> Result<f64, MathError>;

    /// Calculate golden ratio resonance
    fn golden_resonance(&self) -> Result<f64, MathError>;

    /// Calculate Fibonacci resonance
    fn fibonacci_resonance(&self) -> Result<f64, MathError>;

    /// Calculate harmonic resonance
    fn harmonic_resonance(&self) -> Result<f64, MathError>;
}

/// Implementation examples
#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::PI;

    // Example complex quantum number implementation
    #[derive(Debug, Clone, Copy)]
    struct TestComplex {
        real: f64,
        imag: f64,
        iterations: usize,
    }

    impl MeshValue for TestComplex {
        fn coherence(&self) -> Result<f64, MathError> {
            Ok(1.0)
        }

        fn energy(&self) -> Result<f64, MathError> {
            Ok(self.real * self.real + self.imag * self.imag)
        }

        fn magnitude(&self) -> Result<f64, MathError> {
            Ok((self.real * self.real + self.imag * self.imag).sqrt())
        }

        fn to_usize(&self) -> Result<usize, MathError> {
            Ok(self.real as usize)
        }

        fn to_f64(&self) -> Result<f64, MathError> {
            Ok(self.real)
        }

        fn from(value: f64) -> Self {
            Self {
                real: value,
                imag: 0.0,
                iterations: 0,
            }
        }
    }

    impl ComplexQuantum for TestComplex {
        fn real(&self) -> Result<f64, MathError> {
            Ok(self.real)
        }

        fn imag(&self) -> Result<f64, MathError> {
            Ok(self.imag)
        }

        fn phase(&self) -> Result<f64, MathError> {
            Ok(self.imag.atan2(self.real))
        }

        fn new_complex(real: f64, imag: f64) -> Self {
            Self {
                real,
                imag,
                iterations: 0,
            }
        }

        fn conjugate(&self) -> Result<Self, MathError> {
            Ok(Self {
                real: self.real,
                imag: -self.imag,
                iterations: self.iterations,
            })
        }
    }

    impl FractalValue for TestComplex {
        fn in_set(&self) -> Result<bool, MathError> {
            Ok(self.magnitude()? <= 2.0)
        }

        fn escape_time(&self) -> Result<Option<usize>, MathError> {
            if self.in_set()? {
                Ok(None)
            } else {
                Ok(Some(self.iterations))
            }
        }

        fn orbit(&self) -> Result<Vec<(f64, f64)>, MathError> {
            Ok(vec![(self.real, self.imag)])
        }

        fn is_stable(&self) -> Result<bool, MathError> {
            Ok(true)
        }

        fn iterations(&self) -> Result<usize, MathError> {
            Ok(self.iterations)
        }
    }

    #[test]
    fn test_complex_quantum() {
        let z = TestComplex::new_complex(3.0, 4.0);
        assert_eq!(z.magnitude().unwrap(), 5.0);
        assert_eq!(z.phase().unwrap(), 4.0f64.atan2(3.0));
    }

    #[test]
    fn test_fractal_value() {
        let z = TestComplex::new_complex(0.0, 0.0);
        assert!(z.in_set().unwrap());
        assert!(z.is_stable().unwrap());
    }

    #[test]
    fn test_mesh_value() {
        let z = TestComplex::from(1.0);
        assert_eq!(z.to_f64().unwrap(), 1.0);
        assert_eq!(z.to_usize().unwrap(), 1);
    }

    #[test]
    fn test_conjugate() {
        let z = TestComplex::new_complex(1.0, 2.0);
        let conj = z.conjugate().unwrap();
        assert_eq!(conj.imag().unwrap(), -2.0);
    }
}
