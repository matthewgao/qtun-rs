//! Configuration module

use std::sync::OnceLock;

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Config {
    pub key: String,
    pub remote_addrs: String,
    pub listen: String,
    pub transport_threads: usize,
    pub ip: String,
    pub mtu: usize,
    pub server_mode: bool,
    pub no_delay: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key: "hello-world".to_string(),
            remote_addrs: "0.0.0.0:8080".to_string(),
            listen: "0.0.0.0:8080".to_string(),
            transport_threads: 1,
            ip: "10.237.0.1/16".to_string(),
            mtu: 1500,
            server_mode: false,
            no_delay: false,
        }
    }
}

/// Initialize the global configuration
pub fn init_config(config: Config) {
    GLOBAL_CONFIG.set(config).expect("Config already initialized");
}

/// Get the global configuration instance
pub fn get_config() -> &'static Config {
    GLOBAL_CONFIG.get().expect("Config not initialized")
}

/// Try to get the global configuration instance
pub fn try_get_config() -> Option<&'static Config> {
    GLOBAL_CONFIG.get()
}
