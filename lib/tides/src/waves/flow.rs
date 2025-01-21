//! Crystal flow dynamics and analysis
//! Created: 2025-01-21 15:55:53 UTC
//! Author: @isdood

use std::{
    collections::VecDeque,
    sync::Arc,
};

use crate::{
    julia::{
        flow::{JuliaFlowAnalysis, FlowResult},
        dynamics::{JuliaDynamicsProcessor, DynamicsResult},
    },
    chapel::{
        parallel::{ChapelDomainMap, ChapelArrays},
        flow::{ChapelFlowCompute, FlowMetrics},
    },
    core::wave_pattern::WavePattern,
};

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlowError {
    #[error("Invalid flow configuration: {0}")]
    InvalidConfig(String),
    #[error("Flow computation error: {0}")]
    ComputeError(String),
    #[error("Julia error: {0}")]
    JuliaError(String),
    #[error("Chapel error: {0}")]
    ChapelError(String),
}

/// Configuration for crystal flow analysis
#[derive(Debug, Clone)]
pub struct FlowConfig {
    pub resolution: (usize, usize),
    pub time_step: f64,
    pub viscosity: f64,
    pub boundary_condition: BoundaryCondition,
    pub memory_depth: usize,
    pub julia_threads: usize,
    pub chapel_locales: usize,
    pub compute_backend: ComputeBackend,
}

#[derive(Debug, Clone, Copy)]
pub enum BoundaryCondition {
    Periodic,
    Reflective,
    Absorbing,
}

#[derive(Debug, Clone, Copy)]
pub enum ComputeBackend {
    Julia,
    Chapel,
    Hybrid,
}

impl Default for FlowConfig {
    fn default() -> Self {
        Self {
            resolution: (256, 256),
            time_step: 0.01,
            viscosity: 0.001,
            boundary_condition: BoundaryCondition::Periodic,
            memory_depth: 128,
            julia_threads: 4,
            chapel_locales: 2,
            compute_backend: ComputeBackend::Hybrid,
        }
    }
}

/// Crystal flow analyzer
pub struct CrystalFlow {
    config: FlowConfig,
    state: RwLock<FlowState>,
    history: RwLock<VecDeque<FlowState>>,
    wave_pattern: Arc<WavePattern>,
    julia_flow: JuliaFlowAnalysis,
    julia_dynamics: JuliaDynamicsProcessor,
    chapel_flow: ChapelFlowCompute,
}

/// Flow state information
#[derive(Debug, Clone)]
pub struct FlowState {
    pub time: f64,
    pub velocity_field: Vec<Vec<Complex64>>,
    pub pressure_field: Vec<Vec<f64>>,
    pub vorticity_field: Vec<Vec<f64>>,
    pub energy_density: Vec<Vec<f64>>,
    pub circulation: f64,
    pub reynolds_number: f64,
    pub total_energy: f64,
}

impl CrystalFlow {
    /// Create new crystal flow analyzer
    pub fn new(
        config: FlowConfig,
        wave_pattern: Arc<WavePattern>,
    ) -> Result<Self, FlowError> {
        // Initialize Julia components
        let julia_flow = JuliaFlowAnalysis::new(config.julia_threads)
        .map_err(|e| FlowError::JuliaError(e.to_string()))?;

        let julia_dynamics = JuliaDynamicsProcessor::new(config.julia_threads)
        .map_err(|e| FlowError::JuliaError(e.to_string()))?;

        // Initialize Chapel components
        let chapel_flow = ChapelFlowCompute::new(config.chapel_locales)
        .map_err(|e| FlowError::ChapelError(e.to_string()))?;

        let initial_state = FlowState {
            time: 0.0,
            velocity_field: vec![vec![Complex64::new(0.0, 0.0); config.resolution.1]; config.resolution.0],
            pressure_field: vec![vec![0.0; config.resolution.1]; config.resolution.0],
            vorticity_field: vec![vec![0.0; config.resolution.1]; config.resolution.0],
            energy_density: vec![vec![0.0; config.resolution.1]; config.resolution.0],
            circulation: 0.0,
            reynolds_number: 0.0,
            total_energy: 0.0,
        };

        Ok(Self {
            config,
            state: RwLock::new(initial_state),
           history: RwLock::new(VecDeque::with_capacity(config.memory_depth)),
           wave_pattern,
           julia_flow,
           julia_dynamics,
           chapel_flow,
        })
    }

