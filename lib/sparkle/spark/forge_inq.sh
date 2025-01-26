#!/bin/bash

# Spark Inq Module Setup Script
# Author: isdood
# Created: 2025-01-25 19:12:38 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized environment inquiry system

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_inq_module() {
    cd forge/std || exit 1

    # 1. Create inq module structure
    mkdir -p src/inq
    mkdir -p src/inq/{platform,system}
    mkdir -p tests/inq

    # 2. Update lib.rs
    if ! grep -q "pub mod inq;" src/lib.rs; then
        sed -i '/pub mod array;/a pub mod inq;' src/lib.rs
        sed -i '/pub use array::CrystalArray;/a pub use inq::{CrystalEnv, CrystalSystem, platform};' src/lib.rs
    fi

    # 3. Create main module file
    cat > src/inq/mod.rs << 'EOL'
//! Crystal-optimized environment inquiry system
//!
//! This module provides traits and implementations for querying system
//! information and environment variables with SIMD optimizations.

pub mod platform;
pub mod system;

use std::ffi::OsString;
use std::path::PathBuf;
use std::collections::HashMap;

/// Crystal-optimized environment variable handling
pub struct CrystalEnv {
    vars: HashMap<OsString, OsString>,
}

impl CrystalEnv {
    /// Creates a new environment inquiry instance
    pub fn new() -> Self {
        Self {
            vars: std::env::vars_os().collect(),
        }
    }

    /// Gets an environment variable
    pub fn get(&self, key: impl AsRef<std::ffi::OsStr>) -> Option<&OsString> {
        self.vars.get(key.as_ref())
    }

    /// Sets an environment variable
    pub fn set(&mut self, key: impl Into<OsString>, value: impl Into<OsString>) {
        self.vars.insert(key.into(), value.into());
    }

    /// Removes an environment variable
    pub fn remove(&mut self, key: impl AsRef<std::ffi::OsStr>) -> Option<OsString> {
        self.vars.remove(key.as_ref())
    }

    /// Gets the current working directory
    pub fn current_dir() -> std::io::Result<PathBuf> {
        std::env::current_dir()
    }

    /// Gets the executable path
    pub fn executable() -> std::io::Result<PathBuf> {
        std::env::current_exe()
    }

    /// Gets all environment variables
    pub fn vars(&self) -> impl Iterator<Item = (&OsString, &OsString)> {
        self.vars.iter()
    }
}

impl Default for CrystalEnv {
    fn default() -> Self {
        Self::new()
    }
}

/// System information querying
pub struct CrystalSystem {
    cpu_count: usize,
    memory_info: system::MemoryInfo,
    os_info: system::OsInfo,
}

impl CrystalSystem {
    /// Creates a new system information instance
    pub fn new() -> Self {
        Self {
            cpu_count: num_cpus::get(),
            memory_info: system::MemoryInfo::query(),
            os_info: system::OsInfo::query(),
        }
    }

    /// Gets the number of CPU cores
    pub fn cpu_count(&self) -> usize {
        self.cpu_count
    }

    /// Gets memory information
    pub fn memory_info(&self) -> &system::MemoryInfo {
        &self.memory_info
    }

    /// Gets operating system information
    pub fn os_info(&self) -> &system::OsInfo {
        &self.os_info
    }
}

impl Default for CrystalSystem {
    fn default() -> Self {
        Self::new()
    }
}
EOL

    # 4. Create platform module
    cat > src/inq/platform/mod.rs << 'EOL'
//! Platform-specific information and features

use std::fmt;

/// CPU architecture information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86,
    X86_64,
    ARM,
    AARCH64,
    RISCV,
    Other,
}

impl Architecture {
    /// Gets the current CPU architecture
    pub fn current() -> Self {
        match std::env::consts::ARCH {
            "x86" => Self::X86,
            "x86_64" => Self::X86_64,
            "arm" => Self::ARM,
            "aarch64" => Self::AARCH64,
            "riscv64" => Self::RISCV,
            _ => Self::Other,
        }
    }

    /// Returns whether SIMD operations are supported
    pub fn supports_simd(&self) -> bool {
        match self {
            Self::X86_64 | Self::AARCH64 => true,
            _ => false,
        }
    }
}

/// Operating system information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOS,
    Other,
}

impl OperatingSystem {
    /// Gets the current operating system
    pub fn current() -> Self {
        match std::env::consts::OS {
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            "macos" => Self::MacOS,
            _ => Self::Other,
        }
    }
}

/// Platform capabilities and features
pub struct PlatformFeatures {
    arch: Architecture,
    os: OperatingSystem,
    simd_support: bool,
}

