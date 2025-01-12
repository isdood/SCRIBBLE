/// UnstableMatter Core Library
/// Last Updated: 2025-01-12 21:57:39 UTC
/// Author: isdood

#![no_std]

// Re-export core components
pub use core::marker::PhantomData;

// Module declarations
pub mod ufo;
pub mod vector_space;
pub mod align;

// Public exports
pub use ufo::{UFO, Protected, MemoryTrace, ProtectedRegion};
pub use vector_space::{VectorSpace, MeshCell, CellState};

// Type aliases for common use
pub type Flying = ufo::Flying;
pub type Hovering = ufo::Hovering;
pub type Landed = ufo::Landed;

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl Dimensions {
    pub const fn const_new(width: usize, height: usize, depth: usize) -> Self {
        Self { width, height, depth }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    pub const fn const_new(addr: u64) -> Self {
        Self(addr)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub u64);

impl PhysAddr {
    pub const fn const_new(addr: u64) -> Self {
        Self(addr)
    }
}

#[derive(Debug)]
pub struct SpaceTime<T> {
    base: UnstableMatter<T>,
    size: usize,
    offset: usize,
    stride: usize,
    dimensions: Dimensions,
    _phantom: PhantomData<T>,
}

impl<T: Copy> SpaceTime<T> {
    pub const unsafe fn const_new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self {
            base: UnstableMatter::const_at(base_addr),
            size,
            offset,
            stride: core::mem::size_of::<T>(),
            dimensions: Dimensions::const_new(size, 1, 1),
            _phantom: PhantomData,
        }
    }

    pub unsafe fn new(base_addr: usize, size: usize, offset: usize) -> Self {
        Self::const_new(base_addr, size, offset)
    }

    pub unsafe fn read_at(&self, index: usize) -> T {
        assert!(index < self.size);
        let addr = self.base.addr() + (index * self.stride) + self.offset;
        UnstableMatter::at(addr).read()
    }

    pub unsafe fn write_at(&mut self, index: usize, value: T) {
        assert!(index < self.size);
        let addr = self.base.addr() + (index * self.stride) + self.offset;
        UnstableMatter::at(addr).write(value)
    }

    pub const fn from_virt(addr: VirtAddr, size: usize, dimensions: Dimensions) -> Self {
        unsafe {
            Self {
                base: UnstableMatter::const_at(addr.0 as usize),
                size,
                offset: 0,
                stride: core::mem::size_of::<T>(),
                dimensions,
                _phantom: PhantomData,
            }
        }
    }

    pub const fn from_phys(addr: PhysAddr, size: usize, phys_offset: u64, dimensions: Dimensions) -> Self {
        unsafe {
            Self {
                base: UnstableMatter::const_at((addr.0 + phys_offset) as usize),
                size,
                offset: 0,
                stride: core::mem::size_of::<T>(),
                dimensions,
                _phantom: PhantomData,
            }
        }
    }

    pub const fn virt_addr(&self) -> VirtAddr { VirtAddr(self.base.addr() as u64) }
    pub const fn phys_addr(&self, phys_offset: u64) -> PhysAddr { PhysAddr(self.base.addr() as u64 - phys_offset) }
    pub const fn size(&self) -> usize { self.size }
    pub const fn dimensions(&self) -> Dimensions { self.dimensions }
    pub const fn stride(&self) -> usize { self.stride }
    pub const fn offset(&self) -> usize { self.offset }
}

unsafe impl<T> Send for SpaceTime<T> {}
unsafe impl<T> Sync for SpaceTime<T> {}
