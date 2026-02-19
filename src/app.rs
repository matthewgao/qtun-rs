//! Application core logic

use std::collections::HashSet;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use dashmap::DashMap;
use prost::Message;
use rand::Rng;
use tokio::sync::Mutex as TokioMutex;
use tracing::{debug, error, info, warn};

use crate::config::get_config;
use crate::iface::{Iface, PacketIP};
use crate::protocol::{Envelope, MessagePacket, MessagePing, envelope};
use crate::transport::{Client, Server, ServerConn, TransportHandler};
use crate::utils::Timer;

/// Route table: maps destination IP to set of connection addresses
type RouteTable = DashMap<String, HashSet<String>>;

pub struct App {
    routes: Arc<RouteTable>,
    server: Option<Arc<Server<AppHandler>>>,
    client: Option<Arc<TokioMutex<Client<AppHandler>>>>,
    iface: Option<Arc<TokioMutex<Iface>>>,
    timer: Timer,
}

/// Handler for transport layer callbacks
pub struct AppHandler {
    routes: Arc<RouteTable>,
    server: Arc<parking_lot::RwLock<Option<Arc<Server<AppHandler>>>>>,
    iface: Arc<parking_lot::RwLock<Option<Arc<TokioMutex<Iface>>>>>,
}

impl AppHandler {
    pub fn new(routes: Arc<RouteTable>) -> Self {
        Self {
            routes,
            server: Arc::new(parking_lot::RwLock::new(None)),
            iface: Arc::new(parking_lot::RwLock::new(None)),
        }
    }

    pub fn set_server(&self, server: Arc<Server<AppHandler>>) {
        *self.server.write() = Some(server);
    }

    pub fn set_iface(&self, iface: Arc<TokioMutex<Iface>>) {
        *self.iface.write() = Some(iface);
    }
}

