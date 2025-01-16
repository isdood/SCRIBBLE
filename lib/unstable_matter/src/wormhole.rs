/// Quantum Wormhole Implementation
/// Last Updated: 2025-01-16 02:38:44 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    constants::*,
    vector::Vector3D,
    phantom::QuantumCell,
    helium::Helium,
    glitch::WormholeGlitch,
    mesh::MeshCell,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

#[derive(Debug, Clone)]
pub struct WormholeError {
    pub message: String,
    pub glitch: Option<WormholeGlitch>,
}

impl WormholeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: ToString::to_string(message),
            glitch: None,
        }
    }

    pub fn with_glitch(message: &str, glitch: WormholeGlitch) -> Self {
        Self {
            message: ToString::to_string(message),
            glitch: Some(glitch),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WormholeState {
    Opening,
    Stable,
    Closing,
    Collapsed,
    Entangled,
}

#[derive(Debug, Clone)]
pub struct Wormhole {
    entrance: QuantumCell<Vector3D<f64>>,
    exit: QuantumCell<Vector3D<f64>>,
    state: QuantumCell<WormholeState>,
    coherence: Helium<f64>,
    radius: Helium<f64>,
    affected_cells: QuantumCell<Vec<MeshCell>>,
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

    pub fn connect_cell(&mut self, cell: MeshCell) -> Result<(), WormholeGlitch> {
        if !self.is_quantum_stable() {
            return Err(WormholeGlitch::quantum_state_compromised());
        }

        let probability = self.calculate_tunnel_probability(&cell);
        if probability < WORMHOLE_STABILITY_THRESHOLD {
            return Err(WormholeGlitch::stability_failure());
        }

        let mut cells = self.affected_cells.get().clone();
        cells.push(cell);
        self.affected_cells.set(cells);
        self.decay_coherence();

        Ok(())
    }

    pub fn transport(&mut self, mut cell: MeshCell) -> Result<MeshCell, WormholeGlitch> {
        if !self.is_quantum_stable() {
            return Err(WormholeGlitch::quantum_state_compromised());
        }

        if self.get_state() == WormholeState::Collapsed {
            return Err(WormholeGlitch::tunnelling_failed());
        }

        let probability = self.calculate_tunnel_probability(&cell);
        if probability < WORMHOLE_STABILITY_THRESHOLD {
            return Err(WormholeGlitch::stability_failure());
        }

        self.apply_curvature_effects(&mut cell)?;
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
        self.state.get()
    }

    fn calculate_tunnel_probability(&self, cell: &MeshCell) -> f64 {
        let pos = cell.get_position();
        let distance_to_entrance = (pos - self.get_entrance()).magnitude();
        let coherence = self.get_coherence();

        let base_probability = (self.get_radius() - distance_to_entrance) / self.get_radius();
        base_probability * coherence
    }

    fn apply_curvature_effects(&self, cell: &mut MeshCell) -> Result<(), WormholeGlitch> {
        if !self.is_quantum_stable() {
            return Err(WormholeGlitch::quantum_state_compromised());
        }

        let entrance_pos = self.get_entrance();
        let exit_pos = self.get_exit();
        let cell_pos = cell.get_position();

        let distance_ratio = (cell_pos - entrance_pos).magnitude() / self.get_radius();
        if distance_ratio > 1.0 {
            return Err(WormholeGlitch::invalid_destination());
        }

        Ok(())
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence.quantum_load()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_STABILITY_THRESHOLD &&
        self.get_state() != WormholeState::Collapsed
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
        self.timestamp.quantum_store(CURRENT_TIMESTAMP);
    }
}

impl Scribe for Wormhole {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("Wormhole{entrance=");
        self.entrance.get().scribe(precision, output);
        output.push_str(", exit=");
        self.exit.get().scribe(precision, output);
        output.push_str(", r=");
        output.push_f64(self.get_radius(), precision.decimal_places());
        output.push_str(", c=");
        output.push_f64(self.get_coherence(), precision.decimal_places());
        output.push_str(", state=");
        match self.get_state() {
            WormholeState::Opening => output.push_str("opening"),
            WormholeState::Stable => output.push_str("stable"),
            WormholeState::Closing => output.push_str("closing"),
            WormholeState::Collapsed => output.push_str("collapsed"),
            WormholeState::Entangled => output.push_str("entangled"),
        }
        output.push_char('}');
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

    #[test]
    fn test_transport_stability() {
        let mut wormhole = Wormhole::new(
            Vector3D::new(0.0, 0.0, 0.0),
                                         Vector3D::new(10.0, 0.0, 0.0),
                                         5.0
        );

        let cell = MeshCell::new(Vector3D::new(1.0, 0.0, 0.0));
        assert!(wormhole.transport(cell).is_ok());
    }

    #[test]
    fn test_transport_failure() {
        let mut wormhole = Wormhole::new(
            Vector3D::new(0.0, 0.0, 0.0),
                                         Vector3D::new(10.0, 0.0, 0.0),
                                         1.0
        );

        // Force wormhole to collapse
        for _ in 0..100 {
            wormhole.decay_coherence();
        }

        let cell = MeshCell::new(Vector3D::new(1.0, 0.0, 0.0));
        assert!(matches!(
            wormhole.transport(cell),
                         Err(WormholeGlitch::TunnellingFailed)
        ));
    }
}
