//! Crystalline interference pattern analysis and manipulation with Julia/Chapel integration
//! Created: 2025-01-21 16:18:49 UTC
//! Author: @isdood

// Previous imports...
use crate::{
    julia::{
        interference::{JuliaInterferenceProcessor, InterferenceResult},
        coherence::{JuliaCoherenceAnalyzer, CoherenceResult},
    },
    chapel::{
        parallel::{ChapelDomainMap, ChapelParallelInterference},
        coherence::{ChapelCoherenceProcessor, ProcessorResult},
    },
};

// Previous error enum...

#[derive(Debug, Clone)]
pub struct InterferenceConfig {
    // Previous fields...
    pub julia_threads: usize,
    pub chapel_locales: usize,
    pub compute_backend: ComputeBackend,
}

#[derive(Debug, Clone, Copy)]
pub enum ComputeBackend {
    Julia,
    Chapel,
    Hybrid,
}

impl Default for InterferenceConfig {
    fn default() -> Self {
        Self {
            // Previous defaults...
            julia_threads: 4,
            chapel_locales: 2,
            compute_backend: ComputeBackend::Hybrid,
        }
    }
}

pub struct InterferenceAnalyzer {
    // Previous fields...
    julia_processor: JuliaInterferenceProcessor,
    julia_coherence: JuliaCoherenceAnalyzer,
    chapel_interference: ChapelParallelInterference,
    chapel_coherence: ChapelCoherenceProcessor,
}

impl InterferenceAnalyzer {
    pub fn new(config: InterferenceConfig) -> Result<Self, InterferenceError> {
        // Initialize Julia components
        let julia_processor = JuliaInterferenceProcessor::new(config.julia_threads)
        .map_err(|e| InterferenceError::InvalidPattern(e.to_string()))?;

        let julia_coherence = JuliaCoherenceAnalyzer::new(config.julia_threads)
        .map_err(|e| InterferenceError::InvalidPattern(e.to_string()))?;

        // Initialize Chapel components
        let chapel_interference = ChapelParallelInterference::new(config.chapel_locales)
        .map_err(|e| InterferenceError::InvalidPattern(e.to_string()))?;

        let chapel_coherence = ChapelCoherenceProcessor::new(config.chapel_locales)
        .map_err(|e| InterferenceError::InvalidPattern(e.to_string()))?;

        // Previous initialization...

        Ok(Self {
            // Previous fields...
            julia_processor,
            julia_coherence,
            chapel_interference,
            chapel_coherence,
        })
    }

    pub fn analyze_pattern(&mut self, waves: &[Vec<Vec<Complex64>>]) -> Result<(), InterferenceError> {
        match self.config.compute_backend {
            ComputeBackend::Julia => self.analyze_with_julia(waves)?,
            ComputeBackend::Chapel => self.analyze_with_chapel(waves)?,
            ComputeBackend::Hybrid => self.analyze_hybrid(waves)?,
        }

        self.update_history()?;
        Ok(())
    }

    fn analyze_with_julia(&mut self, waves: &[Vec<Vec<Complex64>>]) -> Result<(), InterferenceError> {
        // Process interference using Julia
        let interference_result = self.julia_processor
        .process_interference(waves)
        .map_err(|e| InterferenceError::AnalysisError(e.to_string()))?;

        // Analyze coherence using Julia
        let coherence_result = self.julia_coherence
        .analyze_coherence(&interference_result)
        .map_err(|e| InterferenceError::CoherenceError(e.to_string()))?;

        // Update state with Julia results
        self.update_state_from_julia(interference_result, coherence_result)?;

        Ok(())
    }

    fn analyze_with_chapel(&mut self, waves: &[Vec<Vec<Complex64>>]) -> Result<(), InterferenceError> {
        // Process interference using Chapel
        let interference_result = self.chapel_interference
        .process_parallel(waves)
        .map_err(|e| InterferenceError::AnalysisError(e.to_string()))?;

        // Process coherence using Chapel
        let coherence_result = self.chapel_coherence
        .process_coherence(&interference_result)
        .map_err(|e| InterferenceError::CoherenceError(e.to_string()))?;

        // Update state with Chapel results
        self.update_state_from_chapel(interference_result, coherence_result)?;

        Ok(())
    }

