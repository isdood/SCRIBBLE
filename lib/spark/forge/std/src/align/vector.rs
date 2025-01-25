//! Vector Space Operations for Crystal Computing

use std::num::NonZeroUsize;
use super::{Alignment, crystal::CrystalAxis};

/// A vector space for crystal computing
#[derive(Debug)]
pub struct VectorSpace {
    dimensions: Vec<NonZeroUsize>,
    alignment: Alignment,
}

impl VectorSpace {
    /// Creates a new vector space with given dimensions and alignment
    pub fn new(dimensions: Vec<NonZeroUsize>, alignment: Alignment) -> Self {
        Self { dimensions, alignment }
    }

    /// Converts this vector space to crystal axes
    pub fn to_crystal_axes(&self) -> Vec<CrystalAxis> {
        self.dimensions
            .iter()
            .map(|&size| CrystalAxis::new(size, self.alignment))
            .collect()
    }

    /// Returns the total size of the vector space
    pub fn size(&self) -> usize {
        self.dimensions
            .iter()
            .map(|d| d.get())
            .product()
    }

    /// Returns the alignment requirement
    #[inline]
    pub const fn alignment(&self) -> Alignment {
        self.alignment
    }
}
