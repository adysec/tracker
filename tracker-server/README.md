# Tracker Server

BitTorrent Tracker 服务器，使用进程内存作为后端存储；仅提供标准 `/announce` 端点，并内置多协议上游聚合与负载均衡。

## 功能特性

- 标准 BitTorrent Tracker 协议（BEP‑3/23）：HTTP `/announce`，响应 bencode，支持 compact 和 dictionary 模式
- 智能上游聚合：从上游 Trackers 聚合 peers（HTTP/UDP/WebSocket），可并发抓取并写入本地内存存储
- 负载均衡策略：All / RoundRobin / Random，可通过环境变量切换
- 快速模式（Fast announce）：Wait / Timeout / Async / Off，多场景权衡响应时延与新鲜度
- TLS 回退：优先 rustls，失败时可回退 native-tls（可关闭）
- 原始 info_hash 解析：保留请求中的百分号编码原始字节，兼容 hex/base32/URN/double-encoded
- 自动清理：TTL 机制定期删除过期 peers
- 可静默告警：环境变量控制是否压制非致命告警

## 编译

```bash
cargo build --release
```

编译后的二进制文件位于 `target/release/tracker-server`。

## 使用方法

### 基本用法

```bash
./target/release/tracker-server \
  --bind 0.0.0.0:1337
```

### 命令行参数

| 参数 | 默认值 | 说明 |
|-----|--------|-----|
| `--bind` | 0.0.0.0:1337 | 服务器绑定地址和端口 |
| `--interval` | 900 | Announce 间隔（秒） |
| `--min-interval` | 450 | 最小 announce 间隔（秒） |
| `--peer-ttl` | 3600 | Peer 生存时间（秒） |

### 重要环境变量（可选）

- `LOAD_BALANCE_ALL=1`：强制所有请求聚合所有上游（忽略选择策略），默认关闭
- `UPSTREAM_STRATEGY=all|rr|random`：选择 HTTP 上游策略（默认 `rr`）
- `FAST_ANNOUNCE_MODE=off|wait|timeout|async`：快速模式（默认 `wait`）
- `FAST_MAX_WAIT_MS=xxx`：快速模式最长等待时间（默认 500ms）
- `AGG_CONCURRENCY=xxx`：聚合 HTTP 并发数（默认 32）
- `ENABLE_NATIVE_TLS_FALLBACK=1`：在 rustls 失败时尝试 native-tls（默认关闭）
- `TRACKERS_URLS=https://...;https://...`：上游 tracker 列表地址（多地址用分号分隔）
- `TRACKERS_FILE=/path/trackers.txt`：本地 tracker 列表文件（与 URL 二选一，URL 优先）
- `TRACKERS_REFRESH_SECS=86400`：tracker 列表刷新周期（默认 86400 秒）
- `POLL_INTERVAL_SECS=60`：后台轮询已知 info_hash 的聚合周期（默认 60 秒）
- `SUPPRESS_WARN=1`：关闭部分非致命告警输出

### 示例

```bash
# 使用默认配置
./target/release/tracker-server

# 自定义端口和间隔
./target/release/tracker-server \
  --bind 0.0.0.0:8080 \
  --interval 1800 \
  --peer-ttl 7200
```

## API 端点

### 1. `/announce` - BitTorrent Tracker Announce

**BitTorrent 客户端使用的标准 announce 端点。**

#### 请求参数（GET）

| 参数 | 必填 | 说明 |
|-----|------|-----|
| `info_hash` | 是 | 种子的 info hash（20 字节，URL 编码或 40 字节十六进制） |
| `peer_id` | 是 | 客户端 peer ID（20 字节，URL 编码） |
| `port` | 是 | 客户端监听端口 |
| `uploaded` | 否 | 已上传字节数 |
| `downloaded` | 否 | 已下载字节数 |
| `left` | 否 | 剩余字节数（0 表示做种） |
| `event` | 否 | 事件类型：`started`、`completed`、`stopped` |
| `numwant` | 否 | 期望获取的 peer 数量（默认 50，最大 200） |
| `compact` | 否 | 是否使用 compact 格式（1=是，0=否，默认 1） |
| `no_peer_id` | 否 | 是否省略 peer_id（1=是，0=否，默认 0） |
| `ip` | 否 | 客户端 IP 地址（可选覆盖） |

#### 客户端 IP 检测（支持 CDN/代理）

Tracker 会按以下优先级自动检测客户端真实 IP：

1. **查询参数 `?ip=`**：客户端自己声明的 IP（最高优先级）
2. **CF-Connecting-IP 头**：Cloudflare CDN 提供的真实客户端 IP（推荐用于 CF 场景）
   - 支持多种大小写变体：`CF-Connecting-IP`、`cf-connecting-ip`、`Cf-Connecting-Ip`
