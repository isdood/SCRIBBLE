//! Crystalline Harmony Core
//! ====================
//!
//! Core quantum computing framework implemented through crystalline
//! data structures with harmonic resonance tracking.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-18 20:57:45 UTC
//! Version: 0.1.0
//! License: MIT

#![no_std]
#![cfg_attr(test, no_main)]

extern crate core;

// Core modules
pub mod constants;
pub mod vector;
pub mod harmony;
pub mod aether;
pub mod cube;
pub mod zeronaut;
pub mod phantom;
pub mod scribe;
pub mod align;

// Re-exports for convenience
pub use self::{
    cube::CrystalCube,
    zeronaut::Zeronaut,
    harmony::{Quantum, MeshValue, MeshOps},
    vector::{Vector3D, Vector4D},
};

/// A quantum-safe array implementation
#[derive(Clone)]
pub struct CrystalArray<T: Clone + 'static> {
    /// Internal data buffer
    data: [T; 1024], // Fixed size for no_std
    /// Current length
    len: usize,
}

impl<T: Clone + 'static> CrystalArray<T> {
    /// Creates a new empty CrystalArray
    pub const fn new() -> Self where T: Copy {
        Self {
            data: [unsafe { core::mem::zeroed() }; 1024],
            len: 0,
        }
    }

    /// Creates a new CrystalArray with given capacity (ignored in no_std)
    pub const fn with_capacity(_capacity: usize) -> Self where T: Copy {
        Self::new()
    }

    /// Gets the current length
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Checks if empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Gets the current capacity
    pub const fn capacity(&self) -> usize {
        1024
    }

    /// Gets a reference to the raw pointer
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Gets a mutable reference to the raw pointer
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Pushes an element if there's space
    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.len >= self.capacity() {
            return Err("Array is full");
        }
        self.data[self.len] = value;
        self.len += 1;
        Ok(())
    }

    /// Pops an element
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            Some(self.data[self.len].clone())
        }
    }

    /// Clears the array
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_array_basics() {
        let mut array = CrystalArray::<u8>::new();
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
        assert_eq!(array.capacity(), 1024);

        array.push(42).unwrap();
        assert_eq!(array.len(), 1);
        assert!(!array.is_empty());

        let value = array.pop();
        assert_eq!(value, Some(42));
        assert!(array.is_empty());
    }

    #[test]
    fn test_crystal_array_capacity() {
        let mut array = CrystalArray::<u8>::new();
        for i in 0..1024 {
            array.push(i as u8).unwrap();
        }
        assert!(array.push(0).is_err());
    }

    #[test]
    fn test_crystal_array_clear() {
        let mut array = CrystalArray::<u8>::new();
        array.push(1).unwrap();
        array.push(2).unwrap();
        array.clear();
        assert!(array.is_empty());
    }
}
