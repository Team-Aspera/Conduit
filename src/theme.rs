use iced::widget::container;
use iced::{Background, Border, Color, Theme};

#[derive(Debug, Clone, Copy, Default)]
pub struct SidebarStyle;

impl container::StyleSheet for SidebarStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.96, 0.96, 0.98))),
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
            background: Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.8))),
            border: Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            text_color: Some(Color::WHITE),
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
            background: Some(Background::Color(Color::from_rgb(0.97, 0.98, 1.0))),
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: Color::from_rgb(0.85, 0.88, 0.92),
            },
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
            background: Some(Background::Color(Color::WHITE)),
            ..Default::default()
        }
    }
}
