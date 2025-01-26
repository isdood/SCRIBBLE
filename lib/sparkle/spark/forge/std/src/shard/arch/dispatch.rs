//! Crystal operation dispatch based on architecture

use super::platform::{Platform, Architecture};
use super::features::CpuFeature;

/// Dispatches crystal operations to optimal implementations
#[derive(Debug)]
pub struct Dispatcher {
    path: String,
}

impl Dispatcher {
    /// Creates a new dispatcher for the given platform and features
    pub fn new(platform: &Platform, features: &[CpuFeature]) -> Self {
        let path = match platform.architecture() {
            Architecture::X86_64 => {
                if features.contains(&CpuFeature::AVX512F) {
                    "x86_64/avx512"
                } else if features.contains(&CpuFeature::AVX2) {
                    "x86_64/avx2"
                } else {
                    "x86_64/baseline"
                }
            }
            Architecture::AArch64 => {
                if features.contains(&CpuFeature::SVE) {
                    "aarch64/sve"
                } else {
                    "aarch64/neon"
                }
            }
            Architecture::RISCV64 => {
                if features.contains(&CpuFeature::V) {
                    "riscv64/vector"
                } else {
                    "riscv64/baseline"
                }
            }
            Architecture::Unknown => "generic",
        }.to_string();

        Dispatcher { path }
    }

    /// Returns the optimal dispatch path
    pub fn optimal_path(&self) -> &str {
        &self.path
    }
}