    fn analyze_hybrid(&mut self, waves: &[Vec<Vec<Complex64>>]) -> Result<(), InterferenceError> {
        // Parallel computation using both backends
        let (julia_results, chapel_results) = rayon::join(
            || {
                let interference_result = self.julia_processor.process_interference(waves);
                let coherence_result = interference_result.and_then(|res| {
                    self.julia_coherence.analyze_coherence(&res)
                });
                (interference_result, coherence_result)
            },
            || {
                let interference_result = self.chapel_interference.process_parallel(waves);
                let coherence_result = interference_result.and_then(|res| {
                    self.chapel_coherence.process_coherence(&res)
                });
                (interference_result, coherence_result)
            },
        );

        // Merge results from both backends
        let (julia_interference, julia_coherence) = julia_results.0.and_then(|i| {
            julia_results.1.map(|c| (i, c))
        }).map_err(|e| InterferenceError::AnalysisError(e.to_string()))?;

        let (chapel_interference, chapel_coherence) = chapel_results.0.and_then(|i| {
            chapel_results.1.map(|c| (i, c))
        }).map_err(|e| InterferenceError::AnalysisError(e.to_string()))?;

        // Merge and update results
        self.merge_and_update_results(
            julia_interference,
            julia_coherence,
            chapel_interference,
            chapel_coherence,
        )?;

        Ok(())
    }

    fn update_state_from_julia(
        &mut self,
        interference: InterferenceResult,
        coherence: CoherenceResult,
    ) -> Result<(), InterferenceError> {
        let mut state = self.state.write();

        state.amplitude_pattern = interference.amplitude;
        state.phase_pattern = interference.phase;
        state.interference_map = interference.energy;
        state.coherence_field = coherence.coherence_field;
        state.total_energy = interference.total_energy;

        Ok(())
    }

    fn update_state_from_chapel(
        &mut self,
        interference: Vec<Vec<Complex64>>,
        coherence: ProcessorResult,
    ) -> Result<(), InterferenceError> {
        let mut state = self.state.write();

        // Extract amplitude and phase from complex field
        let (width, height) = self.config.grid_size;
        let mut amplitude = vec![vec![0.0; width]; height];
        let mut phase = vec![vec![0.0; width]; height];

        for i in 0..height {
            for j in 0..width {
                amplitude[i][j] = interference[i][j].norm();
                phase[i][j] = interference[i][j].arg();
            }
        }

        state.amplitude_pattern = amplitude;
        state.phase_pattern = phase;
        state.interference_map = coherence.energy_distribution;
        state.coherence_field = coherence.coherence_field;
        state.total_energy = coherence.total_energy;

        Ok(())
    }

    fn merge_and_update_results(
        &mut self,
        julia_interference: InterferenceResult,
        julia_coherence: CoherenceResult,
        chapel_interference: Vec<Vec<Complex64>>,
        chapel_coherence: ProcessorResult,
    ) -> Result<(), InterferenceError> {
        let mut state = self.state.write();
        let (width, height) = self.config.grid_size;

        // Average amplitude and phase patterns
        for i in 0..height {
            for j in 0..width {
                let chapel_amp = chapel_interference[i][j].norm();
                let chapel_phase = chapel_interference[i][j].arg();

                state.amplitude_pattern[i][j] = (julia_interference.amplitude[i][j] + chapel_amp) / 2.0;
                state.phase_pattern[i][j] = (julia_interference.phase[i][j] + chapel_phase) / 2.0;
            }
        }

        // Average energy and coherence fields
        for i in 0..height {
            for j in 0..width {
                state.interference_map[i][j] = (julia_interference.energy[i][j] +
                chapel_coherence.energy_distribution[i][j]) / 2.0;
                state.coherence_field[i][j] = (julia_coherence.coherence_field[i][j] +
                chapel_coherence.coherence_field[i][j]) / 2.0;
            }
        }

        // Average total energy
        state.total_energy = (julia_interference.total_energy + chapel_coherence.total_energy) / 2.0;

        Ok(())
    }

    // Previous methods remain unchanged...
}

// Update tests to include backend-specific testing...
#[cfg(test)]
mod tests {
    // Previous tests...

    #[test]
    fn test_julia_backend() -> Result<(), InterferenceError> {
        let config = InterferenceConfig {
            compute_backend: ComputeBackend::Julia,
            julia_threads: 2,
            ..Default::default()
        };
        let mut analyzer = InterferenceAnalyzer::new(config)?;
        // Test Julia-specific functionality...
        Ok(())
    }

    #[test]
    fn test_chapel_backend() -> Result<(), InterferenceError> {
        let config = InterferenceConfig {
            compute_backend: ComputeBackend::Chapel,
            chapel_locales: 2,
            ..Default::default()
        };
        let mut analyzer = InterferenceAnalyzer::new(config)?;
        // Test Chapel-specific functionality...
        Ok(())
    }

    #[test]
    fn test_hybrid_backend() -> Result<(), InterferenceError> {
        let config = InterferenceConfig {
            compute_backend: ComputeBackend::Hybrid,
            julia_threads: 2,
            chapel_locales: 2,
            ..Default::default()
        };
        let mut analyzer = InterferenceAnalyzer::new(config)?;
        // Test hybrid functionality...
        Ok(())
    }
}
