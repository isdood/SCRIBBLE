//! Crystalline Scribe Implementation
//! =============================
//!
//! Provides quantum-safe string operations through crystalline
//! data structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:51:54 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::QUANTUM_STABILITY_THRESHOLD,
    harmony::{Quantum, MeshValue},
    CrystalArray,
    CrystalCube
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

/// A quantum-safe string implementation
#[derive(Clone)]
pub struct QuantumString {
    /// Crystalline data buffer
    buffer: CrystalArray<u8>,
    /// Current length of coherent data
    length: usize,
    /// Quantum coherence tracking
    coherence: f64,
}

impl QuantumString {
    /// Creates a new empty quantum string
    pub fn new() -> Self {
        Self {
            buffer: CrystalArray::with_capacity(MAX_QUANTUM_STRING_LENGTH),
            length: 0,
            coherence: 1.0,
        }
    }

    /// Appends a byte slice while maintaining quantum coherence
    pub fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        let new_len = self.length.saturating_add(bytes.len());
        if new_len > MAX_QUANTUM_STRING_LENGTH {
            return Err("Quantum string capacity exceeded");
        }

        // Safety: We check bounds above
        unsafe {
            core::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                                           self.buffer.as_mut_ptr().add(self.length),
                                           bytes.len()
            );
        }

        self.length = new_len;
        self.decohere();
        Ok(())
    }

    /// Gets the current length of coherent data
    pub fn len(&self) -> usize {
        self.length
    }

    /// Checks if the string is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Gets the current quantum coherence value
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Gets a slice of the quantum string data
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.buffer.as_ptr(), self.length)
        }
    }

    /// Gets a mutable slice of the quantum string data
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self.buffer.as_mut_ptr(), self.length)
        }
    }

    /// Clears the quantum string
    pub fn clear(&mut self) {
        self.length = 0;
        self.recohere();
    }
}

impl Quantum for QuantumString {
    fn coherence(&self) -> f64 {
        self.coherence
    }

    fn is_stable(&self) -> bool {
        self.coherence >= QUANTUM_STABILITY_THRESHOLD
    }

    fn decohere(&mut self) {
        self.coherence *= 0.9;
        if self.coherence < QUANTUM_STABILITY_THRESHOLD {
            self.coherence = QUANTUM_STABILITY_THRESHOLD;
        }
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
    }
}

impl Drop for QuantumString {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_string_basics() {
        let mut qstr = QuantumString::new();
        assert!(qstr.is_empty());
        assert_eq!(qstr.len(), 0);

        qstr.push_bytes(b"Hello").unwrap();
        assert_eq!(qstr.len(), 5);
        assert_eq!(qstr.as_bytes(), b"Hello");
    }

    #[test]
    fn test_quantum_string_coherence() {
        let mut qstr = QuantumString::new();
        assert!(qstr.is_stable());

        qstr.push_bytes(b"Test").unwrap();
        assert!(qstr.coherence() < 1.0);

        qstr.recohere();
        assert_eq!(qstr.coherence(), 1.0);
    }

    #[test]
    fn test_quantum_string_limits() {
        let mut qstr = QuantumString::new();
        let large_data = [b'x'; MAX_QUANTUM_STRING_LENGTH + 1];
        assert!(qstr.push_bytes(&large_data).is_err());
    }

    #[test]
    fn test_quantum_string_clear() {
        let mut qstr = QuantumString::new();
        qstr.push_bytes(b"Test").unwrap();
        qstr.clear();
        assert!(qstr.is_empty());
        assert_eq!(qstr.coherence(), 1.0);
    }
}
