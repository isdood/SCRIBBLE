/// Wanda Daemon
/// Last Updated: 2025-01-15 22:44:30 UTC
/// Author: isdood
/// Current User: isdood

use wanda::{WandaService, WandaConfig};
use wanda::paths::{ensure_runtime_dirs, get_socket_path, get_log_path};
use std::time::Duration;
use env_logger;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    info!("Starting Wanda daemon...");

    // Ensure runtime directories exist
    ensure_runtime_dirs()?;

    // Create default configuration
    let config = WandaConfig {
        socket_path: get_socket_path(),
        watch_dir: std::env::current_dir()?,
        scan_interval: Duration::from_secs(30),
        log_path: get_log_path(),
        quantum_threshold: 0.75,
    };

    info!("Initializing service with socket path: {:?}", config.socket_path);

    // Create and run service
    let mut service = WandaService::new(config).await?;
    service.run().await?;

    Ok(())
}
