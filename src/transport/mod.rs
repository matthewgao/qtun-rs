//! Transport layer module - QUIC based client/server

pub mod crypto;
pub mod client_conn;
pub mod server_conn;
pub mod client;
pub mod server;

pub use crypto::*;
pub use client_conn::ClientConn;
pub use server_conn::ServerConn;
pub use client::Client;
pub use server::Server;

use crate::iface::PacketIP;

/// Handler trait for processing data from transport layer
pub trait TransportHandler: Send + Sync {
    fn client_on_data(&self, data: Vec<u8>);
    fn server_on_data(&self, data: Vec<u8>, conn: std::sync::Arc<ServerConn>);
}
