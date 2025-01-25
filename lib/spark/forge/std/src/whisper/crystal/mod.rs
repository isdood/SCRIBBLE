//! Crystal module for Whisper type

use std::f64::consts::PI;

/// Quantum phase state
#[derive(Debug, Clone, Copy)]
pub struct QuantumPhase {
    phase: f64,
}

impl PartialEq for QuantumPhase {
    fn eq(&self, other: &Self) -> bool {
        (self.phase - other.phase).abs() < f64::EPSILON
    }
}

impl Eq for QuantumPhase {}

impl QuantumPhase {
    /// Creates a new quantum phase
    pub fn new(phase: f64) -> Self {
        Self {
            phase: phase % (2.0 * PI),
        }
    }

    /// Gets the phase value
    pub fn value(&self) -> f64 {
        self.phase
    }

    /// Combines two phases
    pub fn combine(&self, other: &Self) -> Self {
        Self::new(self.phase + other.phase)
    }

    /// Shifts the phase
    pub fn shift(&self, delta: f64) -> Self {
        Self::new(self.phase + delta)
    }

    /// Entangles two phases
    pub fn entangle(&self, other: &Self) -> Self {
        Self::new((self.phase + other.phase) / 2.0)
    }
}

impl Default for QuantumPhase {
    fn default() -> Self {
        Self::new(0.0)
    }
}

/// Crystal lattice state
#[derive(Debug, Clone, Copy)]
pub struct CrystalState {
    energy: f64,
    alignment: f64,
}

impl PartialEq for CrystalState {
    fn eq(&self, other: &Self) -> bool {
        (self.energy - other.energy).abs() < f64::EPSILON &&
        (self.alignment - other.alignment).abs() < f64::EPSILON
    }
}

impl Eq for CrystalState {}

impl CrystalState {
    /// Creates a new crystal state
    pub fn new(energy: f64, alignment: f64) -> Self {
        Self {
            energy: energy.max(0.0),
            alignment: alignment.clamp(-1.0, 1.0),
        }
    }

    /// Gets the energy level
    pub fn energy(&self) -> f64 {
        self.energy
    }

    /// Gets the alignment value
    pub fn alignment(&self) -> f64 {
        self.alignment
    }

    /// Resonates with another state
    pub fn resonate(&self, other: &Self) -> Self {
        Self::new(
            (self.energy * other.energy).sqrt(),
            (self.alignment + other.alignment) / 2.0,
        )
    }

    /// Aligns with another state
    pub fn align(&self, other: &Self) -> Self {
        Self::new(
            self.energy,
            (self.alignment + other.alignment).signum(),
        )
    }

    /// Entangles with another state
    pub fn entangle(&self, other: &Self) -> Self {
        Self::new(
            (self.energy + other.energy) / 2.0,
            (self.alignment * other.alignment).abs(),
        )
    }
}

impl Default for CrystalState {
    fn default() -> Self {
        Self::new(1.0, 0.0)
    }
}
