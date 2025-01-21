//! Tides: Wave-based crystal computing framework
//!
//! This library implements harmonic wave computation patterns for crystal structures,
//! utilizing resonance processing via Julia and wave distribution via Chapel.

pub mod core;
pub mod lattice;
pub mod bridge;
pub mod resonance;
pub mod waves;
pub mod harmony;

// Re-exports
pub use crate::core::{Tide, WavePattern};
pub use crate::resonance::CrystalState;
pub use crate::waves::WaveMesh;
