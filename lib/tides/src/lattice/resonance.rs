//! Crystal lattice resonance management and analysis with Julia/Chapel integration
//! Created: 2025-01-21 16:11:44 UTC
//! Author: @isdood

// Previous imports...
use crate::{
    julia::{
        resonance::{JuliaResonanceProcessor, ResonanceResult},
        patterns::{JuliaPatternDetector, PatternResult},
    },
    chapel::{
        parallel::{ChapelDomainMap, ChapelParallelResonance},
        patterns::{ChapelPatternProcessor, ProcessorResult},
    },
};

// Previous error enum and configs...
#[derive(Debug, Clone)]
pub struct ResonanceConfig {
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

impl Default for ResonanceConfig {
    fn default() -> Self {
        Self {
            // Previous defaults...
            julia_threads: 4,
            chapel_locales: 2,
            compute_backend: ComputeBackend::Hybrid,
        }
    }
}

pub struct LatticeResonance {
    // Previous fields...
    julia_processor: JuliaResonanceProcessor,
    julia_detector: JuliaPatternDetector,
    chapel_resonance: ChapelParallelResonance,
    chapel_processor: ChapelPatternProcessor,
}

impl LatticeResonance {
    pub fn new(config: ResonanceConfig) -> Result<Self, ResonanceError> {
        // Initialize Julia components
        let julia_processor = JuliaResonanceProcessor::new(config.julia_threads)
        .map_err(|e| ResonanceError::InvalidConfig(e.to_string()))?;

        let julia_detector = JuliaPatternDetector::new(config.julia_threads)
        .map_err(|e| ResonanceError::InvalidConfig(e.to_string()))?;

        // Initialize Chapel components
        let chapel_resonance = ChapelParallelResonance::new(config.chapel_locales)
        .map_err(|e| ResonanceError::InvalidConfig(e.to_string()))?;

        let chapel_processor = ChapelPatternProcessor::new(config.chapel_locales)
        .map_err(|e| ResonanceError::InvalidConfig(e.to_string()))?;

        Ok(Self {
            config,
            nodes: HashMap::new(),
           patterns: RwLock::new(HashMap::new()),
           state: RwLock::new(ResonanceState::default()),
           julia_processor,
           julia_detector,
           chapel_resonance,
           chapel_processor,
        })
    }

    pub fn update(&mut self, time: f64) -> Result<(), ResonanceError> {
        match self.config.compute_backend {
            ComputeBackend::Julia => self.update_with_julia(time)?,
            ComputeBackend::Chapel => self.update_with_chapel(time)?,
            ComputeBackend::Hybrid => self.update_hybrid(time)?,
        }

        self.update_state()?;
        Ok(())
    }

    fn update_with_julia(&mut self, time: f64) -> Result<(), ResonanceError> {
        // Process resonance using Julia
        let resonance_result = self.julia_processor
        .process_nodes(&self.nodes, time)
        .map_err(|e| ResonanceError::SyncError(e.to_string()))?;

        // Detect patterns using Julia
        let pattern_result = self.julia_detector
        .detect_patterns(&resonance_result)
        .map_err(|e| ResonanceError::PatternError(e.to_string()))?;

        // Update patterns with Julia results
        self.update_patterns_from_julia(resonance_result, pattern_result)?;

        Ok(())
    }

    fn update_with_chapel(&mut self, time: f64) -> Result<(), ResonanceError> {
        // Process resonance using Chapel
        let resonance_result = self.chapel_resonance
        .process_parallel(&self.nodes, time)
        .map_err(|e| ResonanceError::SyncError(e.to_string()))?;

        // Process patterns using Chapel
        let pattern_result = self.chapel_processor
        .process_patterns(&resonance_result)
        .map_err(|e| ResonanceError::PatternError(e.to_string()))?;

        // Update patterns with Chapel results
        self.update_patterns_from_chapel(resonance_result, pattern_result)?;

        Ok(())
    }

    fn update_hybrid(&mut self, time: f64) -> Result<(), ResonanceError> {
        // Parallel computation using both backends
        let (julia_results, chapel_results) = rayon::join(
            || {
                let resonance_result = self.julia_processor.process_nodes(
                    &self.nodes,
                    time,
                );
                let pattern_result = resonance_result.and_then(|res| {
                    self.julia_detector.detect_patterns(&res)
                });
                (resonance_result, pattern_result)
            },
            || {
                let resonance_result = self.chapel_resonance.process_parallel(
                    &self.nodes,
                    time,
                );
                let pattern_result = resonance_result.and_then(|res| {
                    self.chapel_processor.process_patterns(&res)
                });
                (resonance_result, pattern_result)
            },
        );

        // Merge and update results from both backends
        let (julia_resonance, julia_patterns) = julia_results.0.and_then(|r| {
            julia_results.1.map(|p| (r, p))
        }).map_err(|e| ResonanceError::SyncError(e.to_string()))?;

        let (chapel_resonance, chapel_patterns) = chapel_results.0.and_then(|r| {
            chapel_results.1.map(|p| (r, p))
        }).map_err(|e| ResonanceError::SyncError(e.to_string()))?;

        // Merge results
        self.merge_and_update_results(
            julia_resonance,
            julia_patterns,
            chapel_resonance,
            chapel_patterns,
        )?;

        Ok(())
    }

