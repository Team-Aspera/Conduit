# 更新日志

## [0.3.0] - 2026-05-11

### 新增
- 配置持久化：语言、关闭行为、转发规则自动保存到 ~/.config/conduit/config.json
- GitHub Actions CI：自动构建、lint、测试、release
- 单元测试 20 个：覆盖转发逻辑、状态机、序列化

### 变更
- 编译优化：LTO + strip + opt-level="z"，体积 38MB → 23MB
- Makefile 新增 dist/install-dist/uninstall-dist 目标
- .gitignore 补充 IDE、OS、运行时配置

### 修复
- 跨平台 Emoji 渲染：捆绑 Noto Sans Symbols 2 后备字体
- 启用 MSAA 抗锯齿，设最小窗口尺寸 800×600 防止崩溃
- 修复 ksni 0.3.4 API 兼容（Handle、TrayMethods、ToolTip）
- 修复窗口关闭事件订阅（close_requests → event::listen_with）
- 移除编译器 warnings
- 文档全部中文化

## [0.2.3] - 2026-04-27

### 新增
- 设置页面，支持配置窗口关闭行为（最小化到托盘 / 退出）
- 网络共享页面显示当前活跃的共享 IP 和接口信息

### 变更
- 压缩 Logo 图片体积，提升关于页面渲染性能
- 系统监控刷新改为异步

### 修复
- 端口转发本地化与箭头图标渲染问题
- 停止网络共享时移除已分配的 IP 地址

## [0.2.2] - 2026-04-26

### 新增
- 安装/卸载脚本，支持桌面入口（desktop entry）
- 退出时通过 iptables 清理规则
- 侧边栏和关于页面集成 Logo 图片

### 修复
- UDP 转发子任务终止逻辑
- 仅操作成功时更新活跃状态

## [0.2.1] - 2026-04-26

### 变更
- UI 全面改版：侧边栏导航 + 卡片布局
- 关于页面内容居中

### 修复
- 修复中文渲染：更换为 LXGW WenKai Lite 字体
- 修复图标渲染：捆绑 Noto Sans Symbols 2 字体
- 启用 Advanced Shaping 修复 Emoji 显示
- 静默 iptables 清理时的不存在规则警告

## [0.2.0] - 2026-04-26

### 新增
- 系统监控页面，支持自动刷新
- 国际化支持（中文 / 英文）
- 配置文件导入/导出（文件选择器）
- NAT 规则活跃状态检测
- 系统转发 IP 地址配置

### 修复
- 修复中文渲染：捆绑 Noto Sans CJK SC 字体
- 修复检测状态可靠性问题
- 翻译检测失败提示
- 完成所有状态消息本地化

## [0.1.0] - 2026-04-25

### 新增
- 初始版本发布
- 基于 iptables 的系统网络共享（NAT）
- 多任务 TCP/UDP 端口转发
- 基于 Iced 框架的现代化 UI
- Tokio 异步引擎
