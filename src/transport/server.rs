//! QUIC Server implementation

use std::sync::Arc;
use std::time::Duration;
use dashmap::DashMap;
use quinn::{Endpoint, ServerConfig, TransportConfig};
use rcgen::{CertifiedKey, generate_simple_self_signed};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use super::server_conn::{ServerConn, run_server_conn};
use super::TransportHandler;
use crate::config::get_config;

pub struct Server<H: TransportHandler + 'static> {
    public_addr: String,
    handler: Arc<H>,
    key: String,
    /// Map from address string to ServerConn
    conns: Arc<DashMap<String, Arc<ServerConn>>>,
    /// Reverse map from ServerConn pointer to address string
    conns_reverse: Arc<DashMap<usize, String>>,
}

impl<H: TransportHandler + 'static> Server<H> {
    pub fn new(public_addr: String, handler: Arc<H>, key: String) -> Self {
        Self {
            public_addr,
            handler,
            key,
            conns: Arc::new(DashMap::new()),
            conns_reverse: Arc::new(DashMap::new()),
        }
    }

    /// Start the server
    pub async fn start(&self) -> anyhow::Result<()> {
        if get_config().server_mode {
            self.start_listen().await?;
        }
        Ok(())
    }

    async fn start_listen(&self) -> anyhow::Result<()> {
        loop {
            match self.listen().await {
                Ok(_) => {
                    info!(addr = %self.public_addr, "Server listen exit");
                }
                Err(e) => {
                    error!(
                        addr = %self.public_addr,
                        error = %e,
                        "Server listen fail"
                    );
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn listen(&self) -> anyhow::Result<()> {
        // Generate TLS config
        let server_config = self.generate_server_config()?;

        // Configure QUIC
        let mut transport_config = TransportConfig::default();
        transport_config.max_concurrent_bidi_streams(1000u32.into());
        transport_config.receive_window(quinn::VarInt::from_u32(6 * 1024 * 1024));
        transport_config.send_window(6 * 1024 * 1024);
        transport_config.keep_alive_interval(Some(Duration::from_secs(30)));

        let mut config = server_config;
        config.transport_config(Arc::new(transport_config));

        // Create endpoint
        let endpoint = Endpoint::server(config, self.public_addr.parse()?)?;
        
        info!(addr = %self.public_addr, "Server listening");

        loop {
            // Accept connection
            let incoming = match endpoint.accept().await {
                Some(incoming) => incoming,
                None => {
                    info!("Endpoint closed");
                    break;
                }
            };

            let connection = match incoming.await {
                Ok(conn) => conn,
                Err(e) => {
                    warn!(error = %e, "Failed to accept connection");
                    continue;
                }
            };

            debug!(addr = %self.public_addr, "Server accepting stream");

            // Accept stream
            let (send_stream, recv_stream) = match connection.accept_bi().await {
                Ok(streams) => streams,
                Err(e) => {
                    warn!(error = %e, "Failed to accept stream");
                    continue;
                }
            };

            let remote_addr = connection.remote_address().to_string();
            info!(from = %remote_addr, "Server new connection");

            // Create ServerConn
            let (server_conn, write_rx, close_rx) = ServerConn::new(self.key.clone());
            let server_conn = Arc::new(server_conn);

            // Spawn connection handler
            let handler = self.handler.clone();
            let conn_clone = server_conn.clone();
            let conns = self.conns.clone();
            let conns_reverse = self.conns_reverse.clone();
            let conn_ptr = Arc::as_ptr(&server_conn) as usize;

            tokio::spawn(async move {
                let cleanup = {
                    let conns = conns.clone();
                    let conns_reverse = conns_reverse.clone();
                    move || {
                        // Remove connection from maps
                        if let Some((_, addr)) = conns_reverse.remove(&conn_ptr) {
                            conns.remove(&addr);
                        }
                        warn!(from = %remote_addr, "Server read thread exit");
                    }
                };

                if let Err(e) = run_server_conn(
                    conn_clone,
                    send_stream,
                    recv_stream,
                    handler,
                    write_rx,
                    close_rx,
                    cleanup,
                ).await {
                    error!(error = %e, "Server connection error");
                }
            });
        }

        info!(addr = %self.public_addr, "Server listener closed");
        Ok(())
    }

    fn generate_server_config(&self) -> anyhow::Result<ServerConfig> {
        // Generate self-signed certificate
        let subject_alt_names = vec!["localhost".to_string()];
        let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names)?;

        let cert_der = CertificateDer::from(cert.der().to_vec());
        let key_der = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key_pair.serialize_der()));

        let mut server_crypto = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![cert_der], key_der)?;
        server_crypto.alpn_protocols = vec![b"quic-echo-example".to_vec()];

        let server_config = ServerConfig::with_crypto(Arc::new(
            quinn::crypto::rustls::QuicServerConfig::try_from(server_crypto)?
        ));

        Ok(server_config)
    }

    /// Get connection by address
    pub fn get_conn_by_addr(&self, dst: &str) -> Option<Arc<ServerConn>> {
        self.conns.get(dst).map(|r| r.value().clone())
    }

    /// Delete dead connection
    pub fn delete_dead_conn(&self, dst: &str) {
        if let Some((_, conn)) = self.conns.remove(dst) {
            let conn_ptr = Arc::as_ptr(&conn) as usize;
            self.conns_reverse.remove(&conn_ptr);
            warn!(dest = %dst, "Delete dead conn");
        }
    }

    /// Set connection for address
    pub fn set_conn(&self, dst: String, server_conn: Arc<ServerConn>) {
        let conn_ptr = Arc::as_ptr(&server_conn) as usize;
        
        if let Some(existing) = self.conns.get(&dst) {
            if existing.is_closed() {
                // Replace closed connection
                self.conns.insert(dst.clone(), server_conn.clone());
                self.conns_reverse.insert(conn_ptr, dst);
            }
        } else {
            self.conns.insert(dst.clone(), server_conn.clone());
            self.conns_reverse.insert(conn_ptr, dst);
        }
    }

    /// Remove connection by pointer
    pub fn remove_conn_by_ptr(&self, conn: &Arc<ServerConn>) {
        let conn_ptr = Arc::as_ptr(conn) as usize;
        if let Some((_, dst)) = self.conns_reverse.remove(&conn_ptr) {
            self.conns.remove(&dst);
        }
    }
}
