[🇨🇳 简体中文](README.md) | [🇺🇸 English](README.en.md)

# BitTorrent Tracker Aggregation Project

[![Update Status](https://github.com/adysec/tracker/workflows/Daily%20Update%20Tracker/badge.svg)](https://github.com/adysec/tracker/actions)
[![License](https://img.shields.io/github/license/adysec/tracker)](LICENSE)

## 📖 Overview

This project automatically collects, tests, and maintains high-quality BitTorrent tracker server lists.  
Through a daily automated workflow, it aggregates tracker data from multiple open-source projects, performs connectivity tests, and provides curated lists categorized by protocol.

**We provide a recommended tracker endpoint. By using load-balancing based on the full tracker list from this project, you can effectively join all trackers in this project while significantly reducing concurrent requests to individual tracker servers.**

`https://tracker.adysec.com/announce`



## 📋 Tracker Lists

This project provides two types of tracker lists: **full** and **best**.

- The **full list** contains all aggregated tracker servers from upstream open-source projects and is suitable when you want maximum coverage.
- The **best list** is strictly filtered by connectivity tests to keep only truly available, high-quality trackers, giving you the best download experience.

To support different use cases, the best list is further categorized by protocol.  
You can select the appropriate protocol type based on what your BitTorrent client supports and your network environment.

- `trackers_best.txt` is the **recommended** option.  
  It includes high-quality trackers of all supported protocols and offers the best compatibility.
- All lists provide both a primary and a backup download source to ensure high availability.

| Type | Protocol | Description | Primary | Backup |
|------|----------|-------------|---------|--------|
| **Full** | All | All aggregated tracker servers (no availability filtering) | [`trackers_all.txt`](https://down.adysec.com/trackers_all.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_all.txt) |
| **Best** | All | Connectivity-tested, high-quality trackers (recommended) | [`trackers_best.txt`](https://down.adysec.com/trackers_best.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best.txt) |
| **Best** | HTTP | Only available HTTP trackers | [`trackers_best_http.txt`](https://down.adysec.com/trackers_best_http.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_http.txt) |
| **Best** | HTTPS | Only available HTTPS trackers | [`trackers_best_https.txt`](https://down.adysec.com/trackers_best_https.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_https.txt) |
| **Best** | UDP | Only available UDP trackers | [`trackers_best_udp.txt`](https://down.adysec.com/trackers_best_udp.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_udp.txt) |
| **Best** | WSS | Only available WSS trackers | [`trackers_best_wss.txt`](https://down.adysec.com/trackers_best_wss.txt) | [`GitHub`](https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_wss.txt) |

### ✨ Key Features

- 🛡️ **Secure & Reliable** – Integrates threat intelligence and automatically filters malicious IPs
- 🔄 **Daily Updates** – Tracker lists are refreshed every day to keep them up-to-date
- 📊 **Protocol-Aware** – Supports HTTP, HTTPS, UDP and WSS categorized lists
- 🚀 **High Availability** – Only retains trackers that pass connectivity checks
- 🌐 **Multi-Source Aggregation** – Aggregates tracker resources from 10+ well-known open-source projects

## 🔧 Usage

### Configure in BitTorrent Clients

1. **qBittorrent**  
   *Tools → Options → BitTorrent → Automatically add these trackers to new downloads → paste the list URL*

2. **Transmission**  
   *Preferences → Trackers → Add tracker URL*

3. **Other Clients**  
   Add any of the above URLs in the tracker configuration section.

### Command-Line Usage

```bash
# Get the latest recommended tracker list
curl -s https://down.adysec.com/trackers_best.txt

# Get trackers for a specific protocol (e.g., UDP)
curl -s https://down.adysec.com/trackers_best_udp.txt
```

### 🔧 Custom Filtering
Create a blackstr.txt file to filter specific IP addresses or domains:

```bash
# Example: filter malicious IP / domain
echo "192.168.1.100" >> blackstr.txt
echo "malicious-tracker.com" >> blackstr.txt
```

## 📊 Data Sources

This project aggregates tracker resources from the following high-quality open-source projects:

- ngosang/trackerslist
- XIU2/TrackersListCollection
- chenjia404/CnTrackersList
- hezhijie0327/Trackerslist
- DeSireFire/animeTrackerList

## 💡 Motivation

Many public trackers perform poorly in real-world use and severely degrade download experience.
This project aims to automatically aggregate, test, and filter trackers to provide a truly usable, frequently updated list for both personal use and the community.

By leveraging DHT and various tracker information flows, we can collect more torrent metadata (similar to BTDigg) to search and organize useful content.
However, most public torrents are concentrated in adult content, making it difficult to find valuable resources 🐶.

Any tracker naturally logs clients’ IP and port information, which has threat-intelligence value.
For example, you can observe which countries or regions a torrent is most frequently downloaded from, or quickly estimate the location and impact scope in a data-leak incident based on connection distribution.

## ⭐ Star History

If this project is helpful to you, please consider giving it a ⭐ on GitHub!
