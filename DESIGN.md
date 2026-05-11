# Conduit 设计文档

## 概述

Conduit 是一个高性能、跨平台的网络工具，提供图形化界面简化网络共享（NAT）和端口转发任务。目标用户是需要为开发板提供网络接入或进行调试的开发者。

核心设计原则：**简单易用** — 复杂网络操作通过 GUI 一键完成，无需记忆 iptables 命令。

## 架构

```
┌─────────────────────────────────────────────────────┐
│                    Iced GUI                         │
│  (Sidebar / Network Share / Port Forward / Monitor) │
└──────────────────────┬──────────────────────────────┘
                       │ Message
┌──────────────────────▼──────────────────────────────┐
│              ForwarderApp (Application)             │
│  状态管理 / 事件处理 / 命令派发                        │
└──────┬───────────────────────────────────┬──────────┘
       │                                   │
┌──────▼──────┐                  ┌─────────▼─────────┐
│  network.rs │                  │  Tokio Runtime    │
│  (核心逻辑)  │                  │  (异步 I/O)       │
└─────────────┘                  └───────────────────┘
```

模块划分：
- **GUI 层** (`main.rs`) — Iced Application，负责页面渲染、用户交互、状态管理
- **网络层** (`network.rs`) — 系统转发控制、TCP/UDP 端口转发、网络检测

## 核心模块

### GUI 模块 (`main.rs`)
- `ForwarderApp` — 主应用结构体，持有所有状态
- `Message` — 事件枚举，驱动状态变更
- 5 个页面：Network Share、Port Forwarders、System Monitor、Settings、About

### 网络模块 (`network.rs`)
- `get_interfaces()` — 列举网络接口
- `get_system_network_report()` — 获取系统网络状态（IP 转发、NAT 规则、监听端口）
- `start_system_forwarding()` / `stop_system_forwarding()` — 通过 `pkexec` + `iptables` 管理 NAT
- `start_tcp_forward()` / `start_udp_forward()` — 基于 Tokio 的端口转发协程

## CLI 设计

非 CLI 程序，为 GUI 应用。交互通过 Iced GUI 完成。

## 数据流

### 网络共享流程
1. 用户选择 WAN 接口 → LAN 接口 → 点击"Start Share"
2. `start_system_forwarding()` 通过 `pkexec` 以 root 权限执行：
   - 启用 `ip_forward`
   - 为 LAN 接口分配 IP
   - 添加 MASQUERADE 和 FORWARD 规则

### 端口转发流程
1. 用户添加转发规则 → 配置协议/IP/端口 → 点击 Start
2. `start_tcp_forward()` / `start_udp_forward()` 启动 Tokio 异步转发协程
3. 通过 `watch::channel` 实现停止控制

## 配置

- `config.json` — 运行时配置文件（已 gitignore）
- `config.sample.json` — 配置模板
- JSON 格式导入/导出端口转发规则
