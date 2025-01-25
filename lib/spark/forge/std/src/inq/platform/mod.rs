//! Platform-specific information and features

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64;

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
            Self::X86_64 => {
                if cfg!(target_arch = "x86_64") {
                    unsafe {
                        // Check AVX and SSE support
                        let xcr0 = x86_64::_xgetbv(0);
                        let avx_mask = 0b110;
                        xcr0 & avx_mask == avx_mask
                    }
                } else {
                    false
                }
            },
            Self::AARCH64 => {
                if cfg!(target_arch = "aarch64") {
                    true // NEON is always available on AArch64
                } else {
                    false
                }
            },
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
#[derive(Debug)]
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
