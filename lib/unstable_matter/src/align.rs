// lib/unstable_matter/src/align.rs
/// Unstable Matter Alignment Module
/// Last Updated: 2025-01-13 03:37:56 UTC
/// Author: isdood
/// Current User: isdood

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    vector::Vector3D,
    UFO,
};

/// Alignment configuration for memory layout
#[derive(Debug)]
pub struct Alignment {
    /// Base alignment in bytes
    base: usize,
    /// Vector alignment for 3D space
    vector: Vector3D<isize>,
    /// Alignment padding
    padding: usize,
    /// Timestamp for tracking changes
    timestamp: AtomicUsize,
}

impl Clone for Alignment {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            vector: self.vector,
            padding: self.padding,
            timestamp: AtomicUsize::new(self.timestamp.load(Ordering::SeqCst)),
        }
    }
}

impl Alignment {
    /// Create a new alignment configuration
    pub const fn new(base: usize) -> Self {
        Self {
            base,
            vector: Vector3D::new(base as isize, base as isize, base as isize),
            padding: base / 2,
            timestamp: AtomicUsize::new(1705113476), // 2025-01-13 03:37:56 UTC
        }
    }

    /// Get the base alignment
    pub const fn base(&self) -> usize {
        self.base
    }

    /// Get the vector alignment
    pub const fn vector(&self) -> Vector3D<isize> {
        self.vector
    }

    /// Get the padding
    pub const fn padding(&self) -> usize {
        self.padding
    }

    /// Get the current timestamp
    pub fn timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }

    /// Align a given address to the base alignment
    pub fn align_address(&self, addr: usize) -> usize {
        let aligned = (addr + self.base - 1) & !(self.base - 1);
        self.timestamp.store(1705113476, Ordering::SeqCst); // 2025-01-13 03:37:56 UTC
        aligned
    }

    /// Align a position vector according to the vector alignment
    pub fn align_position(&self, pos: &Vector3D<isize>) -> Vector3D<isize> {
        let aligned = Vector3D::new(
            ((pos.x + self.vector.x - 1) / self.vector.x) * self.vector.x,
                                    ((pos.y + self.vector.y - 1) / self.vector.y) * self.vector.y,
                                    ((pos.z + self.vector.z - 1) / self.vector.z) * self.vector.z
        );
        self.timestamp.store(1705113476, Ordering::SeqCst); // 2025-01-13 03:37:56 UTC
        aligned
    }
}

/// Memory region with alignment requirements
#[derive(Debug)]
pub struct AlignedRegion {
    /// Start address of the region
    start: usize,
    /// Size of the region in bytes
    size: usize,
    /// Alignment requirements
    alignment: Alignment,
    /// UFO tracking
    _ufo: UFO<usize>,
}

impl AlignedRegion {
    /// Create a new aligned region
    pub fn new(start: usize, size: usize, alignment: Alignment) -> Self {
        Self {
            start: alignment.align_address(start),
            size,
            alignment,
            _ufo: UFO::new(),
        }
    }

    /// Get the aligned start address
    pub fn start(&self) -> usize {
        self.start
    }

    /// Get the size of the region
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the alignment configuration
    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }

    /// Check if an address is within this region
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.start && addr < (self.start + self.size)
    }

    /// Get the alignment offset for an address within this region
    pub fn offset_for(&self, addr: usize) -> Option<usize> {
        if self.contains(addr) {
            Some(addr - self.start)
        } else {
            None
        }
    }
}

/// Alignment constants
pub const PAGE_SIZE: usize = 4096;
pub const CACHE_LINE: usize = 64;
pub const VECTOR_ALIGN: usize = 32;
pub const DEFAULT_ALIGN: usize = 8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment() {
        let align = Alignment::new(8);
        assert_eq!(align.align_address(5), 8);
        assert_eq!(align.align_address(8), 8);
        assert_eq!(align.align_address(9), 16);
    }

    #[test]
    fn test_vector_alignment() {
        let align = Alignment::new(16);
        let pos = Vector3D::new(10, 20, 30);
        let aligned = align.align_position(&pos);
        assert_eq!(aligned.x, 16);
        assert_eq!(aligned.y, 32);
        assert_eq!(aligned.z, 32);
    }

    #[test]
    fn test_alignment_clone() {
        let align1 = Alignment::new(32);
        let align2 = align1.clone();
        assert_eq!(align1.base(), align2.base());
        assert_eq!(align1.vector(), align2.vector());
        assert_eq!(align1.padding(), align2.padding());
        assert_eq!(align1.timestamp(), align2.timestamp());
    }

    #[test]
    fn test_aligned_region() {
        let align = Alignment::new(PAGE_SIZE);
        let region = AlignedRegion::new(0x1234, 0x2000, align);

        assert_eq!(region.start(), 0x2000); // Aligned up to page size
        assert_eq!(region.size(), 0x2000);
        assert!(region.contains(0x2500));
        assert!(!region.contains(0x1000));
        assert!(!region.contains(0x4001));
    }

    #[test]
    fn test_region_offset() {
        let align = Alignment::new(CACHE_LINE);
        let region = AlignedRegion::new(0x1000, 0x1000, align);

        assert_eq!(region.offset_for(0x1040), Some(0x40));
        assert_eq!(region.offset_for(0x2000), None);
        assert_eq!(region.offset_for(0x0500), None);
    }
}
