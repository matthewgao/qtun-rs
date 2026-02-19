//! Qtun - A VPN tunnel tool based on QUIC protocol

use clap::Parser;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use qtun::app::App;
use qtun::config::{init_config, Config};
use qtun::fileserver;
use qtun::socks5;

/// Command line options
#[derive(Parser, Debug)]
#[command(name = "qtun")]
#[command(version = "1.0.0")]
#[command(about = "A VPN tunnel tool based on QUIC protocol")]
struct CmdOpts {
    /// Encryption key
    #[arg(long, default_value = "hello-world")]
    key: String,

    /// Remote server address (client only)
    #[arg(long, default_value = "2.2.2.2:8080")]
    remote_addrs: String,

    /// Server listen address (server only)
    #[arg(long, default_value = "0.0.0.0:8080")]
    listen: String,

    /// VPN virtual IP with CIDR
    #[arg(long, default_value = "10.237.0.1/16")]
    ip: String,

    /// Log level (info, debug)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// HTTP file server directory
    #[arg(long, default_value = "../static")]
    file_dir: String,

    /// Concurrent transport threads (client only)
    #[arg(long, default_value = "1")]
    transport_threads: usize,

    /// MTU size
    #[arg(long, default_value = "1500")]
    mtu: usize,

    /// SOCKS5 server port
    #[arg(long, default_value = "2080")]
    socks5_port: u16,

    /// HTTP file server port
    #[arg(long, default_value = "6061")]
    file_svr_port: u16,

    /// Run in server mode
    #[arg(long, default_value = "false")]
    server_mode: bool,

    /// TCP no delay
    #[arg(long, default_value = "false")]
    nodelay: bool,

    /// Only enable proxy (no TUN)
    #[arg(long, default_value = "false")]
    proxyonly: bool,
}

fn init_logging(log_level: &str) {
    let filter = match log_level {
        "debug" => EnvFilter::new("debug"),
        "info" => EnvFilter::new("info"),
        "warn" => EnvFilter::new("warn"),
        "error" => EnvFilter::new("error"),
        _ => EnvFilter::new("info"),
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::debug!("This message appears only when log level set to Debug");
    tracing::info!("This message appears when log level set to Debug or Info");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = CmdOpts::parse();

    // Print options
    println!("{:?}", opts);

    // Initialize config
    init_config(Config {
        key: opts.key,
        remote_addrs: opts.remote_addrs,
        listen: opts.listen,
        transport_threads: opts.transport_threads,
        ip: opts.ip,
        mtu: opts.mtu,
        server_mode: opts.server_mode,
        no_delay: opts.nodelay,
    });

    // Initialize logging
    init_logging(&opts.log_level);

    // Proxy only mode
    if opts.proxyonly {
        let mut app = App::new();
        app.set_proxy();
        socks5::start_socks5(&opts.socks5_port.to_string()).await;
        return Ok(());
    }

    // Start services based on mode
    if opts.server_mode {
        // Server mode: start SOCKS5 server
        let socks5_port = opts.socks5_port.to_string();
        tokio::spawn(async move {
            socks5::start_socks5(&socks5_port).await;
        });
    } else {
        // Client mode: start file server
        fileserver::start(&opts.file_dir, opts.file_svr_port).await;
    }

    // Run main application
    let mut app = App::new();
    app.run().await
}