    fn update_patterns_from_julia(
        &mut self,
        resonance: ResonanceResult,
        patterns: PatternResult,
    ) -> Result<(), ResonanceError> {
        let mut pattern_map = self.patterns.write();
        pattern_map.clear();

        for pattern in patterns.patterns {
            let pattern_id = PatternId {
                frequency: self.quantize_frequency(pattern.frequency),
                nodes: pattern.nodes,
            };
            pattern_map.insert(pattern_id, ResonancePattern {
                frequency: pattern.frequency,
                amplitude: pattern.amplitude,
                phase: pattern.phase,
                nodes: pattern.nodes,
                stability: pattern.stability,
                energy: pattern.energy,
            });
        }

        Ok(())
    }

    fn update_patterns_from_chapel(
        &mut self,
        resonance: Vec<(u64, NodeState)>,
                                   patterns: ProcessorResult,
    ) -> Result<(), ResonanceError> {
        let mut pattern_map = self.patterns.write();
        pattern_map.clear();

        for pattern in patterns.patterns {
            let pattern_id = PatternId {
                frequency: self.quantize_frequency(pattern.frequency),
                nodes: pattern.nodes,
            };
            pattern_map.insert(pattern_id, ResonancePattern {
                frequency: pattern.frequency,
                amplitude: pattern.amplitude,
                phase: pattern.phase,
                nodes: pattern.nodes,
                stability: pattern.stability,
                energy: pattern.energy,
            });
        }

        Ok(())
    }

    fn merge_and_update_results(
        &mut self,
        julia_resonance: ResonanceResult,
        julia_patterns: PatternResult,
        chapel_resonance: Vec<(u64, NodeState)>,
                                chapel_patterns: ProcessorResult,
    ) -> Result<(), ResonanceError> {
        let mut pattern_map = self.patterns.write();
        pattern_map.clear();

        // Merge pattern results from both backends
        let mut merged_patterns = HashMap::new();

        // Process Julia patterns
        for pattern in julia_patterns.patterns {
            let pattern_id = PatternId {
                frequency: self.quantize_frequency(pattern.frequency),
                nodes: pattern.nodes.clone(),
            };
            merged_patterns.insert(pattern_id, pattern);
        }

        // Merge with Chapel patterns
        for pattern in chapel_patterns.patterns {
            let pattern_id = PatternId {
                frequency: self.quantize_frequency(pattern.frequency),
                nodes: pattern.nodes.clone(),
            };

            if let Some(existing) = merged_patterns.get_mut(&pattern_id) {
                // Average the results
                existing.amplitude = (existing.amplitude + pattern.amplitude) / 2.0;
                existing.phase = (existing.phase + pattern.phase) / 2.0;
                existing.stability = (existing.stability + pattern.stability) / 2.0;
                existing.energy = (existing.energy + pattern.energy) / 2.0;
            } else {
                merged_patterns.insert(pattern_id, pattern);
            }
        }

        // Update pattern map with merged results
        for (id, pattern) in merged_patterns {
            pattern_map.insert(id, ResonancePattern {
                frequency: pattern.frequency,
                amplitude: pattern.amplitude,
                phase: pattern.phase,
                nodes: pattern.nodes,
                stability: pattern.stability,
                energy: pattern.energy,
            });
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
    fn test_julia_backend() -> Result<(), ResonanceError> {
        let config = ResonanceConfig {
            compute_backend: ComputeBackend::Julia,
            julia_threads: 2,
            ..Default::default()
        };
        let mut resonance = LatticeResonance::new(config)?;
        // Test Julia-specific functionality...
        Ok(())
    }

    #[test]
    fn test_chapel_backend() -> Result<(), ResonanceError> {
        let config = ResonanceConfig {
            compute_backend: ComputeBackend::Chapel,
            chapel_locales: 2,
            ..Default::default()
        };
        let mut resonance = LatticeResonance::new(config)?;
        // Test Chapel-specific functionality...
        Ok(())
    }

    #[test]
    fn test_hybrid_backend() -> Result<(), ResonanceError> {
        let config = ResonanceConfig {
            compute_backend: ComputeBackend::Hybrid,
            julia_threads: 2,
            chapel_locales: 2,
            ..Default::default()
        };
        let mut resonance = LatticeResonance::new(config)?;
        // Test hybrid functionality...
        Ok(())
    }
}