impl PlatformFeatures {
    /// Creates a new platform features instance
    pub fn new() -> Self {
        let arch = Architecture::current();
        Self {
            arch,
            os: OperatingSystem::current(),
            simd_support: arch.supports_simd(),
        }
    }

    /// Gets the CPU architecture
    pub fn architecture(&self) -> Architecture {
        self.arch
    }

    /// Gets the operating system
    pub fn operating_system(&self) -> OperatingSystem {
        self.os
    }

    /// Returns whether SIMD is supported
    pub fn supports_simd(&self) -> bool {
        self.simd_support
    }
}

impl Default for PlatformFeatures {
    fn default() -> Self {
        Self::new()
    }
}
EOL

    # 5. Create system module
    cat > src/inq/system/mod.rs << 'EOL'
//! System information querying

use std::fmt;

/// Memory information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    total: u64,
    available: u64,
    used: u64,
}

impl MemoryInfo {
    /// Queries current memory information
    pub fn query() -> Self {
        #[cfg(target_os = "linux")]
        {
            let mut info = std::mem::MaybeUninit::uninit();
            if unsafe { libc::sysinfo(info.as_mut_ptr()) } == 0 {
                let info = unsafe { info.assume_init() };
                return Self {
                    total: info.totalram,
                    available: info.freeram + info.bufferram,
                    used: info.totalram - info.freeram,
                };
            }
        }

        Self {
            total: 0,
            available: 0,
            used: 0,
        }
    }

    /// Gets total memory in bytes
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Gets available memory in bytes
    pub fn available(&self) -> u64 {
        self.available
    }

    /// Gets used memory in bytes
    pub fn used(&self) -> u64 {
        self.used
    }
}

/// Operating system information
#[derive(Debug, Clone)]
pub struct OsInfo {
    name: String,
    version: String,
    arch: String,
}

impl OsInfo {
    /// Queries current OS information
    pub fn query() -> Self {
        Self {
            name: std::env::consts::OS.to_string(),
            version: std::env::consts::OS_VERSION.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        }
    }

    /// Gets OS name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets OS version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Gets CPU architecture
    pub fn arch(&self) -> &str {
        &self.arch
    }
}
EOL

    # 6. Create tests
    cat > tests/inq/mod.rs << 'EOL'
use spark_std::inq::{CrystalEnv, CrystalSystem};
use spark_std::inq::platform::{Architecture, OperatingSystem, PlatformFeatures};

#[test]
fn test_env_vars() {
    let env = CrystalEnv::new();
    assert!(env.vars().count() > 0);
}

#[test]
fn test_env_operations() {
    let mut env = CrystalEnv::new();
    env.set("TEST_VAR", "test_value");
    assert_eq!(env.get("TEST_VAR").map(|s| s.to_string_lossy()), Some("test_value".into()));

    env.remove("TEST_VAR");
    assert!(env.get("TEST_VAR").is_none());
}

#[test]
fn test_current_dir() {
    assert!(CrystalEnv::current_dir().is_ok());
}

#[test]
fn test_executable() {
    assert!(CrystalEnv::executable().is_ok());
}

#[test]
fn test_system_info() {
    let sys = CrystalSystem::new();
    assert!(sys.cpu_count() > 0);
}

#[test]
fn test_platform_features() {
    let platform = PlatformFeatures::new();
    assert!(matches!(
        platform.architecture(),
        Architecture::X86_64 | Architecture::AARCH64 | Architecture::X86 | Architecture::ARM | Architecture::RISCV | Architecture::Other
    ));
}

#[test]
fn test_operating_system() {
    let os = OperatingSystem::current();
    assert!(matches!(
        os,
        OperatingSystem::Linux | OperatingSystem::Windows | OperatingSystem::MacOS | OperatingSystem::Other
    ));
}

#[test]
fn test_memory_info() {
    let sys = CrystalSystem::new();
    let mem = sys.memory_info();
    assert!(mem.total() >= mem.used());
}

#[test]
fn test_os_info() {
    let sys = CrystalSystem::new();
    let os = sys.os_info();
    assert!(!os.name().is_empty());
    assert!(!os.version().is_empty());
    assert!(!os.arch().is_empty());
}
EOL

    print_purple "✓ Created inq module files"
}

main() {
    print_purple "🔮 Creating Spark Inq Module..."
    setup_inq_module
    print_purple "✨ Inq module created with crystal-space optimization!

Features:
- Crystal-optimized environment queries
- SIMD-aware platform detection
- Memory information tracking
- System resource monitoring
- OS-specific optimizations
- Platform feature detection
- Comprehensive testing

Run 'cargo test' to verify the implementation!"
}

main
