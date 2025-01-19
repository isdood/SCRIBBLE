//! Crystal Growth Pattern Management
//! ===============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19 13:13:04 UTC
//! Last Updated: 2025-01-19 13:13:04 UTC
//! Version: 0.1.0
//! License: MIT

use magicmath::{
    FractalParams,
    FractalState,
    JuliaParams,
    JuliaState,
    JuliaVariant,
    MandelbrotParams,
    MandelbrotState,
    MandelbrotVariant,
    generate_fractal,
    iterate_julia,
    iterate_mandelbrot,
    MathResult,
    MathError,
};
use scribe::Scribe;
use scribe::native_string::String;

use crate::{
    crystal::{CrystalLattice, CrystalNode},
    constants,
    errors::{QuantumError, CoherenceError},
    resonance::{ResonanceMath, ResonanceState},
};

/// Defines different types of crystal growth patterns
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrowthPattern {
    /// Local growth using Julia set patterns
    Local,
    /// Global restructuring using Mandelbrot patterns
    Global,
    /// Hybrid growth combining both patterns
    Hybrid,
    /// Quantum-stabilized growth pattern
    Quantum,
}

impl Scribe for GrowthPattern {
    fn scribe(&self) -> String {
        let mut result = String::new();
        match self {
            GrowthPattern::Local => result.push_str("Local"),
            GrowthPattern::Global => result.push_str("Global"),
            GrowthPattern::Hybrid => result.push_str("Hybrid"),
            GrowthPattern::Quantum => result.push_str("Quantum"),
        }
        result
    }
}

/// Represents the current state of crystal growth
#[derive(Debug)]
pub struct GrowthState {
    pattern: GrowthPattern,
    fractal_state: FractalState,
    coherence_level: f64,
    stability_factor: f64,
    iteration_count: usize,
}

impl GrowthState {
    /// Creates a new growth state with the specified pattern
    pub fn new(pattern: GrowthPattern) -> Self {
        let initial_state = match pattern {
            GrowthPattern::Local => {
                FractalState::Julia(JuliaState::new(
                    constants::JULIA_GROWTH_REAL,
                    constants::JULIA_GROWTH_IMAG
                ))
            },
            GrowthPattern::Global => {
                FractalState::Mandelbrot(MandelbrotState::new(0.0, 0.0))
            },
            GrowthPattern::Hybrid | GrowthPattern::Quantum => {
                FractalState::Julia(JuliaState::new(
                    constants::QUANTUM_GOLDEN_RATIO - 1.0,
                    constants::QUANTUM_STABILITY_THRESHOLD
                ))
            },
        };

        Self {
            pattern,
            fractal_state: initial_state,
            coherence_level: constants::MAX_PHASE_COHERENCE,
            stability_factor: constants::QUANTUM_STABILITY_THRESHOLD,
            iteration_count: 0,
        }
    }

    /// Gets the current growth pattern
    pub fn pattern(&self) -> GrowthPattern {
        self.pattern
    }

    /// Gets the current coherence level
    pub fn coherence_level(&self) -> f64 {
        self.coherence_level
    }

    /// Gets the current stability factor
    pub fn stability_factor(&self) -> f64 {
        self.stability_factor
    }

    /// Gets the current iteration count
    pub fn iteration_count(&self) -> usize {
        self.iteration_count
    }

    /// Updates the growth state based on resonance measurements
    pub fn update_resonance_state(&mut self, resonance_math: &ResonanceMath) -> MathResult<()> {
        self.coherence_level = resonance_math.get_state().harmony;
        self.stability_factor = resonance_math.get_state().resonance;
        self.iteration_count += 1;
        Ok(())
    }
}

/// Manages crystal growth operations using fractal patterns
pub struct CrystalGrowth {
    state: GrowthState,
    params: FractalParams,
    resonance_math: ResonanceMath,
}

impl CrystalGrowth {
    /// Creates a new crystal growth manager
    pub fn new(pattern: GrowthPattern) -> Self {
        Self {
            state: GrowthState::new(pattern),
            params: FractalParams::default()
            .with_max_iterations(constants::MAX_FRACTAL_DEPTH)
            .with_threshold(constants::CRYSTAL_RESONANCE_THRESHOLD),
            resonance_math: ResonanceMath::new(),
        }
    }

