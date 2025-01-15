/// Path management for Wanda
/// Last Updated: 2025-01-15 22:44:30 UTC
/// Author: isdood

use std::path::PathBuf;

/// Get the Wanda runtime directory path
pub fn get_runtime_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    PathBuf::from(home).join(".local/share/wanda")
}

/// Get the socket path
pub fn get_socket_path() -> PathBuf {
    get_runtime_dir().join("wanda.sock")
}

/// Get the log file path
pub fn get_log_path() -> PathBuf {
    get_runtime_dir().join("wanda.log")
}

/// Create necessary runtime directories
pub fn ensure_runtime_dirs() -> std::io::Result<()> {
    let runtime_dir = get_runtime_dir();
    if !runtime_dir.exists() {
        std::fs::create_dir_all(&runtime_dir)?;
    }
    Ok(())
}
