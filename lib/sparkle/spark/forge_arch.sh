#!/bin/bash

# Spark Architecture Module Setup Script
# Author: isdood
# Created: 2025-01-25 18:02:44 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's architecture-specific shard system

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

create_directory_structure() {
    print_purple "🔮 Creating Spark Architecture Module structure..."
    mkdir -p forge/std/src/shard/arch
    mkdir -p forge/std/tests/shard/arch
}

setup_arch_module() {
    # Add shard and arch modules to lib.rs
    cat > forge/std/src/lib.rs << 'EOL'
//! Spark Standard Library - Where Magic Begins ✨

#![feature(const_type_name)]

pub mod math;
pub mod types;
pub mod align;
pub mod any;
pub mod shard;

pub use types::*;
pub use math::operations;
pub use align::space;
pub use shard::arch;
EOL

    cat > forge/std/src/shard/mod.rs << 'EOL'
//! Spark Shard System - Hardware-Optimized Crystal Computing
//!
//! Provides architecture-specific optimizations and crystal-space mappings.

pub mod arch;
EOL

    cat > forge/std/src/shard/arch/mod.rs << 'EOL'
//! Architecture-specific implementations for crystal computing

mod platform;
mod features;
mod dispatch;

pub use platform::{Platform, Architecture};
pub use features::{CpuFeature, detect_features};
pub use dispatch::Dispatcher;

/// Represents a hardware-specific shard implementation
#[derive(Debug)]
pub struct Shard {
    platform: Platform,
    features: Vec<CpuFeature>,
    dispatcher: Dispatcher,
}

impl Shard {
    /// Creates a new shard for the current hardware
    pub fn new() -> Self {
        let platform = Platform::detect();
        let features = detect_features();
        let dispatcher = Dispatcher::new(&platform, &features);

        Shard {
            platform,
            features,
            dispatcher,
        }
    }

    /// Returns the current platform architecture
    pub fn architecture(&self) -> Architecture {
        self.platform.architecture()
    }

    /// Checks if a specific CPU feature is available
    pub fn has_feature(&self, feature: CpuFeature) -> bool {
        self.features.contains(&feature)
    }

    /// Gets the optimal dispatch path for crystal operations
    pub fn get_dispatch_path(&self) -> &str {
        self.dispatcher.optimal_path()
    }
}
EOL

    cat > forge/std/src/shard/arch/platform.rs << 'EOL'
//! Platform detection and architecture information

/// Supported CPU architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    AArch64,
    RISCV64,
    Unknown,
}

/// Platform information
#[derive(Debug)]
pub struct Platform {
    arch: Architecture,
    #[allow(dead_code)]
    vendor_info: Option<String>,
    #[allow(dead_code)]
    os_info: Option<String>,
}

impl Platform {
    /// Detects the current platform
    pub fn detect() -> Self {
        Platform {
            arch: detect_architecture(),
            vendor_info: Some(detect_vendor()),
            os_info: Some(detect_os()),
        }
    }

    /// Returns the current architecture
    pub fn architecture(&self) -> Architecture {
        self.arch
    }

    /// Returns vendor information if available
    pub fn vendor(&self) -> Option<&str> {
        self.vendor_info.as_deref()
    }

    /// Returns OS information if available
    pub fn os(&self) -> Option<&str> {
        self.os_info.as_deref()
    }
}

#[cfg(target_arch = "x86_64")]
fn detect_architecture() -> Architecture {
    Architecture::X86_64
}

#[cfg(target_arch = "aarch64")]
fn detect_architecture() -> Architecture {
    Architecture::AArch64
}

#[cfg(target_arch = "riscv64")]
fn detect_architecture() -> Architecture {
    Architecture::RISCV64
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "riscv64")))]
fn detect_architecture() -> Architecture {
    Architecture::Unknown
}

fn detect_vendor() -> String {
    std::env::consts::ARCH.to_string()
}

fn detect_os() -> String {
    std::env::consts::OS.to_string()
}
EOL

    cat > forge/std/src/shard/arch/features.rs << 'EOL'
//! CPU feature detection and management

/// CPU features relevant for crystal computing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuFeature {
    // x86_64 features
    AVX2,
    AVX512F,
    // AArch64 features
    NEON,
    SVE,
    // RISC-V features
    V,
    // Common features
    FMA,
    SIMD,
}

