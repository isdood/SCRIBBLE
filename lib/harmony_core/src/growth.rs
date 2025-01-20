//! Crystal Growth Pattern Management
//! ============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-20 20:27:04 UTC
//! Version: 0.1.1
//! License: MIT

use magicmath::{
    Vector3D,
    resonance::{Resonance, Phase},
};

use errors::{MathError, QuantumError};
use core::{
    fmt::{self, Display, Write},
    result::Result,
};

use magicmath::constants::{
    HARMONY_STABILITY_THRESHOLD,
    HARMONY_RESONANCE_THRESHOLD,
    MAX_QUANTUM_SIZE, // Updated to use a valid constant
};

/// Crystal growth pattern types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrowthPattern {
    /// Local growth within boundary
    Local,
    /// Global growth across lattice
    Global,
    /// Fractal pattern growth
    Fractal,
    /// Unknown growth pattern
    Unknown,
}

impl Display for GrowthPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrowthPattern::Local => write!(f, "Local"),
            GrowthPattern::Global => write!(f, "Global"),
            GrowthPattern::Fractal => write!(f, "Fractal"),
            GrowthPattern::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Crystal growth state
#[derive(Debug)]
pub struct GrowthState {
    /// Growth pattern
    pattern: GrowthPattern,
    /// Coherence level (0.0 - 1.0)
    coherence_level: f64,
    /// Stability factor
    stability_factor: f64,
    /// Iteration count
    iteration_count: usize,
}

impl GrowthState {
    /// Create a new growth state
    pub fn new(pattern: GrowthPattern) -> Self {
        Self {
            pattern,
            coherence_level: 1.0,
            stability_factor: 1.0,
            iteration_count: 0,
        }
    }

    /// Get current growth pattern
    pub fn pattern(&self) -> GrowthPattern {
        self.pattern
    }

    /// Set growth pattern
    pub fn set_pattern(&mut self, pattern: GrowthPattern) {
        self.pattern = pattern;
    }

    /// Get coherence level
    pub fn coherence_level(&self) -> f64 {
        self.coherence_level
    }

    /// Set coherence level
    pub fn set_coherence_level(&mut self, level: f64) -> Result<(), MathError> {
        if level < 0.0 || level > 1.0 {
            return Err(MathError::InvalidRange); // Fix: Correcting error variant
        }
        self.coherence_level = level;
        Ok(())
    }

    /// Get stability factor
    pub fn stability_factor(&self) -> f64 {
        self.stability_factor
    }

    /// Set stability factor
    pub fn set_stability_factor(&mut self, factor: f64) -> Result<(), MathError> {
        if factor < 0.0 {
            return Err(MathError::InvalidRange); // Fix: Correcting error variant
        }
        self.stability_factor = factor;
        Ok(())
    }

    /// Get iteration count
    pub fn iteration_count(&self) -> usize {
        self.iteration_count
    }

    /// Increment iteration count
    pub fn increment_iterations(&mut self) -> Result<(), QuantumError> {
        if self.iteration_count >= MAX_QUANTUM_SIZE { // Updated to use a valid constant
            return Err(QuantumError::IterationLimit); // Fix: Correcting error variant
        }
        self.iteration_count += 1;
        Ok(())
    }

    /// Check if growth is stable
    pub fn is_stable(&self) -> bool {
        self.coherence_level >= HARMONY_STABILITY_THRESHOLD &&
        self.stability_factor >= HARMONY_RESONANCE_THRESHOLD
    }
}

impl Display for GrowthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Growth State:")?;
        writeln!(f, "Pattern: {}", self.pattern)?;
        writeln!(f, "Coherence: {}", self.coherence_level)?;
        writeln!(f, "Stability: {}", self.stability_factor)?;
        write!(f, "Iterations: {}", self.iteration_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_pattern() {
        let pattern = GrowthPattern::Local;
        assert_eq!(pattern, GrowthPattern::Local);
        assert_ne!(pattern, GrowthPattern::Global);
    }

    #[test]
    fn test_growth_state() {
        let state = GrowthState::new(GrowthPattern::Local);
        assert_eq!(state.pattern(), GrowthPattern::Local);
        assert_eq!(state.coherence_level(), 1.0);
        assert_eq!(state.stability_factor(), 1.0);
        assert_eq!(state.iteration_count(), 0);
    }

    #[test]
    fn test_coherence_limits() {
        let mut state = GrowthState::new(GrowthPattern::Local);
        assert!(state.set_coherence_level(0.5).is_ok());
        assert!(state.set_coherence_level(-0.1).is_err());
        assert!(state.set_coherence_level(1.1).is_err());
    }

    #[test]
    fn test_iteration_limit() {
        let mut state = GrowthState::new(GrowthPattern::Fractal);
        for _ in 0..MAX_QUANTUM_SIZE { // Updated to use a valid constant
            assert!(state.increment_iterations().is_ok());
        }
        assert!(state.increment_iterations().is_err());
    }
}
