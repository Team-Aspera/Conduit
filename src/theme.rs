use iced::widget::container;
use iced::{Background, Border, Theme};

use crate::colors;

#[derive(Debug, Clone, Copy, Default)]
pub struct SidebarStyle;

impl container::StyleSheet for SidebarStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::SIDEBAR_BG)),
            border: Border { width: 0.0, ..Default::default() },
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
            border: Border { ..Default::default() },
            text_color: Some(colors::BADGE_TEXT),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LanCardStyle;

impl container::StyleSheet for LanCardStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::LAN_CARD_BG)),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WanCardStyle;

impl container::StyleSheet for WanCardStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::WAN_CARD_BG)),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ContentStyle;

impl container::StyleSheet for ContentStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::CONTENT_BG)),
            ..Default::default()
        }
    }
}
