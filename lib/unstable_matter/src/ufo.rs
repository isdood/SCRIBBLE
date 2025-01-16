/// UFO Protection Module
/// Last Updated: 2025-01-16 03:14:17 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    quantum::Quantum,
    phantom::{PhantomSpace, QuantumCell},
    vector::Vector3D,
    unstable::UnstableDescriptor,
    scribe::{Scribe, ScribePrecision, QuantumString},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UFOState {
    Landed,
    Hovering,
    Warping,
    Entangled,
    Decoherent,
}

/// UFO Protection trait with quantum awareness
pub trait Protected: Quantum {
    fn protect(&self) -> Result<(), &'static str>;
    fn unprotect(&self) -> Result<(), &'static str>;
}

/// Quantum memory trace for UFO tracking
#[derive(Debug)]
pub struct MemoryTrace {
    phantom_space: PhantomSpace,
    owner: QuantumCell<&'static str>,
}

impl MemoryTrace {
    pub fn new(owner: &'static str) -> Self {
        Self {
            phantom_space: PhantomSpace::new(),
            owner: QuantumCell::new(owner),
        }
    }

    pub fn get_owner(&self) -> &'static str {
        self.owner.get()
    }
}

impl Quantum for MemoryTrace {
    fn get_coherence(&self) -> f64 {
        self.phantom_space.get_coherence()
    }

    fn is_quantum_stable(&self) -> bool {
        self.phantom_space.is_quantum_stable()
    }

    fn decay_coherence(&self) {
        self.phantom_space.decay_coherence();
    }

    fn reset_coherence(&self) {
        self.phantom_space.reset_coherence();
    }
}

impl Scribe for MemoryTrace {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("MemoryTrace[owner=");
        output.push_str(self.get_owner());
        output.push_str(", coherence=");
        output.push_f64(self.get_coherence(), precision.decimal_places());
        output.push_char(']');
    }
}

/// Base UFO structure with quantum mechanics
#[derive(Debug)]
pub struct UFO {
    phantom_space: PhantomSpace,
    trace: MemoryTrace,
    state: QuantumCell<UFOState>,
    warp_factor: QuantumCell<f64>,
    quantum_descriptor: UnstableDescriptor,
    position: QuantumCell<Vector3D<f64>>,
}

impl UFO {
    pub fn new() -> Self {
        Self {
            phantom_space: PhantomSpace::new(),
            trace: MemoryTrace::new("isdood"),
            state: QuantumCell::new(UFOState::Landed),
            warp_factor: QuantumCell::new(1.0),
            quantum_descriptor: UnstableDescriptor::new(),
            position: QuantumCell::new(Vector3D::new(0.0, 0.0, 0.0)),
        }
    }

    pub fn track(&mut self) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("UFO quantum state unstable");
        }
        self.state.set(UFOState::Hovering);
        Ok(())
    }

    pub fn untrack(&mut self) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("UFO quantum state unstable");
        }
        self.state.set(UFOState::Landed);
        Ok(())
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("UFO quantum state unstable");
        }
        let pos = Vector3D::new(x, y, z);
        self.position.set(pos);
        Ok(())
    }

    pub fn get_position(&self) -> Option<Vector3D<f64>> {
        if self.is_quantum_stable() {
            Some(self.position.get())
        } else {
            None
        }
    }

    pub fn get_state(&self) -> UFOState {
        self.state.get()
    }

    pub fn get_warp_factor(&self) -> f64 {
        self.warp_factor.get()
    }

    pub fn is_stable(&self) -> bool {
        self.quantum_descriptor.is_stable()
    }
}

impl Quantum for UFO {
    fn get_coherence(&self) -> f64 {
        (self.phantom_space.get_coherence() +
        self.quantum_descriptor.get_coherence()) / 2.0
    }

    fn is_quantum_stable(&self) -> bool {
        self.phantom_space.is_quantum_stable() &&
        self.quantum_descriptor.is_stable()
    }

    fn decay_coherence(&self) {
        self.phantom_space.decay_coherence();
    }

    fn reset_coherence(&self) {
        self.phantom_space.reset_coherence();
    }
}

impl Protected for UFO {
    fn protect(&self) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Cannot protect unstable UFO");
        }
        Ok(())
    }

    fn unprotect(&self) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Cannot unprotect unstable UFO");
        }
        Ok(())
    }
}

impl Scribe for UFO {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("UFO[");
        self.phantom_space.scribe(precision, output);
        output.push_str(", state=");
        match self.get_state() {
            UFOState::Landed => output.push_str("Landed"),
            UFOState::Hovering => output.push_str("Hovering"),
            UFOState::Warping => output.push_str("Warping"),
            UFOState::Entangled => output.push_str("Entangled"),
            UFOState::Decoherent => output.push_str("Decoherent"),
        }
        output.push_str(", warp=");
        output.push_f64(self.get_warp_factor(), precision.decimal_places());
        output.push_char(']');
    }
}

#[derive(Debug)]
pub struct TrackedUFO {
    base: UFO,
    origin: Vector3D<isize>,
    boundary: Vector3D<isize>,
}

impl TrackedUFO {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self {
            base: UFO::new(),
            origin: Vector3D::new(x, y, z),
            boundary: Vector3D::new(x + 0x1000, y + 0x1000, z + 0x1000),
        }
    }

    pub fn contains(&self, pos: &Vector3D<isize>) -> bool {
        let px = pos.x();
        let py = pos.y();
        let pz = pos.z();
        let ox = self.origin.x();
        let oy = self.origin.y();
        let oz = self.origin.z();
        let bx = self.boundary.x();
        let by = self.boundary.y();
        let bz = self.boundary.z();

        px >= ox && px < bx &&
        py >= oy && py < by &&
        pz >= oz && pz < bz
    }
}

impl Quantum for TrackedUFO {
    fn get_coherence(&self) -> f64 {
        self.base.get_coherence()
    }

    fn is_quantum_stable(&self) -> bool {
        self.base.is_quantum_stable()
    }

    fn decay_coherence(&self) {
        self.base.decay_coherence();
    }

    fn reset_coherence(&self) {
        self.base.reset_coherence();
    }
}

impl Protected for TrackedUFO {
    fn protect(&self) -> Result<(), &'static str> {
        self.base.protect()
    }

    fn unprotect(&self) -> Result<(), &'static str> {
        self.base.unprotect()
    }
}

impl Scribe for TrackedUFO {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("Tracked");
        self.base.scribe(precision, output);
    }
}
