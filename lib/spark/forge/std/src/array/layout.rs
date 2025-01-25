//! Memory layout management for crystal arrays

use super::CrystalArray;
use crate::align::Alignment;
use std::mem;

impl<T> CrystalArray<T> {
    /// Returns the optimal alignment for the current architecture
    pub fn optimal_alignment() -> Alignment {
        let shard = crate::shard::arch::Shard::new();

        match shard.architecture() {
            crate::shard::arch::Architecture::X86_64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::AVX512F) {
                    Alignment::Vector64
                } else if shard.has_feature(crate::shard::arch::CpuFeature::AVX2) {
                    Alignment::Vector32
                } else {
                    Alignment::Vector16
                }
            }
            crate::shard::arch::Architecture::AArch64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::SVE) {
                    Alignment::Vector64
                } else {
                    Alignment::Vector16
                }
            }
            _ => Alignment::Crystal16,
        }
    }

    /// Returns true if the array's memory layout is optimal for SIMD operations
    pub fn is_simd_aligned(&self) -> bool {
        (self.ptr.as_ptr() as usize) % (self.alignment as usize) == 0
    }

    /// Returns the size of a SIMD vector for the current architecture
    pub fn vector_size() -> usize {
        let shard = crate::shard::arch::Shard::new();

        match shard.architecture() {
            crate::shard::arch::Architecture::X86_64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::AVX512F) {
                    64
                } else if shard.has_feature(crate::shard::arch::CpuFeature::AVX2) {
                    32
                } else {
                    16
                }
            }
            crate::shard::arch::Architecture::AArch64 => {
                if shard.has_feature(crate::shard::arch::CpuFeature::SVE) {
                    64
                } else {
                    16
                }
            }
            _ => 16,
        }
    }
}
