//! Lattice operations module

use std::sync::Arc;
use crate::core::SIMDValue;

mod operations;
mod group;
pub use operations::*;
pub use group::*;

#[derive(Debug, Clone)]
pub struct LatticeConfig {
    dimensions: usize,
    symmetry_type: LatticeSymmetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LatticeSymmetry {
    Cubic,
    Tetragonal,
    Hexagonal,
}

pub trait Lattice<T: SIMDValue> {
    fn apply_symmetry(&self, data: &[T]) -> Vec<T>;
    fn get_config(&self) -> &LatticeConfig;
}
