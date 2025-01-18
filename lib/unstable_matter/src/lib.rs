//! Unstable Matter - Quantum-Crystal Memory Management
//! Last Updated: 2025-01-18 19:38:54 UTC
//! Author: isdood
//! Current User: isdood

#![no_std]
#![feature(core_intrinsics)]

// Conditional external dependencies
#[cfg(feature = "shard")]
use shard::{
    core::{ShardRegisterFile, ShardMemory, ShardInstruction, ShardOpcode},
    memory::ShardMemoryPattern,
    vector4d::{Vector4D, HyperRotation, QuatTransform},
    meshmath::MeshValue,
    QUANTUM_COHERENCE_THRESHOLD,
    FAIRY_DUST_COEFFICIENT,
};

// External dependencies
use scribble::cereal;

#[cfg(not(feature = "shard"))]
mod compat {
    pub const QUANTUM_COHERENCE_THRESHOLD: f64 = 0.87;
    pub const FAIRY_DUST_COEFFICIENT: f64 = 0.618033988749895;
}

#[cfg(not(feature = "shard"))]
use self::compat::{QUANTUM_COHERENCE_THRESHOLD, FAIRY_DUST_COEFFICIENT};

// Core modules
pub mod meshmath;
pub mod sun_rise;
pub mod zeronaut;
pub mod vector;
pub mod align;
pub mod aether;
pub mod harmony;
pub mod mesh;
pub mod glitch;
pub mod grav;
pub mod scribe;

// Internal imports
use crate::{
    harmony::HarmonyResult,
    aether::Aether,
    mesh::MeshPattern,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

/// Architecture constants
pub const VERSION: &str = "0.2.0";
pub const ARCH_NAME: &str = "UnstableMatter";
pub const CREATOR: &[u8] = b"isdood";
pub const BUILD_TIMESTAMP: &str = "2025-01-18 19:38:54";

/// System initialization flags
const HARMONY_INIT: u32 = 0x00000001;
const MESH_INIT: u32    = 0x00000002;
const AETHER_INIT: u32  = 0x00000004;

static mut INIT_STATE: u32 = 0;

/// Initialize unstable matter subsystems
pub fn init() -> HarmonyResult<()> {
    // Safety: Single-threaded initialization
    unsafe {
        if INIT_STATE != 0 {
            return Err(harmony::HarmonyError::StabilityFailure);
        }
    }

    // Initialize subsystems in order of dependency
    harmony::init()?;
    unsafe { INIT_STATE |= HARMONY_INIT; }

    mesh::init()?;
    unsafe { INIT_STATE |= MESH_INIT; }

    aether::init()?;
    unsafe { INIT_STATE |= AETHER_INIT; }

    // Verify system stability
    if !check_stability() {
        return Err(harmony::HarmonyError::StabilityFailure);
    }

    Ok(())
}

/// Shutdown unstable matter subsystems
pub fn shutdown() -> HarmonyResult<()> {
    // Safety: Single-threaded shutdown
    unsafe {
        if INIT_STATE == 0 {
            return Ok(());
        }

        // Shutdown in reverse order
        if INIT_STATE & AETHER_INIT != 0 {
            aether::shutdown()?;
            INIT_STATE &= !AETHER_INIT;
        }

        if INIT_STATE & MESH_INIT != 0 {
            mesh::shutdown()?;
            INIT_STATE &= !MESH_INIT;
        }

        if INIT_STATE & HARMONY_INIT != 0 {
            harmony::shutdown()?;
            INIT_STATE &= !HARMONY_INIT;
        }
    }

    Ok(())
}

/// Check system quantum stability
pub fn check_stability() -> bool {
    // Check initialization state
    unsafe {
        if INIT_STATE != (HARMONY_INIT | MESH_INIT | AETHER_INIT) {
            return false;
        }
    }

    // Verify subsystem coherence
    let resonance = harmony::get_resonance();
    if resonance < QUANTUM_COHERENCE_THRESHOLD {
        return false;
    }

    // Check crystal structure stability
    mesh::check_crystal_stability() && aether::check_field_stability()
}

/// Get system version information
pub fn version_info() -> &'static str {
    concat!(
        "UnstableMatter v",
        env!("CARGO_PKG_VERSION"),
            "\nBuilt: ",
            env!("BUILD_TIMESTAMP"),
            "\nCreator: isdood"
    )
}

// Public re-exports
pub use {
    aether::Aether,
    harmony::{HarmonicPattern, HarmonicState, HarmonyError},
    mesh::MeshPattern,
    align::Alignment,
    zeronaut::Zeronaut,
    vector::Vector3D,
    sun_rise::{Sun_rise, sun_rise, sun_rise_quantum},
    scribe::{Scribe, ScribePrecision, QuantumString},
    cereal::CerealResult,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        assert!(init().is_ok());
        assert!(check_stability());
        assert!(shutdown().is_ok());
    }

    #[test]
    fn test_harmonic_resonance() {
        init().unwrap();
        assert!(harmony::get_resonance() > QUANTUM_COHERENCE_THRESHOLD);
        shutdown().unwrap();
    }

    #[test]
    fn test_crystal_stability() {
        init().unwrap();
        assert!(mesh::check_crystal_stability());
        shutdown().unwrap();
    }

    #[test]
    fn test_aether_field() {
        init().unwrap();
        assert!(aether::check_field_stability());
        shutdown().unwrap();
    }

    #[test]
    fn test_version_info() {
        let info = version_info();
        assert!(info.contains("UnstableMatter"));
        assert!(info.contains("isdood"));
    }

    #[test]
    fn test_full_shutdown() {
        init().unwrap();
        shutdown().unwrap();
        unsafe {
            assert_eq!(INIT_STATE, 0);
        }
    }
}
