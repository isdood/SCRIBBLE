use std::path::Path;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use crate::types::{WandaMessage, WandaResponse};
use crate::brain::WandaBrain;

pub struct WandaService {
    config: crate::WandaConfig,
    listener: Option<UnixListener>,
}

impl WandaService {
    pub async fn new(config: crate::WandaConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Remove existing socket file if it exists
        if Path::new(&config.socket_path).exists() {
            fs::remove_file(&config.socket_path)?;
        }

        // Ensure parent directory exists
        if let Some(parent) = config.socket_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Create and bind the listener
        let listener = UnixListener::bind(&config.socket_path)?;
        println!("Listening on {:?}", config.socket_path);

        // Set socket permissions to be readable/writable by user
        let perms = fs::Permissions::from_mode(0o666);
        fs::set_permissions(&config.socket_path, perms)?;

        Ok(Self {
            config,
            listener: Some(listener),
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = self.listener.take().unwrap();

        loop {
            match listener.accept().await {
                Ok((mut stream, _addr)) => {
                    println!("Client connected");
                    if let Err(e) = self.handle_client(&mut stream).await {
                        eprintln!("Error handling client: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Accept error: {}", e);
                }
            }
        }
    }

    async fn handle_client(&self, stream: &mut UnixStream) -> Result<(), Box<dyn std::error::Error>> {
        // Use a buffer with a reasonable size limit
        let mut buffer = Vec::with_capacity(1024);
        let mut temp_buf = [0u8; 1024];

        // Read until we get a complete message or hit EOF
        loop {
            match stream.read(&mut temp_buf).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    buffer.extend_from_slice(&temp_buf[..n]);
                    if buffer.len() > 1024 * 1024 { // 1MB limit
                        return Err("Message too large".into());
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        if buffer.is_empty() {
            return Ok(());
        }

        let message: WandaMessage = match serde_json::from_slice(&buffer) {
            Ok(msg) => msg,
            Err(e) => {
                let error_response = WandaResponse::error(format!("Invalid message format: {}", e));
                stream.write_all(&serde_json::to_vec(&error_response)?).await?;
                stream.flush().await?;
                return Ok(());
            }
        };

        let response = match message {
            WandaMessage::Status => {
                WandaResponse::status(
                    env!("CARGO_PKG_VERSION").to_string(),
                                      std::time::SystemTime::now()
                                      .duration_since(std::time::UNIX_EPOCH)
                                      .unwrap_or_default()
                                      .as_secs()
                )
            }
            WandaMessage::Configure { config } => {
                println!("Received new configuration for watch dir: {:?}", config.watch_dir);
                WandaResponse::status(
                    env!("CARGO_PKG_VERSION").to_string(),
                                      std::time::SystemTime::now()
                                      .duration_since(std::time::UNIX_EPOCH)
                                      .unwrap_or_default()
                                      .as_secs()
                )
            }
            WandaMessage::Analyze { path } => {
                if !path.exists() {
                    WandaResponse::error(format!("Path does not exist: {:?}", path))
                } else {
                    // Initialize brain for analysis
                    let brain = WandaBrain::new();
                    match brain.analyze_path(&path) {
                        Ok(suggestions) => WandaResponse::Analysis {
                            suggestions,
                            timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs()
                        },
                        Err(e) => WandaResponse::error(format!("Analysis failed: {}", e))
                    }
                }
            }
            WandaMessage::Suggest { context } => {
                // TODO: Implement suggestion logic
                WandaResponse::error("Suggestion feature not yet implemented".to_string())
            }
        };

        // Send response and ensure it's flushed
        stream.write_all(&serde_json::to_vec(&response)?).await?;
        stream.flush().await?;

        Ok(())
    }
}
