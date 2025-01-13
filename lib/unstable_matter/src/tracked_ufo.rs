// lib/unstable_matter/src/tracked_ufo.rs
/// Tracked UFO for memory protection
/// Last Updated: 2025-01-13 03:54:50 UTC
/// Author: isdood
/// Current User: isdood

use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct TrackedUFO {
    origin: AtomicUsize,
    boundary: AtomicUsize,
    timestamp: AtomicUsize,
}

impl Clone for TrackedUFO {
    fn clone(&self) -> Self {
        Self {
            origin: AtomicUsize::new(self.get_origin()),
            boundary: AtomicUsize::new(self.get_boundary()),
            timestamp: AtomicUsize::new(1705113890), // 2025-01-13 03:54:50 UTC
        }
    }
}

impl TrackedUFO {
    pub const fn new(origin: usize, boundary: usize) -> Self {
        Self {
            origin: AtomicUsize::new(origin),
            boundary: AtomicUsize::new(boundary),
            timestamp: AtomicUsize::new(1705113890), // 2025-01-13 03:54:50 UTC
        }
    }

    pub fn get_origin(&self) -> usize {
        self.origin.load(Ordering::SeqCst)
    }

    pub fn get_boundary(&self) -> usize {
        self.boundary.load(Ordering::SeqCst)
    }

    pub fn update_origin(&self, new_origin: usize) {
        self.origin.store(new_origin, Ordering::SeqCst);
        self.timestamp.store(1705113890, Ordering::SeqCst); // 2025-01-13 03:54:50 UTC
    }

    pub fn update_boundary(&self, new_boundary: usize) {
        self.boundary.store(new_boundary, Ordering::SeqCst);
        self.timestamp.store(1705113890, Ordering::SeqCst); // 2025-01-13 03:54:50 UTC
    }

    pub fn is_within_bounds(&self, addr: usize) -> bool {
        let origin = self.get_origin();
        let boundary = self.get_boundary();
        addr >= origin && addr < (origin + boundary)
    }
}