    /// Update flow analysis
    pub fn update(&self, time: f64) -> Result<(), FlowError> {
        match self.config.compute_backend {
            ComputeBackend::Julia => {
                self.update_with_julia(time)?;
            }
            ComputeBackend::Chapel => {
                self.update_with_chapel(time)?;
            }
            ComputeBackend::Hybrid => {
                self.update_hybrid(time)?;
            }
        }

        self.update_history()?;
        Ok(())
    }

    /// Update using Julia backend
    fn update_with_julia(&self, time: f64) -> Result<(), FlowError> {
        let wave_state = self.wave_pattern.get_state();

        // Compute flow characteristics using Julia
        let flow_result = self.julia_flow
        .analyze_flow(
            &wave_state.amplitudes,
            self.config.time_step,
            self.config.viscosity,
        )
        .map_err(|e| FlowError::JuliaError(e.to_string()))?;

        // Compute dynamics using Julia
        let dynamics_result = self.julia_dynamics
        .compute_dynamics(&flow_result.velocity_field, time)
        .map_err(|e| FlowError::JuliaError(e.to_string()))?;

        // Update state with Julia results
        self.update_state_from_julia(flow_result, dynamics_result, time)?;

        Ok(())
    }

    /// Update using Chapel backend
    fn update_with_chapel(&self, time: f64) -> Result<(), FlowError> {
        let wave_state = self.wave_pattern.get_state();

        // Compute flow metrics using Chapel's parallel capabilities
        let metrics = self.chapel_flow
        .compute_metrics(
            &wave_state.amplitudes,
            self.config.time_step,
            self.config.viscosity,
        )
        .map_err(|e| FlowError::ChapelError(e.to_string()))?;

        // Update state with Chapel results
        self.update_state_from_chapel(metrics, time)?;

        Ok(())
    }

    /// Update using hybrid Julia/Chapel approach
    fn update_hybrid(&self, time: f64) -> Result<(), FlowError> {
        let wave_state = self.wave_pattern.get_state();

        // Parallel computation using both backends
        let (julia_results, chapel_metrics) = rayon::join(
            || {
                let flow_result = self.julia_flow.analyze_flow(
                    &wave_state.amplitudes,
                    self.config.time_step,
                    self.config.viscosity,
                );
                let dynamics_result = flow_result.and_then(|flow| {
                    self.julia_dynamics.compute_dynamics(&flow.velocity_field, time)
                });
                (flow_result, dynamics_result)
            },
            || {
                self.chapel_flow.compute_metrics(
                    &wave_state.amplitudes,
                    self.config.time_step,
                    self.config.viscosity,
                )
            },
        );

        let (flow_result, dynamics_result) = julia_results;
        let flow_result = flow_result.map_err(|e| FlowError::JuliaError(e.to_string()))?;
        let dynamics_result = dynamics_result.map_err(|e| FlowError::JuliaError(e.to_string()))?;
        let chapel_metrics = chapel_metrics.map_err(|e| FlowError::ChapelError(e.to_string()))?;

        // Merge and update results
        self.merge_and_update_results(flow_result, dynamics_result, chapel_metrics, time)?;

        Ok(())
    }

    /// Update state from Julia results
    fn update_state_from_julia(
        &self,
        flow: FlowResult,
        dynamics: DynamicsResult,
        time: f64,
    ) -> Result<(), FlowError> {
        let mut state = self.state.write();
        state.time = time;
        state.velocity_field = flow.velocity_field;
        state.pressure_field = flow.pressure_field;
        state.vorticity_field = flow.vorticity_field;
        state.energy_density = dynamics.energy_density;
        state.circulation = dynamics.circulation;
        state.reynolds_number = dynamics.reynolds_number;
        state.total_energy = dynamics.total_energy;
        Ok(())
    }

    /// Update state from Chapel results
    fn update_state_from_chapel(
        &self,
        metrics: FlowMetrics,
        time: f64,
    ) -> Result<(), FlowError> {
        let mut state = self.state.write();
        state.time = time;
        state.velocity_field = metrics.velocity_field;
        state.pressure_field = metrics.pressure_field;
        state.vorticity_field = metrics.vorticity_field;
        state.energy_density = metrics.energy_density;
        state.circulation = metrics.circulation;
        state.reynolds_number = metrics.reynolds_number;
        state.total_energy = metrics.total_energy;
        Ok(())
    }

