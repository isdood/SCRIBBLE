#![feature(asm_const)]
#![feature(core_intrinsics)]
#![feature(const_trait_impl)]

/// Unstable Matter Core Library
/// Last Updated: 2025-01-18 19:01:53 UTC
/// Author: isdood
/// Current User: isdood

// Constants module
pub mod constants {
    pub const QUANTUM_COHERENCE_THRESHOLD: f64 = 0.7;
    pub const QUANTUM_STABILITY_THRESHOLD: f64 = 0.8;
    pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
    pub const MESH_VECTOR_ALIGNMENT: usize = 16;
    pub const QUANTUM_THRESHOLD: f64 = 0.5;
    pub const PLANCK_LENGTH: f64 = 1.616255e-35;
    pub const COHERENCE_DECAY_FACTOR: f64 = 0.99;
    pub const CURRENT_TIMESTAMP: &str = "2025-01-18 19:01:53";
    pub const CURRENT_USER: &str = "isdood";
}

// Core modules
pub mod vector;
pub mod aether;
pub mod quantum;
pub mod meshmath;
pub mod mesh;
pub mod mesh_clock;
pub mod grav;
pub mod glitch;
pub mod arch;
pub mod align;
pub mod scribe;
pub mod zeronaut;

// Re-exports for convenient access
pub use {
    vector::Vector3D,
    quantum::{Quantum, QuantumState},
    meshmath::{MeshMath, MeshValue},
    mesh::Mesh,
    mesh_clock::MeshClock,
    grav::GravityField,
    glitch::GlitchField,
    align::Alignment,
    scribe::{Scribe, ScribePrecision, QuantumString},
    zeronaut::Zeronaut,
    constants::*,
};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: Option<&str> = option_env!("GIT_HASH");

// Initialize subsystems
#[inline]
fn init_subsystems() -> Result<(), &'static str> {
    arch::init()?;
    aether::init()?;
    mesh::init()?;
    quantum::init()?;
    Ok(())
}

// Shutdown subsystems
#[inline]
fn shutdown_subsystems() -> Result<(), &'static str> {
    quantum::shutdown()?;
    mesh::shutdown()?;
    aether::shutdown()?;
    arch::shutdown()?;
    Ok(())
}

/// Initialize the unstable matter system
pub fn init() -> Result<(), &'static str> {
    if !arch::check_alignment(constants::MESH_VECTOR_ALIGNMENT) {
        return Err("CPU does not support required vector alignment");
    }
    init_subsystems()
}

/// Shutdown the unstable matter system
pub fn shutdown() -> Result<(), &'static str> {
    shutdown_subsystems()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_init() {
        assert!(init().is_ok());
        assert!(shutdown().is_ok());
    }

    #[test]
    fn test_quantum_coherence() {
        let state = QuantumState::new();
        assert!(state.get_coherence() >= constants::QUANTUM_COHERENCE_THRESHOLD);
    }

    #[test]
    fn test_mesh_creation() {
        let mesh = Mesh::default();
        assert!(mesh.volume() > 0);
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let sum = v1.mesh_add(&v2);
        assert_eq!(sum.prime(), 5.0);
        assert_eq!(sum.resonant(), 7.0);
        assert_eq!(sum.harmonic(), 9.0);
    }
}
