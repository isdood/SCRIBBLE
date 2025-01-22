use super::{Lattice, LatticeSymmetry};
use crate::core::SIMDValue;
use std::collections::HashMap;
use std::fmt;

pub struct LatticeGroup<T: SIMDValue> {
    operations: Vec<Box<dyn Lattice<T>>>,
    pub cache: HashMap<LatticeSymmetry, Vec<T>>,
}

// Manual Debug implementation
impl<T: SIMDValue> fmt::Debug for LatticeGroup<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LatticeGroup")
            .field("operations_count", &self.operations.len())
            .field("cache_size", &self.cache.len())
            .finish()
    }
}

impl<T: SIMDValue> LatticeGroup<T> {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            cache: HashMap::new(),
        }
    }

    pub fn add_operation(&mut self, operation: Box<dyn Lattice<T>>) {
        self.operations.push(operation);
    }

    pub fn apply_group(&self, data: &[T]) -> Vec<T> {
        let mut result = data.to_vec();
        for op in &self.operations {
            result = op.apply_symmetry(&result);
        }
        result
    }

    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{CubicLattice, TetragonalLattice};

    #[test]
    fn test_lattice_group() {
        let mut group = LatticeGroup::<f32>::new();
        group.add_operation(Box::new(CubicLattice::new()));
        group.add_operation(Box::new(TetragonalLattice::new()));

        assert_eq!(group.operation_count(), 2);
        assert_eq!(group.cache_size(), 0);

        // Test Debug implementation
        let debug_str = format!("{:?}", group);
        assert!(debug_str.contains("operations_count: 2"));
        assert!(debug_str.contains("cache_size: 0"));
    }
}
