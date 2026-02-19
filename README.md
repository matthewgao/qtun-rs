# Qtun-RS

基于 QUIC 协议的 VPN 隧道工具 (Rust 版本)

## 功能特性

- **QUIC 隧道**: 基于 QUIC 协议的安全隧道传输，低延迟、高可靠
- **TUN 虚拟网卡**: 支持 macOS/Linux 的 TUN 设备，实现透明代理
- **SOCKS5 代理**: 内置 SOCKS5 代理服务器，支持无认证和用户名密码认证
- **PAC 自动代理**: HTTP 文件服务器分发 PAC 配置文件
- **AES-GCM 加密**: 使用 AES-128-GCM 加密所有隧道流量
- **多连接负载均衡**: 客户端支持多线程并发连接

## 编译

```bash
cd qtun-rs
cargo build --release
```

编译产物位于 `target/release/qtun`

## 使用方法

### 服务端模式

在服务器上运行（需要 root 权限创建 TUN 设备）：

```bash
sudo ./qtun --server-mode \
    --listen 0.0.0.0:8080 \
    --ip 10.237.0.1/16 \
    --key your-secret-key
```

### 客户端模式

在客户端机器上运行（需要 root 权限）：

```bash
sudo ./qtun \
    --remote-addrs your-server-ip:8080 \
    --ip 10.237.0.2/16 \
    --key your-secret-key
```

### 仅代理模式

如果只需要 SOCKS5 代理功能，不需要 TUN 隧道：

```bash
./qtun --proxyonly \
    --remote-addrs your-server-ip:8080 \
    --key your-secret-key \
    --socks5-port 2080
```

## 命令行参数

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `--key` | hello-world | 加密密钥 |
| `--remote-addrs` | 2.2.2.2:8080 | 远程服务器地址（客户端） |
| `--listen` | 0.0.0.0:8080 | 监听地址（服务端） |
| `--ip` | 10.237.0.1/16 | VPN 虚拟 IP（CIDR 格式） |
| `--server-mode` | false | 服务端模式 |
| `--proxyonly` | false | 仅代理模式（不创建 TUN） |
| `--transport-threads` | 1 | 并发传输线程数（客户端） |
| `--mtu` | 1500 | MTU 大小 |
| `--socks5-port` | 2080 | SOCKS5 代理端口 |
| `--file-svr-port` | 6061 | HTTP 文件服务器端口 |
| `--file-dir` | ../static | 静态文件目录 |
| `--log-level` | info | 日志级别（info/debug） |
| `--nodelay` | false | TCP 无延迟模式 |

## 配置示例

### 场景 1: 完整 VPN 隧道

**服务端** (公网服务器 1.2.3.4):
```bash
sudo ./qtun --server-mode \
    --listen 0.0.0.0:8080 \
    --ip 10.237.0.1/16 \
    --key my-vpn-key
```

**客户端**:
```bash
sudo ./qtun \
    --remote-addrs 1.2.3.4:8080 \
    --ip 10.237.0.100/16 \
    --key my-vpn-key
```

### 场景 2: SOCKS5 代理

只使用 SOCKS5 代理，不创建 TUN 设备：

```bash
./qtun --proxyonly \
    --remote-addrs 1.2.3.4:8080 \
    --key my-vpn-key \
    --socks5-port 1080
```

然后配置浏览器或系统代理为 `socks5://127.0.0.1:1080`

### 场景 3: PAC 自动代理

启动后访问 PAC 文件：
```
http://127.0.0.1:6061/proxy.pac
```

可在系统网络设置中配置自动代理 URL。

## 网络架构

```
┌─────────────┐                              ┌─────────────┐
│   Client    │                              │   Server    │
│             │                              │             │
│ ┌─────────┐ │     QUIC (UDP:8080)          │ ┌─────────┐ │
│ │   TUN   │◄├──────────────────────────────┤►│   TUN   │ │
│ │10.237.x │ │     AES-GCM Encrypted        │ │10.237.0.1│ │
│ └─────────┘ │                              │ └─────────┘ │
│             │                              │             │
│ ┌─────────┐ │                              │             │
│ │ SOCKS5  │ │                              │             │
│ │ :2080   │ │                              │             │
│ └─────────┘ │                              │             │
└─────────────┘                              └─────────────┘
```

## 日志

设置环境变量启用详细日志：

```bash
RUST_LOG=debug ./qtun --server-mode ...
```

或使用 `--log-level debug` 参数。

## 注意事项

1. **权限要求**: 创建 TUN 设备需要 root/管理员权限
2. **防火墙**: 确保服务端的 UDP 端口（默认 8080）已开放
3. **IP 分配**: 服务端和客户端的虚拟 IP 应在同一子网内但不能相同
4. **密钥安全**: 生产环境请使用强密码作为加密密钥

## 技术栈

- **异步运行时**: Tokio
- **QUIC 协议**: Quinn
- **TUN 设备**: tun2
- **加密**: aes-gcm (AES-128-GCM)
- **HTTP 服务**: Axum
- **CLI**: Clap

## License

MIT License
