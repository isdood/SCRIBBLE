//! Crystalline Harmony Core
//! =====================
//!
//! Core crystalline quantum computing framework.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:46:09 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]

// Core modules
pub mod aether;
pub mod align;
pub mod constants;
pub mod cube;
pub mod harmony;
pub mod phantom;
pub mod scribe;
pub mod vector;
pub mod zeronaut;

// Re-exports
pub use {
    aether::Aether,
    align::{AlignState, CrystalAlign},
    constants::{
        QUANTUM_STABILITY_THRESHOLD,
        CUBE_TIMESTAMP,
        AETHER_RESONANCE_FACTOR,
    },
    cube::{CrystalCube, SharedCube},
    harmony::{Quantum, MeshValue, MeshOps},
    phantom::QuantumCell,
    scribe::{Scribe, ScribePrecision, QuantumString},
    vector::{Vector3D, Vector4D},
    zeronaut::Zeronaut,
};

/// Core crystalline array type
pub struct CrystalArray<T: Clone + 'static> {
    /// The crystalline data storage
    data: *mut T,
    /// Current length of the array
    len: usize,
    /// Allocated capacity
    capacity: usize,
}

impl<T: Clone + 'static> CrystalArray<T> {
    /// Creates a new empty array
    pub const fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    /// Creates an array with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let data = if capacity > 0 {
            // Safety: We immediately set the length to 0
            unsafe {
                let layout = core::alloc::Layout::array::<T>(capacity).unwrap();
                core::alloc::alloc(layout) as *mut T
            }
        } else {
            core::ptr::null_mut()
        };

        Self {
            data,
            len: 0,
            capacity,
        }
    }

    /// Returns the length of the array
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the array is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the array
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Clone + 'static> Drop for CrystalArray<T> {
    fn drop(&mut self) {
        if !self.data.is_null() && self.capacity > 0 {
            unsafe {
                let layout = core::alloc::Layout::array::<T>(self.capacity).unwrap();
                core::alloc::dealloc(self.data as *mut u8, layout);
            }
        }
    }
}

// Prevent automatic copying of raw pointers
impl<T: Clone + 'static> !Copy for CrystalArray<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_array_basics() {
        let array = CrystalArray::<i32>::new();
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
        assert_eq!(array.capacity(), 0);

        let array = CrystalArray::<i32>::with_capacity(10);
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
        assert_eq!(array.capacity(), 10);
    }
}
