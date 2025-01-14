// lib/unstable_matter/src/space_time.rs
/// Last Updated: 2025-01-14 16:22:31 UTC
/// Author: isdood
/// Current User: isdood

//! < SpaceTime: A Memory-Space-Time Abstraction Layer >

use crate::{
    wrapper::UnstableMatter,
    ufo::UFO,
    fluid::FluidMemory,
};
use core::sync::atomic::AtomicUsize;
use core::marker::PhantomData;

const CURRENT_TIMESTAMP: usize = 1705245751; // 2025-01-14 16:22:31 UTC

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
pub struct SpaceTime<T: Copy + 'static> {
    memory: FluidMemory<T>,
    size: usize,
    offset: usize,
    stride: usize,
    dimensions: Dimensions,
    timestamp: AtomicUsize,
    _ufo: UFO<T>,
}

impl<T: Copy + 'static> SpaceTime<T> {
    pub fn new(memory: FluidMemory<T>, dimensions: Dimensions) -> Self {
        Self {
            size: dimensions.width * dimensions.height * dimensions.depth,
            offset: 0,
            stride: core::mem::size_of::<T>(),
            dimensions,
            timestamp: AtomicUsize::new(CURRENT_TIMESTAMP),
            memory,
            _ufo: UFO::new(),
        }
    }

    pub fn from_virt(addr: VirtAddr, size: usize, dimensions: Dimensions) -> Self {
        let memory = FluidMemory::new(addr.0 as usize, size);
        Self::new(memory, dimensions)
    }

    pub fn from_phys(addr: PhysAddr, size: usize, phys_offset: u64, dimensions: Dimensions) -> Self {
        let virt_addr = VirtAddr(addr.0 + phys_offset);
        Self::from_virt(virt_addr, size, dimensions)
    }

    /// Reads a value from the space-time region
    pub fn read(&self, x: usize, y: usize, z: usize) -> Option<T> {
        let idx = self.calculate_index(x, y, z);
        if idx < self.size {
            Some(self.memory.read(idx))
        } else {
            None
        }
    }

    /// Writes a value to the space-time region
    pub fn write(&mut self, x: usize, y: usize, z: usize, value: T) -> Result<(), &'static str> {
        let idx = self.calculate_index(x, y, z);
        if idx < self.size {
            self.memory.write(idx, value);
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    /// Maps a region of memory into this space-time region
    pub fn map_region(&mut self, source: &SpaceTime<T>, dest_offset: usize) -> Result<(), &'static str> {
        if dest_offset + source.size > self.size {
            return Err("Region mapping would exceed space-time bounds");
        }

        // Perform the mapping
        for i in 0..source.size {
            if let Some(value) = source.read(i, 0, 0) {
                self.write(dest_offset + i, 0, 0, value)?;
            }
        }
        Ok(())
    }

    /// Returns the index in the linear memory for given coordinates
    fn calculate_index(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.dimensions.width * self.dimensions.height +
        y * self.dimensions.width +
        x + self.offset) * self.stride
    }

    // Metadata access methods
    pub fn get_metadata(&self) -> (usize, usize, usize, Dimensions) {
        (self.size, self.offset, self.stride, self.dimensions)
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(core::sync::atomic::Ordering::SeqCst)
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

/// Architecture-specific implementations
pub mod arch {
    use super::*;

    #[derive(Debug)]
    pub struct PageTable {
        entries: SpaceTime<u64>,
    }

    impl PageTable {
        pub fn new(base_addr: PhysAddr) -> Self {
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
            self.entries.read(index, 0, 0).map(PageTableEntry::new)
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
unsafe impl<T: Copy + 'static> Send for SpaceTime<T> {}
