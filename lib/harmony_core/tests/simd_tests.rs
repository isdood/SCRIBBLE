//! SIMD Optimization Tests for Shard Architecture
//! ==========================================
//!
//! Tests for SIMD-optimized operations through crystal lattice structure
//! with quantum state verification.
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-21 00:12:38 UTC
//! Version: 0.1.0
//! License: MIT

use harmony_core::ShardUninit;
use core::arch::x86_64::*;

/// Test fixture for SIMD operations
struct SimdTestFixture {
    buffer: ShardUninit<[f64; 4]>,
    aligned_buffer: ShardUninit<[f64; 4]>,
}

impl SimdTestFixture {
    fn new() -> Self {
        Self {
            buffer: ShardUninit::new(),
            aligned_buffer: ShardUninit::new(),
        }
    }

    #[inline]
    unsafe fn execute_simd_op<F>(&mut self, op: F) -> Result<[f64; 4], String>
    where
    F: FnOnce(&[f64; 4]) -> Result<__m256d, String>,
    {
        // Get the aligned data
        let aligned_data = self.aligned_buffer
        .get()
        .ok_or_else(|| "Buffer not initialized".to_string())?;

        // Execute the SIMD operation
        let simd_result = op(aligned_data)?;

        // Convert SIMD result back to array
        let mut result = [0.0; 4];
        _mm256_store_pd(result.as_mut_ptr(), simd_result);

        Ok(result)
    }
}

#[cfg(test)]
mod simd_crystal_tests {
    use super::*;

    #[test]
    fn test_vector_add_simd() {
        if !is_x86_feature_detected!("avx2") {
            return; // Skip test if AVX2 not available
        }

        let mut fixture = SimdTestFixture::new();

        unsafe {
            // Prepare test vectors
            let v1 = [1.0, 2.0, 3.0, 4.0];
            let v2 = [5.0, 6.0, 7.0, 8.0];

            // Initialize buffers
            fixture.buffer.init(v1);
            fixture.aligned_buffer.init(v2);

            // Load vectors into SIMD registers
            let result = fixture.execute_simd_op(|aligned_data| {
                let simd1 = _mm256_loadu_pd(v1.as_ptr());
                let simd2 = _mm256_load_pd(aligned_data.as_ptr());

                // Simple addition
                Ok(_mm256_add_pd(simd1, simd2))
            }).expect("SIMD operation failed");

            // Verify results
            assert!((result[0] - 6.0).abs() < f64::EPSILON);
            assert!((result[1] - 8.0).abs() < f64::EPSILON);
            assert!((result[2] - 10.0).abs() < f64::EPSILON);
            assert!((result[3] - 12.0).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_crystal_aligned_operations() {
        if !is_x86_feature_detected!("avx2") {
            return;
        }

        let mut fixture = SimdTestFixture::new();

        unsafe {
            // Initialize aligned memory
            let data = [1.0, 2.0, 3.0, 4.0];
            fixture.aligned_buffer.init(data);

            // Load aligned data through SIMD
            let result = fixture.execute_simd_op(|aligned_data| {
                let simd_data = _mm256_load_pd(aligned_data.as_ptr());

                // Simple multiplication by 2
                Ok(_mm256_mul_pd(simd_data, _mm256_set1_pd(2.0)))
            }).expect("Aligned SIMD operation failed");

            // Verify results
            assert!((result[0] - 2.0).abs() < f64::EPSILON);
            assert!((result[1] - 4.0).abs() < f64::EPSILON);
            assert!((result[2] - 6.0).abs() < f64::EPSILON);
            assert!((result[3] - 8.0).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_buffer_operations() {
        let mut fixture = SimdTestFixture::new();

        // Test buffer initialization
        let data = [1.0, 2.0, 3.0, 4.0];
        fixture.buffer.init(data);

        assert!(fixture.buffer.is_initialized());
        assert_eq!(fixture.buffer.get(), Some(&data));
    }
}
