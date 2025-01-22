use super::{Lattice, LatticeConfig, LatticeSymmetry};
use crate::core::SIMDValue;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

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

#[cfg(target_arch = "x86_64")]
mod simd {
    use super::*;

    #[inline(always)]
    pub unsafe fn process_avx2_f32(data: &[f32]) -> Vec<f32> {
        let mut result = Vec::with_capacity(data.len());
        let chunks = data.chunks_exact(32);
        let remainder = chunks.remainder();

        // Process 32 elements at a time using AVX2
        for chunk in chunks {
            let v1 = _mm256_loadu_ps(chunk.as_ptr());
            let v2 = _mm256_loadu_ps(chunk.as_ptr().add(8));
            let v3 = _mm256_loadu_ps(chunk.as_ptr().add(16));
            let v4 = _mm256_loadu_ps(chunk.as_ptr().add(24));

            // Store results
            let mut buffer = [0.0f32; 32];
            _mm256_storeu_ps(buffer.as_mut_ptr(), v1);
            _mm256_storeu_ps(buffer.as_mut_ptr().add(8), v2);
            _mm256_storeu_ps(buffer.as_mut_ptr().add(16), v3);
            _mm256_storeu_ps(buffer.as_mut_ptr().add(24), v4);

            result.extend_from_slice(&buffer);
        }

        // Handle remaining elements
        result.extend_from_slice(remainder);
        result
    }
}

impl CubicLattice {
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn new() -> Self {
        Self {
            config: LatticeConfig {
                dimensions: 3,
                symmetry_type: LatticeSymmetry::Hexagonal,
            },
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl<T: SIMDValue> Lattice<T> for CubicLattice {
    #[inline]
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        if data.len() < 32 {
            return data.to_vec();
        }

        unsafe {
            if is_x86_feature_detected!("avx2") {
                if let Some(data_f32) = data.iter().map(|x| x.to_f32()).collect::<Option<Vec<f32>>>() {
                    return simd::process_avx2_f32(&data_f32)
                        .into_iter()
                        .map(|x| T::from_f32(x).unwrap())
                        .collect();
                }
            }
        }
        data.to_vec()
    }

    #[inline(always)]
    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl<T: SIMDValue> Lattice<T> for CubicLattice {
    #[inline]
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        data.to_vec()
    }

    #[inline(always)]
    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl<T: SIMDValue> Lattice<T> for TetragonalLattice {
    #[inline]
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        data.to_vec()
    }

    #[inline(always)]
    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

impl<T: SIMDValue> Lattice<T> for HexagonalLattice {
    #[inline]
    fn apply_symmetry(&self, data: &[T]) -> Vec<T> {
        data.to_vec()
    }

    #[inline(always)]
    fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}
