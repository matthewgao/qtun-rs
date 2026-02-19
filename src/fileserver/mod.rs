//! File server module for serving static files (e.g., PAC files)

use std::net::SocketAddr;
use axum::Router;
use tower_http::services::ServeDir;
use tracing::info;

/// Start the file server on the given port
pub async fn start(dir: &str, port: u16) {
    let dir = dir.to_string();
    
    tokio::spawn(async move {
        loop {
            info!(dir = %dir, port = port, "Starting file HTTP server");
            
            let app = Router::new()
                .nest_service("/", ServeDir::new(&dir));

            let addr = SocketAddr::from(([0, 0, 0, 0], port));
            
            match tokio::net::TcpListener::bind(addr).await {
                Ok(listener) => {
                    if let Err(e) = axum::serve(listener, app).await {
                        tracing::error!(error = %e, "File server error");
                    }
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to bind file server");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    });
}