    /// Calculates the next growth iteration for a crystal lattice
    pub fn next_iteration(&mut self, lattice: &mut CrystalLattice) -> Result<(), QuantumError> {
        // Update resonance state
        self.state.update_resonance_state(&self.resonance_math)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Resonance state update failed")))?;

        // Check stability conditions
        if self.state.coherence_level() < constants::MIN_PHASE_COHERENCE {
            return Err(QuantumError::CoherenceError(
                CoherenceError::new("Coherence level too low for growth")
            ));
        }

        // Generate growth pattern
        match self.state.pattern() {
            GrowthPattern::Local => self.apply_local_growth(lattice),
            GrowthPattern::Global => self.apply_global_growth(lattice),
            GrowthPattern::Hybrid => self.apply_hybrid_growth(lattice),
            GrowthPattern::Quantum => self.apply_quantum_growth(lattice),
        }
    }

    /// Applies local growth pattern using Julia sets
    fn apply_local_growth(&mut self, lattice: &mut CrystalLattice) -> Result<(), QuantumError> {
        let julia_params = JuliaParams::default()
        .with_real(constants::JULIA_GROWTH_REAL)
        .with_imag(constants::JULIA_GROWTH_IMAG);

        let result = iterate_julia(
            JuliaState::new(
                self.state.coherence_level(),
                            self.state.stability_factor()
            ),
            &julia_params,
            JuliaVariant::Standard
        ).map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Julia iteration failed")))?;

        self.apply_growth_pattern(lattice, result)
    }

    /// Applies global growth pattern using Mandelbrot sets
    fn apply_global_growth(&mut self, lattice: &mut CrystalLattice) -> Result<(), QuantumError> {
        let mandelbrot_params = MandelbrotParams::default()
        .with_max_iterations(constants::MAX_FRACTAL_DEPTH);

        let result = iterate_mandelbrot(
            MandelbrotState::new(
                self.state.coherence_level(),
                                 self.state.stability_factor()
            ),
            &mandelbrot_params,
            MandelbrotVariant::Standard
        ).map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Mandelbrot iteration failed")))?;

        self.apply_growth_pattern(lattice, result)
    }

    /// Applies hybrid growth pattern combining Julia and Mandelbrot sets
    fn apply_hybrid_growth(&mut self, lattice: &mut CrystalLattice) -> Result<(), QuantumError> {
        // Combine both patterns with weighted influence
        let local_weight = self.state.coherence_level();
        let global_weight = self.state.stability_factor();

        // Apply both patterns and blend results
        self.apply_local_growth(lattice)?;
        self.apply_global_growth(lattice)?;

        Ok(())
    }

    /// Applies quantum-stabilized growth pattern
    fn apply_quantum_growth(&mut self, lattice: &mut CrystalLattice) -> Result<(), QuantumError> {
        // Use quantum math operations for growth
        let quantum_params = FractalParams::default()
        .with_max_iterations(constants::MAX_FRACTAL_DEPTH)
        .with_threshold(self.state.coherence_level());

        let result = generate_fractal(
            self.state.fractal_state.clone(),
                                      &quantum_params
        ).map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Quantum fractal generation failed")))?;

        self.apply_growth_pattern(lattice, result)
    }

    /// Applies a growth pattern to the crystal lattice
    fn apply_growth_pattern(
        &self,
        lattice: &mut CrystalLattice,
        pattern: FractalState
    ) -> Result<(), QuantumError> {
        // Convert fractal pattern to crystal growth instructions
        // and apply them to the lattice
        lattice.apply_growth_pattern(pattern)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Failed to apply growth pattern")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_state_creation() {
        let state = GrowthState::new(GrowthPattern::Local);
        assert_eq!(state.pattern(), GrowthPattern::Local);
        assert_eq!(state.coherence_level(), constants::MAX_PHASE_COHERENCE);
        assert_eq!(state.stability_factor(), constants::QUANTUM_STABILITY_THRESHOLD);
        assert_eq!(state.iteration_count(), 0);
    }

    #[test]
    fn test_crystal_growth_creation() {
        let growth = CrystalGrowth::new(GrowthPattern::Local);
        assert_eq!(growth.state.pattern(), GrowthPattern::Local);
    }

    #[test]
    fn test_growth_pattern_display() {
        assert_eq!(GrowthPattern::Local.scribe().to_str(), "Local");
        assert_eq!(GrowthPattern::Global.scribe().to_str(), "Global");
        assert_eq!(GrowthPattern::Hybrid.scribe().to_str(), "Hybrid");
        assert_eq!(GrowthPattern::Quantum.scribe().to_str(), "Quantum");
    }

    #[test]
    fn test_resonance_state_update() {
        let mut state = GrowthState::new(GrowthPattern::Quantum);
        let resonance_math = ResonanceMath::new();
        assert!(state.update_resonance_state(&resonance_math).is_ok());
        assert_eq!(state.iteration_count(), 1);
    }
}
