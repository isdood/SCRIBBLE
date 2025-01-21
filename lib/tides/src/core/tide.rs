//! Crystal tide and wave motion simulation
//! Created: 2025-01-21 14:00:58 UTC
//! Author: @isdood

use std::{
    sync::Arc,
    collections::VecDeque,
};

use num_complex::Complex64;
use parking_lot::RwLock;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TideError {
    #[error("Invalid wave configuration: {0}")]
    InvalidConfig(String),
    #[error("Tide propagation failed: {0}")]
    PropagationError(String),
    #[error("Flow calculation failed: {0}")]
    FlowError(String),
    #[error("Pattern synchronization failed: {0}")]
    SyncError(String),
}

/// Configuration for crystal tide simulation
#[derive(Debug, Clone)]
pub struct TideConfig {
    pub grid_size: usize,
    pub time_step: f64,
    pub damping: f64,
    pub coupling_strength: f64,
    pub resonance_threshold: f64,
    pub memory_length: usize,
}

impl Default for TideConfig {
    fn default() -> Self {
        Self {
            grid_size: 64,
            time_step: 0.01,
            damping: 0.995,
            coupling_strength: 0.5,
            resonance_threshold: 0.001,
            memory_length: 256,
        }
    }
}

/// Crystal tide simulator
pub struct TideSimulator {
    config: TideConfig,
    state: Arc<RwLock<TideState>>,
    history: VecDeque<TideState>,
    time: f64,
}

/// Current state of the tide simulation
#[derive(Debug, Clone)]
pub struct TideState {
    pub amplitudes: Vec<Vec<Complex64>>,
    pub velocities: Vec<Vec<Complex64>>,
    pub flows: Vec<Vec<FlowVector>>,
    pub energy: f64,
}

/// Flow vector in the crystal lattice
#[derive(Debug, Clone, Copy)]
pub struct FlowVector {
    pub direction: (f64, f64),
    pub magnitude: f64,
    pub phase: f64,
}

impl TideSimulator {
    /// Create new tide simulator with configuration
    pub fn new(config: TideConfig) -> Result<Self, TideError> {
        if config.grid_size == 0 {
            return Err(TideError::InvalidConfig("Grid size must be positive".into()));
        }

        let initial_state = TideState {
            amplitudes: vec![vec![Complex64::new(0.0, 0.0); config.grid_size]; config.grid_size],
            velocities: vec![vec![Complex64::new(0.0, 0.0); config.grid_size]; config.grid_size],
            flows: vec![vec![FlowVector {
                direction: (0.0, 0.0),
                magnitude: 0.0,
                phase: 0.0,
            }; config.grid_size]; config.grid_size],
            energy: 0.0,
        };

        let mut history = VecDeque::with_capacity(config.memory_length);
        history.push_back(initial_state.clone());

        Ok(Self {
            config,
            state: Arc::new(RwLock::new(initial_state)),
           history,
           time: 0.0,
        })
    }

    /// Step the simulation forward
    pub fn step(&mut self) -> Result<(), TideError> {
        let mut state = self.state.write();

        // Calculate new amplitudes and velocities
        let (new_amplitudes, new_velocities) = self.calculate_wave_propagation(&state)?;

        // Update flows
        let new_flows = self.calculate_flows(&new_amplitudes, &new_velocities)?;

        // Calculate system energy
        let energy = self.calculate_energy(&new_amplitudes, &new_velocities);

        // Create new state
        let new_state = TideState {
            amplitudes: new_amplitudes,
            velocities: new_velocities,
            flows: new_flows,
            energy,
        };

        // Update history
        if self.history.len() >= self.config.memory_length {
            self.history.pop_front();
        }
        self.history.push_back(new_state.clone());

        // Update current state
        *state = new_state;
        self.time += self.config.time_step;

        Ok(())
    }

    /// Calculate wave propagation
    fn calculate_wave_propagation(
        &self,
        state: &TideState,
    ) -> Result<(Vec<Vec<Complex64>>, Vec<Vec<Complex64>>), TideError> {
        let size = self.config.grid_size;
        let dt = self.config.time_step;
        let coupling = self.config.coupling_strength;
        let damping = self.config.damping;

        let mut new_amplitudes = vec![vec![Complex64::new(0.0, 0.0); size]; size];
        let mut new_velocities = vec![vec![Complex64::new(0.0, 0.0); size]; size];

        // Parallel computation of wave propagation
        (0..size).into_par_iter().try_for_each(|i| {
            for j in 0..size {
                // Calculate Laplacian
                let laplacian = self.calculate_laplacian(&state.amplitudes, i, j)?;

                // Update velocity using wave equation
                let acceleration = coupling * laplacian;
                new_velocities[i][j] = state.velocities[i][j] * damping + acceleration * dt;

                // Update amplitude
                new_amplitudes[i][j] = state.amplitudes[i][j] + new_velocities[i][j] * dt;
            }
            Ok::<(), TideError>(())
        })?;

        Ok((new_amplitudes, new_velocities))
    }

