// lib/unstable_matter/src/align.rs
/// Last Updated: 2025-01-14 20:38:30 UTC
/// Author: isdood
/// Current User: isdood

use core::{
    ptr::NonNull,
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::constants::CURRENT_TIMESTAMP;

const ALIGN_TIMESTAMP: usize = 1705263412; // 2025-01-14 20:56:52 UTC

#[derive(Debug, Clone)]
pub struct Alignment {
    value: UnsafeCell<usize>,
    timestamp: AtomicUsize,
}

impl Alignment {
    pub fn new(value: usize) -> Self {
        assert!(value.is_power_of_two(), "Alignment must be a power of 2");
        Self {
            value: UnsafeCell::new(value),
            timestamp: AtomicUsize::new(ALIGN_TIMESTAMP),
        }
    }

    pub fn get_value(&self) -> usize {
        unsafe { *self.value.get() }
    }

    pub fn align_address(&self, addr: usize) -> usize {
        let value = self.get_value();
        (addr + value - 1) & !(value - 1)
    }

    pub fn get_coherence(&self) -> f64 {
        let current = CURRENT_TIMESTAMP;
        let timestamp = self.timestamp.load(Ordering::SeqCst);
        let dt = (current - timestamp) as f64;
        (1.0 / (1.0 + dt * 1e-9)).max(0.0)
    }
}

#[derive(Debug)]
pub struct AlignedSpace {
    ptr: NonNull<u8>,
    size: usize,
    alignment: Alignment,
}

impl AlignedSpace {
    pub fn new(size: usize, alignment: Alignment) -> Self {
        let aligned_size = alignment.align_address(size);
        let layout = core::alloc::Layout::from_size_align(
            aligned_size,
            alignment.get_value()
        ).unwrap();

        unsafe {
            let raw_ptr = core::alloc::alloc(layout);
            let ptr = NonNull::new(raw_ptr).expect("allocation failed");

            Self {
                ptr,
                size: aligned_size,
                alignment,
            }
        }
    }

    pub fn get_ptr(&self) -> NonNull<u8> {
        self.ptr
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_alignment(&self) -> &Alignment {
        &self.alignment
    }

    pub fn get_coherence(&self) -> f64 {
        self.alignment.get_coherence()
    }
}

impl Clone for AlignedSpace {
    fn clone(&self) -> Self {
        let new_space = Self::new(self.size, self.alignment.clone());
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.ptr.as_ptr(),
                                           new_space.ptr.as_ptr(),
                                           self.size
            );
        }
        new_space
    }
}

impl Drop for AlignedSpace {
    fn drop(&mut self) {
        unsafe {
            let layout = core::alloc::Layout::from_size_align(
                self.size,
                self.alignment.get_value()
            ).unwrap();
            core::alloc::dealloc(self.ptr.as_ptr(), layout);
        }
    }
}

pub fn vector_align() -> Alignment {
    Alignment::new(VECTOR_ALIGN)
}

pub fn cache_align() -> Alignment {
    Alignment::new(CACHE_LINE)
}

struct QuantumPool {
    blocks: [UnsafeCell<u8>; QUANTUM_BLOCK_SIZE * QUANTUM_POOL_SIZE],
    free_list: AtomicUsize,
}

impl QuantumPool {
    const fn new() -> Self {
        const ZERO: UnsafeCell<u8> = UnsafeCell::new(0);
        Self {
            blocks: [ZERO; QUANTUM_BLOCK_SIZE * QUANTUM_POOL_SIZE],
            free_list: AtomicUsize::new(0),
        }
    }

    fn alloc(&self) -> Option<NonNull<u8>> {
        let index = self.free_list.fetch_add(1, Ordering::AcqRel);
        if index >= self.blocks.len() {
            self.free_list.fetch_sub(1, Ordering::AcqRel);
            None
        } else {
            unsafe {
                Some(NonNull::new_unchecked(self.blocks[index].get()))
            }
        }
    }

    fn dealloc(&self, ptr: NonNull<u8>) {
        let _index = self.free_list.fetch_sub(1, Ordering::AcqRel) - 1;
        unsafe {
            ptr.as_ptr().write_bytes(0, QUANTUM_BLOCK_SIZE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TIMESTAMP: usize = 1705261907; // 2025-01-14 20:31:47 UTC

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
        let space = AlignedSpace::new(100, align).unwrap();
        assert_eq!(space.get_size(), 128); // Aligned to quantum block size
        assert_eq!(space.get_alignment().get_value(), 16);
        assert!(space.get_coherence() <= 1.0);
        assert!(space.get_position().x() >= 0);
    }

    #[test]
    fn test_quantum_stability() {
        let mut space = AlignedSpace::new(100, Alignment::new(16)).unwrap();
        assert!(space.is_quantum_stable());

        for _ in 0..10 {
            space.decay_coherence();
        }
        assert!(!space.is_quantum_stable());

        space.reset_coherence();
        assert!(space.is_quantum_stable());
    }

    #[test]
    fn test_quantum_block_alignment() {
        let align = Alignment::new(QUANTUM_BLOCK_SIZE);
        let space = AlignedSpace::new(100, align).unwrap();
        assert_eq!(space.get_ptr().as_ptr() as usize % QUANTUM_BLOCK_SIZE, 0);
    }

    #[test]
    fn test_quantum_pool_allocation() {
        // Test small allocation (should use quantum pool)
        let align = Alignment::new(16);
        let small_space = AlignedSpace::new(32, align).unwrap();
        let small_ptr = small_space.get_ptr().as_ptr() as usize;
        assert!(small_ptr >= QUANTUM_POOL.blocks.as_ptr() as usize &&
        small_ptr < (QUANTUM_POOL.blocks.as_ptr() as usize +
        QUANTUM_BLOCK_SIZE * QUANTUM_POOL_SIZE));

        // Test large allocation (should use system allocator)
        let large_space = AlignedSpace::new(QUANTUM_BLOCK_SIZE * 2, align).unwrap();
        let large_ptr = large_space.get_ptr().as_ptr() as usize;
        assert!(large_ptr < QUANTUM_POOL.blocks.as_ptr() as usize ||
        large_ptr >= (QUANTUM_POOL.blocks.as_ptr() as usize +
        QUANTUM_BLOCK_SIZE * QUANTUM_POOL_SIZE));
    }

    #[test]
    fn test_coherence_decay_on_realign() {
        let mut space = AlignedSpace::new(100, Alignment::new(16)).unwrap();
        let initial_coherence = space.get_coherence();

        // Force realignment
        unsafe {
            let ptr = space.get_ptr();
            let offset_ptr = NonNull::new_unchecked((ptr.as_ptr() as usize + 1) as *mut u8);
            space.ptr = offset_ptr;
        }

        space.realign();
        assert!(space.get_coherence() < initial_coherence);
    }

    #[test]
    fn test_position_tracking() {
        let space = AlignedSpace::new(100, Alignment::new(16)).unwrap();
        let pos = space.get_position();
        assert_eq!(pos.x(), space.get_ptr().as_ptr() as isize);
        assert_eq!(pos.y(), space.get_size() as isize);
        assert_eq!(pos.z(), space.get_alignment().get_value() as isize);
    }

    #[test]
    fn test_zero_size_allocation() {
        let align = Alignment::new(16);
        assert!(AlignedSpace::new(0, align).is_none());
    }
}
