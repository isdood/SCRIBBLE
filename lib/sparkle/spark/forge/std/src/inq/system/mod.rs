//! System information querying

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
            use libc::{sysinfo, sysinfo as sysinfo_t};
            let mut info: sysinfo_t = unsafe { std::mem::zeroed() };
            if unsafe { sysinfo(&mut info) } == 0 {
                let unit = info.mem_unit as u64;
                return Self {
                    total: info.totalram * unit,
                    available: (info.freeram + info.bufferram) * unit,
                    used: info.totalram * unit - info.freeram * unit,
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
        let version = if cfg!(target_os = "linux") {
            std::fs::read_to_string("/etc/os-release")
                .ok()
                .and_then(|s| {
                    s.lines()
                        .find(|l| l.starts_with("VERSION_ID="))
                        .map(|l| l.trim_start_matches("VERSION_ID=").trim_matches('"').to_string())
                })
                .unwrap_or_else(|| String::from("unknown"))
        } else if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(&["/C", "ver"])
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .unwrap_or_else(|| String::from("unknown"))
        } else if cfg!(target_os = "macos") {
            std::process::Command::new("sw_vers")
                .arg("-productVersion")
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .unwrap_or_else(|| String::from("unknown"))
        } else {
            String::from("unknown")
        };

        Self {
            name: std::env::consts::OS.to_string(),
            version,
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