    /// Calculate Laplacian at a point
    fn calculate_laplacian(
        &self,
        amplitudes: &[Vec<Complex64>],
        i: usize,
        j: usize,
    ) -> Result<Complex64, TideError> {
        let size = self.config.grid_size;
        let mut sum = Complex64::new(0.0, 0.0);
        let center = amplitudes[i][j];

        // Check neighboring points
        let neighbors = [
            (i.wrapping_sub(1), j),
            (i.wrapping_add(1), j),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
        ];

        for (ni, nj) in neighbors.iter() {
            if *ni < size && *nj < size {
                sum += amplitudes[*ni][*nj] - center;
            }
        }

        Ok(sum)
    }

    /// Calculate flow vectors
    fn calculate_flows(
        &self,
        amplitudes: &[Vec<Complex64>],
        velocities: &[Vec<Complex64>],
    ) -> Result<Vec<Vec<FlowVector>>, TideError> {
        let size = self.config.grid_size;
        let mut flows = vec![vec![FlowVector {
            direction: (0.0, 0.0),
            magnitude: 0.0,
            phase: 0.0,
        }; size]; size];

        // Parallel computation of flows
        (0..size).into_par_iter().try_for_each(|i| {
            for j in 0..size {
                flows[i][j] = self.calculate_flow_vector(amplitudes, velocities, i, j)?;
            }
            Ok::<(), TideError>(())
        })?;

        Ok(flows)
    }

    /// Calculate flow vector at a point
    fn calculate_flow_vector(
        &self,
        amplitudes: &[Vec<Complex64>],
        velocities: &[Vec<Complex64>],
        i: usize,
        j: usize,
    ) -> Result<FlowVector, TideError> {
        let size = self.config.grid_size;
        let mut dx = Complex64::new(0.0, 0.0);
        let mut dy = Complex64::new(0.0, 0.0);

        // Calculate gradients
        if i > 0 && i < size - 1 {
            dx = (amplitudes[i+1][j] - amplitudes[i-1][j]) * 0.5;
        }
        if j > 0 && j < size - 1 {
            dy = (amplitudes[i][j+1] - amplitudes[i][j-1]) * 0.5;
        }

        let magnitude = (dx.norm_sqr() + dy.norm_sqr()).sqrt();
        let phase = velocities[i][j].arg();
        let direction = if magnitude > 0.0 {
            (dx.norm() / magnitude, dy.norm() / magnitude)
        } else {
            (0.0, 0.0)
        };

        Ok(FlowVector {
            direction,
            magnitude,
            phase,
        })
    }

    /// Calculate total system energy
    fn calculate_energy(
        &self,
        amplitudes: &[Vec<Complex64>],
        velocities: &[Vec<Complex64>],
    ) -> f64 {
        amplitudes.par_iter().zip(velocities).map(|(amp_row, vel_row)| {
            amp_row.iter().zip(vel_row).map(|(amp, vel)| {
                amp.norm_sqr() + vel.norm_sqr()
            }).sum::<f64>()
        }).sum()
    }

    /// Get current tide state
    pub fn get_state(&self) -> TideState {
        self.state.read().clone()
    }

    /// Get flow pattern at point
    pub fn get_flow(&self, i: usize, j: usize) -> Option<FlowVector> {
        let state = self.state.read();
        state.flows.get(i)?.get(j).cloned()
    }

    /// Check if point is in resonance
    pub fn is_resonant(&self, i: usize, j: usize) -> bool {
        if let Some(state) = self.state.read().flows.get(i) {
            if let Some(flow) = state.get(j) {
                return flow.magnitude >= self.config.resonance_threshold;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_tide_initialization() {
        let config = TideConfig::default();
        let simulator = TideSimulator::new(config).unwrap();
        let state = simulator.get_state();

        assert_eq!(state.amplitudes.len(), 64);
        assert_eq!(state.velocities.len(), 64);
        assert_eq!(state.flows.len(), 64);
        assert_relative_eq!(state.energy, 0.0);
    }

    #[test]
    fn test_wave_propagation() {
        let mut simulator = TideSimulator::new(TideConfig::default()).unwrap();

        // Initialize with a point disturbance
        {
            let mut state = simulator.state.write();
            state.amplitudes[32][32] = Complex64::new(1.0, 0.0);
        }

        // Step simulation
        simulator.step().unwrap();
        let state = simulator.get_state();

        // Check that energy has propagated
        assert!(state.energy > 0.0);
    }
}
