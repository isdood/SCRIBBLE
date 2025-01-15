use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WandaMessage {
    Status,
    Analyze { path: PathBuf },
    Suggest { context: String },
    Configure { config: WandaConfig },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WandaResponse {
    Status { version: String, uptime: u64 },
    Error { message: String },
}

impl WandaResponse {
    pub fn error(message: String) -> Self {
        WandaResponse::Error { message }
    }

    pub fn status(version: String, uptime: u64) -> Self {
        WandaResponse::Status { version, uptime }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WandaConfig {
    pub socket_path: PathBuf,
    pub watch_dir: PathBuf,
    pub scan_interval: Duration,
    pub log_path: PathBuf,
    pub quantum_threshold: f64,
}
