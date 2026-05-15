# Conduit 设计文档

## 概述

Conduit 是一个跨平台网络工具，为复杂的网络转发任务（NAT 共享、端口转发）提供现代化图形界面。基于 Rust、Iced GUI 框架和 Tokio 异步运行时构建。

核心目标：
- 为开发板等设备提供一键式网络共享（NAT + IP 转发）
- 支持多条 TCP/UDP 端口转发规则并发运行
- 纯 GUI 操作，无需手写 iptables 命令

## 架构

```
src/
  main.rs     入口，模块声明 + ForwarderApp::run
  app.rs      ForwarderApp 结构体 + Application impl (new, update, subscription)
  pages.rs    每个页面的 view 渲染方法
  types.rs    所有类型定义（Language, Message, Config 等）
  network.rs  底层网络操作（iptables、TCP/UDP 转发）
  theme.rs    自定义容器样式
  i18n.rs     编译时加载 JSON 翻译
  widgets.rs  按钮工厂函数
  colors.rs   全局颜色常量

locales/
  zh-CN.json  中文翻译键值对
  en.json     英文翻译键值对
```

## 核心模块

### app.rs — 应用状态与事件处理

`ForwarderApp` 是应用的核心状态结构体，包含当前页面、语言设置、共享配置、转发规则列表等字段。

实现 `Application` trait：
- `new()` — 初始化接口列表、加载配置、恢复转发状态
- `update()` — 处理所有 Message 事件（页面切换、语言切换、转发控制等）
- `view()` — 构建 UI 布局（sidebar + 内容区）
- `subscription()` — 订阅窗口关闭事件、系统托盘事件

### pages.rs — 页面渲染

每个页面有独立的 view 方法：
- `view_share_page()` — 网络共享页面（外网接口池、LAN 卡片、独立开关）
- `view_forward_page()` — 端口转发页面（规则列表、增删改）
- `view_monitor_page()` — 系统监控页面（转发流、NAT 规则）
- `view_settings_page()` — 设置页面（窗口关闭行为）
- `view_about_page()` — 关于页面

### network.rs — 网络操作

- `get_interfaces()` — 获取系统网络接口列表（名称、MAC、IP）
- `detect_system_forward_status()` — 检测当前 iptables NAT 状态
- `start_system_forwarding()` / `stop_system_forwarding()` — 启停 NAT 共享
- `start_tcp_forward()` / `start_udp_forward()` — 启停端口转发（异步）

### i18n.rs — 国际化

使用 `include_str!` 在编译时嵌入 `locales/*.json`，`once_cell::Lazy` 解析为 HashMap。`Language::get()` 委托给 `i18n::get()` 查询翻译。

### types.rs — 核心类型

关键数据结构：
- `Message` 枚举 — 所有 UI 事件的类型化表示
- `LanShare` / `LanShareConfig` — LAN 共享配置 + 运行时状态（is_active, status）
- `PortForwarder` / `PortForwarderConfig` — 端口转发规则
- `AppConfig` — 持久化配置（JSON 序列化）

## 数据流

### 网络共享流程

```
用户点击 LAN 卡片"开始共享"
  → Message::ToggleLanShare(idx)
  → ForwarderApp::update()
    → 检查 interface 和 wans 是否有效
    → 调用 network::start_system_forwarding() [通过 pkexec 提权]
      → 开启 ip_forward
      → 添加 IP 到 LAN 接口
      → 添加 iptables MASQUERADE 规则
    → 收到 SysForwardingResult(idx, target, res)
    → 更新对应 LanShare.is_active/status
```

### 端口转发流程

```
用户点击"▶ Start"
  → Message::TogglePortForwarding(id)
  → ForwarderApp::update()
    → 验证端口号
    → tokio::spawn(start_tcp_forward / start_udp_forward)
      → 绑定源端口
      → 循环 accept + copy_bidirectional
    → 更新 PortForwarder.is_active/status
```

## 配置

配置文件路径：`~/.config/conduit/config.json`

格式：
```json
{
  "language": "Chinese",
  "close_behavior": "Quit",
  "forwarders": [
    { "protocol": "TCP", "src_addr": "0.0.0.0", "src_port": "8080", "dst_addr": "10.0.0.1", "dst_port": "80" }
  ],
  "lan_shares": [
    { "interface": "eth0", "ip": "192.168.10.1", "mask": "24", "wans": [] }
  ]
}
```
