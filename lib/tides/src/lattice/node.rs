//! Crystal lattice node representation and manipulation with Julia/Chapel integration
//! Created: 2025-01-21 16:16:22 UTC
//! Author: @isdood

// Previous imports...
use crate::{
    julia::{
        nodes::{JuliaNodeProcessor, NodeResult},
        dynamics::{JuliaDynamicsProcessor, DynamicsResult},
    },
    chapel::{
        parallel::{ChapelDomainMap, ChapelParallelNode},
        dynamics::{ChapelNodeDynamics, DynamicsOutput},
    },
};

// Previous error enum and NodeCounter...

#[derive(Debug, Clone)]
pub struct NodeConfig {
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

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            // Previous defaults...
            julia_threads: 4,
            chapel_locales: 2,
            compute_backend: ComputeBackend::Hybrid,
        }
    }
}

pub struct LatticeNode {
    // Previous fields...
    julia_processor: JuliaNodeProcessor,
    julia_dynamics: JuliaDynamicsProcessor,
    chapel_node: ChapelParallelNode,
    chapel_dynamics: ChapelNodeDynamics,
}

impl LatticeNode {
    pub fn new(config: NodeConfig, position: [f64; 3]) -> Result<Self, NodeError> {
        let id = NODE_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Initialize Julia components
        let julia_processor = JuliaNodeProcessor::new(config.julia_threads)
        .map_err(|e| NodeError::InvalidConfig(e.to_string()))?;

        let julia_dynamics = JuliaDynamicsProcessor::new(config.julia_threads)
        .map_err(|e| NodeError::InvalidConfig(e.to_string()))?;

        // Initialize Chapel components
        let chapel_node = ChapelParallelNode::new(config.chapel_locales)
        .map_err(|e| NodeError::InvalidConfig(e.to_string()))?;

        let chapel_dynamics = ChapelNodeDynamics::new(config.chapel_locales)
        .map_err(|e| NodeError::InvalidConfig(e.to_string()))?;

        Ok(Self {
            id,
            config,
            state: RwLock::new(NodeState::default_at_position(position)),
           connections: RwLock::new(HashSet::new()),
           julia_processor,
           julia_dynamics,
           chapel_node,
           chapel_dynamics,
        })
    }

    pub fn update(&self, time: f64) -> Result<(), NodeError> {
        match self.config.compute_backend {
            ComputeBackend::Julia => self.update_with_julia(time)?,
            ComputeBackend::Chapel => self.update_with_chapel(time)?,
            ComputeBackend::Hybrid => self.update_hybrid(time)?,
        }
        Ok(())
    }

    fn update_with_julia(&self, time: f64) -> Result<(), NodeError> {
        let state = self.state.read();
        let connections = self.connections.read();

        // Process node state using Julia
        let node_result = self.julia_processor
        .process_state(&state, &connections, time)
        .map_err(|e| NodeError::StateError(e.to_string()))?;

        // Process dynamics using Julia
        let dynamics_result = self.julia_dynamics
        .compute_dynamics(&node_result, self.config.damping_factor)
        .map_err(|e| NodeError::StateError(e.to_string()))?;

        // Update state with Julia results
        self.update_state_from_julia(node_result, dynamics_result)?;

        Ok(())
    }

    fn update_with_chapel(&self, time: f64) -> Result<(), NodeError> {
        let state = self.state.read();
        let connections = self.connections.read();

        // Process node state using Chapel
        let node_result = self.chapel_node
        .process_parallel(&state, &connections, time)
        .map_err(|e| NodeError::StateError(e.to_string()))?;

        // Process dynamics using Chapel
        let dynamics_result = self.chapel_dynamics
        .compute_parallel(&node_result, self.config.damping_factor)
        .map_err(|e| NodeError::StateError(e.to_string()))?;

        // Update state with Chapel results
        self.update_state_from_chapel(node_result, dynamics_result)?;

        Ok(())
    }

