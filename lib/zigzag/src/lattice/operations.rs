use super::{Lattice, LatticeConfig, LatticeSymmetry};
use crate::core::SIMDValue;

#[derive(Debug, Clone)]
pub struct CubicLattice {
    pub config: LatticeConfig,
}

#[derive(Debug, Clone)]
pub struct TetragonalLattice {
    pub config: LatticeConfig,
}

#[derive(Debug, Clone)]
pub struct HexagonalLattice {
    pub config: LatticeConfig,
}

impl CubicLattice {
    pub fn new() -> Self {
        Self {
            config: LatticeConfig {
                dimensions: 3,
                symmetry_type: LatticeSymmetry::Cubic,
            },
        }
    }
}

impl TetragonalLattice {
    pub fn new() -> Self {
        Self {
            config: LatticeConfig {
                dimensions: 3,
                symmetry_type: LatticeSymmetry::Tetragonal,
            },
        }
    }
}

impl HexagonalLattice {
    pub fn new() -> Self {
        Self {
            config: LatticeConfig {
                dimensions: 3,
                symmetry_type: LatticeSymmetry::Hexagonal,
            },
        }
    }
}

impl<T: SIMDValue> Lattice<T> for CubicLattice {
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        data.to_vec()
    }

    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl<T: SIMDValue> Lattice<T> for TetragonalLattice {
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        data.to_vec()
    }

    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl<T: SIMDValue> Lattice<T> for HexagonalLattice {
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        data.to_vec()
    }

    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}
