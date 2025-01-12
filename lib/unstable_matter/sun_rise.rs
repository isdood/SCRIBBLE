/// Sun_rise: Static Initialization for Vector Spaces
/// Last Updated: 2025-01-12 21:41:28 UTC
/// Author: isdood
///
/// This module provides atomic initialization for
/// vector spaces with strong synchronization
/// guarantees. Sun_rise ensures only one thread
/// can initialize each static value.

// lib/unstable_matter/src/sun_rise.rs

use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

pub struct Sun_rise<T> {
    initialized: AtomicBool,
    value: UnsafeCell<Option<T>>,
}

unsafe impl<T: Send + Sync> Sync for Sun_rise<T> {}

impl<T> Sun_rise<T> {
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(None),
        }
    }

    pub fn init(&self, value: T) -> bool {
        if self.initialized.load(Ordering::Acquire) {
            return false;
        }

        unsafe {
            *self.value.get() = Some(value);
        }

        self.initialized.store(true, Ordering::Release);
        true
    }

    pub fn get(&self) -> Option<&T> {
        if !self.initialized.load(Ordering::Acquire) {
            return None;
        }

        unsafe {
            (*self.value.get()).as_ref()
        }
    }

    pub fn get_mut(&self) -> Option<&mut T> {
        if !self.initialized.load(Ordering::Acquire) {
            return None;
        }

        unsafe {
            (*self.value.get()).as_mut()
        }
    }
}

#[macro_export]
macro_rules! sun_rise {
    ($init:expr) => {{
        static SUN_RISE: $crate::sun_rise::Sun_rise<_> = $crate::sun_rise::Sun_rise::new();
        if SUN_RISE.get().is_none() {
            SUN_RISE.init($init);
        }
        SUN_RISE.get().unwrap()
    }};
}
