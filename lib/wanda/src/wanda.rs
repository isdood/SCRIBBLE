/// Wanda CLI Interface
/// Last Updated: 2025-01-15 06:14:53 UTC
/// Author: isdood
/// Current User: isdood

use clap::{App, Arg, SubCommand};
use clap::{App, Arg, SubCommand};
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Add this line
use serde_json;
use std::path::PathBuf;

mod types; // Add this line
use types::{WandaConfig, WandaMessage, WandaResponse, print_response}; // Add this line

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("wanda")
    // ... (keep existing app configuration)

    let matches = app.get_matches();

    // Connect to Wanda service
    let mut stream = UnixStream::connect("/var/run/wanda.sock").await?;

    match matches.subcommand() {
        Some(("analyze", args)) => {
            let path = args.value_of("path").unwrap();
            let message = WandaMessage::Analyze {
                path: PathBuf::from(path),
            };
            send_message(&mut stream, &message).await?;
        }
        Some(("suggest", args)) => {
            let context = args.value_of("context").unwrap();
            let message = WandaMessage::Suggest {
                context: context.to_string(),
            };
            send_message(&mut stream, &message).await?;
        }
        Some(("status", _)) => {
            let message = WandaMessage::Status;
            send_message(&mut stream, &message).await?;
        }
        Some(("config", args)) => {
            if let Some(watch_dir) = args.value_of("watch-dir") {
                let config = WandaConfig {
                    watch_dir: PathBuf::from(watch_dir),
                    socket_path: PathBuf::from("/var/run/wanda.sock"),
                    scan_interval: std::time::Duration::from_secs(30),
                    log_path: PathBuf::from("/var/log/wanda.log"),
                    quantum_threshold: 0.75,
                };
                let message = WandaMessage::Configure { config };
                send_message(&mut stream, &message).await?;
            }
        }
        _ => println!("Use --help for usage information"),
    }

    Ok(())
}

async fn send_message(stream: &mut UnixStream, message: &WandaMessage) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_vec(message)?;
    stream.write_all(&json).await?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    let wanda_response: WandaResponse = serde_json::from_slice(&response)?;
    print_response(wanda_response);

    Ok(())
}
