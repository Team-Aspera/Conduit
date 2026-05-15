use iced::widget::container;
use iced::{Background, Border, Color, Theme};

use crate::colors;

fn is_dark(theme: &Theme) -> bool {
    matches!(theme, Theme::Dark)
}

// 暗色配色参考 GitHub Dark 风格：
//   背景 #0d1117  表层 #161b22  卡片 #21262d  文字 #e6edf3

#[derive(Debug, Clone, Copy, Default)]
pub struct SidebarStyle;

impl container::StyleSheet for SidebarStyle {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(if is_dark(style) {
                style.palette().background
            } else {
                Color::from_rgb(0.96, 0.96, 0.98)
            })),
            border: Border {
                width: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BadgeStyle;

impl container::StyleSheet for BadgeStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::BADGE_BG)),
            border: Border {
                ..Default::default()
            },
            text_color: Some(colors::BADGE_TEXT),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LanCardStyle;

impl container::StyleSheet for LanCardStyle {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(if is_dark(style) {
                Color::from_rgb(0.16, 0.18, 0.20)
            } else {
                Color::from_rgb(0.97, 0.98, 1.0)
            })),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WanCardStyle;

impl container::StyleSheet for WanCardStyle {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(if is_dark(style) {
                Color::from_rgb(0.14, 0.16, 0.15)
            } else {
                Color::from_rgb(0.96, 0.97, 0.95)
            })),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ContentStyle;

impl container::StyleSheet for ContentStyle {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(if is_dark(style) {
                Color::from_rgb(0.12, 0.13, 0.14)
            } else {
                Color::WHITE
            })),
            ..Default::default()
        }
    }
}
