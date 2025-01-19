// lib/quartz/src/arc.rs

//! CrystalArc - Crystal-based Atomic Reference Counter
//! Last Updated: 2025-01-19 16:15:10 UTC
//! Author: isdood

use harmony_core::aether::{AetherCell, AetherOrdering};
use magicmath::ops::{Add, Sub};
use scribe::Scribe;

/// Inner structure holding the data and reference count
struct Inner<T> {
    data: AetherCell<T>,
    ref_count: AetherCell<usize>,
}

/// Custom Arc implementation for crystal-based computing
pub struct CrystalArc<T> {
    inner: Scribe<Inner<T>>,
}

impl<T> CrystalArc<T> {
    /// Create a new CrystalArc instance
    pub fn new(data: T) -> Self {
        let inner = Inner {
            data: AetherCell::new(data),
            ref_count: AetherCell::new(1),
        };

        Self {
            inner: Scribe::new(inner),
        }
    }

    /// Clone the CrystalArc to get another shared reference
    pub fn clone(&self) -> Self {
        {
            let inner = self.inner.read();
            let current_count = inner.ref_count.load(&AetherOrdering::Relaxed);
            inner.ref_count.store(current_count.add(1), &AetherOrdering::Relaxed);
        }

        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> std::ops::Deref for CrystalArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = self.inner.read();
        inner.data.get()
    }
}

impl<T> Drop for CrystalArc<T> {
    fn drop(&mut self) {
        let inner = self.inner.read();
        let current_count = inner.ref_count.load(&AetherOrdering::Relaxed);
        if current_count.sub(1) == 0 {
            drop(inner);
        } else {
            inner.ref_count.store(current_count.sub(1), &AetherOrdering::Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_arc_creation() {
        let arc = CrystalArc::new(42);
        assert_eq!(*arc, 42);
    }

    #[test]
    fn test_crystal_arc_clone() {
        let arc1 = CrystalArc::new(42);
        let arc2 = arc1.clone();
        assert_eq!(*arc1, 42);
        assert_eq!(*arc2, 42);
    }

    #[test]
    fn test_crystal_arc_ref_count() {
        let arc1 = CrystalArc::new(42);
        let arc2 = arc1.clone();
        let arc3 = arc2.clone();
        assert_eq!(*arc1, 42);
        assert_eq!(*arc2, 42);
        assert_eq!(*arc3, 42);
    }
}
