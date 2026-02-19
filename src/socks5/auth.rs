//! SOCKS5 Authentication

use std::collections::HashMap;
use std::io;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub const SOCKS5_VERSION: u8 = 5;
pub const NO_AUTH: u8 = 0;
pub const NO_ACCEPTABLE: u8 = 255;
pub const USER_PASS_AUTH: u8 = 2;
pub const USER_AUTH_VERSION: u8 = 1;
pub const AUTH_SUCCESS: u8 = 0;
pub const AUTH_FAILURE: u8 = 1;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User authentication failed")]
    UserAuthFailed,
    #[error("No supported authentication mechanism")]
    NoSupportedAuth,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Unsupported auth version: {0}")]
    UnsupportedVersion(u8),
}

/// Authentication context
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub method: u8,
    pub payload: HashMap<String, String>,
}

impl AuthContext {
    pub fn new(method: u8) -> Self {
        Self {
            method,
            payload: HashMap::new(),
        }
    }

    pub fn with_payload(method: u8, payload: HashMap<String, String>) -> Self {
        Self { method, payload }
    }
}

/// Credential store trait
pub trait CredentialStore: Send + Sync {
    fn valid(&self, username: &str, password: &str) -> bool;
}

/// Simple in-memory credential store
#[derive(Clone)]
pub struct StaticCredentials {
    credentials: HashMap<String, String>,
}

impl StaticCredentials {
    pub fn new() -> Self {
        Self {
            credentials: HashMap::new(),
        }
    }

    pub fn add(&mut self, username: String, password: String) {
        self.credentials.insert(username, password);
    }
}

impl Default for StaticCredentials {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialStore for StaticCredentials {
    fn valid(&self, username: &str, password: &str) -> bool {
        self.credentials
            .get(username)
            .map(|p| p == password)
            .unwrap_or(false)
    }
}

/// Authenticator enum - avoids dyn trait issues with async generics
#[derive(Clone)]
pub enum Authenticator {
    NoAuth,
    UserPass(StaticCredentials),
}

impl Authenticator {
    pub fn get_code(&self) -> u8 {
        match self {
            Authenticator::NoAuth => NO_AUTH,
            Authenticator::UserPass(_) => USER_PASS_AUTH,
        }
    }

    pub async fn authenticate<R, W>(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<AuthContext, AuthError>
    where
        R: AsyncReadExt + Unpin + Send,
        W: AsyncWriteExt + Unpin + Send,
    {
        match self {
            Authenticator::NoAuth => {
                writer.write_all(&[SOCKS5_VERSION, NO_AUTH]).await?;
                Ok(AuthContext::new(NO_AUTH))
            }
            Authenticator::UserPass(credentials) => {
                // Tell client to use user/pass auth
                writer.write_all(&[SOCKS5_VERSION, USER_PASS_AUTH]).await?;

                // Read version and username length
                let mut header = [0u8; 2];
                reader.read_exact(&mut header).await?;

                if header[0] != USER_AUTH_VERSION {
                    return Err(AuthError::UnsupportedVersion(header[0]));
                }

                // Read username
                let user_len = header[1] as usize;
                let mut user = vec![0u8; user_len];
                reader.read_exact(&mut user).await?;

                // Read password length
                let mut pass_len_buf = [0u8; 1];
                reader.read_exact(&mut pass_len_buf).await?;
                let pass_len = pass_len_buf[0] as usize;

                // Read password
                let mut pass = vec![0u8; pass_len];
                reader.read_exact(&mut pass).await?;

                let username = String::from_utf8_lossy(&user).to_string();
                let password = String::from_utf8_lossy(&pass).to_string();

                // Verify credentials
                if credentials.valid(&username, &password) {
                    writer.write_all(&[USER_AUTH_VERSION, AUTH_SUCCESS]).await?;
                    let mut payload = HashMap::new();
                    payload.insert("Username".to_string(), username);
                    Ok(AuthContext::with_payload(USER_PASS_AUTH, payload))
                } else {
                    writer.write_all(&[USER_AUTH_VERSION, AUTH_FAILURE]).await?;
                    Err(AuthError::UserAuthFailed)
                }
            }
        }
    }
}

/// Read authentication methods from client
pub async fn read_methods<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut header = [0u8; 1];
    reader.read_exact(&mut header).await?;

    let num_methods = header[0] as usize;
    let mut methods = vec![0u8; num_methods];
    reader.read_exact(&mut methods).await?;

    Ok(methods)
}

/// Send no acceptable auth response
pub async fn no_acceptable_auth<W: AsyncWriteExt + Unpin>(writer: &mut W) -> Result<(), AuthError> {
    writer.write_all(&[SOCKS5_VERSION, NO_ACCEPTABLE]).await?;
    Err(AuthError::NoSupportedAuth)
}
