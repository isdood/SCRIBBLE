// lib/unstable_matter/src/prelude.rs
pub use core::{
    cell::UnsafeCell,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
    alloc::{GlobalAlloc, Layout},
    ops::{Add, Sub, Mul},
    fmt,
};

pub use crate::{
    constants::{
        MESH_TIMESTAMP,
        QUANTUM_THRESHOLD,
        LIGHT_SPEED,
        PLANCK_LENGTH,
        VECTOR_ALIGN,
        CACHE_LINE,
        VECTOR_QUANTUM_STATE,
    },
    align::{Alignment, AlignedSpace, vector_align},
    helium::Helium,
    ufo::UFO,
    vector::{Vector3D, Vector4D},
    phantom::PhantomSpace,
    cube::Box,
};

// Constants for the current timestamp
pub const CURRENT_TIMESTAMP: usize = 1705262799; // 2025-01-14 20:46:39 UTC

pub trait Protected {
    fn protect(&self);
}
