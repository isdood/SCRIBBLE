//! CPU-specific SIMD intrinsics
//! Created: 2025-01-22 00:25:03
//! Author: isdood

#![allow(unused_unsafe)]
use std::arch::x86_64::*;
use std::mem::{transmute, size_of};
use crate::superpurple::core::SIMDValue;

/// Ensure pointer is properly aligned
#[inline]
fn ensure_alignment<T>(ptr: *const T, alignment: usize) -> bool {
    (ptr as usize) % alignment == 0
}

/// AVX-512 operations
pub mod avx512 {
    use super::*;

    #[inline]
    pub unsafe fn dot_product_f32(a: &[f32], b: &[f32]) -> f32 {
        debug_assert!(a.len() >= 16 && b.len() >= 16);
        debug_assert!(ensure_alignment(a.as_ptr(), 64));
        debug_assert!(ensure_alignment(b.as_ptr(), 64));

        let mut result = _mm512_setzero_ps();

        for (chunk_a, chunk_b) in a.chunks_exact(16)
            .zip(b.chunks_exact(16))
        {
            let va = _mm512_load_ps(chunk_a.as_ptr());
            let vb = _mm512_load_ps(chunk_b.as_ptr());
            result = _mm512_add_ps(result, _mm512_mul_ps(va, vb));
        }

        _mm512_reduce_add_ps(result)
    }

    #[inline]
    pub unsafe fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        debug_assert!(a.len() >= 8 && b.len() >= 8);
        debug_assert!(ensure_alignment(a.as_ptr(), 64));
        debug_assert!(ensure_alignment(b.as_ptr(), 64));

        let mut result = _mm512_setzero_pd();

        for (chunk_a, chunk_b) in a.chunks_exact(8)
            .zip(b.chunks_exact(8))
        {
            let va = _mm512_load_pd(chunk_a.as_ptr());
            let vb = _mm512_load_pd(chunk_b.as_ptr());
            result = _mm512_add_pd(result, _mm512_mul_pd(va, vb));
        }

        _mm512_reduce_add_pd(result)
    }
}

/// AVX2 operations
pub mod avx2 {
    use super::*;

    #[inline]
    pub unsafe fn dot_product_f32(a: &[f32], b: &[f32]) -> f32 {
        debug_assert!(a.len() >= 8 && b.len() >= 8);
        debug_assert!(ensure_alignment(a.as_ptr(), 32));
        debug_assert!(ensure_alignment(b.as_ptr(), 32));

        let mut sum = _mm256_setzero_ps();

        for (chunk_a, chunk_b) in a.chunks_exact(8)
            .zip(b.chunks_exact(8))
        {
            let va = _mm256_load_ps(chunk_a.as_ptr());
            let vb = _mm256_load_ps(chunk_b.as_ptr());
            sum = _mm256_add_ps(sum, _mm256_mul_ps(va, vb));
        }

        let temp = _mm256_hadd_ps(sum, sum);
        let temp = _mm256_hadd_ps(temp, temp);
        let lo = _mm256_extractf128_ps(temp, 0);
        let hi = _mm256_extractf128_ps(temp, 1);
        _mm_cvtss_f32(_mm_add_ps(lo, hi))
    }

    #[inline]
    pub unsafe fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        debug_assert!(a.len() >= 4 && b.len() >= 4);
        debug_assert!(ensure_alignment(a.as_ptr(), 32));
        debug_assert!(ensure_alignment(b.as_ptr(), 32));

        let mut sum = _mm256_setzero_pd();

        for (chunk_a, chunk_b) in a.chunks_exact(4)
            .zip(b.chunks_exact(4))
        {
            let va = _mm256_load_pd(chunk_a.as_ptr());
            let vb = _mm256_load_pd(chunk_b.as_ptr());
            sum = _mm256_add_pd(sum, _mm256_mul_pd(va, vb));
        }

        let temp = _mm256_hadd_pd(sum, sum);
        let lo = _mm256_extractf128_pd(temp, 0);
        let hi = _mm256_extractf128_pd(temp, 1);
        _mm_cvtsd_f64(_mm_add_pd(lo, hi))
    }
}