3. **X-Forwarded-For 头**：标准代理转发头，自动提取第一个 IP
   - 支持大小写变体：`X-Forwarded-For`、`x-forwarded-for`
4. **X-Real-IP 头**：Nginx 等反向代理常用的真实 IP 头
   - 支持大小写变体：`X-Real-IP`、`x-real-ip`
5. **TCP 连接 IP**：直连场景下的客户端 socket 地址（无代理时使用）

**大小写兼容性说明：**
- HTTP 头名称本身是大小写不敏感的（RFC 7230），但不同的代理/CDN 可能使用不同的大小写形式
- 本实现会尝试多种常见的大小写变体，确保最大兼容性
- 例如：Cloudflare 通常使用 `CF-Connecting-IP`，但部分配置可能使用小写形式

**Cloudflare CDN 部署说明：**

当 tracker-server 部署在 Cloudflare CDN 后面时：
- ✅ **可以正常获取**真实客户端 IP（通过 `CF-Connecting-IP` 头）
- ✅ **可以正常获取**客户端 Port（从查询参数 `?port=`）
- ⚠️ 需要在 Cloudflare 中配置将 tracker 域名设置为 **DNS Only**（灰云），或者：
  - 在 Cloudflare Page Rules 中为 `/announce` 路径禁用代理（Bypass Cache）
  - 原因：Cloudflare 默认会缓存某些 GET 请求，可能导致 announce 响应不准确

**其他 CDN/反向代理：**
- Nginx：会设置 `X-Real-IP` 和 `X-Forwarded-For`
- Apache：使用 `mod_remoteip` 设置 `X-Forwarded-For`
- 其他 CDN：通常会设置 `X-Forwarded-For`

**安全过滤：**
- 所有来自 `127.0.0.1` 的 peer 会被自动过滤，不会写入存储

#### 响应格式（Bencoded）

**Compact 模式（默认）：**
```
d8:intervali900e12:min intervali450e10:tracker id16:a1b2c3d4e5f6...8:completei5e10:incompletei10e5:peers<binary>e
```

peers 字段为二进制字符串，每 6 字节表示一个 peer（4 字节 IPv4 + 2 字节端口）。

**Dictionary 模式（compact=0）：**
```
d8:intervali900e...5:peersld2:ip12:192.168.1.1004:porti6881e7:peer id20:...eed2:ip13:203.0.113.454:porti51413eee
```

peers 字段为字典列表，每个字典包含 `ip`、`port`，可选 `peer id`。

#### 示例

```bash
# Compact 格式（默认）
curl 'http://localhost:1337/announce?info_hash=%01%23%45%67%89%AB%CD%EF%01%23%45%67%89%AB%CD%EF%01%23%45%67&peer_id=-UT3450-ABCDEFGHIJKL&port=6881&uploaded=0&downloaded=0&left=1234567890&event=started'

# Dictionary 格式
curl 'http://localhost:1337/announce?info_hash=0123456789ABCDEF0123456789ABCDEF01234567&peer_id=-UT3450-ABCDEFGHIJKL&port=6881&compact=0&left=0'

# Stopped 事件
curl 'http://localhost:1337/announce?info_hash=...&peer_id=...&port=6881&event=stopped'
```

注：自本版本起，已移除非标准的 `/peers/:info_hash` JSON API。

### info_hash 参数格式支持

`/announce` 支持以下多种 `info_hash` 表达形式：

- Percent-Encoded 原始 20 字节：`%aa%bb%cc...`（BEP‑3 标准格式）
- 40 字节十六进制：`0123456789abcdef...`（大小写不敏感）
- 32 字节 Base32（BTIH）：`MTGDK...`（来自 magnet 的 xt）
- 带前缀 URN：`urn:btih:...`（自动剥离）
- 双重编码：`%25aa%25bb...`（自动修正）

## 输出日志

所有日志采用结构化格式（tracing）：

### 启动日志
```
2025-01-14T12:00:00.123Z  INFO tracker_server: tracker-server started bind="0.0.0.0:1337"
2025-01-14T12:00:00.456Z  INFO tracker_server: starting HTTP server addr=0.0.0.0:1337
```

### Announce 日志
```
2025-01-14T12:01:23.456Z  INFO tracker_server: announce served ih="a1b2c3d4..." peers_out=10 compact=true
2025-01-14T12:01:23.789Z  INFO tracker_server: peer stopped ih="a1b2c3d4..." ip="192.168.1.100" port=6881

### 上游聚合日志
```
INFO tracker_server: http aggregated ih="a1b2..." attempted=8 succeeded=6 inserted=120
INFO tracker_server: udp aggregated ih="a1b2..." hubs=40 parsed=25 inserted=220
INFO tracker_server: ws aggregated ih="a1b2..." hubs=8 parsed=3 inserted=36
```
```

### 日志说明
- 仅保留与客户端请求直接相关的 INFO 日志（announce served / peer stopped 等）。
- 与上游聚合相关的日志降为 DEBUG 级别，默认不会输出；可通过 `RUST_LOG=debug` 查看。

