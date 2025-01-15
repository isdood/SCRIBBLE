/// Quantum Wormhole Implementation
/// Last Updated: 2025-01-15 01:58:00 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    vector::Vector3D,
    phantom::QuantumCell,
    helium::Helium,
    mesh::MeshCell,
    glitch::WormholeGlitch,
};

#[derive(Debug)]
pub enum WormholeState {
    Opening,
    Stable,
    Closing,
    Collapsed,
}

#[derive(Debug)]
pub struct Wormhole {
    entrance: QuantumCell<Vector3D<f64>>,
    exit: QuantumCell<Vector3D<f64>>,
    state: QuantumCell<WormholeState>,
    coherence: Helium<f64>,
    radius: Helium<f64>,
    affected_cells: QuantumCell<Vec<MeshCell<f64>>>,
    timestamp: Helium<usize>,
}

impl Wormhole {
    pub fn new(entrance: Vector3D<f64>, exit: Vector3D<f64>, radius: f64) -> Self {
        Self {
            entrance: QuantumCell::new(entrance),
            exit: QuantumCell::new(exit),
            state: QuantumCell::new(WormholeState::Opening),
            coherence: Helium::new(1.0),
            radius: Helium::new(radius),
            affected_cells: QuantumCell::new(Vec::new()),
            timestamp: Helium::new(CURRENT_TIMESTAMP),
        }
    }

    pub fn transport<T>(&mut self, cell: MeshCell<T>) -> Result<MeshCell<T>, WormholeGlitch> {
        if !self.is_quantum_stable() {
            return Err(WormholeGlitch::QuantumStateCompromised);
        }

        if *self.state.get() == WormholeState::Collapsed {
            return Err(WormholeGlitch::TunnellingFailed);
        }

        let probability = self.calculate_tunnel_probability(&cell);
        if probability < WORMHOLE_STABILITY_THRESHOLD {
            return Err(WormholeGlitch::StabilityFailure);
        }

        self.apply_curvature_effects(&cell)?;
        self.decay_coherence();

        Ok(cell)
    }

    pub fn get_entrance(&self) -> Vector3D<f64> {
        self.entrance.get().clone()
    }

    pub fn get_exit(&self) -> Vector3D<f64> {
        self.exit.get().clone()
    }

    pub fn get_radius(&self) -> f64 {
        self.radius.quantum_load()
    }

    pub fn get_state(&self) -> WormholeState {
        self.state.get().clone()
    }

    fn calculate_tunnel_probability<T>(&self, cell: &MeshCell<T>) -> f64 {
        let pos = cell.get_position();
        let distance_to_entrance = (pos - self.get_entrance()).magnitude();
        let coherence = self.get_coherence();

        let base_probability = (self.get_radius() - distance_to_entrance) / self.get_radius();
        base_probability * coherence
    }

    fn apply_curvature_effects<T>(&self, cell: &mut MeshCell<T>) -> Result<(), WormholeGlitch> {
        if !self.is_quantum_stable() {
            return Err(WormholeGlitch::QuantumStateCompromised);
        }

        let entrance_pos = self.get_entrance();
        let exit_pos = self.get_exit();
        let cell_pos = cell.get_position();

        let distance_ratio = (cell_pos - entrance_pos).magnitude() / self.get_radius();
        if distance_ratio > 1.0 {
            return Err(WormholeGlitch::InvalidDestination);
        }

        Ok(())
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence.quantum_load()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD &&
        *self.state.get() != WormholeState::Collapsed
    }

    fn decay_coherence(&self) {
        let current = self.coherence.quantum_load();
        let new_coherence = current * COHERENCE_DECAY_FACTOR;
        self.coherence.quantum_store(new_coherence);

        // Update wormhole state based on coherence
        let new_state = match new_coherence {
            c if c > 0.9 => WormholeState::Stable,
            c if c > WORMHOLE_STABILITY_THRESHOLD => WormholeState::Opening,
            c if c > QUANTUM_STABILITY_THRESHOLD => WormholeState::Closing,
            _ => WormholeState::Collapsed,
        };

        self.state.set(new_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wormhole_creation() {
        let entrance = Vector3D::new(0.0, 0.0, 0.0);
        let exit = Vector3D::new(10.0, 0.0, 0.0);
        let wormhole = Wormhole::new(entrance.clone(), exit.clone(), 1.0);

        assert_eq!(wormhole.get_entrance(), entrance);
        assert_eq!(wormhole.get_exit(), exit);
        assert!(wormhole.is_quantum_stable());
    }

    #[test]
    fn test_coherence_decay() {
        let wormhole = Wormhole::new(
            Vector3D::new(0.0, 0.0, 0.0),
                                     Vector3D::new(10.0, 0.0, 0.0),
                                     1.0
        );

        let initial_coherence = wormhole.get_coherence();
        wormhole.decay_coherence();
        assert!(wormhole.get_coherence() < initial_coherence);
    }

    #[test]
    fn test_state_transition() {
        let wormhole = Wormhole::new(
            Vector3D::new(0.0, 0.0, 0.0),
                                     Vector3D::new(10.0, 0.0, 0.0),
                                     1.0
        );

        // Force multiple decays to trigger state transitions
        for _ in 0..100 {
            wormhole.decay_coherence();
        }

        assert_eq!(wormhole.get_state(), WormholeState::Collapsed);
    }
}
