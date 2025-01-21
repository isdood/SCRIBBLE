//! Crystal resonator pattern management and synchronization with Julia/Chapel integration
//! Created: 2025-01-21 16:17:40 UTC
//! Author: @isdood

// Previous imports...
use crate::{
    julia::{
        resonator::{JuliaResonatorProcessor, ResonatorResult},
        patterns::{JuliaPatternAnalyzer, PatternResult},
    },
    chapel::{
        parallel::{ChapelDomainMap, ChapelParallelResonator},
        patterns::{ChapelPatternProcessor, ProcessorResult},
    },
};

// Previous error enum...

#[derive(Debug, Clone)]
pub struct ResonatorConfig {
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

impl Default for ResonatorConfig {
    fn default() -> Self {
        Self {
            // Previous defaults...
            julia_threads: 4,
            chapel_locales: 2,
            compute_backend: ComputeBackend::Hybrid,
        }
    }
}

pub struct CrystalResonator {
    // Previous fields...
    julia_processor: JuliaResonatorProcessor,
    julia_analyzer: JuliaPatternAnalyzer,
    chapel_resonator: ChapelParallelResonator,
    chapel_processor: ChapelPatternProcessor,
}

impl CrystalResonator {
    pub fn new(config: ResonatorConfig) -> Result<Self, ResonatorError> {
        // Previous initialization...

        // Initialize Julia components
        let julia_processor = JuliaResonatorProcessor::new(config.julia_threads)
        .map_err(|e| ResonatorError::InvalidConfig(e.to_string()))?;

        let julia_analyzer = JuliaPatternAnalyzer::new(config.julia_threads)
        .map_err(|e| ResonatorError::InvalidConfig(e.to_string()))?;

        // Initialize Chapel components
        let chapel_resonator = ChapelParallelResonator::new(config.chapel_locales)
        .map_err(|e| ResonatorError::InvalidConfig(e.to_string()))?;

        let chapel_processor = ChapelPatternProcessor::new(config.chapel_locales)
        .map_err(|e| ResonatorError::InvalidConfig(e.to_string()))?;

        Ok(Self {
            // Previous fields...
            julia_processor,
            julia_analyzer,
            chapel_resonator,
            chapel_processor,
        })
    }

    pub fn update(&mut self, time: f64) -> Result<(), ResonatorError> {
        match self.config.compute_backend {
            ComputeBackend::Julia => self.update_with_julia(time)?,
            ComputeBackend::Chapel => self.update_with_chapel(time)?,
            ComputeBackend::Hybrid => self.update_hybrid(time)?,
        }

        self.update_state()?;
        self.verify_energy_conservation()?;
        Ok(())
    }

    fn update_with_julia(&mut self, time: f64) -> Result<(), ResonatorError> {
        // Process resonator state using Julia
        let resonator_result = self.julia_processor
        .process_resonator(&self.resonance_nodes, time)
        .map_err(|e| ResonatorError::SyncError(e.to_string()))?;

        // Analyze patterns using Julia
        let pattern_result = self.julia_analyzer
        .analyze_patterns(&resonator_result)
        .map_err(|e| ResonatorError::PatternError(e.to_string()))?;

        // Update with Julia results
        self.update_from_julia(resonator_result, pattern_result)?;

        Ok(())
    }

    fn update_with_chapel(&mut self, time: f64) -> Result<(), ResonatorError> {
        // Process resonator state using Chapel
        let resonator_result = self.chapel_resonator
        .process_parallel(&self.resonance_nodes, time)
        .map_err(|e| ResonatorError::SyncError(e.to_string()))?;

        // Process patterns using Chapel
        let pattern_result = self.chapel_processor
        .process_patterns(&resonator_result)
        .map_err(|e| ResonatorError::PatternError(e.to_string()))?;

        // Update with Chapel results
        self.update_from_chapel(resonator_result, pattern_result)?;

        Ok(())
    }

