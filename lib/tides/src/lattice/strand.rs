//! Crystal lattice strand formation and manipulation with Julia/Chapel computation
//! Created: 2025-01-21 16:10:35 UTC
//! Author: @isdood

// Previous imports...
use crate::{
    julia::{
        strand::{JuliaStrandProcessor, StrandResult},
        patterns::{JuliaPatternAnalyzer, PatternResult},
    },
    chapel::{
        parallel::{ChapelDomainMap, ChapelParallelStrand},
        patterns::{ChapelPatternMatcher, MatchResult},
    },
};

// Previous error enum and configs...
#[derive(Debug, Clone)]
pub struct StrandConfig {
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

impl Default for StrandConfig {
    fn default() -> Self {
        Self {
            // Previous defaults...
            julia_threads: 4,
            chapel_locales: 2,
            compute_backend: ComputeBackend::Hybrid,
        }
    }
}

pub struct LatticeStrand {
    // Previous fields...
    julia_processor: JuliaStrandProcessor,
    julia_analyzer: JuliaPatternAnalyzer,
    chapel_strand: ChapelParallelStrand,
    chapel_matcher: ChapelPatternMatcher,
}

impl LatticeStrand {
    pub fn new(config: StrandConfig, wave_pattern: Arc<WavePattern>) -> Result<Self, StrandError> {
        // Initialize Julia components
        let julia_processor = JuliaStrandProcessor::new(config.julia_threads)
        .map_err(|e| StrandError::FormationError(e.to_string()))?;

        let julia_analyzer = JuliaPatternAnalyzer::new(config.julia_threads)
        .map_err(|e| StrandError::FormationError(e.to_string()))?;

        // Initialize Chapel components
        let chapel_strand = ChapelParallelStrand::new(config.chapel_locales)
        .map_err(|e| StrandError::FormationError(e.to_string()))?;

        let chapel_matcher = ChapelPatternMatcher::new(config.chapel_locales)
        .map_err(|e| StrandError::FormationError(e.to_string()))?;

        Ok(Self {
            config,
            strands: RwLock::new(HashMap::new()),
           history: RwLock::new(VecDeque::with_capacity(config.memory_length)),
           wave_pattern,
           julia_processor,
           julia_analyzer,
           chapel_strand,
           chapel_matcher,
        })
    }

    pub fn update(&self, time: f64) -> Result<(), StrandError> {
        match self.config.compute_backend {
            ComputeBackend::Julia => self.update_with_julia(time)?,
            ComputeBackend::Chapel => self.update_with_chapel(time)?,
            ComputeBackend::Hybrid => self.update_hybrid(time)?,
        }

        self.update_history()?;
        Ok(())
    }

    fn update_with_julia(&self, time: f64) -> Result<(), StrandError> {
        let strands = self.strands.read();

        // Process strands using Julia
        let strand_results = self.julia_processor
        .process_strands(strands.values(), time)
        .map_err(|e| StrandError::FormationError(e.to_string()))?;

        // Analyze patterns using Julia
        let pattern_results = self.julia_analyzer
        .analyze_patterns(&strand_results, self.wave_pattern.as_ref())
        .map_err(|e| StrandError::FormationError(e.to_string()))?;

        // Update strands with Julia results
        self.update_strands_from_julia(strand_results, pattern_results)?;

        Ok(())
    }

    fn update_with_chapel(&self, time: f64) -> Result<(), StrandError> {
        let strands = self.strands.read();

        // Process strands using Chapel
        let strand_results = self.chapel_strand
        .process_parallel(strands.values(), time)
        .map_err(|e| StrandError::FormationError(e.to_string()))?;

        // Match patterns using Chapel
        let match_results = self.chapel_matcher
        .match_patterns(&strand_results, self.wave_pattern.as_ref())
        .map_err(|e| StrandError::FormationError(e.to_string()))?;

        // Update strands with Chapel results
        self.update_strands_from_chapel(strand_results, match_results)?;

        Ok(())
    }

