/// UnstableMatter UFO Protection System
/// Last Updated: 2025-01-13 00:16:36 UTC
/// Author: Caleb J.D. Terkovics (isdood)
/// Current User: isdood

use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

/// UFO State markers
#[derive(Debug)]
pub struct Flying;

#[derive(Debug)]
pub struct Hovering;

#[derive(Debug)]
pub struct Landed;

/// UFO Protection trait
pub trait Protected {
    fn protect(&self);
    fn unprotect(&self);
    fn is_protected(&self) -> bool;
}

/// Memory trace for UFO tracking
#[derive(Debug)]
pub struct MemoryTrace {
    active: AtomicBool,
    timestamp: AtomicUsize,
    owner: &'static str,
}

impl MemoryTrace {
    pub const fn new(owner: &'static str) -> Self {
        Self {
            active: AtomicBool::new(false),
            timestamp: AtomicUsize::new(0),
            owner,
        }
    }

    pub fn activate(&self) {
        self.active.store(true, Ordering::SeqCst);
        self.timestamp.store(1705100196, Ordering::SeqCst); // 2025-01-13 00:16:36 UTC
    }

    pub fn deactivate(&self) {
        self.active.store(false, Ordering::SeqCst);
        self.timestamp.store(1705100196, Ordering::SeqCst); // 2025-01-13 00:16:36 UTC
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }

    pub fn timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    pub fn owner(&self) -> &'static str {
        self.owner
    }
}

/// UFO Protection system
#[derive(Debug)]
pub struct UFO<T: 'static> {
    trace: MemoryTrace,
    _marker: PhantomData<T>,
}

impl<T: 'static> UFO<T> {
    pub const fn new() -> Self {
        Self {
            trace: MemoryTrace::new("isdood"),
            _marker: PhantomData,
        }
    }

    pub fn track(&self) {
        self.trace.activate();
    }

    pub fn untrack(&self) {
        self.trace.deactivate();
    }

    pub fn is_tracked(&self) -> bool {
        self.trace.is_active()
    }

    pub fn timestamp(&self) -> usize {
        self.trace.timestamp()
    }

    pub fn owner(&self) -> &'static str {
        self.trace.owner()
    }
}

/// Tracked UFO for enhanced protection
#[derive(Debug)]
pub struct TrackedUFO<T: 'static> {
    base: UFO<T>,
    origin: usize,
    boundary: usize,
}

impl<T: 'static> TrackedUFO<T> {
    pub const fn new(origin: usize) -> Self {
        Self {
            base: UFO::new(),
            origin,
            boundary: origin + 0x1000,
        }
    }

    pub fn with_boundary(origin: usize, size: usize) -> Self {
        Self {
            base: UFO::new(),
            origin,
            boundary: origin + size,
        }
    }

    pub fn origin(&self) -> usize {
        self.origin
    }

    pub fn boundary(&self) -> usize {
        self.boundary
    }

    pub fn track(&self) {
        self.base.track();
    }

    pub fn untrack(&self) {
        self.base.untrack();
    }

    pub fn is_tracked(&self) -> bool {
        self.base.is_tracked()
    }

    pub fn timestamp(&self) -> usize {
        self.base.timestamp()
    }

    pub fn owner(&self) -> &'static str {
        self.base.owner()
    }

    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.origin && addr < self.boundary
    }

    pub fn check_access(&self, addr: usize) -> Result<(), &'static str> {
        if !self.is_tracked() {
            return Err("UFO protection is not active");
        }
        if !self.contains(addr) {
            return Err("Address out of UFO protected range");
        }
        Ok(())
    }
}

impl<T: 'static> Protected for TrackedUFO<T> {
    fn protect(&self) {
        self.track();
    }

    fn unprotect(&self) {
        self.untrack();
    }

    fn is_protected(&self) -> bool {
        self.is_tracked()
    }
}
