use anyhow::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{error::Error, io};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

struct ProxyConfig {
    listen_addr: String,
    backend_addr: String,
    active_connections: Arc<AtomicUsize>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let config = ProxyConfig {
        listen_addr: "127.0.0.1:8080".to_string(),
        backend_addr: "127.0.0.1:3000".to_string(),
        active_connections: Arc::new(AtomicUsize::new(0)),
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

    let active = &config.active_connections.clone();
    loop {
        // When a TCP connection is requested, accept it
        match listener.accept().await {
            Ok((client_stream, client_addr)) => {
                info!("New connection: {:?}", client_addr);
                // Atomically increment active connections
                active.fetch_add(1, Ordering::SeqCst);
                info!("Active connections: {}", active.load(Ordering::SeqCst));
                let backend_addr = config.backend_addr.clone();

                // Spawn an async Tokio task to handle the connection
                // Note the **async move** syntax - Alp
                tokio::spawn(async move {
                    handle_connection(client_stream, client_addr.to_string(), backend_addr).await;
                });
                // Atomically decrement active connections
                active.fetch_sub(1, Ordering::SeqCst);
                info!("Active connections: {}", active.load(Ordering::SeqCst));
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

// Handle a single client connection
async fn handle_connection(
    mut client_stream: TcpStream,
    client_addr: String,
    backend_addr: String,
) -> Result<()> {
    // Connect to backend server
    let mut backend_stream = match TcpStream::connect(&backend_addr).await {
        Ok(stream) => {
            info!("Connected to backend {}", backend_addr);
            stream
        }
        Err(e) => {
            warn!("Failed to connect to backend {}: {}", backend_addr, e);
            // Try to send the error message to client before closing
            let _ = client_stream
                .write_all(b"HTTP/1.1 502 Bad Gateway\r\n\rBackend unavailable")
                .await;
            return Err(e.into());
        }
    };

    // Split streams into read and writes
    let (mut client_read, mut client_write) = client_stream.split();
    let (mut backend_read, mut backend_write) = backend_stream.split();

    // Copy data bidirectionally and simultaniuosly
    let client_to_backend = tokio::io::copy(&mut client_read, &mut backend_write);
    let backend_to_client = tokio::io::copy(&mut backend_read, &mut client_write);
    // Wait on the success of both or failure of 1 of the tasks
    match tokio::try_join!(client_to_backend, backend_to_client) {
        Ok((client_bytes, backend_bytes)) => {
            info!(
                "Connection Closed.\tClient->Backend: {}bytes\tBackend->Client: {}bytes",
                client_bytes, backend_bytes
            );
        }
        Err(e) => {
            warn!("Connection error: {}", e);
        }
    }

    Ok(())
}
