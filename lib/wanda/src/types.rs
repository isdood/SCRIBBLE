// src/types.rs
use std::path::PathBuf;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WandaConfig {
    pub watch_dir: PathBuf,
    pub socket_path: PathBuf,
    pub scan_interval: Duration,
    pub log_path: PathBuf,
    pub quantum_threshold: f64,
}

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
pub struct WandaResponse {
    pub status: ResponseStatus,
    pub message: String,
    pub data: Option<ResponseData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseData {
    Analysis(Vec<String>),
    Suggestion(Vec<String>),
    Status(SystemStatus),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub quantum_coherence: f64,
    pub uptime: Duration,
    pub active_patterns: usize,
}

pub fn print_response(response: WandaResponse) {
    match response.status {
        ResponseStatus::Success => println!("✓ {}", response.message),
        ResponseStatus::Error => eprintln!("✗ {}", response.message),
    }

    if let Some(data) = response.data {
        match data {
            ResponseData::Analysis(items) | ResponseData::Suggestion(items) => {
                for item in items {
                    println!("  • {}", item);
                }
            }
            ResponseData::Status(status) => {
                println!("System Status:");
                println!("  • Quantum Coherence: {:.2}", status.quantum_coherence);
                println!("  • Uptime: {:?}", status.uptime);
                println!("  • Active Patterns: {}", status.active_patterns);
            }
        }
    }
}
