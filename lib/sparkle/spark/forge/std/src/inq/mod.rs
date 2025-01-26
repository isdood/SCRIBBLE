//! Crystal-optimized environment inquiry system

pub mod platform;
pub mod system;

use std::ffi::OsString;
use std::path::PathBuf;
use std::collections::HashMap;

/// Crystal-optimized environment variable handling
#[derive(Debug)]
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
#[derive(Debug)]
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
