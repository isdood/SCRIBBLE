use super::{Lattice, LatticeConfig, LatticeSymmetry};
use crate::core::SIMDValue;
use std::sync::Arc;
use parking_lot::RwLock;

type SharedBuffer = Arc<RwLock<Vec<f32>>>;

#[derive(Debug, Clone)]
pub struct CubicLattice {
    pub config: LatticeConfig,
    buffer: SharedBuffer,
}

#[derive(Debug, Clone)]
pub struct TetragonalLattice {
    pub config: LatticeConfig,
    buffer: SharedBuffer,
}

#[derive(Debug, Clone)]
pub struct HexagonalLattice {
    pub config: LatticeConfig,
    buffer: SharedBuffer,
}

impl CubicLattice {
    #[inline]
    pub fn new() -> Self {
        Self {
            config: LatticeConfig {
                dimensions: 3,
                symmetry_type: LatticeSymmetry::Cubic,
            },
            buffer: Arc::new(RwLock::new(Vec::with_capacity(256))),
        }
    }
}

impl TetragonalLattice {
    #[inline]
    pub fn new() -> Self {
        Self {
            config: LatticeConfig {
                dimensions: 3,
                symmetry_type: LatticeSymmetry::Tetragonal,
            },
            buffer: Arc::new(RwLock::new(Vec::with_capacity(256))),
        }
    }
}

impl HexagonalLattice {
    #[inline]
    pub fn new() -> Self {
        Self {
            config: LatticeConfig {
                dimensions: 3,
                symmetry_type: LatticeSymmetry::Hexagonal,
            },
            buffer: Arc::new(RwLock::new(Vec::with_capacity(256))),
        }
    }
}

impl<T: SIMDValue> Lattice<T> for CubicLattice {
    #[inline]
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        let mut buffer = self.buffer.write();
        buffer.clear();
        buffer.reserve(data.len());

        if let Some(data_f32) = data.iter().map(|x| x.to_f32()).collect::<Option<Vec<f32>>>() {
            buffer.extend_from_slice(&data_f32);
            return buffer.iter()
                .map(|&x| T::from_f32(x).unwrap())
                .collect();
        }

        data.to_vec()
    }

    #[inline]
    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl<T: SIMDValue> Lattice<T> for TetragonalLattice {
    #[inline]
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        let mut buffer = self.buffer.write();
        buffer.clear();
        buffer.reserve(data.len());

        if let Some(data_f32) = data.iter().map(|x| x.to_f32()).collect::<Option<Vec<f32>>>() {
            buffer.extend_from_slice(&data_f32);
            return buffer.iter()
                .map(|&x| T::from_f32(x).unwrap())
                .collect();
        }

        data.to_vec()
    }

    #[inline]
    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl<T: SIMDValue> Lattice<T> for HexagonalLattice {
    #[inline]
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        let mut buffer = self.buffer.write();
        buffer.clear();
        buffer.reserve(data.len());

        if let Some(data_f32) = data.iter().map(|x| x.to_f32()).collect::<Option<Vec<f32>>>() {
            buffer.extend_from_slice(&data_f32);
            return buffer.iter()
                .map(|&x| T::from_f32(x).unwrap())
                .collect();
        }

        data.to_vec()
    }

    #[inline]
    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_lattice() {
        let lattice = CubicLattice::new();
        let input = vec![1.0f32, 2.0, 3.0];
        let result = lattice.apply_symmetry(&input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_tetragonal_lattice() {
        let lattice = TetragonalLattice::new();
        let input = vec![1.0f32, 2.0, 3.0];
        let result = lattice.apply_symmetry(&input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_hexagonal_lattice() {
        let lattice = HexagonalLattice::new();
        let input = vec![1.0f32, 2.0, 3.0];
        let result = lattice.apply_symmetry(&input);
        assert_eq!(result, input);
    }
}
