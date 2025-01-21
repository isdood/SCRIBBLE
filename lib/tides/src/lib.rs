//! Crystal wave simulation and analysis library
//! Created: 2025-01-21 15:59:33 UTC
//! Author: @isdood

pub mod core;
pub mod waves;
pub mod lattice;
pub mod resonance;
pub mod mesh;
pub mod julia;
pub mod chapel;
pub mod analysis;
pub mod utils;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CrystalError {
    #[error("Wave error: {0}")]
    WaveError(#[from] waves::WaveError),

    #[error("Lattice error: {0}")]
    LatticeError(#[from] lattice::LatticeError),

    #[error("Resonance error: {0}")]
    ResonanceError(#[from] resonance::ResonanceError),

    #[error("Mesh error: {0}")]
    MeshError(#[from] mesh::MeshError),

    #[error("Julia computation error: {0}")]
    JuliaError(#[from] julia::JuliaError),

    #[error("Chapel computation error: {0}")]
    ChapelError(#[from] chapel::ChapelError),

    #[error("Analysis error: {0}")]
    AnalysisError(#[from] analysis::AnalysisError),
}

/// Configuration for the crystal simulation
#[derive(Debug, Clone)]
pub struct CrystalConfig {
    pub dimensions: (usize, usize, usize),
    pub time_step: f64,
    pub total_time: f64,
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

impl Default for CrystalConfig {
    fn default() -> Self {
        Self {
            dimensions: (128, 128, 128),
            time_step: 0.01,
            total_time: 10.0,
            julia_threads: 4,
            chapel_locales: 2,
            compute_backend: ComputeBackend::Hybrid,
        }
    }
}

/// Main crystal simulation interface
pub struct Crystal {
    config: CrystalConfig,
    wave_system: waves::WaveSystem,
    lattice: lattice::Lattice,
    resonance: resonance::Resonance,
    mesh: mesh::CrystalMesh,
    analyzer: analysis::Analyzer,
}

impl Crystal {
    /// Create new crystal simulation
    pub fn new(config: CrystalConfig) -> Result<Self, CrystalError> {
        // Initialize Julia runtime
        julia::init_julia(config.julia_threads)?;

        // Initialize Chapel runtime
        chapel::init_chapel(config.chapel_locales)?;

        // Create wave system
        let wave_config = waves::WaveConfig {
            dimensions: config.dimensions,
            time_step: config.time_step,
            compute_backend: config.compute_backend,
            ..Default::default()
        };
        let wave_system = waves::WaveSystem::new(wave_config)?;

        // Create lattice
        let lattice_config = lattice::LatticeConfig {
            dimensions: config.dimensions,
            compute_backend: config.compute_backend,
            ..Default::default()
        };
        let lattice = lattice::Lattice::new(lattice_config)?;

        // Create resonance system
        let resonance_config = resonance::ResonanceConfig {
            compute_backend: config.compute_backend,
            ..Default::default()
        };
        let resonance = resonance::Resonance::new(resonance_config)?;

        // Create mesh
        let mesh_config = mesh::MeshConfig {
            resolution: config.dimensions,
            compute_backend: config.compute_backend,
            ..Default::default()
        };
        let mesh = mesh::CrystalMesh::new(mesh_config, wave_system.get_pattern())?;

        // Create analyzer
        let analyzer = analysis::Analyzer::new(config.compute_backend)?;

        Ok(Self {
            config,
            wave_system,
            lattice,
            resonance,
            mesh,
            analyzer,
        })
    }

    /// Initialize the crystal simulation
    pub fn initialize(&mut self) -> Result<(), CrystalError> {
        // Initialize wave patterns
        self.wave_system.initialize()?;

        // Initialize lattice structure
        self.lattice.initialize()?;

        // Generate mesh
        self.mesh.generate()?;

        // Initialize analysis
        self.analyzer.initialize()?;

        Ok(())
    }

    /// Run simulation step
    pub fn step(&mut self) -> Result<(), CrystalError> {
        // Update wave system
        self.wave_system.update()?;

        // Update lattice
        self.lattice.update()?;

        // Update resonance
        self.resonance.update(
            self.lattice.get_nodes(),
                              self.wave_system.get_pattern(),
        )?;

        // Analyze current state
        self.analyzer.analyze(
            &self.wave_system,
            &self.lattice,
            &self.resonance,
            &self.mesh,
        )?;

        Ok(())
    }

    /// Run simulation for specified time
    pub fn run(&mut self) -> Result<(), CrystalError> {
        let steps = (self.config.total_time / self.config.time_step) as usize;

        self.initialize()?;

        for _ in 0..steps {
            self.step()?;
        }

        Ok(())
    }

    /// Get current simulation state
    pub fn get_state(&self) -> Result<SimulationState, CrystalError> {
        Ok(SimulationState {
            wave_state: self.wave_system.get_state()?,
           lattice_state: self.lattice.get_state()?,
           resonance_state: self.resonance.get_state()?,
           mesh_state: self.mesh.get_state(),
           analysis: self.analyzer.get_results()?,
        })
    }
}

/// Complete simulation state
#[derive(Debug, Clone)]
pub struct SimulationState {
    pub wave_state: waves::WaveState,
    pub lattice_state: lattice::LatticeState,
    pub resonance_state: resonance::ResonanceState,
    pub mesh_state: mesh::MeshState,
    pub analysis: analysis::AnalysisResults,
}

impl Drop for Crystal {
    fn drop(&mut self) {
        // Cleanup Julia runtime
        if let Err(e) = julia::cleanup_julia() {
            eprintln!("Error cleaning up Julia runtime: {}", e);
        }

        // Cleanup Chapel runtime
        if let Err(e) = chapel::cleanup_chapel() {
            eprintln!("Error cleaning up Chapel runtime: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_initialization() -> Result<(), CrystalError> {
        let config = CrystalConfig::default();
        let mut crystal = Crystal::new(config)?;
        crystal.initialize()?;
        Ok(())
    }

    #[test]
    fn test_simulation_step() -> Result<(), CrystalError> {
        let config = CrystalConfig::default();
        let mut crystal = Crystal::new(config)?;
        crystal.initialize()?;
        crystal.step()?;
        Ok(())
    }

    #[test]
    fn test_julia_backend() -> Result<(), CrystalError> {
        let config = CrystalConfig {
            compute_backend: ComputeBackend::Julia,
            dimensions: (32, 32, 32), // Smaller for testing
            ..Default::default()
        };
        let mut crystal = Crystal::new(config)?;
        crystal.initialize()?;
        crystal.step()?;
        Ok(())
    }

    #[test]
    fn test_chapel_backend() -> Result<(), CrystalError> {
        let config = CrystalConfig {
            compute_backend: ComputeBackend::Chapel,
            dimensions: (32, 32, 32), // Smaller for testing
            ..Default::default()
        };
        let mut crystal = Crystal::new(config)?;
        crystal.initialize()?;
        crystal.step()?;
        Ok(())
    }

    #[test]
    fn test_hybrid_backend() -> Result<(), CrystalError> {
        let config = CrystalConfig {
            compute_backend: ComputeBackend::Hybrid,
            dimensions: (32, 32, 32), // Smaller for testing
            ..Default::default()
        };
        let mut crystal = Crystal::new(config)?;
        crystal.initialize()?;
        crystal.step()?;
        Ok(())
    }

    #[test]
    fn test_short_simulation() -> Result<(), CrystalError> {
        let config = CrystalConfig {
            dimensions: (32, 32, 32), // Smaller for testing
            total_time: 0.1,
            time_step: 0.01,
            ..Default::default()
        };
        let mut crystal = Crystal::new(config)?;
        crystal.run()?;
        Ok(())
    }

    #[test]
    fn test_state_access() -> Result<(), CrystalError> {
        let config = CrystalConfig {
            dimensions: (32, 32, 32), // Smaller for testing
            ..Default::default()
        };
        let mut crystal = Crystal::new(config)?;
        crystal.initialize()?;
        let state = crystal.get_state()?;
        assert!(!state.wave_state.amplitudes.is_empty());
        Ok(())
    }
}
