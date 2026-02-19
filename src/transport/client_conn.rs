//! Client connection handling

use std::sync::Arc;
use bytes::{Buf, BufMut, BytesMut};
use quinn::{Connection, RecvStream, SendStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use super::crypto::{Aes128GcmCipher, generate_nonce, NONCE_SIZE};
use super::TransportHandler;

const READ_BUF_SIZE: usize = 65536;

pub struct ClientConn {
    remote_addr: String,
    key: String,
    index: usize,
    cipher: Option<Aes128GcmCipher>,
    write_tx: mpsc::Sender<Vec<u8>>,
    close_tx: mpsc::Sender<()>,
    connected: Arc<parking_lot::RwLock<bool>>,
    local_port: Arc<parking_lot::RwLock<String>>,
}

impl ClientConn {
    pub fn new(remote_addr: String, key: String, index: usize) -> (Self, mpsc::Receiver<Vec<u8>>, mpsc::Receiver<()>) {
        let (write_tx, write_rx) = mpsc::channel(256);
        let (close_tx, close_rx) = mpsc::channel(1);

        let conn = Self {
            remote_addr,
            key,
            index,
            cipher: None,
            write_tx,
            close_tx,
            connected: Arc::new(parking_lot::RwLock::new(false)),
            local_port: Arc::new(parking_lot::RwLock::new(String::new())),
        };

        (conn, write_rx, close_rx)
    }

    pub fn is_connected(&self) -> bool {
        *self.connected.read()
    }

    fn set_connected(&self, value: bool) {
        *self.connected.write() = value;
    }

    pub fn get_conn_port(&self) -> String {
        self.local_port.read().clone()
    }

    pub fn set_conn_port(&self, port: String) {
        *self.local_port.write() = port;
    }

    pub fn write_tx(&self) -> mpsc::Sender<Vec<u8>> {
        self.write_tx.clone()
    }

    pub fn close_tx(&self) -> mpsc::Sender<()> {
        self.close_tx.clone()
    }

    pub async fn write(&self, data: Vec<u8>) {
        if let Err(e) = self.write_tx.send(data).await {
            warn!("Failed to send data to write channel: {}", e);
        }
    }

    pub async fn close(&self) {
        let _ = self.close_tx.send(()).await;
    }
}

/// Run the client connection
pub async fn run_client_conn<H: TransportHandler + 'static>(
    conn: Arc<ClientConn>,
    connection: Connection,
    handler: Arc<H>,
    mut write_rx: mpsc::Receiver<Vec<u8>>,
    mut close_rx: mpsc::Receiver<()>,
) -> anyhow::Result<()> {
    // Open a bidirectional stream
    let (send_stream, recv_stream) = connection.open_bi().await?;

    // Extract local port from connection
    let local_addr = connection.local_ip().map(|ip| ip.to_string()).unwrap_or_default();
    // Try to get the port from the socket address
    // Note: Quinn doesn't expose local port directly, we'll use a workaround
    let port = "0".to_string(); // Placeholder - in real implementation, track this differently
    conn.set_conn_port(port);
    
    // Set connected
    conn.set_connected(true);
    info!(
        index = conn.index,
        remote_addr = %conn.remote_addr,
        "Successfully connected to server"
    );

    // Initialize cipher if key is provided
    let cipher = if !conn.key.is_empty() {
        Some(Aes128GcmCipher::new(&conn.key)?)
    } else {
        info!("Outgoing encryption disabled");
        None
    };

    // Spawn write process
    let write_cipher = cipher.clone();
    let write_conn = conn.clone();
    let write_handle = tokio::spawn(async move {
        write_process(write_conn, send_stream, write_cipher, write_rx, close_rx).await
    });

    // Run read process in current task
    let read_result = read_process(conn.clone(), recv_stream, cipher, handler).await;
    
    conn.set_connected(false);
    
    // Cancel write task
    write_handle.abort();
    
    read_result
}

async fn write_process(
    conn: Arc<ClientConn>,
    mut send_stream: SendStream,
    cipher: Option<Aes128GcmCipher>,
    mut write_rx: mpsc::Receiver<Vec<u8>>,
    mut close_rx: mpsc::Receiver<()>,
) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            Some(data) = write_rx.recv() => {
                if let Err(e) = write_data(&mut send_stream, &cipher, &data).await {
                    error!(
                        index = conn.index,
                        error = %e,
                        "Write failed"
                    );
                    break;
                }
            }
            _ = close_rx.recv() => {
                info!(index = conn.index, "Write process received close signal");
                break;
            }
        }
    }
    
    conn.set_connected(false);
    let _ = send_stream.finish();
    Ok(())
}

async fn read_process<H: TransportHandler>(
    conn: Arc<ClientConn>,
    mut recv_stream: RecvStream,
    cipher: Option<Aes128GcmCipher>,
    handler: Arc<H>,
) -> anyhow::Result<()> {
    let mut read_buf = vec![0u8; READ_BUF_SIZE];
    
    loop {
        match read_data(&mut recv_stream, &cipher, &mut read_buf).await {
            Ok(data) => {
                handler.client_on_data(data);
            }
            Err(e) => {
                error!(
                    index = conn.index,
                    remote_addr = %conn.remote_addr,
                    error = %e,
                    "Read failed"
                );
                break;
            }
        }
    }
    
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
    // Read secure flag
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