    fn update_hybrid(&self, time: f64) -> Result<(), NodeError> {
        let state = self.state.read();
        let connections = self.connections.read();

        // Parallel computation using both backends
        let (julia_results, chapel_results) = rayon::join(
            || {
                let node_result = self.julia_processor.process_state(
                    &state,
                    &connections,
                    time,
                );
                let dynamics_result = node_result.and_then(|res| {
                    self.julia_dynamics.compute_dynamics(&res, self.config.damping_factor)
                });
                (node_result, dynamics_result)
            },
            || {
                let node_result = self.chapel_node.process_parallel(
                    &state,
                    &connections,
                    time,
                );
                let dynamics_result = node_result.and_then(|res| {
                    self.chapel_dynamics.compute_parallel(&res, self.config.damping_factor)
                });
                (node_result, dynamics_result)
            },
        );

        // Merge and update results from both backends
        let (julia_node, julia_dynamics) = julia_results.0.and_then(|n| {
            julia_results.1.map(|d| (n, d))
        }).map_err(|e| NodeError::StateError(e.to_string()))?;

        let (chapel_node, chapel_dynamics) = chapel_results.0.and_then(|n| {
            chapel_results.1.map(|d| (n, d))
        }).map_err(|e| NodeError::StateError(e.to_string()))?;

        // Merge results
        self.merge_and_update_results(
            julia_node,
            julia_dynamics,
            chapel_node,
            chapel_dynamics,
        )?;

        Ok(())
    }

    fn update_state_from_julia(
        &self,
        node_result: NodeResult,
        dynamics: DynamicsResult,
    ) -> Result<(), NodeError> {
        let mut state = self.state.write();

        state.amplitude = node_result.amplitude;
        state.phase = node_result.phase;
        state.frequency = node_result.frequency;
        state.energy = dynamics.energy;
        state.stability = dynamics.stability;

        Ok(())
    }

    fn update_state_from_chapel(
        &self,
        node_result: NodeState,
        dynamics: DynamicsOutput,
    ) -> Result<(), NodeError> {
        let mut state = self.state.write();

        state.amplitude = node_result.amplitude;
        state.phase = node_result.phase;
        state.frequency = node_result.frequency;
        state.energy = dynamics.energy;
        state.stability = dynamics.stability;

        Ok(())
    }

    fn merge_and_update_results(
        &self,
        julia_node: NodeResult,
        julia_dynamics: DynamicsResult,
        chapel_node: NodeState,
        chapel_dynamics: DynamicsOutput,
    ) -> Result<(), NodeError> {
        let mut state = self.state.write();

        // Average results from both backends
        state.amplitude = (julia_node.amplitude + chapel_node.amplitude) / Complex64::new(2.0, 0.0);
        state.phase = (julia_node.phase + chapel_node.phase) / 2.0;
        state.frequency = (julia_node.frequency + chapel_node.frequency) / 2.0;
        state.energy = (julia_dynamics.energy + chapel_dynamics.energy) / 2.0;
        state.stability = (julia_dynamics.stability + chapel_dynamics.stability) / 2.0;

        Ok(())
    }

    // Previous methods remain unchanged...
}

// Update tests to include backend-specific testing...
#[cfg(test)]
mod tests {
    // Previous tests...

    #[test]
    fn test_julia_backend() -> Result<(), NodeError> {
        let config = NodeConfig {
            compute_backend: ComputeBackend::Julia,
            julia_threads: 2,
            ..Default::default()
        };
        let node = LatticeNode::new(config, [0.0, 0.0, 0.0])?;
        node.update(0.1)?;
        Ok(())
    }

    #[test]
    fn test_chapel_backend() -> Result<(), NodeError> {
        let config = NodeConfig {
            compute_backend: ComputeBackend::Chapel,
            chapel_locales: 2,
            ..Default::default()
        };
        let node = LatticeNode::new(config, [0.0, 0.0, 0.0])?;
        node.update(0.1)?;
        Ok(())
    }

    #[test]
    fn test_hybrid_backend() -> Result<(), NodeError> {
        let config = NodeConfig {
            compute_backend: ComputeBackend::Hybrid,
            julia_threads: 2,
            chapel_locales: 2,
            ..Default::default()
        };
        let node = LatticeNode::new(config, [0.0, 0.0, 0.0])?;
        node.update(0.1)?;
        Ok(())
    }
}
