//! Server connection handling

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use bytes::{BufMut, BytesMut};
use prost::Message;
use quinn::{RecvStream, SendStream, Connection as QuinnConnection};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use super::crypto::{Aes128GcmCipher, generate_nonce, NONCE_SIZE, CryptoError};
use super::TransportHandler;
use crate::iface::PacketIP;
use crate::protocol::{Envelope, MessagePacket, envelope};

const READ_BUF_SIZE: usize = 65536;

pub struct ServerConn {
    key: String,
    cipher: Option<Aes128GcmCipher>,
    write_tx: mpsc::Sender<Vec<u8>>,
    close_tx: mpsc::Sender<()>,
    is_closed: AtomicBool,
}

impl ServerConn {
    pub fn new(key: String) -> (Self, mpsc::Receiver<Vec<u8>>, mpsc::Receiver<()>) {
        let (write_tx, write_rx) = mpsc::channel(256);
        let (close_tx, close_rx) = mpsc::channel(1);

        // Initialize cipher if key is provided
        let cipher = if !key.is_empty() {
            match Aes128GcmCipher::new(&key) {
                Ok(c) => Some(c),
                Err(e) => {
                    error!("Failed to create cipher: {}", e);
                    None
                }
            }
        } else {
            warn!("Incoming encryption disabled");
            None
        };

        let conn = Self {
            key,
            cipher,
            write_tx,
            close_tx,
            is_closed: AtomicBool::new(false),
        };

        (conn, write_rx, close_rx)
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.load(Ordering::Relaxed)
    }

    pub fn set_closed(&self, value: bool) {
        self.is_closed.store(value, Ordering::Relaxed);
    }

    pub async fn write(&self, data: Vec<u8>) {
        if let Err(e) = self.write_tx.send(data).await {
            warn!("Failed to send data to write channel: {}", e);
        }
    }

    pub fn stop(&self) {
        let _ = self.close_tx.try_send(());
    }

    /// Send a packet through this connection
    pub async fn send_packet(&self, pkt: &PacketIP) {
        let env = Envelope {
            r#type: Some(envelope::Type::Packet(MessagePacket {
                payload: pkt.as_bytes().to_vec(),
            })),
        };
        
        let data = env.encode_to_vec();
        self.write(data).await;
    }
}

/// Run the server connection read/write processes
pub async fn run_server_conn<H: TransportHandler + 'static>(
    conn: Arc<ServerConn>,
    mut send_stream: SendStream,
    mut recv_stream: RecvStream,
    handler: Arc<H>,
    mut write_rx: mpsc::Receiver<Vec<u8>>,
    mut close_rx: mpsc::Receiver<()>,
    cleanup: impl FnOnce() + Send + 'static,
) -> anyhow::Result<()> {
    // Spawn write process
    let write_conn = conn.clone();
    let write_cipher_key = conn.key.clone();
    let write_handle = tokio::spawn(async move {
        let cipher = if !write_cipher_key.is_empty() {
            Aes128GcmCipher::new(&write_cipher_key).ok()
        } else {
            None
        };
        write_process(write_conn, send_stream, cipher, write_rx, close_rx).await
    });

    // Run read process
    let read_result = read_process(conn.clone(), recv_stream, handler).await;
    
    // Mark as closed
    conn.set_closed(true);
    conn.stop();
    
    // Wait for write process to finish
    write_handle.abort();
    
    // Run cleanup
    cleanup();
    
    read_result
}

async fn write_process(
    conn: Arc<ServerConn>,
    mut send_stream: SendStream,
    cipher: Option<Aes128GcmCipher>,
    mut write_rx: mpsc::Receiver<Vec<u8>>,
    mut close_rx: mpsc::Receiver<()>,
) -> anyhow::Result<()> {
    info!("ServerConn::ProcessWrite Start");
    
    loop {
        tokio::select! {
            Some(data) = write_rx.recv() => {
                if let Err(e) = write_data(&mut send_stream, &cipher, &data).await {
                    warn!(error = %e, "ServerConn::ProcessWrite End with error");
                    break;
                }
            }
            _ = close_rx.recv() => {
                info!("ServerConn::ProcessWrite stop");
                break;
            }
        }
    }
    
    conn.set_closed(true);
    let _ = send_stream.finish();
    warn!("ServerConn::ProcessWrite conn closed");
    Ok(())
}

async fn read_process<H: TransportHandler>(
    conn: Arc<ServerConn>,
    mut recv_stream: RecvStream,
    handler: Arc<H>,
) -> anyhow::Result<()> {
    let mut read_buf = vec![0u8; READ_BUF_SIZE];
    
    loop {
        match read_data(&mut recv_stream, &conn.cipher, &mut read_buf).await {
            Ok(data) => {
                handler.server_on_data(data, conn.clone());
            }
            Err(e) => {
                if let Some(crypto_err) = e.downcast_ref::<CryptoError>() {
                    if matches!(crypto_err, CryptoError::CipherNotMatch) {
                        error!("Fail to match key, break");
                        break;
                    }
                }
                error!(error = %e, "ServerConn::run conn read fail, break");
                break;
            }
        }
    }
    
    warn!("ServerConn::conn run, exit");
    Ok(())
}

async fn write_data(
    stream: &mut SendStream,
    cipher: &Option<Aes128GcmCipher>,
    data: &[u8],
) -> anyhow::Result<()> {
    let mut buf = BytesMut::with_capacity(data.len() + 32);
    
    let secure: u8 = if cipher.is_some() { 1 } else { 0 };
    buf.put_u8(secure);
    
    if secure == 0 {
        buf.put_u16_le(data.len() as u16);
        buf.put_slice(data);
    } else {
        let cipher = cipher.as_ref().unwrap();
        let nonce = generate_nonce();
        let encrypted = cipher.encrypt(&nonce, data)?;
        
        buf.put_u16_le(encrypted.len() as u16);
        buf.put_slice(&encrypted);
        buf.put_slice(&nonce);
    }
    
    stream.write_all(&buf).await?;
    Ok(())
}

async fn read_data(
    stream: &mut RecvStream,
    cipher: &Option<Aes128GcmCipher>,
    buf: &mut [u8],
) -> anyhow::Result<Vec<u8>> {
    // Read secure flag and length
    let mut header = [0u8; 3];
    stream.read_exact(&mut header).await?;
    
    let secure = header[0];
    let data_len = u16::from_le_bytes([header[1], header[2]]) as usize;
    
    if data_len > buf.len() {
        anyhow::bail!("Data too large: {}", data_len);
    }
    
    // Read data
    stream.read_exact(&mut buf[..data_len]).await?;
    
    if secure == 0 {
        Ok(buf[..data_len].to_vec())
    } else {
        // Read nonce
        let mut nonce = [0u8; NONCE_SIZE];
        stream.read_exact(&mut nonce).await?;
        
        let cipher = cipher.as_ref().ok_or_else(|| anyhow::anyhow!("Cipher not initialized"))?;
        let plain = cipher.decrypt(&nonce, &buf[..data_len])?;
        Ok(plain)
    }
}
