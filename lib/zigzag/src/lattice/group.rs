use super::Lattice;
use crate::core::SIMDValue;

#[derive(Debug)]
pub struct LatticeGroup<T: SIMDValue> {
    operations: Vec<Box<dyn Lattice<T>>>,
}

impl<T: SIMDValue> LatticeGroup<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            operations: Vec::with_capacity(3),
        }
    }

    #[inline]
    pub fn add_operation(&mut self, operation: Box<dyn Lattice<T>>) {
        self.operations.push(operation);
    }

    #[inline]
    pub fn apply_group(&self, data: &[T]) -> Vec<T> {
        // Pre-allocate result vector with exact capacity
        let mut result = Vec::with_capacity(data.len());
        result.extend_from_slice(data);

        // Apply operations in-place when possible
        for op in &self.operations {
            result = op.apply_symmetry(&result);
        }

        result
    }
}