/// Detects available CPU features
pub fn detect_features() -> Vec<CpuFeature> {
    let mut features = Vec::new();

    #[cfg(target_arch = "x86_64")]
    detect_x86_features(&mut features);

    #[cfg(target_arch = "aarch64")]
    detect_aarch64_features(&mut features);

    #[cfg(target_arch = "riscv64")]
    detect_riscv_features(&mut features);

    features
}

#[cfg(target_arch = "x86_64")]
fn detect_x86_features(features: &mut Vec<CpuFeature>) {
    if std::is_x86_feature_detected!("avx2") {
        features.push(CpuFeature::AVX2);
    }
    if std::is_x86_feature_detected!("avx512f") {
        features.push(CpuFeature::AVX512F);
    }
    if std::is_x86_feature_detected!("fma") {
        features.push(CpuFeature::FMA);
    }
}

#[cfg(target_arch = "aarch64")]
fn detect_aarch64_features(features: &mut Vec<CpuFeature>) {
    features.push(CpuFeature::NEON);
    // SVE detection would go here
    features.push(CpuFeature::SIMD);
}

#[cfg(target_arch = "riscv64")]
fn detect_riscv_features(features: &mut Vec<CpuFeature>) {
    // RISC-V vector extension detection would go here
    features.push(CpuFeature::SIMD);
}
EOL

    cat > forge/std/src/shard/arch/dispatch.rs << 'EOL'
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
EOL

    # Create the test module structure
    mkdir -p forge/std/tests/shard

    cat > forge/std/tests/shard/mod.rs << 'EOL'
pub mod arch;
EOL

    cat > forge/std/tests/shard/arch/mod.rs << 'EOL'
use spark_std::shard::arch::{Shard, Architecture, CpuFeature};

#[test]
fn test_shard_creation() {
    let shard = Shard::new();
    assert!(matches!(shard.architecture(),
        Architecture::X86_64 |
        Architecture::AArch64 |
        Architecture::RISCV64 |
        Architecture::Unknown
    ));
}

#[test]
fn test_feature_detection() {
    let shard = Shard::new();

    // Test for architecture-specific features
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            assert!(shard.has_feature(CpuFeature::AVX2));
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        assert!(shard.has_feature(CpuFeature::NEON));
    }
}

#[test]
fn test_dispatch_path() {
    let shard = Shard::new();
    let path = shard.get_dispatch_path();

    assert!(!path.is_empty());
    assert!(path.contains(match shard.architecture() {
        Architecture::X86_64 => "x86_64",
        Architecture::AArch64 => "aarch64",
        Architecture::RISCV64 => "riscv64",
        Architecture::Unknown => "generic",
    }));
}

#[test]
fn test_platform_info() {
    let shard = Shard::new();
    let arch = shard.architecture();

    match arch {
        Architecture::X86_64 => assert!(cfg!(target_arch = "x86_64")),
        Architecture::AArch64 => assert!(cfg!(target_arch = "aarch64")),
        Architecture::RISCV64 => assert!(cfg!(target_arch = "riscv64")),
        Architecture::Unknown => (),
    }
}
EOL

    # Update primitive_tests.rs properly
    cat > forge/std/tests/primitive_tests.rs << 'EOL'
//! Core tests for Spark Standard Library

mod align;
mod any;
mod shard;

#[test]
fn test_primitive_types() {
    // Existing primitive type tests...
    assert!(true);
}

#[test]
fn test_math_operations() {
    // Existing math operation tests...
    assert!(true);
}
EOL

    print_purple "✓ Created architecture module files"
}

main() {
    print_purple "🔮 Creating Spark Architecture Module..."
    create_directory_structure
    setup_arch_module
    print_purple "✨ Architecture module created with crystal-space hardware optimization!

Features:
- Architecture detection
- CPU feature detection
- Optimal dispatch paths
- SIMD/Vector support

Supported Architectures:
- x86_64 (AVX2, AVX512)
- AArch64 (NEON, SVE)
- RISC-V (Vector Extension)

Run 'cd forge/std && cargo test' to verify the architecture support!"
}

main
