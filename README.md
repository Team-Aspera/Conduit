# Conduit

<p align="center">
  <img src="assets/images/Conduit.png" alt="Conduit Logo" width="400">
</p>

Conduit 是一个跨平台的网络工具，基于 **Rust**、**Iced** 和 **Tokio** 构建，为复杂的网络转发任务提供现代化图形界面。

## 功能特性

- **系统网络共享 (NAT)**：从多个 WAN 接口共享网络到指定 LAN 接口（如为开发板提供网络接入）
- **多任务端口转发**：并发 TCP/UDP 端口转发，支持多条转发规则同时运行
- **异步引擎**：基于 `tokio` 的高吞吐、低延迟数据代理
- **现代化界面**：使用 `iced` 框架构建的简洁响应式 UI

## 快速开始

### 前置条件

- [Rust](https://www.rust-lang.org/tools/install)（最新稳定版）
- `pkexec`（Linux 下系统级 NAT 配置需要）

### 安装与运行

```bash
# 克隆仓库
git clone git@github.com:xjimlinx/Conduit.git
cd Conduit

# 运行应用
cargo run --release
```

## 使用指南

1. **网络共享**：选择一个或多个 WAN 接口，指定 LAN 目标接口和网关 IP，点击"开始共享"
2. **端口转发**：切换到"端口转发"标签页，点击"添加新转发"，配置协议（TCP/UDP）和端口，点击"开始"

## 协议

MIT
