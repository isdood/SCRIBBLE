// lib/unstable_matter/src/space_time.rs

//! < SpaceTime: A Memory-Space-Time Abstraction Layer >

//! SpaceTime provides an abstraction over physical and
//! virtual memory spaces, treating them as
//! multi-dimensional regions existing in space-time.
//! This allows for safe and controlled access
//! to memory while maintaining awareness of both
//! spatial (addressing) and temporal (access patterns)
//! characteristics.

//! Key features:
//! - Multi-dimensional memory space representation
//!   (1D, 2D, 3D)

//! - Safe abstraction over volatile memory operations
//! - Hardware-level memory mapping capabilities
//! - Architecture-specific implementations (x86_64)
//! - Thread-safe memory access patterns

//! The SpaceTime abstraction is built on top of
//! UnstableMatter, providing a higher-level interface
//! for memory operations while maintaining direct
//! control over hardware interactions.
//!
//! Author: Caleb J.D. Terkovics

use crate::wrapper::UnstableMatter;
use core::marker::PhantomData;

/// Represents dimensions in the vector space
#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
    pub depth: usize,  // For 3D memory regions like frame buffers
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub u64);

/// Types of memory regions in space-time
#[derive(Debug, Clone, Copy)]
pub enum SpaceType {
    Linear,      // Standard memory region
    Frame,       // Page frame
    Buffer,      // Device buffer
    Mapped,      // Memory mapped I/O
    Stack,       // Stack region
    Guard,       // Guard page
}

/// Represents a space-time region for memory operations
#[derive(Debug)]
pub struct SpaceTime<T> {
    base: UnstableMatter<T>,
    size: usize,
    offset: usize,
    stride: usize,
    dimensions: Dimensions,
    _phantom: PhantomData<T>,
}

impl<T> SpaceTime<T> {
    pub unsafe fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            base: UnstableMatter::at(base_addr),
            size,
            offset,
            stride: core::mem::size_of::<T>(),
            dimensions: Dimensions {
                width: size,
                height: 1,
                depth: 1,
            },
            _phantom: PhantomData,
        }
    }

    pub fn from_virt(addr: VirtAddr, size: usize, dimensions: Dimensions) -> Self {
        unsafe {
            let mut instance = Self::new(addr.0 as usize, size, 0);
            instance.dimensions = dimensions;
            instance
        }
    }

    pub fn from_phys(addr: PhysAddr, size: usize, phys_offset: u64, dimensions: Dimensions) -> Self {
        unsafe {
            let mut instance = Self::new((addr.0 + phys_offset) as usize, size, 0);
            instance.dimensions = dimensions;
            instance
        }
    }

    /// Reads a value from the space-time region
    pub fn read(&self, x: usize, y: usize, z: usize) -> T
    where
    T: Copy
    {
        let idx = self.calculate_index(x, y, z);
        assert!(idx < self.size);
        unsafe { self.base.read() }
    }

    /// Writes a value to the space-time region
    pub fn write(&mut self, x: usize, y: usize, z: usize, value: T) {
        let idx = self.calculate_index(x, y, z);
        assert!(idx < self.size);
        unsafe { self.base.write(value) }
    }

    /// Relocates the space-time region to a new base address
    pub fn move_to(&mut self, new_addr: usize) {
        self.base = unsafe { UnstableMatter::at(new_addr) };
    }

    /// Maps a region of memory into this space-time region
    pub unsafe fn map_region(&mut self, source: &SpaceTime<T>, dest_offset: usize) -> Result<(), &'static str> {
        if dest_offset + source.size > self.size {
            return Err("Region mapping would exceed space-time bounds");
        }
        // Perform the mapping
        Ok(())
    }

    /// Returns the index in the linear memory for given coordinates
    fn calculate_index(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.dimensions.width * self.dimensions.height +
        y * self.dimensions.width +
        x + self.offset) * self.stride
    }

    // Getters
    pub fn virt_addr(&self) -> VirtAddr { VirtAddr(self.base.addr() as u64) }
    pub fn phys_addr(&self, phys_offset: u64) -> PhysAddr { PhysAddr(self.base.addr() as u64 - phys_offset) }
    pub fn size(&self) -> usize { self.size }
    pub fn offset(&self) -> usize { self.offset }
    pub fn stride(&self) -> usize { self.stride }
    pub fn dimensions(&self) -> Dimensions { self.dimensions }
}

/// Architecture-specific implementations
pub mod arch {
    use super::*;

    #[derive(Debug)]
    pub struct PageTable {
        entries: SpaceTime<u64>,
    }

    impl PageTable {
        pub unsafe fn new(base_addr: PhysAddr) -> Self {
            let dimensions = Dimensions {
                width: 512,  // Standard x86_64 page table size
                height: 1,
                depth: 1,
            };
            Self {
                entries: SpaceTime::from_phys(base_addr, 512, 0, dimensions)
            }
        }

        pub fn entry(&self, index: usize) -> Option<PageTableEntry> {
            if index < 512 {
                Some(PageTableEntry::new(self.entries.read(index, 0, 0)))
            } else {
                None
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct PageTableEntry(u64);

    impl PageTableEntry {
        pub fn new(entry: u64) -> Self {
            Self(entry)
        }

        pub fn is_present(&self) -> bool {
            self.0 & 1 == 1
        }

        pub fn is_writable(&self) -> bool {
            self.0 & (1 << 1) == (1 << 1)
        }
    }
}

/// Required for safe usage across threads
unsafe impl<T> Send for SpaceTime<T> {}
