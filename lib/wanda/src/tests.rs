// src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::net::UnixStream;

    #[tokio::test]
    async fn test_service_initialization() {
        let temp_dir = tempdir().unwrap();
        let socket_path = temp_dir.path().join("wanda.sock");

        let config = WandaConfig {
            watch_dir: PathBuf::from("/tmp"),
            socket_path: socket_path.clone(),
            scan_interval: Duration::from_secs(30),
            log_path: PathBuf::from("/tmp/wanda.log"),
            quantum_threshold: 0.75,
        };

        let service = WandaService::new(config).await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_command() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        std::fs::write(&test_file, "fn main() { println!(\"Hello, World!\"); }").unwrap();

        let message = WandaMessage::Analyze {
            path: test_file,
        };

        let response = send_test_message(message).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status, ResponseStatus::Success);
    }

    async fn send_test_message(message: WandaMessage) -> Result<WandaResponse, Box<dyn std::error::Error>> {
        let socket_path = "/var/run/wanda.sock";
        let mut stream = UnixStream::connect(socket_path).await?;

        let json = serde_json::to_vec(&message)?;
        stream.write_all(&json).await?;

        let mut response = Vec::new();
        stream.read_to_end(&mut response).await?;

        Ok(serde_json::from_slice(&response)?)
    }
}
