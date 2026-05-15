use iced::widget::{
    button, checkbox, column, container, horizontal_space, image, pick_list, row, scrollable, text,
    text_input, vertical_space,
};
use iced::{Alignment, Element, Length, theme};

use crate::app::ForwarderApp;
use crate::colors;
use crate::theme::*;
use crate::types::*;

impl ForwarderApp {
    pub fn view_settings_page(&self) -> Element<'_, Message> {
        let lang = self.language;
        column![
            text(lang.get("title_settings")).size(28),
            vertical_space().height(20),
            container(
                column![
                    text(lang.get("label_close_behavior"))
                        .size(16)
                        .style(theme::Text::Color(colors::TITLE_BLUE)),
                    row![
                        button(lang.get("opt_minimize"))
                            .on_press(Message::SetCloseBehavior(CloseBehavior::Minimize))
                            .style(if self.close_behavior == CloseBehavior::Minimize {
                                theme::Button::Primary
                            } else {
                                theme::Button::Secondary
                            })
                            .padding(10),
                        button(lang.get("opt_quit"))
                            .on_press(Message::SetCloseBehavior(CloseBehavior::Quit))
                            .style(if self.close_behavior == CloseBehavior::Quit {
                                theme::Button::Primary
                            } else {
                                theme::Button::Secondary
                            })
                            .padding(10),
                    ]
                    .spacing(10)
                ]
                .spacing(15)
            )
            .padding(20)
            .style(theme::Container::Box),
        ]
        .spacing(20)
        .max_width(600)
        .into()
    }

    pub fn view_about_page(&self) -> Element<'_, Message> {
        let lang = self.language;
        container(
            column![
                image(self.logo_full.clone()).width(250),
                text(format!("v{}", env!("CARGO_PKG_VERSION")))
                    .size(14)
                    .style(theme::Text::Color(colors::TEXT_GRAY)),
                vertical_space().height(20),
                text(lang.get("about_desc")).size(16),
                vertical_space().height(30),
                text("GitHub: github.com/xjimlinx/Conduit").size(12),
                text("Built with Iced & Tokio")
                    .size(12)
                    .style(theme::Text::Color(colors::TEXT_DIM)),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    pub fn view_monitor_page(&self) -> Element<'_, Message> {
        let lang = self.language;
        if let Some(report) = &self.system_report {
            let section_card = |title: String, items: &Vec<String>| {
                let content: Element<Message> = if items.is_empty() {
                    text("No active data")
                        .size(12)
                        .style(theme::Text::Color(colors::TEXT_GRAY))
                        .into()
                } else {
                    let elements: Vec<Element<Message>> = items
                        .iter()
                        .map(|i| {
                            container(text(i).size(11).font(iced::Font::MONOSPACE))
                                .padding([2, 5])
                                .into()
                        })
                        .collect();
                    column(elements).spacing(4).into()
                };

                let card: Element<Message> = container(column![
                    text(title)
                        .size(16)
                        .style(theme::Text::Color(colors::TITLE_BLUE)),
                    vertical_space().height(8),
                    content,
                ])
                .width(Length::Fill)
                .padding(15)
                .style(theme::Container::Box)
                .into();
                card
            };

            column![
                row![
                    text(lang.get("title_monitor")).size(28),
                    horizontal_space().width(Length::Fill),
                    row![
                        text(format!(
                            "{} {}s",
                            if lang.get("nav_share") == "网络共享" {
                                "刷新频率:"
                            } else {
                                "Interval:"
                            },
                            self.refresh_interval
                        ))
                        .size(12),
                        button("1s").on_press(Message::SetRefreshInterval(1)).style(
                            if self.refresh_interval == 1 {
                                theme::Button::Primary
                            } else {
                                theme::Button::Secondary
                            }
                        ),
                        button("5s").on_press(Message::SetRefreshInterval(5)).style(
                            if self.refresh_interval == 5 {
                                theme::Button::Primary
                            } else {
                                theme::Button::Secondary
                            }
                        ),
                        button("10s")
                            .on_press(Message::SetRefreshInterval(10))
                            .style(if self.refresh_interval == 10 {
                                theme::Button::Primary
                            } else {
                                theme::Button::Secondary
                            }),
                    ]
                    .spacing(5)
                    .align_items(Alignment::Center),
                    button(lang.get("btn_refresh")).on_press(Message::RefreshSystemReport),
                ]
                .spacing(15)
                .align_items(Alignment::Center),
                container(
                    row![
                        text(lang.get("label_ip_forward")).size(16),
                        horizontal_space().width(10),
                        text(if report.ip_forward_enabled {
                            lang.get("label_enabled")
                        } else {
                            lang.get("label_disabled")
                        })
                        .size(14)
                        .style(theme::Text::Color(
                            if report.ip_forward_enabled {
                                colors::IP_FORWARD_ON
                            } else {
                                colors::IP_FORWARD_OFF
                            }
                        ))
                    ]
                    .align_items(Alignment::Center)
                )
                .padding(10)
                .style(theme::Container::Box),
                scrollable(
                    column![
                        section_card(
                            lang.get("monitor_active_flows").to_string(),
                            &report.active_connections
                        ),
                        section_card(
                            lang.get("monitor_nat_rules").to_string(),
                            &report.nat_masquerade
                        ),
                        section_card(
                            lang.get("monitor_port_rules").to_string(),
                            &report.port_forwards
                        ),
                        section_card(
                            lang.get("monitor_listen_ports").to_string(),
                            &report.listening_ports
                        ),
                    ]
                    .spacing(20)
                )
                .height(Length::Fill),
            ]
            .spacing(20)
            .into()
        } else {
            container(text("Loading System Report...").size(20))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
    }

    pub fn view_share_page(&self) -> Element<'_, Message> {
        let lang = self.language;

        let wan_cards: Element<Message> = {
            let mut col = column![].spacing(6);
            for iface in &self.interfaces {
                let ip_section: Element<Message> = if iface.ips.is_empty() {
                    let no_ip: Element<Message> = text("-").size(11).into();
                    no_ip
                } else {
                    let mut v4: Vec<&str> = Vec::new();
                    let mut v6: Vec<&str> = Vec::new();
                    for ip in &iface.ips {
                        if ip.contains(':') {
                            v6.push(ip);
                        } else {
                            v4.push(ip);
                        }
                    }
                    let mut col = column![].spacing(2);
                    if !v4.is_empty() {
                        let line: Element<Message> =
                            text(format!("IPv4: {}", v4.join(", "))).size(11).into();
                        col = col.push(line);
                    }
                    if !v6.is_empty() {
                        let line: Element<Message> =
                            text(format!("IPv6: {}", v6.join(", "))).size(11).into();
                        col = col.push(line);
                    }
                    col.into()
                };
                let card: Element<Message> = container(
                    column![
                        container(text(&iface.name).size(14))
                            .padding([6, 10])
                            .width(Length::Fill)
                            .style(theme::Container::Custom(Box::new(crate::theme::BadgeStyle))),
                        container(
                            column![text(format!("MAC: {}", iface.mac)).size(11), ip_section,]
                                .spacing(4),
                        )
                        .padding(10)
                        .width(Length::Fill)
                        .style(theme::Container::Custom(Box::new(
                            crate::theme::WanCardStyle
                        ))),
                    ]
                    .spacing(0),
                )
                .style(theme::Container::Box)
                .width(Length::Fill)
                .into();
                col = col.push(card);
            }
            let inner: Element<Message> = col.into();
            container(inner).padding([0, 14, 0, 0]).into()
        };

        let lan_cards: Vec<Element<Message>> = self
            .lan_shares
            .iter()
            .enumerate()
            .map(|(idx, share)| {
                container(
                    column![
                        row![
                            text(lang.get("label_lan")).width(80).size(14),
                            pick_list(
                                &self.interfaces[..],
                                if share.config.interface.is_empty() {
                                    None
                                } else {
                                    self.interfaces
                                        .iter()
                                        .find(|i| i.name == share.config.interface)
                                        .cloned()
                                },
                                move |v| Message::UpdateLanShare(idx, "interface".into(), v.name),
                            )
                            .width(Length::Fill),
                            button(text("✕").shaping(iced::widget::text::Shaping::Advanced))
                                .on_press(Message::RemoveLanShare(idx))
                                .style(theme::Button::Destructive)
                                .padding([3, 8]),
                        ]
                        .spacing(6)
                        .align_items(Alignment::Center),
                        row![
                            text(format!("{} /", lang.get("label_lan_ip")))
                                .size(14)
                                .width(80),
                            text_input("192.168.10.1", &share.config.ip)
                                .on_input(move |v| Message::UpdateLanShare(idx, "ip".into(), v)),
                            text_input("24", &share.config.mask)
                                .on_input(move |v| Message::UpdateLanShare(idx, "mask".into(), v,))
                                .width(60),
                        ]
                        .spacing(6)
                        .align_items(Alignment::Center),
                        {
                            let mut wan_col = column![].spacing(3);
                            for iface in self
                                .interfaces
                                .iter()
                                .filter(|i| i.name != share.config.interface)
                            {
                                let cb: Element<Message> =
                                    checkbox(&iface.name, share.config.wans.contains(&iface.name))
                                        .on_toggle(move |a| {
                                            Message::LanWanToggled(idx, iface.name.clone(), a)
                                        })
                                        .into();
                                wan_col = wan_col.push(cb);
                            }
                            let w: Element<Message> = container(wan_col).padding([4, 0]).into();
                            w
                        },
                        button(if share.is_active {
                            lang.get("btn_stop_share")
                        } else {
                            lang.get("btn_start_share")
                        })
                        .on_press(Message::ToggleLanShare(idx))
                        .style(if share.is_active {
                            theme::Button::Destructive
                        } else {
                            theme::Button::Primary
                        })
                        .width(Length::Fill)
                        .padding(8),
                    ]
                    .spacing(6),
                )
                .padding(8)
                .style(theme::Container::Custom(Box::new(
                    crate::theme::LanCardStyle,
                )))
                .into()
            })
            .collect();

        let mut lan_col_inner = column![].spacing(10);
        for card in lan_cards {
            lan_col_inner = lan_col_inner.push(card);
        }
        let lan_col: Element<Message> = container(lan_col_inner).padding([0, 14, 0, 0]).into();

        let mut children: Vec<Element<Message>> = Vec::new();

        children.push(text(lang.get("title_share")).size(28).into());
        children.push(vertical_space().height(10).into());

        children.push(
            container(
                column![
                    row![
                        text(lang.get("label_wan"))
                            .size(16)
                            .style(theme::Text::Color(colors::TITLE_BLUE)),
                        horizontal_space().width(Length::Fill),
                        button(lang.get("btn_detect"))
                            .on_press(Message::DetectSystemForward)
                            .padding(8),
                    ]
                    .align_items(Alignment::Center),
                    scrollable(wan_cards).height(180),
                ]
                .spacing(10),
            )
            .padding(15)
            .style(theme::Container::Box)
            .into(),
        );

        children.push(
            container(
                column![
                    row![
                        text(lang.get("label_lan"))
                            .size(16)
                            .style(theme::Text::Color(colors::TITLE_BLUE)),
                        horizontal_space().width(Length::Fill),
                        button(
                            text(format!("➕ {}", lang.get("btn_add_new")))
                                .shaping(iced::widget::text::Shaping::Advanced),
                        )
                        .on_press(Message::AddLanShare)
                        .style(theme::Button::Secondary)
                        .padding(10),
                    ]
                    .align_items(Alignment::Center),
                    scrollable(lan_col).height(200),
                ]
                .spacing(10),
            )
            .padding(15)
            .style(theme::Container::Box)
            .into(),
        );

        if self.sys_active {
            let share_badges: Vec<Element<Message>> = self
                .lan_shares
                .iter()
                .filter(|s| !s.config.interface.is_empty())
                .map(|share| {
                    container(
                        row![
                            container(text(&share.config.interface).size(12))
                                .padding([2, 8])
                                .style(theme::Container::Custom(Box::new(BadgeStyle))),
                            horizontal_space().width(10),
                            text(format!("{}/{}", share.config.ip, share.config.mask))
                                .size(13)
                                .font(iced::Font::MONOSPACE),
                        ]
                        .align_items(Alignment::Center),
                    )
                    .padding(8)
                    .style(theme::Container::Box)
                    .into()
                })
                .collect();
            if !share_badges.is_empty() {
                let list: Element<Message> = column(share_badges).spacing(8).into();
                children.push(
                    container(
                        column![
                            text(lang.get("label_current_share"))
                                .size(16)
                                .style(theme::Text::Color(colors::TITLE_BLUE),),
                            list,
                        ]
                        .spacing(10),
                    )
                    .padding(15)
                    .style(theme::Container::Box)
                    .into(),
                );
            }
        }

        children.push(
            row![
                container(
                    row![
                        text("🔔")
                            .size(14)
                            .shaping(iced::widget::text::Shaping::Advanced),
                        text(&self.sys_status).size(13),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                )
                .padding(10)
                .style(theme::Container::Box)
                .width(Length::Fill),
                button(
                    row![
                        text("🔄")
                            .size(14)
                            .shaping(iced::widget::text::Shaping::Advanced),
                        text(lang.get("btn_refresh_iface")).size(13),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                )
                .padding(10)
                .on_press(Message::RefreshInterfaces)
                .style(theme::Button::Secondary),
            ]
            .spacing(10)
            .into(),
        );

        container(
            scrollable(column(children).spacing(10))
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn view_forward_page(&self) -> Element<'_, Message> {
        let lang = self.language;

        let list = self
            .port_forwarders
            .iter()
            .fold(column![].spacing(15), |col, f| {
                col.push(
                    container(
                        column![
                            row![
                                container(text(f.protocol.to_string()).size(12))
                                    .padding([2, 8])
                                    .style(theme::Container::Custom(Box::new(BadgeStyle))),
                                text_input("Src IP", &f.src_addr)
                                    .on_input(move |v| Message::SrcAddrChanged(f.id, v))
                                    .width(Length::Fill),
                                text(":"),
                                text_input("Port", &f.src_port)
                                    .on_input(move |v| Message::SrcPortChanged(f.id, v))
                                    .width(70),
                                text(" ➔ ")
                                    .size(18)
                                    .shaping(iced::widget::text::Shaping::Advanced),
                                text_input("Dst IP", &f.dst_addr)
                                    .on_input(move |v| Message::DstAddrChanged(f.id, v))
                                    .width(Length::Fill),
                                text(":"),
                                text_input("Port", &f.dst_port)
                                    .on_input(move |v| Message::DstPortChanged(f.id, v))
                                    .width(70),
                            ]
                            .spacing(10)
                            .align_items(Alignment::Center),
                            row![
                                text(format!("● {}", f.status))
                                    .size(12)
                                    .style(theme::Text::Color(if f.is_active {
                                        colors::STATUS_GREEN
                                    } else {
                                        colors::TEXT_DIM
                                    }))
                                    .width(Length::Fill),
                                button(if f.is_active {
                                    text("⏹ Stop").shaping(iced::widget::text::Shaping::Advanced)
                                } else {
                                    text("▶ Start").shaping(iced::widget::text::Shaping::Advanced)
                                })
                                .on_press(Message::TogglePortForwarding(f.id))
                                .style(if f.is_active {
                                    theme::Button::Destructive
                                } else {
                                    theme::Button::Primary
                                })
                                .padding([5, 15]),
                                button(text("🗑").shaping(iced::widget::text::Shaping::Advanced))
                                    .on_press(Message::RemoveForwarder(f.id))
                                    .style(theme::Button::Secondary)
                                    .padding([5, 10]),
                            ]
                            .spacing(10)
                            .align_items(Alignment::Center),
                        ]
                        .spacing(10)
                        .padding(15),
                    )
                    .style(theme::Container::Box),
                )
            });

        column![
            row![
                text(lang.get("title_forward")).size(28),
                horizontal_space().width(Length::Fill),
                button(
                    text(format!("➕ {}", lang.get("btn_add_new")))
                        .shaping(iced::widget::text::Shaping::Advanced)
                )
                .on_press(Message::AddForwarder)
                .style(theme::Button::Primary)
                .padding(10),
                button(lang.get("btn_import"))
                    .on_press(Message::ImportConfig)
                    .padding(10),
                button(lang.get("btn_export"))
                    .on_press(Message::ExportConfig)
                    .padding(10),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            scrollable(list).height(Length::Fill)
        ]
        .spacing(20)
        .into()
    }
}
