use crate::core::SIMDValue;
use std::fmt::Debug;

mod operations;
mod group;
pub use operations::*;
pub use group::*;

#[derive(Debug, Clone)]
pub struct LatticeConfig {
    pub dimensions: usize,
    pub symmetry_type: LatticeSymmetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LatticeSymmetry {
    Cubic,
    Tetragonal,
    Hexagonal,
}

pub trait Lattice<T: SIMDValue>: Debug {
    fn apply_symmetry(&self, data: &[T]) -> Vec<T>;
    fn get_config(&self) -> &LatticeConfig;
}
