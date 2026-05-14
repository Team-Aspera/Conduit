use iced::Color;

// ── 背景色 ──

/// 侧边栏背景 (theme.rs SidebarStyle)
pub const SIDEBAR_BG: Color = Color::from_rgb(0.96, 0.96, 0.98);

/// 蓝色徽章/卡片头背景 (theme.rs BadgeStyle)
/// WAN 卡片接口名称头部、分享状态 badge
pub const BADGE_BG: Color = Color::from_rgb(0.2, 0.5, 0.8);

/// 徽章/卡片头文字颜色 (theme.rs BadgeStyle)
pub const BADGE_TEXT: Color = Color::WHITE;

/// LAN 卡片底色 (theme.rs LanCardStyle)
pub const LAN_CARD_BG: Color = Color::from_rgb(0.96, 0.96, 0.98);

/// WAN 卡片身体底色 (theme.rs WanCardStyle)
pub const WAN_CARD_BG: Color = Color::from_rgb(0.96, 0.96, 0.98);

/// 内容区白色背景 (theme.rs ContentStyle)
pub const CONTENT_BG: Color = Color::WHITE;


// ── 文字色 ──

/// 各区块标题蓝色 (pages.rs — WAN框/LAN框/设置页 标题)
pub const TITLE_BLUE: Color = Color::from_rgb(0.2, 0.4, 0.7);

/// 灰色说明文字 (pages.rs — 版本号、about_desc、无数据提示)
pub const TEXT_GRAY: Color = Color::from_rgb(0.5, 0.5, 0.5);

/// 淡化文字 (pages.rs — "Built with Iced & Tokio"、非活跃状态)
pub const TEXT_DIM: Color = Color::from_rgb(0.6, 0.6, 0.6);


// ── 状态色 ──

/// 活跃状态绿色 (pages.rs — 转发活跃、IP转发已开启)
pub const STATUS_GREEN: Color = Color::from_rgb(0.2, 0.7, 0.2);

/// IP 转发已开启 (pages.rs — 监控页 ip_forward 状态)
pub const IP_FORWARD_ON: Color = Color::from_rgb(0.2, 0.6, 0.2);

/// IP 转发已关闭 (pages.rs — 监控页 ip_forward 状态)
pub const IP_FORWARD_OFF: Color = Color::from_rgb(0.7, 0.2, 0.2);
