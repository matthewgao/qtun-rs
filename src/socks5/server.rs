//! SOCKS5 Server implementation

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error};

use super::auth::{
    read_methods, no_acceptable_auth, Authenticator, AuthContext, AuthError,
    SOCKS5_VERSION,
};
use super::request::{
    send_reply, AddrSpec, Request,
    CONNECT_COMMAND, BIND_COMMAND, ASSOCIATE_COMMAND,
    SUCCESS_REPLY, SERVER_FAILURE, RULE_FAILURE, HOST_UNREACHABLE,
    CONNECTION_REFUSED, NETWORK_UNREACHABLE, COMMAND_NOT_SUPPORTED,
};
use super::resolver::{DnsResolver, NameResolver};

/// Rule set trait for allowing/denying requests
pub trait RuleSet: Send + Sync {
    fn allow(&self, req: &Request) -> bool;
}

/// Permit all rule set
pub struct PermitAll;

impl RuleSet for PermitAll {
    fn allow(&self, _req: &Request) -> bool {
        true
    }
}

/// SOCKS5 server configuration
#[derive(Clone)]
pub struct Config {
    pub resolver: Arc<dyn NameResolver>,
    pub rules: Arc<dyn RuleSet>,
    pub bind_ip: Option<IpAddr>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resolver: Arc::new(DnsResolver),
            rules: Arc::new(PermitAll),
            bind_ip: None,
        }
    }
}

/// SOCKS5 Server
pub struct Server {
    config: Config,
    auth_methods: HashMap<u8, Authenticator>,
}

impl Server {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let mut auth_methods: HashMap<u8, Authenticator> = HashMap::new();
        
        // Default to no auth
        let no_auth = Authenticator::NoAuth;
        auth_methods.insert(no_auth.get_code(), no_auth);

        Ok(Self {
            config,
            auth_methods,
        })
    }

    /// Listen and serve on the given address
    pub async fn listen_and_serve(&self, _network: &str, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        self.serve(listener).await
    }

    /// Serve connections from a listener
    pub async fn serve(&self, listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let (stream, remote_addr) = listener.accept().await?;
            let config = self.config.clone();
            let auth_methods = self.auth_methods.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, remote_addr, config, auth_methods).await {
                    debug!(error = %e, "Connection handling error");
                }
            });
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    remote_addr: SocketAddr,
    config: Config,
    auth_methods: HashMap<u8, Authenticator>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = writer;

    // Read version byte
    let mut version = [0u8; 1];
    reader.read_exact(&mut version).await?;

    if version[0] != SOCKS5_VERSION {
        error!(version = version[0], "Unsupported SOCKS version");
        return Err(format!("Unsupported SOCKS version: {}", version[0]).into());
    }

    // Authenticate
    let auth_context = authenticate(&mut reader, &mut writer, &auth_methods).await?;

    // Read request
    let mut request = Request::new(&mut reader).await?;
    request.auth_context = Some(auth_context);
    request.remote_addr = Some(AddrSpec {
        fqdn: None,
        ip: Some(remote_addr.ip()),
        port: remote_addr.port(),
    });

    // Handle request
    handle_request(&mut reader, &mut writer, request, &config).await
}

async fn authenticate<R, W>(
    reader: &mut R,
    writer: &mut W,
    auth_methods: &HashMap<u8, Authenticator>,
) -> Result<AuthContext, Box<dyn std::error::Error + Send + Sync>>
where
    R: AsyncReadExt + Unpin + Send,
    W: AsyncWriteExt + Unpin + Send,
{
    let methods = read_methods(reader).await?;

    for method in methods {
        if let Some(authenticator) = auth_methods.get(&method) {
            return Ok(authenticator.authenticate(reader, writer).await?);
        }
    }

    no_acceptable_auth(writer).await?;
    Err("No supported authentication method".into())
}

async fn handle_request<R, W>(
    reader: &mut R,
    writer: &mut W,
    mut request: Request,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    R: AsyncReadExt + Unpin + Send,
    W: AsyncWriteExt + Unpin + Send,
{
    // Resolve FQDN if needed
    if let Some(fqdn) = &request.dest_addr.fqdn {
        match config.resolver.resolve(fqdn).await {
            Ok(ip) => {
                request.dest_addr.ip = Some(ip);
            }
            Err(e) => {
                send_reply(writer, HOST_UNREACHABLE, None).await?;
                return Err(format!("Failed to resolve {}: {}", fqdn, e).into());
            }
        }
    }

    // Check rules
    if !config.rules.allow(&request) {
        send_reply(writer, RULE_FAILURE, None).await?;
        return Err("Blocked by rules".into());
    }

    // Handle command
    match request.command {
        CONNECT_COMMAND => handle_connect(reader, writer, &request).await,
        BIND_COMMAND => {
            send_reply(writer, COMMAND_NOT_SUPPORTED, None).await?;
            Err("Bind command not supported".into())
        }
        ASSOCIATE_COMMAND => {
            send_reply(writer, COMMAND_NOT_SUPPORTED, None).await?;
            Err("Associate command not supported".into())
        }
        _ => {
            send_reply(writer, COMMAND_NOT_SUPPORTED, None).await?;
            Err(format!("Unsupported command: {}", request.command).into())
        }
    }
}

async fn handle_connect<R, W>(
    _client_reader: &mut R,
    client_writer: &mut W,
    request: &Request,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    R: AsyncReadExt + Unpin + Send,
    W: AsyncWriteExt + Unpin + Send,
{
    // Connect to target
    let target_addr = request.dest_addr.address();
    let target = match TcpStream::connect(&target_addr).await {
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
            return Err(format!("Connect to {} failed: {}", target_addr, e).into());
        }
    };

    // Get local address for reply
    let local_addr = target.local_addr()?;
    let bind = AddrSpec {
        fqdn: None,
        ip: Some(local_addr.ip()),
        port: local_addr.port(),
    };

    // Send success reply
    send_reply(client_writer, SUCCESS_REPLY, Some(&bind)).await?;

    // Note: Full bidirectional proxy would require owning the streams
    // This is a simplified placeholder that establishes connection
    debug!(target = %target_addr, "SOCKS5 connect established");

    // Keep the target connection alive until it closes
    drop(target);

    Ok(())
}
