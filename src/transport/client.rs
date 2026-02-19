//! QUIC Client implementation

use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;
use bytes::BytesMut;
use prost::Message;
use quinn::{ClientConfig, Connection, Endpoint, TransportConfig};
use tokio::sync::mpsc;
use tokio::time::{interval, sleep};
use tracing::{debug, error, info, warn};

use super::client_conn::{ClientConn, run_client_conn};
use super::TransportHandler;
use crate::config::get_config;
use crate::iface::PacketIP;
use crate::protocol::{Envelope, MessagePacket, MessagePing, envelope};

pub struct Client<H: TransportHandler + 'static> {
    remote_addr: String,
    key: String,
    threads: usize,
    handler: Arc<H>,
    conns: Vec<Arc<ClientConn>>,
    serial: AtomicI64,
}

impl<H: TransportHandler + 'static> Client<H> {
    pub fn new(remote_addr: String, key: String, threads: usize, handler: Arc<H>) -> Self {
        Self {
            remote_addr,
            key,
            threads,
            handler,
            conns: Vec::new(),
            serial: AtomicI64::new(0),
        }
    }

    /// Start the client and connect to server
    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.conns = Vec::with_capacity(self.threads);

        for conn_index in 0..self.threads {
            match self.create_connection(conn_index).await {
                Ok(conn) => {
                    self.conns.push(conn);
                }
                Err(e) => {
                    warn!(
                        index = conn_index,
                        error = %e,
                        "Failed to create connection"
                    );
                }
            }
        }

        info!(
            server_addr = %self.remote_addr,
            conn_num = self.conns.len(),
            "Connections have been established"
        );

        // Start ping loop
        self.start_ping_loop();

        Ok(())
    }

    async fn create_connection(&self, index: usize) -> anyhow::Result<Arc<ClientConn>> {
        // Create QUIC endpoint
        let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
        
        // Configure TLS
        let crypto = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
            .with_no_client_auth();

        let mut transport_config = TransportConfig::default();
        transport_config.max_concurrent_bidi_streams(1000u32.into());
        transport_config.receive_window(quinn::VarInt::from_u32(6 * 1024 * 1024));
        transport_config.send_window(6 * 1024 * 1024);
        transport_config.keep_alive_interval(Some(Duration::from_secs(30)));

        let mut client_config = ClientConfig::new(Arc::new(
            quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?
        ));
        client_config.transport_config(Arc::new(transport_config));
        endpoint.set_default_client_config(client_config);

        // Resolve server address
        let server_addr = self.remote_addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Cannot resolve server address"))?;

        // Connect with retry
        let mut connected = false;
        let mut connection: Option<Connection> = None;
        
        for attempt in 0..10 {
            match endpoint.connect(server_addr, "localhost") {
                Ok(connecting) => {
                    match connecting.await {
                        Ok(conn) => {
                            connection = Some(conn);
                            connected = true;
                            break;
                        }
                        Err(e) => {
                            warn!(
                                attempt = attempt,
                                error = %e,
                                "Connection attempt failed"
                            );
                        }
                    }
                }
                Err(e) => {
                    warn!(
                        attempt = attempt,
                        error = %e,
                        "Failed to initiate connection"
                    );
                }
            }
            sleep(Duration::from_millis(500)).await;
        }

        if !connected {
            anyhow::bail!("Failed to connect to server after 10 attempts");
        }

        let quinn_conn = connection.unwrap();
        
        // Create ClientConn
        let (conn, write_rx, close_rx) = ClientConn::new(
            self.remote_addr.clone(),
            self.key.clone(),
            index,
        );
        let conn = Arc::new(conn);

        // Spawn connection handler
        let handler = self.handler.clone();
        let conn_clone = conn.clone();
        tokio::spawn(async move {
            if let Err(e) = run_client_conn(conn_clone, quinn_conn, handler, write_rx, close_rx).await {
                error!(index = index, error = %e, "Client connection error");
            }
        });

        Ok(conn)
    }

    fn start_ping_loop(&self) {
        let conns: Vec<Arc<ClientConn>> = self.conns.iter().cloned().collect();
        let remote_addr = self.remote_addr.clone();
        let key = self.key.clone();

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(1));
            loop {
                ticker.tick().await;
                
                for conn in &conns {
                    if conn.is_connected() {
                        send_ping(conn, &key).await;
                    }
                }
            }
        });
    }

    /// Send packet to server (load balanced across connections)
    pub async fn send_packet(&self, pkt: &PacketIP) {
        if self.conns.is_empty() {
            return;
        }

        let env = Envelope {
            r#type: Some(envelope::Type::Packet(MessagePacket {
                payload: pkt.as_bytes().to_vec(),
            })),
        };

        let data = env.encode_to_vec();
        
        if self.threads == 1 {
            self.conns[0].write(data).await;
        } else {
            let serial = self.serial.fetch_add(1, Ordering::Relaxed);
            let next = (serial as usize) % self.threads;
            if next < self.conns.len() {
                self.conns[next].write(data).await;
            }
        }
    }

    pub fn stop(&self) {
        for conn in &self.conns {
            let close_tx = conn.close_tx();
            tokio::spawn(async move {
                let _ = close_tx.send(()).await;
            });
        }
    }
}

async fn send_ping(conn: &Arc<ClientConn>, _key: &str) {
    let config = get_config();
    
    // Parse IP from CIDR
    let ip = config.ip.split('/').next().unwrap_or(&config.ip);
    let local_addr = format!("{}:{}", ip, conn.get_conn_port());

    let ping = MessagePing {
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64,
        local_addr: local_addr.clone(),
        local_private_addr: "not_use".to_string(),
        dc: "client".to_string(),
        ip: ip.to_string(),
    };

    let env = Envelope {
        r#type: Some(envelope::Type::Ping(ping)),
    };

    debug!(
        local_addr = %local_addr,
        client_vip = %ip,
        "Sending ping"
    );

    let data = env.encode_to_vec();
    conn.write(data).await;
}

/// Skip server certificate verification (for self-signed certs)
#[derive(Debug)]
struct SkipServerVerification;

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
        ]
    }
}
