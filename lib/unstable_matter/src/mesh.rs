/// Mesh Module - Core Spatial Grid Implementation
/// Last Updated: 2025-01-18 18:53:10 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    vector::Vector3D,
    quantum::Quantum,
    aether::{AetherOrdering, AetherCell},
    meshmath::{MeshMath, MeshValue},
    GRAVITATIONAL_CONSTANT,
    QUANTUM_COHERENCE_THRESHOLD
};

#[derive(Debug)]
pub struct Mesh {
    width: usize,
    height: usize,
    depth: usize,
    dimensions: Vector3D<usize>,
    position: AetherCell<Vector3D<f64>>,
    stability: AetherCell<bool>,
    coherence: AetherCell<f64>,
}

impl Mesh {
    /// Create a new mesh from dimensions vector
    #[inline]
    pub fn new(vec: Vector3D<usize>) -> Self {
        Self {
            width: vec.x(),
            height: vec.y(),
            depth: vec.z(),
            dimensions: vec,
            position: AetherCell::new(Vector3D::void()),
            stability: AetherCell::new(true),
            coherence: AetherCell::new(1.0),
        }
    }

    /// Get total mesh volume
    #[inline]
    pub fn volume(&self) -> usize {
        self.width * self.height * self.depth
    }

    /// Update mesh position with force
    #[inline]
    pub fn update_position(&mut self, position: &Vector3D<f64>, force: &Vector3D<f64>) -> Result<(), &'static str> {
        let new_position = position.mesh_add(force);
        self.set_position(&new_position)
    }

    /// Set mesh position
    #[inline]
    pub fn set_position(&mut self, position: &Vector3D<f64>) -> Result<(), &'static str> {
        self.position.store(position.clone(), &AetherOrdering::Quantum)
    }

    /// Get current mesh position
    #[inline]
    pub fn get_position(&self) -> Result<Vector3D<f64>, &'static str> {
        self.position.load(&AetherOrdering::Quantum)
    }

    /// Calculate gravitational force between mesh and black hole
    #[inline]
    pub fn calculate_gravity(&self, blackhole_pos: &Vector3D<f64>, mass: f64) -> Result<Vector3D<f64>, &'static str> {
        let position = self.get_position()?;
        let distance = position.mesh_sub(blackhole_pos).mesh_magnitude();

        if distance < 0.01 {
            return Ok(Vector3D::void());
        }

        let force = -GRAVITATIONAL_CONSTANT * mass / (distance * distance);
        Ok(self.get_position()?.mesh_mul(force))
    }

    /// Check if mesh is stable
    #[inline]
    pub fn is_stable(&self) -> Result<bool, &'static str> {
        self.stability.load(&AetherOrdering::Quantum)
    }

    /// Set mesh stability
    #[inline]
    pub fn set_stable(&mut self, stable: bool) -> Result<(), &'static str> {
        self.stability.store(stable, &AetherOrdering::Quantum)
    }

    /// Get mesh coherence
    #[inline]
    pub fn get_coherence(&self) -> Result<f64, &'static str> {
        self.coherence.load(&AetherOrdering::Quantum)
    }

    /// Set mesh coherence
    #[inline]
    pub fn set_coherence(&mut self, coherence: f64) -> Result<(), &'static str> {
        self.coherence.store(coherence, &AetherOrdering::Quantum)
    }

    /// Check if mesh has quantum coherence
    #[inline]
    pub fn has_quantum_coherence(&self) -> Result<bool, &'static str> {
        Ok(self.get_coherence()? > QUANTUM_COHERENCE_THRESHOLD)
    }

    /// Reset mesh to initial state
    #[inline]
    pub fn reset(&mut self) -> Result<(), &'static str> {
        self.set_stable(true)?;
        self.set_coherence(1.0)?;
        self.set_position(&Vector3D::void())
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new(Vector3D::new(64, 64, 64))
    }
}

impl Quantum for Mesh {
    fn get_coherence(&self) -> f64 {
        self.coherence.load(&AetherOrdering::Quantum).unwrap_or(0.0)
    }

    fn is_quantum_stable(&self) -> bool {
        self.stability.load(&AetherOrdering::Quantum).unwrap_or(false)
    }

    fn decay_coherence(&self) {
        if let Ok(current) = self.coherence.load(&AetherOrdering::Quantum) {
            let _ = self.coherence.store(current * 0.99, &AetherOrdering::Quantum);
        }
    }

    fn reset_coherence(&self) {
        let _ = self.coherence.store(1.0, &AetherOrdering::Quantum);
    }
}