    fn update_hybrid(&mut self, time: f64) -> Result<(), ResonatorError> {
        // Parallel computation using both backends
        let (julia_results, chapel_results) = rayon::join(
            || {
                let resonator_result = self.julia_processor.process_resonator(
                    &self.resonance_nodes,
                    time,
                );
                let pattern_result = resonator_result.and_then(|res| {
                    self.julia_analyzer.analyze_patterns(&res)
                });
                (resonator_result, pattern_result)
            },
            || {
                let resonator_result = self.chapel_resonator.process_parallel(
                    &self.resonance_nodes,
                    time,
                );
                let pattern_result = resonator_result.and_then(|res| {
                    self.chapel_processor.process_patterns(&res)
                });
                (resonator_result, pattern_result)
            },
        );

        // Merge results from both backends
        let (julia_resonator, julia_patterns) = julia_results.0.and_then(|r| {
            julia_results.1.map(|p| (r, p))
        }).map_err(|e| ResonatorError::SyncError(e.to_string()))?;

        let (chapel_resonator, chapel_patterns) = chapel_results.0.and_then(|r| {
            chapel_results.1.map(|p| (r, p))
        }).map_err(|e| ResonatorError::SyncError(e.to_string()))?;

        // Merge and update results
        self.merge_and_update_results(
            julia_resonator,
            julia_patterns,
            chapel_resonator,
            chapel_patterns,
        )?;

        Ok(())
    }

    fn update_from_julia(
        &mut self,
        resonator: ResonatorResult,
        patterns: PatternResult,
    ) -> Result<(), ResonatorError> {
        // Update resonance nodes
        self.resonance_nodes = resonator.nodes;

        // Update mode cache
        self.mode_cache.clear();
        for pattern in patterns.patterns {
            self.mode_cache.insert(pattern.mode, pattern.into());
        }

        Ok(())
    }

    fn update_from_chapel(
        &mut self,
        resonator: Vec<ResonanceNode>,
        patterns: ProcessorResult,
    ) -> Result<(), ResonatorError> {
        // Update resonance nodes
        self.resonance_nodes.clear();
        for node in resonator {
            let coords = (node.position[0] as usize, node.position[1] as usize);
            self.resonance_nodes.insert(coords, node);
        }

        // Update mode cache
        self.mode_cache.clear();
        for pattern in patterns.patterns {
            self.mode_cache.insert(pattern.mode, pattern.into());
        }

        Ok(())
    }

    fn merge_and_update_results(
        &mut self,
        julia_resonator: ResonatorResult,
        julia_patterns: PatternResult,
        chapel_resonator: Vec<ResonanceNode>,
        chapel_patterns: ProcessorResult,
    ) -> Result<(), ResonatorError> {
        // Merge resonance nodes
        self.resonance_nodes.clear();

        // Process Julia nodes
        for (coords, node) in julia_resonator.nodes {
            self.resonance_nodes.insert(coords, node);
        }

        // Merge with Chapel nodes
        for node in chapel_resonator {
            let coords = (node.position[0] as usize, node.position[1] as usize);
            if let Some(existing) = self.resonance_nodes.get_mut(&coords) {
                // Average the results
                existing.frequency = (existing.frequency + node.frequency) / 2.0;
                existing.amplitude = (existing.amplitude + node.amplitude) / Complex64::new(2.0, 0.0);
                existing.quality = (existing.quality + node.quality) / 2.0;
            } else {
                self.resonance_nodes.insert(coords, node);
            }
        }

        // Merge mode patterns
        self.mode_cache.clear();

        // Process Julia patterns
        for pattern in julia_patterns.patterns {
            self.mode_cache.insert(pattern.mode.clone(), pattern.into());
        }

        // Merge with Chapel patterns
        for pattern in chapel_patterns.patterns {
            if let Some(existing) = self.mode_cache.get_mut(&pattern.mode) {
                // Average the pattern properties
                existing.merge_with(&pattern.into())?;
            } else {
                self.mode_cache.insert(pattern.mode, pattern.into());
            }
        }

        Ok(())
    }

    // Previous methods remain unchanged...
}

// Update tests to include backend-specific testing...
#[cfg(test)]
mod tests {
    // Previous tests...

    #[test]
    fn test_julia_backend() -> Result<(), ResonatorError> {
        let config = ResonatorConfig {
            compute_backend: ComputeBackend::Julia,
            julia_threads: 2,
            ..Default::default()
        };
        let mut resonator = CrystalResonator::new(config)?;
        resonator.update(0.1)?;
        Ok(())
    }

    #[test]
    fn test_chapel_backend() -> Result<(), ResonatorError> {
        let config = ResonatorConfig {
            compute_backend: ComputeBackend::Chapel,
            chapel_locales: 2,
            ..Default::default()
        };
        let mut resonator = CrystalResonator::new(config)?;
        resonator.update(0.1)?;
        Ok(())
    }

    #[test]
    fn test_hybrid_backend() -> Result<(), ResonatorError> {
        let config = ResonatorConfig {
            compute_backend: ComputeBackend::Hybrid,
            julia_threads: 2,
            chapel_locales: 2,
            ..Default::default()
        };
        let mut resonator = CrystalResonator::new(config)?;
        resonator.update(0.1)?;
        Ok(())
    }
}
