//! TUN interface implementation

use std::process::Command;
use anyhow::Result;
use ipnet::Ipv4Net;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, error, info};

use super::PacketIP;

#[cfg(target_os = "macos")]
use tun2::AbstractDevice;

/// TUN interface wrapper
pub struct Iface {
    name: String,
    ip: String,
    mtu: usize,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    device: Option<tun2::AsyncDevice>,
}

impl Iface {
    /// Create a new TUN interface
    pub fn new(name: &str, ip: &str, mtu: usize) -> Self {
        Self {
            name: name.to_string(),
            ip: ip.to_string(),
            mtu,
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            device: None,
        }
    }

    /// Start the TUN interface
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub async fn start(&mut self) -> Result<()> {
        // Parse IP/CIDR
        let network: Ipv4Net = self.ip.parse()?;
        let ip = network.addr();
        let netmask = network.netmask();

        // Create TUN config
        let mut config = tun2::Configuration::default();
        config.address(ip)
              .netmask(netmask)
              .mtu(self.mtu as u16)
              .up();

        #[cfg(target_os = "linux")]
        config.platform_config(|config| {
            config.ensure_root_privileges(true);
        });

        // Create TUN device
        let device = tun2::create_as_async(&config)?;
        
        #[cfg(target_os = "macos")]
        let tun_name = device.tun_name()?;
        #[cfg(target_os = "linux")]
        let tun_name = device.tun_name().unwrap_or_else(|_| "tun0".to_string());
        
        self.name = tun_name.clone();
        
        info!(tun_name = %tun_name, "TUN interface created");

        // Configure interface using system commands
        self.configure_interface(&ip.to_string(), &netmask.to_string())?;

        #[cfg(target_os = "macos")]
        self.add_system_route(&ip.to_string())?;

        self.device = Some(device);
        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    pub async fn start(&mut self) -> Result<()> {
        anyhow::bail!("TUN interface not supported on this platform")
    }

    fn configure_interface(&self, ip: &str, netmask: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            let output = Command::new("ifconfig")
                .args([
                    &self.name,
                    ip,
                    ip,
                    "netmask",
                    netmask,
                    "mtu",
                    &self.mtu.to_string(),
                    "up",
                ])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!(cmd_output = %stderr, "Run ifconfig fail");
                anyhow::bail!("Failed to configure interface: {}", stderr);
            }
        }

        #[cfg(target_os = "linux")]
        {
            let output = Command::new("ifconfig")
                .args([
                    &self.name,
                    ip,
                    "netmask",
                    netmask,
                    "mtu",
                    &self.mtu.to_string(),
                    "up",
                ])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!(cmd_output = %stderr, "Run ifconfig fail");
                anyhow::bail!("Failed to configure interface: {}", stderr);
            }
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn add_system_route(&self, ip: &str) -> Result<()> {
        // Calculate subnet from IP (e.g., 10.4.4.3 -> 10.4.4.0)
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            anyhow::bail!("Invalid IP address format");
        }
        let subnet = format!("{}.{}.{}.0", parts[0], parts[1], parts[2]);
        
        debug!(subnet = %subnet, "Adding route");

        let output = Command::new("route")
            .args(["add", "-net", &subnet, ip])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!(cmd_output = %stderr, "Add system route fail");
            // Don't fail if route already exists
            if !stderr.contains("exists") {
                anyhow::bail!("Failed to add route: {}", stderr);
            }
        }

        Ok(())
    }

    /// Get the interface name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Read a packet from the TUN interface
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub async fn read(&mut self, pkt: &mut PacketIP) -> Result<usize> {
        if let Some(device) = &mut self.device {
            let n = device.read(pkt.as_bytes_mut()).await?;
            Ok(n)
        } else {
            anyhow::bail!("TUN device not initialized")
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    pub async fn read(&mut self, _pkt: &mut PacketIP) -> Result<usize> {
        anyhow::bail!("TUN interface not supported on this platform")
    }

    /// Write a packet to the TUN interface
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub async fn write(&mut self, pkt: &PacketIP) -> Result<usize> {
        if let Some(device) = &mut self.device {
            let n = device.write(pkt.as_bytes()).await?;
            Ok(n)
        } else {
            anyhow::bail!("TUN device not initialized")
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    pub async fn write(&mut self, _pkt: &PacketIP) -> Result<usize> {
        anyhow::bail!("TUN interface not supported on this platform")
    }
}
