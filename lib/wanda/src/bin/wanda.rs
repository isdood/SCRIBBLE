/// Wanda CLI Tool
/// Last Updated: 2025-01-15 22:39:34 UTC
/// Author: isdood
/// Current User: isdood
///
/// Command-line interface for the Wanda AI Assistant service.
/// Provides functionality for:
/// - Analyzing files and directories
/// - Getting AI suggestions
/// - Checking service status
/// - Configuring service settings

use clap::{App, Arg, SubCommand};
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json;
use std::path::{Path, PathBuf};
use std::io;
use std::time::Duration;
use std::os::linux::fs::MetadataExt;

use wanda::{WandaConfig, WandaMessage, WandaResponse, get_socket_path};

const MAX_RETRIES: u32 = 5;
const RETRY_DELAY: Duration = Duration::from_millis(500);
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Attempts to connect to the Unix socket with retries
async fn connect_with_retry(socket_path: &Path) -> io::Result<UnixStream> {
    let mut attempts = 0;
    let mut last_error = None;

    while attempts < MAX_RETRIES {
        match UnixStream::connect(socket_path).await {
            Ok(stream) => {
                println!("Successfully connected to Wanda service");
                return Ok(stream);
            },
            Err(e) => {
                last_error = Some(e);
                attempts += 1;
                if attempts < MAX_RETRIES {
                    eprintln!("Connection attempt {} failed, retrying in {:?}...",
                              attempts, RETRY_DELAY);
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to connect after {} attempts", MAX_RETRIES)
        )
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("wanda")
    .version(VERSION)
    .author("isdood")
    .about("Wanda AI Assistant CLI")
    .subcommand(SubCommand::with_name("analyze")
    .about("Analyze a file or directory")
    .arg(Arg::with_name("path")
    .required(true)
    .help("Path to analyze")))
    .subcommand(SubCommand::with_name("suggest")
    .about("Get suggestions")
    .arg(Arg::with_name("context")
    .required(true)
    .help("Context for suggestions")))
    .subcommand(SubCommand::with_name("status")
    .about("Get Wanda's status"))
    .subcommand(SubCommand::with_name("config")
    .about("Configure Wanda")
    .arg(Arg::with_name("watch-dir")
    .long("watch-dir")
    .takes_value(true)
    .help("Directory to watch")));

    let matches = app.get_matches();

    // Connect to Wanda service using the standard socket path
    let socket_path = get_socket_path();
    println!("Connecting to Wanda service at {:?}...", socket_path);

    if !socket_path.exists() {
        eprintln!("Error: Socket file does not exist at {:?}", socket_path);
        eprintln!("Make sure the service is running: systemctl --user status wanda");
        std::process::exit(1);
    }

    let mut stream = match connect_with_retry(&socket_path).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error: Could not connect to Wanda service at {:?}", socket_path);
            eprintln!("Make sure the service is running: systemctl --user status wanda");
            eprintln!("Error details: {}", e);
            if let Ok(metadata) = std::fs::metadata(&socket_path) {
                eprintln!("Socket permissions: {:o}", metadata.st_mode() & 0o777);
            }
            std::process::exit(1);
        }
    };

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
                    socket_path: socket_path.clone(),
                    scan_interval: Duration::from_secs(30),
                    log_path: get_socket_path().with_extension("log"),
                    quantum_threshold: 0.75,
                };
                let message = WandaMessage::Configure { config };
                send_message(&mut stream, &message).await?;
            }
        }
        _ => {
            println!("Use --help for usage information");
        }
    }

    Ok(())
}

async fn send_message(stream: &mut UnixStream, message: &WandaMessage)
-> Result<(), Box<dyn std::error::Error>>
{
    // Send the message
    let json = serde_json::to_vec(message)?;
    stream.write_all(&json).await?;
    stream.shutdown().await?; // Add this line to signal we're done writing

    // Read the response
    let mut buffer = Vec::with_capacity(1024);
    let mut temp_buf = [0u8; 1024];

    loop {
        match stream.read(&mut temp_buf).await {
            Ok(0) => break, // EOF
            Ok(n) => {
                buffer.extend_from_slice(&temp_buf[..n]);
                if buffer.len() > 1024 * 1024 { // 1MB limit
                    return Err("Response too large".into());
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    if buffer.is_empty() {
        return Err("Empty response from server".into());
    }

    let wanda_response: WandaResponse = serde_json::from_slice(&buffer)?;
    println!("{}", serde_json::to_string_pretty(&wanda_response)?);

    Ok(())
}
