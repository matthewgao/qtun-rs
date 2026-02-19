//! SOCKS5 Request handling

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use thiserror::Error;

use super::auth::{AuthContext, SOCKS5_VERSION};

pub const CONNECT_COMMAND: u8 = 1;
pub const BIND_COMMAND: u8 = 2;
pub const ASSOCIATE_COMMAND: u8 = 3;

pub const IPV4_ADDRESS: u8 = 1;
pub const FQDN_ADDRESS: u8 = 3;
pub const IPV6_ADDRESS: u8 = 4;

pub const SUCCESS_REPLY: u8 = 0;
pub const SERVER_FAILURE: u8 = 1;
pub const RULE_FAILURE: u8 = 2;
pub const NETWORK_UNREACHABLE: u8 = 3;
pub const HOST_UNREACHABLE: u8 = 4;
pub const CONNECTION_REFUSED: u8 = 5;
pub const TTL_EXPIRED: u8 = 6;
pub const COMMAND_NOT_SUPPORTED: u8 = 7;
pub const ADDR_TYPE_NOT_SUPPORTED: u8 = 8;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Unrecognized address type")]
    UnrecognizedAddrType,
    #[error("Unsupported command version: {0}")]
    UnsupportedVersion(u8),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Command not supported: {0}")]
    CommandNotSupported(u8),
    #[error("Blocked by rules")]
    BlockedByRules,
}

/// Address specification
#[derive(Debug, Clone)]
pub struct AddrSpec {
    pub fqdn: Option<String>,
    pub ip: Option<IpAddr>,
    pub port: u16,
}

impl AddrSpec {
    pub fn new() -> Self {
        Self {
            fqdn: None,
            ip: None,
            port: 0,
        }
    }

    pub fn address(&self) -> String {
        if let Some(ip) = &self.ip {
            format!("{}:{}", ip, self.port)
        } else if let Some(fqdn) = &self.fqdn {
            format!("{}:{}", fqdn, self.port)
        } else {
            format!("0.0.0.0:{}", self.port)
        }
    }
}

impl Default for AddrSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AddrSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(fqdn) = &self.fqdn {
            if let Some(ip) = &self.ip {
                write!(f, "{} ({})", fqdn, ip)?;
            } else {
                write!(f, "{}", fqdn)?;
            }
        } else if let Some(ip) = &self.ip {
            write!(f, "{}", ip)?;
        }
        write!(f, ":{}", self.port)
    }
}

/// SOCKS5 Request
#[derive(Debug)]
pub struct Request {
    pub version: u8,
    pub command: u8,
    pub auth_context: Option<AuthContext>,
    pub remote_addr: Option<AddrSpec>,
    pub dest_addr: AddrSpec,
}

impl Request {
    /// Create a new request by reading from the connection
    pub async fn new<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Self, RequestError> {
        // Read header: version, command, reserved
        let mut header = [0u8; 3];
        reader.read_exact(&mut header).await?;

        if header[0] != SOCKS5_VERSION {
            return Err(RequestError::UnsupportedVersion(header[0]));
        }

        // Read destination address
        let dest_addr = read_addr_spec(reader).await?;

        Ok(Self {
            version: SOCKS5_VERSION,
            command: header[1],
            auth_context: None,
            remote_addr: None,
            dest_addr,
        })
    }
}

/// Read address specification from reader
pub async fn read_addr_spec<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<AddrSpec, RequestError> {
    let mut addr_type = [0u8; 1];
    reader.read_exact(&mut addr_type).await?;

    let mut spec = AddrSpec::new();

    match addr_type[0] {
        IPV4_ADDRESS => {
            let mut addr = [0u8; 4];
            reader.read_exact(&mut addr).await?;
            spec.ip = Some(IpAddr::V4(Ipv4Addr::from(addr)));
        }
        IPV6_ADDRESS => {
            let mut addr = [0u8; 16];
            reader.read_exact(&mut addr).await?;
            spec.ip = Some(IpAddr::V6(Ipv6Addr::from(addr)));
        }
        FQDN_ADDRESS => {
            let mut len = [0u8; 1];
            reader.read_exact(&mut len).await?;
            let fqdn_len = len[0] as usize;
            let mut fqdn = vec![0u8; fqdn_len];
            reader.read_exact(&mut fqdn).await?;
            spec.fqdn = Some(String::from_utf8_lossy(&fqdn).to_string());
        }
        _ => {
            return Err(RequestError::UnrecognizedAddrType);
        }
    }

    // Read port
    let mut port = [0u8; 2];
    reader.read_exact(&mut port).await?;
    spec.port = u16::from_be_bytes(port);

    Ok(spec)
}

/// Send reply to client
pub async fn send_reply<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    resp: u8,
    addr: Option<&AddrSpec>,
) -> io::Result<()> {
    let (addr_type, addr_body, addr_port) = match addr {
        None => (IPV4_ADDRESS, vec![0u8; 4], 0u16),
        Some(a) => {
            if let Some(fqdn) = &a.fqdn {
                let mut body = vec![fqdn.len() as u8];
                body.extend_from_slice(fqdn.as_bytes());
                (FQDN_ADDRESS, body, a.port)
            } else if let Some(IpAddr::V4(ip)) = &a.ip {
                (IPV4_ADDRESS, ip.octets().to_vec(), a.port)
            } else if let Some(IpAddr::V6(ip)) = &a.ip {
                (IPV6_ADDRESS, ip.octets().to_vec(), a.port)
            } else {
                (IPV4_ADDRESS, vec![0u8; 4], 0u16)
            }
        }
    };

    let mut msg = vec![SOCKS5_VERSION, resp, 0, addr_type];
    msg.extend_from_slice(&addr_body);
    msg.push((addr_port >> 8) as u8);
    msg.push((addr_port & 0xff) as u8);

    writer.write_all(&msg).await
}

/// Proxy data between two streams
pub async fn proxy<R, W>(mut src: R, mut dst: W) -> io::Result<()>
where
    R: AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    let mut buf = vec![0u8; 8192];
    loop {
        let n = src.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        dst.write_all(&buf[..n]).await?;
    }
    Ok(())
}

/// Handle connect command - simplified placeholder
/// Note: Full bidirectional proxy requires owned streams
pub async fn handle_connect_simple(
    dest_addr: &AddrSpec,
    client_writer: &mut (impl AsyncWriteExt + Unpin),
) -> Result<TcpStream, RequestError> {
    // Connect to target
    let target = match TcpStream::connect(dest_addr.address()).await {
        Ok(t) => t,
        Err(e) => {
            let msg = e.to_string();
            let resp = if msg.contains("refused") {
                CONNECTION_REFUSED
            } else if msg.contains("unreachable") {
                NETWORK_UNREACHABLE
            } else {
                HOST_UNREACHABLE
            };
            send_reply(client_writer, resp, None).await?;
            return Err(RequestError::ConnectionFailed(msg));
        }
    };

    // Get local address
    let local_addr = target.local_addr()?;
    let bind = AddrSpec {
        fqdn: None,
        ip: Some(local_addr.ip()),
        port: local_addr.port(),
    };

    // Send success reply
    send_reply(client_writer, SUCCESS_REPLY, Some(&bind)).await?;

    Ok(target)
}
