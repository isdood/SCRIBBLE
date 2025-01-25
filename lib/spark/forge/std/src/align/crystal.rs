//! Crystal Space Definitions and Operations

use std::num::NonZeroUsize;
use super::Alignment;

/// Represents a dimension in crystal space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrystalAxis {
    size: NonZeroUsize,
    alignment: Alignment,
}

impl CrystalAxis {
    /// Creates a new crystal axis with specified size and alignment
    #[inline]
    pub fn new(size: NonZeroUsize, alignment: Alignment) -> Self {
        Self { size, alignment }
    }

    /// Returns the size of this axis
    #[inline]
    pub const fn size(&self) -> NonZeroUsize {
        self.size
    }

    /// Returns the alignment requirement for this axis
    #[inline]
    pub const fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Calculates the aligned size for this axis
    #[inline]
    pub fn aligned_size(&self) -> usize {
        let align = self.alignment.as_bytes();
        (self.size.get() + align - 1) & !(align - 1)
    }
}

/// Represents a multi-dimensional crystal space
#[derive(Debug)]
pub struct CrystalSpace {
    axes: Vec<CrystalAxis>,
    total_size: usize,
}

impl CrystalSpace {
    /// Creates a new crystal space with given axes
    pub fn new(axes: Vec<CrystalAxis>) -> Self {
        let total_size = axes.iter()
            .map(|axis| axis.aligned_size())
            .product();

        Self { axes, total_size }
    }

    /// Returns the total size of the crystal space
    #[inline]
    pub fn size(&self) -> usize {
        self.total_size
    }

    /// Returns a slice of the crystal axes
    #[inline]
    pub fn axes(&self) -> &[CrystalAxis] {
        &self.axes
    }
}
