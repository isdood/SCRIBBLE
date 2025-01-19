use crate::aether::AetherGrid;

pub struct CrystalMetrics {
    aether: AetherGrid,
    crystal_ops: Vec<CrystalOperation>,
    coherence_history: Vec<f64>,
}

impl CrystalMetrics {
    pub fn new(aether: AetherGrid) -> Self {
        Self {
            aether,
            crystal_ops: Vec::new(),
            coherence_history: Vec::new(),
        }
    }

    #[inline]
    pub fn record_operation(&mut self, op: CrystalOperation) {
        self.crystal_ops.push(op);
        self.coherence_history.push(self.aether.measure_coherence());
    }

    pub fn get_crystal_stability(&self) -> f64 {
        self.aether.calculate_lattice_stability()
    }
}
