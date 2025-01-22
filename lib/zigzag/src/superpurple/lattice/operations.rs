//! Lattice operations implementation
//! Created: 2025-01-22 00:35:31
//! Author: isdood

use super::{Lattice, LatticeConfig, LatticeSymmetry};
use crate::superpurple::core::SIMDValue;
use std::arch::x86_64::*;

pub struct CubicLattice {
    config: LatticeConfig,
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

impl<T: SIMDValue> Lattice<T> for CubicLattice {
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                // AVX-512 implementation
                for chunk in data.chunks(16) {
                    if chunk.len() == 16 {
                        let input = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        // Apply cubic symmetry transformation
                        let output = self.transform_cubic_512(input);
                        let mut buffer = vec![0.0f32; 16];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), output);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                // Fallback implementation
                result.extend_from_slice(data);
            }
        }

        result
    }

    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl CubicLattice {
    #[inline]
    unsafe fn transform_cubic_512(&self, input: __m512) -> __m512 {
        // Apply cubic symmetry transformation using AVX-512
        let permute_mask = _mm512_set_epi32(
            15, 14, 13, 12,
            11, 10, 9, 8,
            7, 6, 5, 4,
            3, 2, 1, 0
        );
        _mm512_permutexvar_ps(permute_mask, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_symmetry() {
        let lattice = CubicLattice::new();
        let data = vec![1.0f32; 16];
        let result = lattice.apply_symmetry(&data);
        assert_eq!(result.len(), data.len());
    }
}
pub struct TetragonalLattice {
    config: LatticeConfig,
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

impl<T: SIMDValue> Lattice<T> for TetragonalLattice {
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                for chunk in data.chunks(16) {
                    if chunk.len() == 16 {
                        let input = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let output = self.transform_tetragonal_512(input);
                        let mut buffer = vec![0.0f32; 16];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), output);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                result.extend_from_slice(data);
            }
        }

        result
    }

    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl TetragonalLattice {
    #[inline]
    unsafe fn transform_tetragonal_512(&self, input: __m512) -> __m512 {
        let permute_mask = _mm512_set_epi32(
            15, 14, 13, 12,
            8, 9, 10, 11,
            7, 6, 5, 4,
            0, 1, 2, 3
        );
        _mm512_permutexvar_ps(permute_mask, input)
    }
}

pub struct HexagonalLattice {
    config: LatticeConfig,
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

impl<T: SIMDValue> Lattice<T> for HexagonalLattice {
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(data.len());

        unsafe {
            if is_x86_feature_detected!("avx512f") {
                for chunk in data.chunks(16) {
                    if chunk.len() == 16 {
                        let input = _mm512_loadu_ps(chunk.as_ptr() as *const f32);
                        let output = self.transform_hexagonal_512(input);
                        let mut buffer = vec![0.0f32; 16];
                        _mm512_storeu_ps(buffer.as_mut_ptr(), output);
                        result.extend(buffer.iter().map(|&x| T::from(x).unwrap()));
                    }
                }
            } else {
                result.extend_from_slice(data);
            }
        }

        result
    }

    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl HexagonalLattice {
    #[inline]
    unsafe fn transform_hexagonal_512(&self, input: __m512) -> __m512 {
        let permute_mask = _mm512_set_epi32(
            15, 14, 13,
            12, 11, 10,
            9, 8, 7,
            6, 5, 4,
            3, 2, 1, 0
        );
        let rotated = _mm512_permutexvar_ps(permute_mask, input);

        // Apply 60-degree rotation transformation
        let cos60 = _mm512_set1_ps(0.5);
        let sin60 = _mm512_set1_ps(0.866025404);

        let x = _mm512_shuffle_ps(rotated, rotated, 0x00);
        let y = _mm512_shuffle_ps(rotated, rotated, 0x55);

        let new_x = _mm512_add_ps(
            _mm512_mul_ps(x, cos60),
            _mm512_mul_ps(y, sin60)
        );
        let new_y = _mm512_sub_ps(
            _mm512_mul_ps(y, cos60),
            _mm512_mul_ps(x, sin60)
        );

        _mm512_shuffle_ps(new_x, new_y, 0x44)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tetragonal_symmetry() {
        let lattice = TetragonalLattice::new();
        let data = vec![1.0f32; 16];
        let result = lattice.apply_symmetry(&data);
        assert_eq!(result.len(), data.len());
    }

    #[test]
    fn test_hexagonal_symmetry() {
        let lattice = HexagonalLattice::new();
        let data = vec![1.0f32; 16];
        let result = lattice.apply_symmetry(&data);
        assert_eq!(result.len(), data.len());
    }
}
