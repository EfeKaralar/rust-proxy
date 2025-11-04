use anyhow::Result;
use std::{error::Error, io};
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

struct ProxyConfig {
    listen_addr: String,
    backend_addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let config = ProxyConfig {
        listen_addr: "127.0.0.1:8080".to_string(),
        backend_addr: "127.0.0.1:8080".to_string(),
    };

    info!("Starting proxy server on {}", config.listen_addr);
    info!("Forwarding to backend server on {}", config.backend_addr);

    run_proxy(config).await
}

// Main proxy loop - accepts connections and spawns handlers
async fn run_proxy(config: ProxyConfig) -> Result<()> {
    // Bind the listener to the address
    let listener = TcpListener::bind(&config.listen_addr).await?;
    info!("Proxy listening on {}", config.listen_addr);

    loop {
        // When a TCP connection is requested, accept it
        match listener.accept().await {
            Ok((client_stream, client_addr)) => {
                info!("New connection: {:?}", client_addr);
                let backend_addr = config.backend_addr.clone();

                // Spawn an async Tokio task to handle the connection
                // Note the **async move** syntax - Alp
                tokio::spawn(async move {
                    handle_connection(client_stream, client_addr.to_string(), backend_addr).await;
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

// Handle a single client connection
async fn handle_connection(client_stream: TcpStream, client_addr: String, backend_addr: String) {}
