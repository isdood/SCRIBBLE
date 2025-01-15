// src/service.rs
use tokio::net::UnixListener;
use std::path::Path;
use crate::brain::WandaBrain;
use crate::types::{WandaMessage, WandaResponse, ResponseStatus};

pub struct WandaService {
    brain: WandaBrain,
    socket_path: PathBuf,
}

impl WandaService {
    pub async fn new(config: WandaConfig) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            brain: WandaBrain::new(),
           socket_path: config.socket_path,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if Path::new(&self.socket_path).exists() {
            std::fs::remove_file(&self.socket_path)?;
        }

        let listener = UnixListener::bind(&self.socket_path)?;

        while let Ok((mut stream, _)) = listener.accept().await {
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).await?;

            let message: WandaMessage = serde_json::from_slice(&buffer)?;
            let response = self.handle_message(message).await?;

            let response_json = serde_json::to_vec(&response)?;
            stream.write_all(&response_json).await?;
        }

        Ok(())
    }

    async fn handle_message(&mut self, message: WandaMessage) -> Result<WandaResponse, Box<dyn std::error::Error>> {
        match message {
            WandaMessage::Analyze { path } => {
                let suggestions = self.brain.analyze_code(&std::fs::read_to_string(path)?)?;
                Ok(WandaResponse {
                    status: ResponseStatus::Success,
                    message: "Analysis complete".to_string(),
                   suggestions,
                })
            },
            WandaMessage::Suggest { context } => {
                let suggestions = self.brain.analyze_code(&context)?;
                Ok(WandaResponse {
                    status: ResponseStatus::Success,
                    message: "Suggestions generated".to_string(),
                   suggestions,
                })
            },
            WandaMessage::Status => {
                Ok(WandaResponse {
                    status: ResponseStatus::Success,
                    message: format!("Quantum coherence: {:.2}", self.brain.get_coherence()),
                   suggestions: vec![],
                })
            },
            WandaMessage::Configure { config } => {
                // Apply configuration changes
                self.reconfigure(config).await?;
                Ok(WandaResponse {
                    status: ResponseStatus::Success,
                    message: "Configuration updated".to_string(),
                   suggestions: vec![],
                })
            }
        }
    }

    async fn reconfigure(&mut self, config: WandaConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Update service configuration
        self.socket_path = config.socket_path;
        Ok(())
    }
}