## 本地存储模型

内存中存储 peer 信息：

```json
{
  "info_hash": "40字节十六进制字符串",
  "ip": "IPv4 地址",
  "port": 端口号,
  "left": "剩余字节数 (optional)",
  "last_seen": "最后活跃时间戳"
}
```

**去重机制：** 使用 `info_hash + ip + port` 作为唯一键，确保幂等性。

## 工作原理

### 1. Announce 处理 + 快速模式

```
客户端请求
    ↓
解析 info_hash, peer_id, port
    ↓
检查 event 类型
    ├─ stopped → 删除对应 peer 并记录日志
    └─ 其他 → 写入/更新本地存储
         ↓
查询现有 peers
    ↓
计算 seeders/leechers
    ↓
触发快速模式（必要时聚合 HTTP 上游，等待/限时等待/异步）
  ↓
生成 bencoded 响应
    ↓
返回给客户端
```

### 2. 后台清理任务

每 5 分钟运行一次：

1. 计算截止时间：`now - peer_ttl`
2. 查询 `last_seen < 截止时间` 的 peers
3. 删除过期 peers
4. 记录删除数量

### 3. 上游聚合任务

- 周期性轮询已知的 `info_hash` 集合（可配置间隔）
- 并发请求 HTTP/UDP/WS 多协议上游 tracker
- 解析 compact peers，批量 upsert 到本地存储

## 与 BitTorrent 客户端集成

### 在 .torrent 文件中使用

```
announce: http://your-server.com:1337/announce
```

### 在 magnet 链接中使用

```
magnet:?xt=urn:btih:...&tr=http://your-server.com:1337/announce
```

### 支持的客户端

理论上支持所有标准的 BitTorrent 客户端：
- qBittorrent
- Transmission
- Deluge
- μTorrent
- BitComet
- Aria2
- 等等

## 性能优化

### 存储说明

本项目当前使用进程内存保存 peer 数据；重启进程后数据会清空。

### 反向代理（推荐）

使用 Nginx 提高并发处理能力：

```nginx
upstream tracker {
    server 127.0.0.1:1337;
    server 127.0.0.1:1338;  # 可以运行多个实例
}

server {
    listen 80;
    server_name tracker.example.com;
    
    location / {
        proxy_pass http://tracker;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### 运行多个实例

```bash
# 实例 1
./target/release/tracker-server --bind 0.0.0.0:1337

# 实例 2
./target/release/tracker-server --bind 0.0.0.0:1338
```

### 调整 TTL

根据使用场景调整 peer 生存时间：

```bash
# 快速更新（适合活跃网络）
./target/release/tracker-server --peer-ttl 1800

# 长期保留（适合小众种子）
./target/release/tracker-server --peer-ttl 86400
```

## 常见问题

### Q: 如何保证种子数统计是实时准确的？

- `complete` 基于当前 `left == 0` 的活跃 peer 数量实时计算。
- `incomplete` 基于当前 `left != 0` 或未知 `left` 的活跃 peer 数量实时计算。
- `event=stopped` 会即时移除该 peer。
- 超过 `peer_ttl` 的 peer 会被后台任务清理。

### Q: 端口被占用？

```bash
# 更改绑定端口
./target/release/tracker-server --bind 0.0.0.0:8080
```

### Q: 如何调试 announce 请求？

开启 debug 日志：

```bash
RUST_LOG=debug ./target/release/tracker-server
```

### Q: 支持 IPv6 吗？

当前版本仅支持 IPv4。IPv6 支持可以在未来版本中添加。

### Q: 可以添加访问控制吗？

建议在反向代理（如 Nginx）层面实现：
- IP 白名单/黑名单
- 速率限制
- 认证

## 数据流向

```
BitTorrent 客户端
    ↓
HTTP GET /announce
    ↓
Tracker Server (本程序)
    ↓
本地内存存储
  ↑
Metadata Spider（通过 announce 获取 peers）
```

## 与其他组件的集成

### 数据来源

- **DHT Spider**: 发现 peers 并通过 announce 更新 peer 信息
- **BitTorrent 客户端**: 通过 announce 更新 peer 信息

### 数据消费

- **BitTorrent 客户端**：通过 announce 响应获取 peers
- **Metadata Spider**：通过标准 announce 获取 peers

## 开发

### 项目结构

```
src/
├── main.rs          # 入口点，HTTP 服务器
└── store.rs         # 本地 peer 存储与统计
```

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
cargo fmt
```

## 协议参考

- [BEP-3: The BitTorrent Protocol Specification](http://www.bittorrent.org/beps/bep_0003.html)
- [BEP-23: Tracker Returns Compact Peer Lists](http://www.bittorrent.org/beps/bep_0023.html)

## 许可证

查看 LICENSE 文件。
