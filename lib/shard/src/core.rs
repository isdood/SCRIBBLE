// core.rs

use crate::crystal_compute::{ComputeCrystal, CrystalScheduler, QuantumOptimizer};

/// Enhanced register file for the Shard architecture
#[derive(Debug, Clone)]
pub struct ShardRegisterFile {
    // Existing fields remain the same
    pub v_regs: [Vector4D; 8],
    pub qs_regs: [Vec<f64>; 4],
    pub cr_regs: [Vec<f64>; 4],
    pub rp_regs: [[f64; 8]; 4],
    pub pc4d: Vector4D,
    pub qf: u64,

    // New fields for crystal compute integration
    /// Active compute crystal reference
    pub compute_crystal: Option<Arc<RwLock<ComputeCrystal>>>,
    /// Performance metrics for current crystal
    pub crystal_metrics: CrystalMetrics,
}

/// Extended Shard Instruction Set
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShardOpcode {
    // Existing opcodes...
    
    // New Crystal Compute Operations
    CGROW_OPT,    // Optimized crystal growth
    CADAPT,       // Adapt crystal to workload
    COPT,         // Optimize crystal structure
    CPERF,        // Get crystal performance metrics
    CSCHED,       // Schedule workload on crystal
    CMEM,         // Crystal memory operation
}

impl ShardRegisterFile {
    /// Initialize with crystal compute support
    pub fn new() -> Self {
        Self {
            // Initialize existing fields...
            v_regs: [Vector4D::zero(); 8],
            qs_regs: std::array::from_fn(|_| Vec::with_capacity(16)),
            cr_regs: std::array::from_fn(|_| Vec::with_capacity(16)),
            rp_regs: [[0.0; 8]; 4],
            pc4d: Vector4D::zero(),
            qf: 0,
            
            // Initialize new fields
            compute_crystal: None,
            crystal_metrics: CrystalMetrics::default(),
        }
    }

    /// Attach compute crystal to register file
    pub fn attach_compute_crystal(&mut self, crystal: Arc<RwLock<ComputeCrystal>>) {
        self.compute_crystal = Some(crystal);
        self.update_crystal_metrics();
    }

    /// Update crystal performance metrics
    fn update_crystal_metrics(&mut self) {
        if let Some(crystal) = &self.compute_crystal {
            let crystal = crystal.read();
            self.crystal_metrics = crystal.metrics.clone();
            
            // Update crystal registers based on performance
            if let Some(cr) = self.cr_regs[0].first_mut() {
                *cr = crystal.efficiency;
            }
            if let Some(cr) = self.cr_regs[1].first_mut() {
                *cr = crystal.growth_state.stability;
            }
        }
    }
}

/// Enhanced memory system with crystal compute support
#[derive(Debug)]
pub struct ShardMemory {
    // Existing fields
    pub l1q: QuantumHashMap<Vector4D, f64>,
    pub l2c: CrystalLattice,
    pub l3h: HyperGrid,
    pub aether_state: AetherGrid,
    
    // New fields for crystal compute
    pub crystal_scheduler: CrystalScheduler,
    pub memory_manager: CrystalMemoryManager,
    pub quantum_optimizer: QuantumOptimizer,
}

impl ShardMemory {
    pub fn new() -> Self {
        let config = CrystalConfig {
            coherence_threshold: QUANTUM_COHERENCE_THRESHOLD,
            crystal_growth_rate: CRYSTAL_RESONANCE_THRESHOLD,
            creator: b"isdood".to_vec(),
        };

        Self {
            // Initialize existing fields
            l1q: QuantumHashMap::with_capacity(CACHE_MAX_ENTRIES),
            l2c: CrystalLattice::new(config.clone()),
            l3h: HyperGrid::new(config.clone()),
            aether_state: AetherGrid::new(config),
            
            // Initialize new components
            crystal_scheduler: CrystalScheduler::new(),
            memory_manager: CrystalMemoryManager::new(),
            quantum_optimizer: QuantumOptimizer::new(),
        }
    }

    /// Schedule workload using crystal compute
    pub fn schedule_workload(&mut self, workload: Workload) -> Result<(), Error> {
        // Distribute workload across crystal structure
        let allocation = self.crystal_scheduler.distribute_workload(workload)?;
        
        // Optimize memory access patterns
        self.memory_manager.optimize_access_patterns()?;
        
        // Apply quantum optimizations
        for crystal in self.crystal_scheduler.crystals.iter_mut() {
            self.quantum_optimizer.apply_quantum_optimization(&mut crystal.write())?;
        }
        
        Ok(())
    }

    /// Optimize crystal compute structure
    pub fn optimize_compute_structure(&mut self) -> Result<OptimizationStats, Error> {
        // Optimize crystal structure
        let stats = self.crystal_scheduler.optimize_structure()?;
        
        // Update memory hierarchy
        self.memory_manager.update_hierarchy(&stats)?;
        
        // Apply quantum optimizations
        self.quantum_optimizer.optimize_global_state(&self.crystal_scheduler)?;
        
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_compute_integration() {
        let mut mem = ShardMemory::new();
        
        // Create test workload
        let workload = Workload::new()
            .with_compute_intensity(0.8)
            .with_memory_access_pattern(AccessPattern::Strided)
            .with_priority(Priority::High);
            
        // Schedule workload
        assert!(mem.schedule_workload(workload).is_ok());
        
        // Verify crystal structure optimization
        let stats = mem.optimize_compute_structure().unwrap();
        assert!(stats.efficiency > EFFICIENCY_THRESHOLD);
    }

    #[test]
    fn test_register_crystal_attachment() {
        let mut regs = ShardRegisterFile::new();
        let crystal = Arc::new(RwLock::new(ComputeCrystal::new()));
        
        regs.attach_compute_crystal(crystal);
        assert!(regs.compute_crystal.is_some());
        
        // Verify metrics update
        regs.update_crystal_metrics();
        assert!(regs.crystal_metrics.throughput >= 0.0);
    }
}
