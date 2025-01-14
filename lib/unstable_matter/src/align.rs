// lib/unstable_matter/src/align.rs
/// Memory Alignment Module for Vector Space
/// Last Updated: 2025-01-14 06:01:15 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    vector::Vector3D,
    phantom::PhantomSpace,
    VECTOR_ALIGN,
    CACHE_LINE,
};

#[derive(Debug, Clone, Copy)]
pub struct Alignment {
    value: usize,
    phantom_space: PhantomSpace<usize>,
}

#[derive(Debug)]
pub struct AlignedSpace {
    base: usize,
    size: usize,
    alignment: Alignment,
    phantom_space: PhantomSpace<usize>,
}

impl Alignment {
    pub fn new(value: usize) -> Self {
        assert!(value.is_power_of_two(), "Alignment must be a power of 2");
        let mut alignment = Self {
            value,
            phantom_space: PhantomSpace::new(),
        };
        alignment.phantom_space.set_position(value as isize, 0, 0);
        alignment
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn align_address(&self, addr: usize) -> usize {
        (addr + self.value - 1) & !(self.value - 1)
    }

    pub fn get_coherence(&self) -> f64 {
        self.phantom_space.get_coherence()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.phantom_space.is_quantum_stable()
    }
}

impl AlignedSpace {
    pub fn new(base: usize, size: usize, alignment: Alignment) -> Self {
        let aligned_base = alignment.align_address(base);
        let mut space = Self {
            base: aligned_base,
            size,
            alignment,
            phantom_space: PhantomSpace::new(),
        };
        space.phantom_space.set_position(
            aligned_base as isize,
            aligned_base as isize,
            aligned_base as isize
        );
        space
    }

    pub fn get_base(&self) -> usize {
        self.base
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_alignment(&self) -> &Alignment {
        &self.alignment
    }

    pub fn get_vector_position(&self) -> Vector3D<isize> {
        self.phantom_space.get_position()
    }

    pub fn get_coherence(&self) -> f64 {
        (self.phantom_space.get_coherence() +
        self.alignment.get_coherence()) / 2.0
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.phantom_space.is_quantum_stable() &&
        self.alignment.is_quantum_stable()
    }

    pub fn decay_coherence(&mut self) {
        self.phantom_space.decay_coherence();
    }

    pub fn reset_coherence(&mut self) {
        self.phantom_space.reset_coherence();
    }

    pub fn realign(&mut self) {
        let new_base = self.alignment.align_address(self.base);
        if new_base != self.base {
            self.base = new_base;
            self.phantom_space.set_position(
                new_base as isize,
                new_base as isize,
                new_base as isize
            );
        }
    }
}

pub fn vector_align() -> Alignment {
    Alignment::new(VECTOR_ALIGN)
}

pub fn cache_align() -> Alignment {
    Alignment::new(CACHE_LINE)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TIMESTAMP: usize = 1705207275; // 2025-01-14 06:01:15 UTC

    #[test]
    fn test_alignment() {
        let align = Alignment::new(16);
        assert_eq!(align.get_value(), 16);
        assert_eq!(align.align_address(10), 16);
        assert_eq!(align.align_address(16), 16);
        assert_eq!(align.align_address(17), 32);
        assert!(align.get_coherence() <= 1.0);
    }

    #[test]
    #[should_panic(expected = "Alignment must be a power of 2")]
    fn test_invalid_alignment() {
        Alignment::new(3);
    }

    #[test]
    fn test_aligned_space() {
        let align = Alignment::new(16);
        let space = AlignedSpace::new(10, 100, align);
        assert_eq!(space.get_base(), 16);
        assert_eq!(space.get_size(), 100);
        assert_eq!(space.get_alignment().get_value(), 16);
        assert!(space.get_coherence() <= 1.0);
    }

    #[test]
    fn test_quantum_stability() {
        let mut space = AlignedSpace::new(
            10,
            100,
            Alignment::new(16)
        );
        assert!(!space.is_quantum_stable());

        space.reset_coherence();
        assert!(space.is_quantum_stable());

        for _ in 0..10 {
            space.decay_coherence();
        }
        assert!(!space.is_quantum_stable());
    }

    #[test]
    fn test_realignment() {
        let mut space = AlignedSpace::new(10, 100, Alignment::new(16));
        let initial_pos = space.get_vector_position();
        space.realign();
        let final_pos = space.get_vector_position();
        assert_eq!(initial_pos, final_pos);
        assert_eq!(space.get_base(), 16);
    }

    #[test]
    fn test_vector_space_coherence() {
        let mut space = AlignedSpace::new(16, 100, Alignment::new(16));
        assert!(space.get_coherence() <= 1.0);

        space.reset_coherence();
        assert_eq!(space.get_coherence(), 1.0);

        space.decay_coherence();
        assert!(space.get_coherence() < 1.0);
    }
}
