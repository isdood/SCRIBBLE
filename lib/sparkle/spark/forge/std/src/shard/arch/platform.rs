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
