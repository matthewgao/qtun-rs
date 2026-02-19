//! TUN interface module

pub mod packet;
pub mod tun;

pub use packet::PacketIP;
pub use tun::Iface;