impl TransportHandler for AppHandler {
    fn client_on_data(&self, data: Vec<u8>) {
        let env = match Envelope::decode(data.as_slice()) {
            Ok(e) => e,
            Err(e) => {
                error!(error = %e, "Failed to decode envelope");
                return;
            }
        };

        if let Some(envelope::Type::Packet(packet)) = env.r#type {
            let pkt = PacketIP::from_bytes(packet.payload);
            
            debug!(
                pkt_len = pkt.len(),
                src = %pkt.source_ip(),
                dst = %pkt.destination_ip(),
                "Received protobuf packet"
            );

            // Write to TUN interface
            let iface_opt = self.iface.read().clone();
            if let Some(iface) = iface_opt {
                tokio::spawn(async move {
                    let mut iface = iface.lock().await;
                    if let Err(e) = iface.write(&pkt).await {
                        error!(error = %e, "Failed to write to TUN interface");
                    }
                });
            }
        }
    }

    fn server_on_data(&self, data: Vec<u8>, conn: Arc<ServerConn>) {
        let env = match Envelope::decode(data.as_slice()) {
            Ok(e) => e,
            Err(e) => {
                error!(error = %e, "Failed to decode envelope");
                return;
            }
        };

        match env.r#type {
            Some(envelope::Type::Ping(ping)) => {
                // Add route based on ping info
                let ip = ping.ip.clone();
                let local_addr = ping.local_addr.clone();

                self.routes
                    .entry(ip.clone())
                    .or_insert_with(HashSet::new)
                    .insert(local_addr.clone());

                debug!(
                    local = %local_addr,
                    ip = %ip,
                    "Proto Ping"
                );

                // Log route table
                let routes: Vec<_> = self.routes.iter()
                    .map(|r| (r.key().clone(), r.value().clone()))
                    .collect();
                info!(route = ?routes, "Route Table");

                // Register connection with server
                if let Some(server) = self.server.read().as_ref() {
                    server.set_conn(local_addr, conn);
                }
            }
            Some(envelope::Type::Packet(packet)) => {
                let pkt = PacketIP::from_bytes(packet.payload);

                debug!(
                    pkt_len = pkt.len(),
                    src = %pkt.source_ip(),
                    dst = %pkt.destination_ip(),
                    "Received protobuf packet"
                );

                // Write to TUN interface
                let iface_opt = self.iface.read().clone();
                if let Some(iface) = iface_opt {
                    tokio::spawn(async move {
                        let mut iface = iface.lock().await;
                        if let Err(e) = iface.write(&pkt).await {
                            error!(error = %e, "Failed to write to TUN interface");
                        }
                    });
                }
            }
            None => {}
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(DashMap::new()),
            server: None,
            client: None,
            iface: None,
            timer: Timer::new(),
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let config = get_config();
        let handler = Arc::new(AppHandler::new(self.routes.clone()));

        if config.server_mode {
            // Server mode
            let server = Arc::new(Server::new(
                config.listen.clone(),
                handler.clone(),
                config.key.clone(),
            ));
            handler.set_server(server.clone());
            
            let server_clone = server.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.start().await {
                    error!(error = %e, "Server error");
                }
            });
            
            self.server = Some(server);
            self.start_clean_route();
        } else {
            // Client mode
            let mut client = Client::new(
                config.remote_addrs.clone(),
                config.key.clone(),
                config.transport_threads,
                handler.clone(),
            );
            client.start().await?;
            self.client = Some(Arc::new(TokioMutex::new(client)));
            self.set_proxy();
        }

        self.start_tun_interface(handler).await
    }

    fn start_clean_route(&mut self) {
        let routes = self.routes.clone();
        let server = self.server.clone();

        self.timer.register_task(
            move || {
                info!("Starting to clean route");
                
                let mut to_remove: Vec<(String, String)> = Vec::new();
                
                for entry in routes.iter() {
                    let dst = entry.key().clone();
                    for conn_addr in entry.value().iter() {
                        if let Some(server) = &server {
                            if let Some(conn) = server.get_conn_by_addr(conn_addr) {
                                if conn.is_closed() {
                                    to_remove.push((dst.clone(), conn_addr.clone()));
                                }
                            } else {
                                to_remove.push((dst.clone(), conn_addr.clone()));
                            }
                        }
                    }
                }

                for (dst, conn_addr) in to_remove {
                    info!(
                        conn = %conn_addr,
                        dst = %dst,
                        "Removing dead conn from route"
                    );
                    if let Some(mut conns) = routes.get_mut(&dst) {
                        conns.remove(&conn_addr);
                    }
                    if let Some(server) = &server {
                        server.delete_dead_conn(&conn_addr);
                    }
                }
            },
            Duration::from_secs(60),
        );
        self.timer.start();
    }

    async fn start_tun_interface(&mut self, handler: Arc<AppHandler>) -> anyhow::Result<()> {
        let config = get_config();
        let mut iface = Iface::new("", &config.ip, config.mtu);
        iface.start().await?;

        let iface = Arc::new(TokioMutex::new(iface));
        handler.set_iface(iface.clone());
        self.iface = Some(iface.clone());

        // Calculate number of workers
        let num_cpus = num_cpus::get();
        let num_workers = (num_cpus * 2).clamp(4, 32);

        info!(
            num_workers = num_workers,
            num_cpu = num_cpus,
            "Starting TUN packet workers"
        );

        // Spawn workers
        for i in 0..num_workers - 1 {
            let iface = iface.clone();
            let routes = self.routes.clone();
            let server = self.server.clone();
            let client = self.client.clone();
            
            tokio::spawn(async move {
                fetch_and_process_tun_pkt(i, iface, routes, server, client).await;
            });
        }

        // Run last worker in current task
        fetch_and_process_tun_pkt(
            num_workers - 1,
            iface,
            self.routes.clone(),
            self.server.clone(),
            self.client.clone(),
        ).await;

        Ok(())
    }

    pub fn set_proxy(&self) {
        #[cfg(target_os = "macos")]
        {
            let output = Command::new("networksetup")
                .args(["-setautoproxyurl", "Wi-Fi", "http://127.0.0.1:6061/proxy.pac"])
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        info!("Set system proxy successfully");
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        error!(cmd_output = %stderr, "Set system proxy fail");
                    }
                }
                Err(e) => {
                    error!(error = %e, "Failed to execute networksetup");
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            info!("Set system proxy not supported, please set it manually");
        }

        #[cfg(target_os = "windows")]
        {
            info!("Set system proxy not supported, please set it manually");
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

async fn fetch_and_process_tun_pkt(
    worker_num: usize,
    iface: Arc<TokioMutex<Iface>>,
    routes: Arc<RouteTable>,
    server: Option<Arc<Server<AppHandler>>>,
    client: Option<Arc<TokioMutex<Client<AppHandler>>>>,
) {
    let config = get_config();
    let mtu = config.mtu;
    let mut pkt = PacketIP::new(mtu);

    loop {
        // Read from TUN
        let n = {
            let mut iface = iface.lock().await;
            match iface.read(&mut pkt).await {
                Ok(n) => n,
                Err(e) => {
                    error!(error = %e, "Failed to read from TUN interface");
                    continue;
                }
            }
        };

        pkt.truncate(n);
        let src = pkt.source_ip().to_string();
        let dst = pkt.destination_ip().to_string();

        debug!(
            worker = worker_num,
            src = %src,
            dst = %dst,
            len = n,
            "Got TUN packet"
        );

        if config.server_mode {
            // Server mode: route packet to appropriate client
            if let Some(server) = &server {
                let mut found = false;
                
                if let Some(conns) = routes.get(&dst) {
                    if conns.is_empty() {
                        info!(
                            worker = worker_num,
                            src = %src,
                            dst = %dst,
                            "Has route but no connection, packet dropped"
                        );
                        pkt.resize(mtu);
                        continue;
                    }

                    // Pick random connection
                    let keys: Vec<_> = conns.iter().cloned().collect();
                    let idx = rand::thread_rng().gen_range(0..keys.len());
                    let conn_addr = &keys[idx];

                    if let Some(conn) = server.get_conn_by_addr(conn_addr) {
                        if conn.is_closed() {
                            info!(
                                worker = worker_num,
                                src = %src,
                                dst = %dst,
                                "Connection closed, removing"
                            );
                            drop(conns);
                            if let Some(mut entry) = routes.get_mut(&dst) {
                                entry.remove(conn_addr);
                            }
                            server.delete_dead_conn(conn_addr);
                        } else {
                            debug!(
                                worker = worker_num,
                                src = %src,
                                dst = %dst,
                                len = n,
                                "Sending packet"
                            );
                            conn.send_packet(&pkt).await;
                            found = true;
                        }
                    }
                }

                if !found {
                    info!(
                        worker = worker_num,
                        src = %src,
                        dst = %dst,
                        "No route, packet dropped"
                    );
                }
            }
        } else {
            // Client mode: send to server
            if let Some(client) = &client {
                client.lock().await.send_packet(&pkt).await;
            }
        }

        // Reset packet for next read
        pkt.resize(mtu);
    }
}

// Add num_cpus as a helper
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}
