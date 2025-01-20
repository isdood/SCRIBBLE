//! Crystal Threading Implementation
//! Last Updated: 2025-01-20 13:46:05 UTC
//! Author: isdood
//! Current User: isdood

use crate::Error;

mod executor;
mod memory;
mod optimizer;
mod scheduler;

pub use executor::QuantumExecutor;
pub use memory::CrystalMemoryManager;
pub use optimizer::CrystalOptimizer;
pub use scheduler::CrystalScheduler;

use quartz::{
    CrystalArc,
    CrystalLattice,
    CrystalNode,
    Resonator,
    ResonancePattern,
    CRYSTAL_RESONANCE_HZ,
    BLEND_COHERENCE_THRESHOLD,
    MAX_BLEND_DEPTH,
    AetherCell,
    AetherField,
};

/// Crystal system configuration
#[derive(Debug, Clone)]
pub struct CrystalConfig {
    /// Lattice dimensions [x, y, z, t]
    pub dimensions: [usize; 4],
    /// Base resonance frequency
    pub resonance_frequency: f64,
    /// Quantum harmonics
    pub harmonics: u32,
    /// Minimum harmony threshold
    pub min_harmony: f64,
    /// Maximum blend depth
    pub max_blend_depth: usize,
}

impl Default for CrystalConfig {
    fn default() -> Self {
        Self {
            dimensions: [8, 8, 8, 4],
            resonance_frequency: CRYSTAL_RESONANCE_HZ,
            harmonics: 3,
            min_harmony: BLEND_COHERENCE_THRESHOLD,
            max_blend_depth: MAX_BLEND_DEPTH,
        }
    }
}

/// Crystal system state
#[derive(Debug)]
pub struct CrystalSystem {
    /// System configuration
    config: CrystalConfig,
    /// Quantum executor
    executor: CrystalArc<QuantumExecutor>,
    /// Memory manager
    memory: CrystalArc<CrystalMemoryManager>,
    /// Optimizer
    optimizer: CrystalArc<CrystalOptimizer>,
    /// Scheduler
    scheduler: CrystalArc<CrystalScheduler>,
    /// Crystal lattice
    lattice: CrystalArc<CrystalLattice>,
    /// System resonator
    resonator: CrystalArc<Resonator>,
}

impl CrystalSystem {
    /// Create new crystal system
    pub fn new(config: CrystalConfig) -> Self {
        let resonator = CrystalArc::new(Resonator::new(ResonancePattern::Crystal {
            frequency: config.resonance_frequency,
            harmonics: config.harmonics,
        }));

        let lattice = CrystalArc::new(CrystalLattice::new(config.dimensions));

        Self {
            executor: CrystalArc::new(QuantumExecutor::new()),
            memory: CrystalArc::new(CrystalMemoryManager::new()),
            optimizer: CrystalArc::new(CrystalOptimizer::new()),
            scheduler: CrystalArc::new(CrystalScheduler::new()),
            lattice,
            resonator,
            config,
        }
    }

    /// Initialize system with quantum coherence
    pub async fn initialize(&self) -> Result<(), Error> {
        // Initialize quantum executor
        self.executor.run().await;

        // Initialize memory system
        self.memory.initialize()?;

        // Initialize optimizer
        self.optimizer.initialize()?;

        // Initialize scheduler
        self.scheduler.initialize()?;

        Ok(())
    }

    /// Get system metrics
    pub fn get_metrics(&self) -> SystemMetrics {
        SystemMetrics {
            executor_metrics: self.executor.get_metrics(),
            memory_metrics: self.memory.get_metrics(),
            optimization_metrics: self.optimizer.get_metrics(),
            scheduler_metrics: self.scheduler.get_metrics(),
            resonance_stability: self.resonator.stability(),
            quantum_coherence: self.get_system_coherence(),
            harmony: self.get_system_harmony(),
        }
    }

    /// Get system quantum coherence
    pub fn get_system_coherence(&self) -> f64 {
        let executor_coherence = self.executor.get_coherence();
        let memory_coherence = self.memory.get_coherence();
        let optimizer_coherence = self.optimizer.get_coherence();
        let scheduler_coherence = self.scheduler.get_coherence();

        (executor_coherence + memory_coherence + optimizer_coherence + scheduler_coherence) / 4.0
    }

    /// Get system harmony
    pub fn get_system_harmony(&self) -> f64 {
        let executor_harmony = self.executor.get_harmony();
        let memory_harmony = self.memory.get_harmony();
        let optimizer_harmony = self.optimizer.get_harmony();
        let scheduler_harmony = self.scheduler.get_harmony();

        (executor_harmony + memory_harmony + optimizer_harmony + scheduler_harmony) / 4.0
    }

    /// Get executor reference
    pub fn executor(&self) -> &CrystalArc<QuantumExecutor> {
        &self.executor
    }

    /// Get memory manager reference
    pub fn memory(&self) -> &CrystalArc<CrystalMemoryManager> {
        &self.memory
    }

    /// Get optimizer reference
    pub fn optimizer(&self) -> &CrystalArc<CrystalOptimizer> {
        &self.optimizer
    }

    /// Get scheduler reference
    pub fn scheduler(&self) -> &CrystalArc<CrystalScheduler> {
        &self.scheduler
    }

    /// Get lattice reference
    pub fn lattice(&self) -> &CrystalArc<CrystalLattice> {
        &self.lattice
    }

    /// Get resonator reference
    pub fn resonator(&self) -> &CrystalArc<Resonator> {
        &self.resonator
    }
}

/// System metrics
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    /// Executor metrics
    pub executor_metrics: ExecutorMetrics,
    /// Memory metrics
    pub memory_metrics: MemoryMetrics,
    /// Optimization metrics
    pub optimization_metrics: OptimizationMetrics,
    /// Scheduler metrics
    pub scheduler_metrics: SchedulerMetrics,
    /// System resonance stability
    pub resonance_stability: f64,
    /// System quantum coherence
    pub quantum_coherence: f64,
    /// System harmony
    pub harmony: f64,
}

/// Re-exports for convenience
pub mod prelude {
    pub use super::{
        CrystalSystem,
        CrystalConfig,
        SystemMetrics,
        QuantumExecutor,
        CrystalMemoryManager,
        CrystalOptimizer,
        CrystalScheduler,
    };

    pub use quartz::{
        CrystalArc,
        CrystalLattice,
        CrystalNode,
        Resonator,
        ResonancePattern,
        AetherCell,
        AetherField,
        CRYSTAL_RESONANCE_HZ,
        BLEND_COHERENCE_THRESHOLD,
        MAX_BLEND_DEPTH,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_initialization() {
        let config = CrystalConfig::default();
        let system = CrystalSystem::new(config);

        assert!(system.initialize().await.is_ok());
    }

    #[test]
    fn test_system_coherence() {
        let config = CrystalConfig::default();
        let system = CrystalSystem::new(config);

        let coherence = system.get_system_coherence();
        assert!(coherence >= BLEND_COHERENCE_THRESHOLD);
    }

    #[test]
    fn test_system_harmony() {
        let config = CrystalConfig::default();
        let system = CrystalSystem::new(config);

        let harmony = system.get_system_harmony();
        assert!(harmony >= BLEND_COHERENCE_THRESHOLD);
    }

    #[test]
    fn test_system_resonance() {
        let config = CrystalConfig::default();
        let system = CrystalSystem::new(config);

        let metrics = system.get_metrics();
        assert!(metrics.resonance_stability >= BLEND_COHERENCE_THRESHOLD);
    }
}
