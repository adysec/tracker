# tracker

## 简介

本项目自动收集和测试各种BitTorrent tracker服务器，提供全量和优选的tracker列表。支持黑名单过滤功能，自动过滤威胁情报中的恶意IP地址。

## Tracker列表

### 全量tracker服务器

包含所有收集到的tracker服务器地址（未经可用性测试）：

```
https://down.adysec.com/trackers_all.txt
或
https://raw.githubusercontent.com/adysec/tracker/main/trackers_all.txt
```

### 优选tracker服务器（按协议分类）

经过连通性测试的可用tracker服务器，按协议分类：

#### HTTP协议
```
https://down.adysec.com/trackers_best_http.txt
或
https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_http.txt
```

#### HTTPS协议
```
https://down.adysec.com/trackers_best_https.txt
或
https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_https.txt
```

#### UDP协议
```
https://down.adysec.com/trackers_best_udp.txt
或
https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_udp.txt
```

#### WSS协议
```
https://down.adysec.com/trackers_best_wss.txt
或
https://raw.githubusercontent.com/adysec/tracker/main/trackers_best_wss.txt
```

#### 兼容性文件（保持向后兼容）
```
https://down.adysec.com/trackers_best.txt
或
https://raw.githubusercontent.com/adysec/tracker/main/trackers_best.txt
```

## 功能特性

### 🔒 安全过滤
- 支持黑名单过滤功能，使用 `blackstr.txt` 文件过滤恶意IP地址
- 自动过滤威胁情报中标记的恶意tracker服务器
- 确保提供的tracker列表安全可靠

### 📊 协议分类
- 自动将tracker按协议分类（HTTP、HTTPS、UDP、WSS）
- 用户可根据需要选择特定协议的tracker
- 便于不同应用场景的使用

### 🔄 自动更新
- 每日自动更新tracker列表
- 自动进行连通性测试
- 只保留可用的tracker服务器

### 📈 数据源丰富
收集来源包括但不限于：
- ngosang/trackerslist
- XIU2/TrackersListCollection
- chenjia404/CnTrackersList
- hezhijie0327/Trackerslist
- 以及其他多个开源项目

## 使用说明

### 在BT客户端中使用
将对应协议的tracker列表URL添加到您的BitTorrent客户端中，客户端会自动获取最新的tracker列表。

### 本地部署
1. 克隆项目：`git clone https://github.com/adysec/tracker.git`
2. 运行格式化脚本：`bash format.sh trackers_all.txt`
3. 运行测试脚本：`bash test_trackers.sh trackers_all.txt`

### 自定义黑名单
在项目根目录创建 `blackstr.txt` 文件，每行添加一个需要过滤的IP地址或域名。
