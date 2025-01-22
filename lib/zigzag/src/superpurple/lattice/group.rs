//! Lattice group operations
//! Created by: isdood
//! Date: 2025-01-22 00:40:48

use super::{Lattice, LatticeSymmetry};
use crate::superpurple::core::SIMDValue;
use std::collections::HashMap;

pub struct LatticeGroup<T: SIMDValue> {
    operations: Vec<Box<dyn Lattice<T>>>,
    cache: HashMap<LatticeSymmetry, Vec<T>>,
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

    pub fn apply_cached(&mut self, symmetry: LatticeSymmetry, data: &[T]) -> Vec<T> {
        if let Some(cached) = self.cache.get(&symmetry) {
            cached.clone()
        } else {
            let result = self.apply_group(data);
            self.cache.insert(symmetry, result.clone());
            result
        }
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{CubicLattice, TetragonalLattice, HexagonalLattice};

    #[test]
    fn test_lattice_group() {
        let mut group = LatticeGroup::<f32>::new();
        group.add_operation(Box::new(CubicLattice::new()));
        group.add_operation(Box::new(TetragonalLattice::new()));

        let data = vec![1.0f32; 16];
        let result = group.apply_group(&data);
        assert_eq!(result.len(), data.len());
    }
}
