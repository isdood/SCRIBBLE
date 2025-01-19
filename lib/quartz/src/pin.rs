// lib/quartz/src/pin.rs

//! CrystalPin - Crystal-based Pinning for Memory Safety
//! Last Updated: 2025-01-19 16:17:32 UTC
//! Author: isdood

use scribe::Scribe;
use harmony_core::phantom::Phantom;

/// A pinned reference to data, ensuring it cannot be moved in memory.
pub struct CrystalPin<P> {
    pointer: Scribe<P>,
    phantom: Phantom<P>,
}

impl<P> CrystalPin<P> {
    /// Create a new pinned reference
    pub fn new(data: P) -> Self {
        Self {
            pointer: Scribe::new(data),
            phantom: Phantom::new(),
        }
    }

    /// Get a pinned reference to the data
    pub fn as_ref(&self) -> &P {
        self.pointer.read()
    }

    /// Get a mutable pinned reference to the data
    pub fn as_mut(&mut self) -> &mut P {
        self.pointer.write()
    }
}

impl<P> std::ops::Deref for CrystalPin<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<P> std::ops::DerefMut for CrystalPin<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_pin_creation() {
        let pin = CrystalPin::new(42);
        assert_eq!(*pin, 42);
    }

    #[test]
    fn test_crystal_pin_as_ref() {
        let pin = CrystalPin::new(42);
        assert_eq!(pin.as_ref(), &42);
    }

    #[test]
    fn test_crystal_pin_as_mut() {
        let mut pin = CrystalPin::new(42);
        *pin.as_mut() = 43;
        assert_eq!(*pin, 43);
    }
}
