//! Lattice operations module
//! Created: 2025-01-22 00:35:31
//! Author: isdood

use std::sync::Arc;
use crate::superpurple::core::SIMDValue;

mod operations;
pub use operations::*;

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
mod group;
pub use group::LatticeGroup;
