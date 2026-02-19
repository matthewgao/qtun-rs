//! DNS Resolver

use std::net::{IpAddr, ToSocketAddrs};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Failed to resolve hostname: {0}")]
    ResolveFailed(String),
}

/// Name resolver trait
#[async_trait]
pub trait NameResolver: Send + Sync {
    async fn resolve(&self, name: &str) -> Result<IpAddr, ResolverError>;
}

/// DNS resolver using system DNS
pub struct DnsResolver;

#[async_trait]
impl NameResolver for DnsResolver {
    async fn resolve(&self, name: &str) -> Result<IpAddr, ResolverError> {
        // Use blocking DNS resolution in a spawn_blocking context
        let name = name.to_string();
        tokio::task::spawn_blocking(move || {
            let addr = format!("{}:0", name);
            addr.to_socket_addrs()
                .map_err(|e| ResolverError::ResolveFailed(e.to_string()))?
                .next()
                .map(|a| a.ip())
                .ok_or_else(|| ResolverError::ResolveFailed("No addresses found".to_string()))
        })
        .await
        .map_err(|e| ResolverError::ResolveFailed(e.to_string()))?
    }
}

impl Default for DnsResolver {
    fn default() -> Self {
        Self
    }
}
