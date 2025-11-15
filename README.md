# BitTorrent Tracker 聚合项目

[![更新状态](https://github.com/adysec/tracker/workflows/Daily%20Update%20Tracker/badge.svg)](https://github.com/adysec/tracker/actions)
[![许可证](https://img.shields.io/github/license/adysec/tracker)](LICENSE)

## 📖 项目简介

本项目自动收集、测试和维护高质量的 BitTorrent Tracker 服务器列表。通过每日自动化流程，从多个开源项目聚合 Tracker 数据，进行连通性测试，并提供按协议分类的优选列表。

**提供推荐tracker服务器，使用本项目中全量tracker服务器进行负载均衡配置，理论上相当于加入了本项目所有tracker服务器，且可以大幅度减少对tracker服务器的并发请求**

[`https://tracker.adysec.com/announce`](https://tracker.adysec.com/announce)
```
       客户端
          │
          ▼
    推荐 Tracker
       │
  ┌────┼────┼────┐
  ▼    ▼    ▼
  A    B    C
```

## 📋 Tracker 列表

本项目提供全量和优选两种类型的 Tracker 列表。全量列表包含从各大开源项目聚合的所有 Tracker 服务器，适合需要最大覆盖范围的用户；优选列表则经过严格的连通性测试，仅保留真正可用的高质量 Tracker，确保最佳的下载体验。

为满足不同应用场景的需求，优选列表进一步按协议分类，用户可根据 BitTorrent 客户端的支持情况和网络环境选择合适的协议类型。其中 `trackers_best.txt` 为我们的推荐使用选项，它包含了所有协议的优质 Tracker，兼容性最佳。所有列表均提供主要和备用两个下载源，确保服务的高可用性。

| 类型 | 协议 | 说明 | 主要下载 | 备用下载 |
|------|------|------|----------|----------|
| **全量** | 全部 | 包含所有聚合的 Tracker 服务器（未经可用性筛选） | [`trackers_all.txt`](https://down.adysec.com/trackers_all.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_all.txt) |
| **优选** | 全部 | 经过连通性测试的高质量 Tracker 服务器（推荐） | [`trackers_best.txt`](https://down.adysec.com/trackers_best.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best.txt) |
| **优选** | HTTP | 仅包含 HTTP 协议的可用 Tracker | [`trackers_best_http.txt`](https://down.adysec.com/trackers_best_http.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_http.txt) |
| **优选** | HTTPS | 仅包含 HTTPS 协议的可用 Tracker | [`trackers_best_https.txt`](https://down.adysec.com/trackers_best_https.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_https.txt) |
| **优选** | UDP | 仅包含 UDP 协议的可用 Tracker | [`trackers_best_udp.txt`](https://down.adysec.com/trackers_best_udp.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_udp.txt) |
| **优选** | WSS | 仅包含 WSS 协议的可用 Tracker | [`trackers_best_wss.txt`](https://down.adysec.com/trackers_best_wss.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_wss.txt) |

### ✨ 核心特性

- 🛡️ **安全可靠**：集成威胁情报，自动过滤恶意 IP 地址
- 🔄 **实时更新**：每日自动更新，确保 Tracker 列表时效性
- 📊 **协议分类**：支持 HTTP、HTTPS、UDP、WSS 四种协议分类
- 🚀 **高可用性**：仅保留经过连通性测试的可用 Tracker
- 🌐 **多源聚合**：整合十余个知名开源项目的 Tracker 资源

## 🔧 使用方法

### BitTorrent 客户端配置

1. **qBittorrent**：选项 → BitTorrent → 自动添加以下 Tracker → 粘贴列表 URL
2. **Transmission**：编辑首选项 → Tracker → 添加 Tracker URL
3. **其他客户端**：在 Tracker 设置中添加上述任一 URL

### 命令行使用

```bash
# 获取最新 Tracker 列表
curl -s https://down.adysec.com/trackers_best.txt

# 获取特定协议的 Tracker
curl -s https://down.adysec.com/trackers_best_udp.txt
```

## 自定义配置

创建 blackstr.txt 文件来过滤特定的 IP 地址或域名：

```bash
# 示例：过滤恶意 IP
echo "192.168.1.100" >> blackstr.txt
echo "malicious-tracker.com" >> blackstr.txt
```

## 📊 数据来源

本项目聚合以下优质开源项目的 Tracker 资源：

- ngosang/trackerslist
- XIU2/TrackersListCollection
- chenjia404/CnTrackersList
- hezhijie0327/Trackerslist
- DeSireFire/animeTrackerList
- NewTrackon
- 以及其他多个社区维护的项目

## ⭐ Star History

如果这个项目对您有帮助，请给我们一个 Star！
