# Conduit

> 跨平台网络工具，为开发板联网和端口转发提供图形化界面。

## 安装

```bash
make build
sudo make install
```

或直接运行：

```bash
cargo run --release
```

## 快速开始

```bash
# 网络共享：选择 WAN 接口，配置 LAN 共享，点击"开始共享"
# 端口转发：切换到端口转发页，添加转发规则，点击"开始"
```

## 使用指南

### 网络共享

1. 外网接口池显示所有可用网卡（名称、MAC、IP 地址）
2. 点击 LAN 卡片右上角的"添加新转发"添加 LAN 共享
3. 每张 LAN 卡片可独立选择 LAN 接口、IP/掩码、勾选 WAN 接口
4. 每张卡片有独立的"开始共享"/"停止共享"按钮
5. 点击 WAN 框右上角的"检测状态"检测当前 iptables 状态

### 端口转发

1. 切换到"端口转发"页面
2. 添加新转发：配置协议（TCP/UDP）、源地址和端口、目标地址和端口
3. 点击"▶ Start"启动转发，"⏹ Stop"停止

## 项目结构

```
src/
  main.rs      入口
  app.rs       应用逻辑（状态、事件处理）
  pages.rs     各页面渲染
  network.rs   网络操作（iptables、TCP/UDP 转发）
  types.rs     类型定义
  theme.rs     自定义样式
  i18n.rs      国际化
  widgets.rs   按钮工厂
  colors.rs    颜色常量
locales/
  zh-CN.json   中文翻译
  en.json      英文翻译
```

## 相关文档

- [设计文档](DESIGN.md) — 架构和设计决策

## 协议

MIT
