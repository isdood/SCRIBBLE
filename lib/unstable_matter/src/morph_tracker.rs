/// Quantum Morphological Tracking System
/// Last Updated: 2025-01-14 21:35:03 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    vector::FloatVector3D,
    helium::{Helium, HeliumOrdering},
    phantom::QuantumCell,
    constants::CURRENT_TIMESTAMP,
};

const QUANTUM_COHERENCE_THRESHOLD: f64 = 0.5;
const MAX_MARKERS: usize = 6;

/// Represents different types of files that can be morphed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Rust,
    Quantum,
    Entangled,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeMarker {
    pub position: FloatVector3D,
    pub marker_type: MarkerType,
    pub coherence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerType {
    Point,
    QuantumNode,
    EntanglementPoint,
}

#[derive(Debug)]
pub struct MorphTracker {
    timestamp: Helium<usize>,
    modifier: QuantumCell<&'static str>,
    markers: QuantumCell<[Option<EdgeMarker>; MAX_MARKERS]>,
    file_type: QuantumCell<FileType>,
    coherence: Helium<f64>,
}

impl EdgeMarker {
    pub const fn new() -> Self {
        Self {
            position: FloatVector3D::new(0.0, 0.0, 0.0),
            marker_type: MarkerType::Point,
            coherence: 1.0,
        }
    }

    pub fn with_type(marker_type: MarkerType) -> Self {
        Self {
            position: FloatVector3D::new(0.0, 0.0, 0.0),
            marker_type,
            coherence: 1.0,
        }
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.coherence > QUANTUM_COHERENCE_THRESHOLD
    }
}

impl Clone for MorphTracker {
    fn clone(&self) -> Self {
        Self {
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            modifier: QuantumCell::new(*self.modifier.get()),
            markers: QuantumCell::new(*self.markers.get()),
            file_type: QuantumCell::new(*self.file_type.get()),
            coherence: Helium::new(self.get_coherence()),
        }
    }
}

impl MorphTracker {
    pub const fn new() -> Self {
        const EMPTY_MARKER: Option<EdgeMarker> = None;
        Self {
            timestamp: Helium::new(CURRENT_TIMESTAMP),
            modifier: QuantumCell::new("isdood"),
            markers: QuantumCell::new([EMPTY_MARKER; MAX_MARKERS]),
            file_type: QuantumCell::new(FileType::Rust),
            coherence: Helium::new(1.0),
        }
    }

    pub fn create_morph_type(&self) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        match *self.file_type.get() {
            FileType::Rust => self.create_rust_morph_type(),
            FileType::Quantum => self.create_quantum_morph_type(),
            FileType::Entangled => self.create_entangled_morph_type(),
            FileType::Other => Err("Unsupported file type"),
        }
    }

    fn create_rust_morph_type(&self) -> Result<(), &'static str> {
        self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
        self.decay_coherence();
        Ok(())
    }

    fn create_quantum_morph_type(&self) -> Result<(), &'static str> {
        self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
        self.decay_coherence();
        Ok(())
    }

    fn create_entangled_morph_type(&self) -> Result<(), &'static str> {
        if self.get_coherence() < 0.8 {
            return Err("Insufficient quantum coherence for entanglement");
        }
        self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
        self.decay_coherence();
        Ok(())
    }

    pub fn set_file_type(&mut self, file_type: FileType) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        self.file_type.set(file_type);
        self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
        self.decay_coherence();
        Ok(())
    }

    pub fn get_file_type(&self) -> FileType {
        *self.file_type.get()
    }

    pub fn register_file_type(&mut self, file_type: FileType) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        self.file_type.set(file_type);
        self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
        self.decay_coherence();
        Ok(())
    }

    pub fn get_marker(&self, index: usize) -> Option<EdgeMarker> {
        if !self.is_quantum_stable() {
            return None;
        }

        if index < MAX_MARKERS {
            self.markers.get()[index]
        } else {
            None
        }
    }

    pub fn set_marker(&mut self, index: usize, marker: EdgeMarker) -> Result<(), &'static str> {
        if !self.is_quantum_stable() {
            return Err("Quantum state unstable");
        }

        if index < MAX_MARKERS {
            let mut current_markers = *self.markers.get();
            current_markers[index] = Some(marker);
            self.markers.set(current_markers);
            self.timestamp.store(CURRENT_TIMESTAMP, HeliumOrdering::Release);
            self.decay_coherence();
            Ok(())
        } else {
            Err("Marker index out of bounds")
        }
    }

    pub fn get_modifier(&self) -> &'static str {
        *self.modifier.get()
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(HeliumOrdering::Relaxed)
    }

    pub fn get_coherence(&self) -> f64 {
        self.coherence.load(HeliumOrdering::Relaxed)
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.get_coherence() > QUANTUM_COHERENCE_THRESHOLD
    }

    fn decay_coherence(&self) {
        let current = self.coherence.load(HeliumOrdering::Acquire);
        self.coherence.store(current * 0.99, HeliumOrdering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morph_tracker_creation() {
        let tracker = MorphTracker::new();
        assert_eq!(tracker.get_file_type(), FileType::Rust);
        assert_eq!(tracker.get_modifier(), "isdood");
        assert!(tracker.is_quantum_stable());
    }

    #[test]
    fn test_morph_tracker_clone() {
        let tracker = MorphTracker::new();
        let cloned = tracker.clone();
        assert_eq!(tracker.get_file_type(), cloned.get_file_type());
        assert_eq!(tracker.get_coherence(), cloned.get_coherence());
    }

    #[test]
    fn test_file_type_change() {
        let mut tracker = MorphTracker::new();
        assert!(tracker.set_file_type(FileType::Quantum).is_ok());
        assert_eq!(tracker.get_file_type(), FileType::Quantum);
    }

    #[test]
    fn test_quantum_stability() {
        let mut tracker = MorphTracker::new();

        // Force decoherence
        for _ in 0..100 {
            let _ = tracker.set_file_type(FileType::Quantum);
        }

        assert!(!tracker.is_quantum_stable());
        assert!(tracker.set_file_type(FileType::Quantum).is_err());
    }

    #[test]
    fn test_marker_operations() {
        let mut tracker = MorphTracker::new();
        let marker = EdgeMarker::with_type(MarkerType::QuantumNode);

        assert!(tracker.set_marker(0, marker).is_ok());
        assert!(tracker.get_marker(0).is_some());
        assert!(tracker.get_marker(MAX_MARKERS).is_none());
        assert!(tracker.set_marker(MAX_MARKERS, marker).is_err());
    }

    #[test]
    fn test_entangled_morph_type() {
        let tracker = MorphTracker::new();
        tracker.file_type.set(FileType::Entangled);
        assert!(tracker.create_morph_type().is_ok());

        // Force decoherence
        for _ in 0..50 {
            tracker.decay_coherence();
        }

        assert!(tracker.create_morph_type().is_err());
    }
}
