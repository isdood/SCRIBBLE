// lib/unstable_matter/src/vector_space.rs
/// Vector Space Implementation
/// Last Updated: 2025-01-13 04:13:43 UTC
/// Author: isdood
/// Current User: isdood

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    space_config::SpaceMetadata,
    tracked_ufo::TrackedUFO,
    morph_tracker::MorphTracker,
    UFOState,
    Vector3D,
};

#[derive(Debug)]
pub struct VectorSpace {
    origin: AtomicUsize,
    ufo_state: TrackedUFO,
    metadata: SpaceMetadata,
    morph_tracker: MorphTracker,
    state: UFOState,
    timestamp: AtomicUsize,
}

impl Clone for VectorSpace {
    fn clone(&self) -> Self {
        Self {
            origin: AtomicUsize::new(self.get_origin()),
            ufo_state: self.ufo_state.clone(),
            metadata: self.metadata.clone(),
            morph_tracker: self.morph_tracker.clone(),
            state: self.state,
            timestamp: AtomicUsize::new(1705115623), // 2025-01-13 04:13:43 UTC
        }
    }
}

impl VectorSpace {
    pub fn new(origin: usize, metadata: SpaceMetadata) -> Self {
        Self {
            origin: AtomicUsize::new(origin),
            ufo_state: TrackedUFO::new(origin, metadata.get_size()),
            metadata,
            morph_tracker: MorphTracker::new(),
            state: UFOState::Flying,
            timestamp: AtomicUsize::new(1705115623), // 2025-01-13 04:13:43 UTC
        }
    }

    pub fn get_origin(&self) -> usize {
        self.origin.load(Ordering::SeqCst)
    }

    pub fn get_ufo_state(&self) -> &TrackedUFO {
        &self.ufo_state
    }

    pub fn get_metadata(&self) -> &SpaceMetadata {
        &self.metadata
    }

    pub fn get_morph_tracker(&self) -> &MorphTracker {
        &self.morph_tracker
    }

    pub fn get_state(&self) -> UFOState {
        self.state
    }

    pub fn update_origin(&self, new_origin: usize) {
        self.origin.store(new_origin, Ordering::SeqCst);
        self.ufo_state.update_origin(new_origin);
        self.timestamp.store(1705115623, Ordering::SeqCst); // 2025-01-13 04:13:43 UTC
    }

    pub fn transition_state(&mut self, new_state: UFOState) {
        self.state = new_state;
        self.timestamp.store(1705115623, Ordering::SeqCst); // 2025-01-13 04:13:43 UTC
    }

    pub fn is_valid_address(&self, addr: usize) -> bool {
        self.ufo_state.is_within_bounds(addr)
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_space() {
        let metadata = SpaceMetadata::new(0x1000);
        let space = VectorSpace::new(0x1000, metadata);
        let cloned_space = space.clone();

        assert_eq!(space.get_origin(), cloned_space.get_origin());
        assert_eq!(space.get_metadata().get_size(), cloned_space.get_metadata().get_size());
        assert_eq!(space.get_state(), cloned_space.get_state());
    }

    #[test]
    fn test_vector_space_state_transition() {
        let metadata = SpaceMetadata::new(0x1000);
        let mut space = VectorSpace::new(0x1000, metadata);

        assert_eq!(space.get_state(), UFOState::Flying);
        space.transition_state(UFOState::Hovering);
        assert_eq!(space.get_state(), UFOState::Hovering);
    }

    #[test]
    fn test_vector_space_address_validation() {
        let metadata = SpaceMetadata::new(0x1000);
        let space = VectorSpace::new(0x1000, metadata);

        assert!(space.is_valid_address(0x1500));
        assert!(!space.is_valid_address(0x500));
        assert!(!space.is_valid_address(0x2001));
    }

    #[test]
    fn test_vector_space_timestamp() {
        let metadata = SpaceMetadata::new(0x1000);
        let mut space = VectorSpace::new(0x1000, metadata);
        let initial_timestamp = space.get_timestamp();
        space.transition_state(UFOState::Hovering);
        assert!(space.get_timestamp() >= initial_timestamp);
    }
}
