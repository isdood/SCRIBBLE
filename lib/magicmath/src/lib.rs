//! Mathematical utilities and constants
//! Author: isdood
//! Created: 2025-01-23 02:13:29 UTC

pub mod constants {
    pub const PHI: f64 = 1.618033988749895;
    pub const RESONANCE_FACTOR: f64 = 1.414213562373095;
    pub const HARMONY_STABILITY_THRESHOLD: f64 = 0.999999;
}

pub trait MeshValue: Clone {
    fn zero() -> Self;
    fn one() -> Self;
}

#[derive(Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
