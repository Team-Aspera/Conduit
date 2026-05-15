use iced::Color;

// ── 品牌色（不随主题变化）──

/// 蓝色徽章/卡片头背景 (theme.rs BadgeStyle)
pub const BADGE_BG: Color = Color::from_rgb(0.2, 0.5, 0.8);
/// 徽章/卡片头文字颜色 (theme.rs BadgeStyle)
pub const BADGE_TEXT: Color = Color::WHITE;

// ── 文字色（用于 pages.rs 显式指定）──

/// 各区块标题蓝色
pub const TITLE_BLUE: Color = Color::from_rgb(0.2, 0.4, 0.7);
/// 灰色说明文字
pub const TEXT_GRAY: Color = Color::from_rgb(0.5, 0.5, 0.5);
/// 淡化文字
pub const TEXT_DIM: Color = Color::from_rgb(0.6, 0.6, 0.6);

// ── 状态色 ──

/// 活跃状态绿色
pub const STATUS_GREEN: Color = Color::from_rgb(0.2, 0.7, 0.2);
/// IP 转发已开启
pub const IP_FORWARD_ON: Color = Color::from_rgb(0.2, 0.6, 0.2);
/// IP 转发已关闭
pub const IP_FORWARD_OFF: Color = Color::from_rgb(0.7, 0.2, 0.2);
