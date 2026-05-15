use iced::widget::button::Button;
use iced::{Element, theme};

use crate::types::Message;

#[allow(dead_code)]
pub fn primary<'a>(label: impl Into<Element<'a, Message>>, msg: Message) -> Button<'a, Message> {
    Button::new(label)
        .on_press(msg)
        .style(theme::Button::Primary)
        .padding(12)
}

#[allow(dead_code)]
pub fn destructive<'a>(
    label: impl Into<Element<'a, Message>>,
    msg: Message,
) -> Button<'a, Message> {
    Button::new(label)
        .on_press(msg)
        .style(theme::Button::Destructive)
        .padding(12)
}

#[allow(dead_code)]
pub fn secondary<'a>(label: impl Into<Element<'a, Message>>, msg: Message) -> Button<'a, Message> {
    Button::new(label)
        .on_press(msg)
        .style(theme::Button::Secondary)
        .padding(10)
}
