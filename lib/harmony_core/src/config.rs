// lib/harmony_core/src/config.rs

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct TunnelConfig {
    pub tunnel: TunnelSettings,
    pub resources: ResourceSettings,
    pub scaling: ScalingSettings,
    pub coherence: CoherenceSettings,
    pub monitoring: MonitoringSettings,
    pub dream_layers: DreamLayerSettings,
}

impl TunnelConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config_dir = std::env::var("SCRIBBLE_CONFIG_DIR")
        .unwrap_or_else(|_| "config".to_string());

        let env = std::env::var("SCRIBBLE_ENV")
        .unwrap_or_else(|_| "development".to_string());

        let settings = config::Config::builder()
        .add_source(config::File::with_name(&format!("{}/default/tunnel", config_dir)))
        .add_source(config::File::with_name(&format!("{}/{}/tunnel", config_dir, env)).required(false))
        .build()?;

        settings.try_deserialize()
    }
}
