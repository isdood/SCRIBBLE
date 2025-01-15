// src/types.rs
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub enum WandaMessage {
    Analyze {
        path: PathBuf,
    },
    Suggest {
        context: String,
    },
    Status,
    Configure {
        config: WandaConfig,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WandaConfig {
    pub watch_dir: PathBuf,
    pub socket_path: PathBuf,
    pub scan_interval: Duration,
    pub log_path: PathBuf,
    pub quantum_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WandaResponse {
    pub status: ResponseStatus,
    pub message: String,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error,
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub confidence: f64,
    pub description: String,
    pub impact: String,
    pub implementation: Option<String>,
}