    /// Merge and update results from both backends
    fn merge_and_update_results(
        &self,
        flow: FlowResult,
        dynamics: DynamicsResult,
        chapel: FlowMetrics,
        time: f64,
    ) -> Result<(), FlowError> {
        let mut state = self.state.write();
        state.time = time;

        // Use Julia results for high-precision computations
        state.velocity_field = flow.velocity_field;
        state.vorticity_field = flow.vorticity_field;

        // Use Chapel results for parallel-optimized computations
        state.pressure_field = chapel.pressure_field;
        state.energy_density = chapel.energy_density;

        // Average the scalar metrics
        state.circulation = (dynamics.circulation + chapel.circulation) / 2.0;
        state.reynolds_number = (dynamics.reynolds_number + chapel.reynolds_number) / 2.0;
        state.total_energy = (dynamics.total_energy + chapel.total_energy) / 2.0;

        Ok(())
    }

    /// Update flow history
    fn update_history(&self) -> Result<(), FlowError> {
        let mut history = self.history.write();
        let current_state = self.state.read().clone();

        if history.len() >= self.config.memory_depth {
            history.pop_front();
        }
        history.push_back(current_state);

        Ok(())
    }

    /// Get current flow state
    pub fn get_state(&self) -> FlowState {
        self.state.read().clone()
    }

    /// Get flow history
    pub fn get_history(&self) -> Vec<FlowState> {
        self.history.read().iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_flow_initialization() -> Result<(), FlowError> {
        let config = FlowConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let flow = CrystalFlow::new(config.clone(), wave_pattern)?;

        let state = flow.get_state();
        assert_eq!(state.velocity_field.len(), config.resolution.0);
        assert_eq!(state.velocity_field[0].len(), config.resolution.1);
        assert_relative_eq!(state.total_energy, 0.0);
        Ok(())
    }

    #[test]
    fn test_julia_backend() -> Result<(), FlowError> {
        let config = FlowConfig {
            compute_backend: ComputeBackend::Julia,
            ..Default::default()
        };
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let flow = CrystalFlow::new(config, wave_pattern)?;

        flow.update(0.0)?;
        let state = flow.get_state();
        assert!(state.time >= 0.0);
        Ok(())
    }

    #[test]
    fn test_chapel_backend() -> Result<(), FlowError> {
        let config = FlowConfig {
            compute_backend: ComputeBackend::Chapel,
            ..Default::default()
        };
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let flow = CrystalFlow::new(config, wave_pattern)?;

        flow.update(0.0)?;
        let state = flow.get_state();
        assert!(state.time >= 0.0);
        Ok(())
    }

    #[test]
    fn test_hybrid_backend() -> Result<(), FlowError> {
        let config = FlowConfig {
            compute_backend: ComputeBackend::Hybrid,
            ..Default::default()
        };
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let flow = CrystalFlow::new(config, wave_pattern)?;

        flow.update(0.0)?;
        let state = flow.get_state();
        assert!(state.time >= 0.0);
        Ok(())
    }

    #[test]
    fn test_energy_conservation() -> Result<(), FlowError> {
        let config = FlowConfig::default();
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let flow = CrystalFlow::new(config, wave_pattern)?;

        // Initial update
        flow.update(0.0)?;
        let initial_energy = flow.get_state().total_energy;

        // Second update
        flow.update(0.1)?;
        let final_energy = flow.get_state().total_energy;

        // Energy should be conserved or decrease due to viscosity
        assert!(final_energy <= initial_energy);
        Ok(())
    }

    #[test]
    fn test_history_management() -> Result<(), FlowError> {
        let mut config = FlowConfig::default();
        config.memory_depth = 3;
        let wave_pattern = Arc::new(WavePattern::new(Default::default())?);
        let flow = CrystalFlow::new(config, wave_pattern)?;

        for i in 0..5 {
            flow.update(i as f64)?;
        }

        let history = flow.get_history();
        assert!(history.len() <= 3);
        Ok(())
    }
}
