//! Crystalline Scribe Implementation
//! ===========================
//!
//! Core quantum string operations through crystalline
//! structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 21:19:42 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::QUANTUM_STABILITY_THRESHOLD,
    harmony::Quantum,
    vector::Vector3D,
    idk::ShardUninit,
};

/// A quantum string state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StringState {
    /// Coherent string
    Coherent,
    /// Entangled string
    Entangled,
    /// Decoherent string
    Decoherent,
}

/// A quantum string for crystalline operations
#[derive(Clone)]
pub struct QuantumString {
    /// String data
    data: [ShardUninit<u8>; 1024],
    /// Current length
    length: usize,
    /// String position
    position: Vector3D<f64>,
    /// Current state
    state: StringState,
    /// Quantum coherence
    coherence: f64,
}

impl QuantumString {
    /// Creates a new quantum string from bytes
    pub fn new(bytes: &[u8]) -> Option<Self> {
        if bytes.len() > 1024 {
            return None;
        }

        let mut data = [ShardUninit::uninit(); 1024];
        for (i, &byte) in bytes.iter().enumerate() {
            data[i] = ShardUninit::new(byte);
        }

        Some(Self {
            data,
            length: bytes.len(),
             position: Vector3D::zero(),
             state: StringState::Coherent,
             coherence: 1.0,
        })
    }

    /// Gets the string length
    pub fn len(&self) -> usize {
        self.length
    }

    /// Checks if string is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Gets the string bytes
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = self.data.as_ptr() as *const u8;
            core::slice::from_raw_parts(ptr, self.length)
        }
    }

    /// Gets the string position
    pub fn position(&self) -> &Vector3D<f64> {
        &self.position
    }

    /// Gets the current state
    pub fn state(&self) -> StringState {
        self.state
    }

    /// Sets the position and updates quantum state
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position = Vector3D::new(x, y, z);
        self.decohere();
    }

    /// Attempts to inscribe into a crystal cube
    pub fn inscribe(&mut self, cube: &mut CrystalCube<u8>) -> Result<(), &'static str> {
        if self.state == StringState::Decoherent {
            return Err("Cannot inscribe decoherent string");
        }

        let pos = self.position();
        let start_x = meshmath::floor(pos.x) as usize;
        let y = meshmath::floor(pos.y) as usize;
        let z = meshmath::floor(pos.z) as usize;

        for (i, byte) in self.as_bytes().iter().enumerate() {
            let x = start_x + i;
            if let Some(cell) = cube.get_mut(x, y, z) {
                *cell = *byte;
            } else {
                return Err("String exceeds cube bounds");
            }
        }

        self.state = StringState::Entangled;
        self.decohere();
        cube.decohere();
        Ok(())
    }

    /// Attempts to extract from a crystal cube
    pub fn extract(&mut self, cube: &CrystalCube<u8>) -> Result<(), &'static str> {
        let pos = self.position();
        let start_x = meshmath::floor(pos.x) as usize;
        let y = meshmath::floor(pos.y) as usize;
        let z = meshmath::floor(pos.z) as usize;

        let mut length = 0;
        for i in 0..1024 {
            let x = start_x + i;
            if let Some(&byte) = cube.get(x, y, z) {
                if byte == 0 {
                    break;
                }
                self.data[i] = ShardUninit::new(byte);
                length += 1;
            } else {
                break;
            }
        }

        if length == 0 {
            return Err("No string found at position");
        }

        self.length = length;
        self.state = StringState::Coherent;
        self.decohere();
        Ok(())
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
            self.state = StringState::Decoherent;
        }
    }

    fn recohere(&mut self) {
        self.coherence = 1.0;
        self.state = StringState::Coherent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_string_basics() {
        let string = QuantumString::new(b"Hello, quantum world!").unwrap();
        assert_eq!(string.len(), 19);
        assert_eq!(string.as_bytes(), b"Hello, quantum world!");
        assert_eq!(string.state(), StringState::Coherent);
        assert!(string.is_stable());
    }

    #[test]
    fn test_quantum_string_limits() {
        let long_data = [0u8; 1025];
        assert!(QuantumString::new(&long_data).is_none());
    }

    #[test]
    fn test_string_empty() {
        let string = QuantumString::new(b"").unwrap();
        assert!(string.is_empty());
        assert_eq!(string.len(), 0);
    }
}