    fn update_hybrid(&self, time: f64) -> Result<(), StrandError> {
        let strands = self.strands.read();

        // Parallel computation using both backends
        let (julia_results, chapel_results) = rayon::join(
            || {
                let strand_results = self.julia_processor.process_strands(
                    strands.values(),
                                                                          time,
                );
                let pattern_results = strand_results.and_then(|strands| {
                    self.julia_analyzer.analyze_patterns(&strands, self.wave_pattern.as_ref())
                });
                (strand_results, pattern_results)
            },
            || {
                let strand_results = self.chapel_strand.process_parallel(
                    strands.values(),
                                                                         time,
                );
                let match_results = strand_results.and_then(|strands| {
                    self.chapel_matcher.match_patterns(&strands, self.wave_pattern.as_ref())
                });
                (strand_results, match_results)
            },
        );

        // Merge and update results
        let (julia_strands, julia_patterns) = julia_results.0.and_then(|s| {
            julia_results.1.map(|p| (s, p))
        }).map_err(|e| StrandError::FormationError(e.to_string()))?;

        let (chapel_strands, chapel_patterns) = chapel_results.0.and_then(|s| {
            chapel_results.1.map(|p| (s, p))
        }).map_err(|e| StrandError::FormationError(e.to_string()))?;

        // Merge results from both backends
        self.merge_and_update_results(
            julia_strands,
            julia_patterns,
            chapel_strands,
            chapel_patterns,
        )?;

        Ok(())
    }

    // Add new helper methods for backend-specific updates...
    fn update_strands_from_julia(
        &self,
        strand_results: StrandResult,
        pattern_results: PatternResult,
    ) -> Result<(), StrandError> {
        let mut strands = self.strands.write();

        // Update strands with Julia computation results
        for (id, result) in strand_results.iter() {
            if let Some(strand) = strands.get_mut(id) {
                strand.phase = result.phase;
                strand.amplitude = result.amplitude;
                strand.stability = result.stability;
                strand.energy = result.energy;
                strand.alignment = result.alignment;
            }
        }

        Ok(())
    }

    fn update_strands_from_chapel(
        &self,
        strand_results: Vec<(StrandId, Strand)>,
                                  match_results: MatchResult,
    ) -> Result<(), StrandError> {
        let mut strands = self.strands.write();

        // Update strands with Chapel computation results
        for (id, result) in strand_results {
            if let Some(strand) = strands.get_mut(&id) {
                *strand = result;
            }
        }

        Ok(())
    }

    fn merge_and_update_results(
        &self,
        julia_strands: StrandResult,
        julia_patterns: PatternResult,
        chapel_strands: Vec<(StrandId, Strand)>,
                                chapel_patterns: MatchResult,
    ) -> Result<(), StrandError> {
        let mut strands = self.strands.write();

        // Merge results from both backends
        for (id, strand) in strands.iter_mut() {
            if let Some(julia_result) = julia_strands.get(id) {
                if let Some(chapel_result) = chapel_strands.iter().find(|(i, _)| i == id) {
                    // Average results from both backends
                    strand.phase = (julia_result.phase + chapel_result.1.phase) / 2.0;
                    strand.amplitude = (julia_result.amplitude + chapel_result.1.amplitude) / 2.0;
                    strand.stability = (julia_result.stability + chapel_result.1.stability) / 2.0;
                    strand.energy = (julia_result.energy + chapel_result.1.energy) / 2.0;
                    strand.alignment = (julia_result.alignment + chapel_result.1.alignment) / 2.0;
                }
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
    fn test_julia_backend() -> Result<(), StrandError> {
        let config = StrandConfig {
            compute_backend: ComputeBackend::Julia,
            julia_threads: 2,
            ..Default::default()
        };
        // Test Julia-specific functionality...
        Ok(())
    }

    #[test]
    fn test_chapel_backend() -> Result<(), StrandError> {
        let config = StrandConfig {
            compute_backend: ComputeBackend::Chapel,
            chapel_locales: 2,
            ..Default::default()
        };
        // Test Chapel-specific functionality...
        Ok(())
    }

    #[test]
    fn test_hybrid_backend() -> Result<(), StrandError> {
        let config = StrandConfig {
            compute_backend: ComputeBackend::Hybrid,
            julia_threads: 2,
            chapel_locales: 2,
            ..Default::default()
        };
        // Test hybrid functionality...
        Ok(())
    }
}
