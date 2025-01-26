use super::Alignment;

/// Crystal axis for spatial operations
pub struct CrystalAxis {
    size: usize,
    alignment: Alignment,
}

impl CrystalAxis {
    /// Creates a new crystal axis
    pub fn new(size: usize, alignment: Alignment) -> Self {
        Self { size, alignment }
    }

    /// Returns the size of the axis
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the alignment of the axis
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
}
