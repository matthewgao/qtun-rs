//! SOCKS5 proxy module

pub mod auth;
pub mod request;
pub mod resolver;
pub mod server;

pub use auth::*;
pub use request::*;
pub use resolver::*;
pub use server::*;

use tracing::info;

/// Start the SOCKS5 server on the given port
pub async fn start_socks5(port: &str) {
    let addr = format!("0.0.0.0:{}", port);
    let config = Config::default();
    
    loop {
        let server = match Server::new(config.clone()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to create SOCKS5 server: {}", e);
                continue;
            }
        };

        info!(addr = %addr, "Starting SOCKS5 server");
        
        if let Err(e) = server.listen_and_serve("tcp", &addr).await {
            eprintln!("SOCKS5 server exit: {}, restarting", e);
        } else {
            println!("SOCKS5 server started");
        }
    }
}
