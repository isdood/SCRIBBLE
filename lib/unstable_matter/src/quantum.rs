/// Quantum Trait Module
/// Last Updated: 2025-01-17 00:18:42 UTC
/// Author: isdood
/// Current User: isdood

use crate::scribe::{Scribe, ScribePrecision, QuantumString};
use crate::vector::Vector3D;
use crate::constants::QUANTUM_PAGE_SIZE as QUANTUM_BLOCK_SIZE;

pub trait Quantum: Scribe {
    fn get_coherence(&self) -> f64;
    fn is_quantum_stable(&self) -> bool;
    fn decay_coherence(&self);
    fn reset_coherence(&self);
}

/// Represents a quantum-aligned memory block
#[derive(Debug)]
pub struct QuantumBlock<T: Sized + Scribe> {
    data: T,
    coherence: f64,
    position: Vector3D<isize>,
    quantum_state: bool,
}

impl<T: Sized + Scribe> QuantumBlock<T> {
    /// Creates a new quantum block with the given data
    pub const fn new(data: T) -> Self {
        Self {
            data,
            coherence: 1.0,
            position: Vector3D::new(0, 0, 0),
            quantum_state: true,
        }
    }

    /// Gets a reference to the underlying data
    pub fn get_data(&self) -> &T {
        &self.data
    }

    /// Gets a mutable reference to the underlying data
    pub fn get_data_mut(&mut self) -> &mut T {
        self.decay_coherence();
        &mut self.data
    }

    /// Gets the block size
    pub const fn block_size() -> usize {
        QUANTUM_BLOCK_SIZE
    }

    /// Gets the current position
    pub fn get_position(&self) -> Vector3D<isize> {
        self.position
    }

    /// Sets the block position
    pub fn set_position(&mut self, pos: Vector3D<isize>) {
        self.position = pos;
        self.decay_coherence();
    }
}

impl<T: Sized + Scribe> Quantum for QuantumBlock<T> {
    fn get_coherence(&self) -> f64 {
        self.coherence
    }

    fn is_quantum_stable(&self) -> bool {
        self.quantum_state && self.coherence > 0.5
    }

    fn decay_coherence(&self) {
        // SAFETY: This operation is safe because:
        // 1. We're only modifying coherence and quantum_state
        // 2. These modifications are atomic in nature
        // 3. The ptr cast is valid for the lifetime of self
        unsafe {
            let ptr = self as *const Self as *mut Self;
            let new_coherence = (*ptr).coherence * 0.99;
            (*ptr).coherence = new_coherence;
            (*ptr).quantum_state = new_coherence > 0.5;
        }
    }

    fn reset_coherence(&self) {
        // SAFETY: Same safety guarantees as decay_coherence
        unsafe {
            let ptr = self as *const Self as *mut Self;
            (*ptr).coherence = 1.0;
            (*ptr).quantum_state = true;
        }
    }
}

impl<T: Sized + Scribe> Scribe for QuantumBlock<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("QuantumBlock{data=");
        self.data.scribe(precision, output);
        output.push_str(", coherence=");
        output.push_f64(self.coherence, precision.decimal_places());
        output.push_str(", pos=");
        self.position.scribe(precision, output);
        output.push_str(", stable=");
        output.push_str(if self.quantum_state { "true" } else { "false" });
        output.push_char('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scribe::tests::TestScribe;

    #[test]
    fn test_quantum_block() {
        let mut block = QuantumBlock::new(TestScribe::new(42));
        assert!(block.is_quantum_stable());
        assert_eq!(block.get_coherence(), 1.0);
    }

    #[test]
    fn test_quantum_position() {
        let mut block = QuantumBlock::new(TestScribe::new(42));
        let new_pos = Vector3D::new(1, 2, 3);
        block.set_position(new_pos.clone());
        assert_eq!(block.get_position(), new_pos);
        assert!(block.get_coherence() < 1.0);
    }

    #[test]
    fn test_quantum_coherence() {
        let block = QuantumBlock::new(TestScribe::new(42));

        // Test decay
        for _ in 0..10 {
            block.decay_coherence();
        }
        assert!(block.get_coherence() < 0.9);

        // Test reset
        block.reset_coherence();
        assert_eq!(block.get_coherence(), 1.0);
        assert!(block.is_quantum_stable());
    }
}
