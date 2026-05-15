use iced::futures::SinkExt;
use iced::widget::image::Handle;
use iced::widget::{button, column, container, image, row, text, vertical_space};
use iced::{Alignment, Application, Command, Element, Length, Theme, theme};

use crate::network;
use crate::theme::*;
use crate::types::*;

#[cfg(target_os = "linux")]
use ksni::TrayMethods;

#[cfg(not(target_os = "linux"))]
use tray_icon::{TrayIcon, TrayIconBuilder};

#[cfg(target_os = "linux")]
struct ConduitTray {
    tx: iced::futures::channel::mpsc::Sender<Message>,
}

#[cfg(target_os = "linux")]
impl ksni::Tray for ConduitTray {
    fn id(&self) -> String {
        "conduit".into()
    }
    fn icon_name(&self) -> String {
        "conduit".into()
    }
    fn title(&self) -> String {
        "Conduit".into()
    }
    fn tool_tip(&self) -> ksni::ToolTip {
        ksni::ToolTip {
            title: "Conduit".into(),
            description: "Conduit Network Utility".into(),
            ..Default::default()
        }
    }
    fn activate(&mut self, _x: i32, _y: i32) {
        let mut tx = self.tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(Message::TrayClicked).await;
        });
    }
}

pub struct ForwarderApp {
    pub(crate) current_page: Page,
    pub(crate) language: Language,
    pub(crate) close_behavior: CloseBehavior,

    pub(crate) logo_only: Handle,
    pub(crate) logo_full: Handle,

    #[cfg(not(target_os = "linux"))]
    pub(crate) _tray_icon: Option<TrayIcon>,

    pub(crate) interfaces: Vec<network::InterfaceInfo>,
    pub(crate) selected_wans: Vec<String>,
    pub(crate) lan_shares: Vec<LanShare>,
    pub(crate) sys_active: bool,
    pub(crate) sys_status: std::borrow::Cow<'static, str>,

    pub(crate) system_report: Option<network::SystemReport>,
    pub(crate) refresh_interval: u64,

    pub(crate) port_forwarders: Vec<PortForwarder>,
}

impl Application for ForwarderApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let ifaces: Vec<network::InterfaceInfo> = network::get_interfaces()
            .into_iter()
            .filter(|i| {
                let name = i.name.as_str();
                name != "lo"
                    && !name.starts_with("veth")
                    && !name.starts_with("docker")
                    && !name.starts_with("br-")
            })
            .collect();

        let (sys_active, active_wans, _) = network::detect_system_forward_status();
        let report = network::get_system_network_report();
        let cfg = AppConfig::load();

        let logo_only =
            Handle::from_memory(include_bytes!("../assets/images/Conduit-logoonly.png").as_slice());
        let logo_full =
            Handle::from_memory(include_bytes!("../assets/images/Conduit.png").as_slice());

        let port_forwarders: Vec<PortForwarder> = cfg
            .forwarders
            .iter()
            .map(|fc| PortForwarder {
                id: uuid::Uuid::new_v4(),
                protocol: fc.protocol,
                src_addr: fc.src_addr.clone(),
                src_port: fc.src_port.clone(),
                dst_addr: fc.dst_addr.clone(),
                dst_port: fc.dst_port.clone(),
                is_active: false,
                status: std::borrow::Cow::Owned(format!(
                    "{} ({})",
                    cfg.language.get("status_ready"),
                    cfg.language.get("status_imported")
                )),
                stop_tx: None,
            })
            .collect();

        #[cfg(target_os = "linux")]
        {
            let (tx, _rx) = iced::futures::channel::mpsc::channel(100);
            let tray = ConduitTray { tx };
            tokio::spawn(async move {
                let _ = tray.spawn().await;
            });
        }

        let status_key = if sys_active {
            "status_active"
        } else {
            "status_ready"
        };

        (
            Self {
                current_page: Page::SystemForward,
                language: cfg.language,
                close_behavior: cfg.close_behavior,
                logo_only,
                logo_full,
                #[cfg(not(target_os = "linux"))]
                _tray_icon: None,
                interfaces: ifaces,
                selected_wans: active_wans,
                lan_shares: vec![LanShare {
                    config: LanShareConfig::default(),
                    is_active: false,
                    status: cfg.language.get("status_ready").into(),
                    stop_tx: None,
                }],
                sys_active,
                sys_status: cfg.language.get(status_key).into(),
                system_report: Some(report),
                refresh_interval: 1,
                port_forwarders,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Conduit".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Exit => {
                tracing::info!("Exiting application...");
                return iced::window::close(iced::window::Id::MAIN);
            }
            Message::TrayClicked => {
                tracing::info!("Tray clicked, restoring window...");
                return iced::window::change_mode(
                    iced::window::Id::MAIN,
                    iced::window::Mode::Windowed,
                );
            }
            Message::CloseRequested => {
                if self.close_behavior == CloseBehavior::Minimize {
                    tracing::info!("Minimizing to tray...");
                    return Command::batch(vec![
                        iced::window::change_mode(
                            iced::window::Id::MAIN,
                            iced::window::Mode::Hidden,
                        ),
                        iced::window::minimize(iced::window::Id::MAIN, true),
                    ]);
                }
                tracing::info!("Quitting application with cleanup...");
                let shares: Vec<_> = self
                    .lan_shares
                    .iter()
                    .filter(|s| !s.config.interface.is_empty())
                    .map(|s| {
                        let wans = if s.config.wans.is_empty() {
                            self.selected_wans.clone()
                        } else {
                            s.config.wans.clone()
                        };
                        (
                            s.config.interface.clone(),
                            s.config.ip.clone(),
                            s.config.mask.clone(),
                            wans,
                        )
                    })
                    .collect();
                if !shares.is_empty() {
                    return Command::perform(
                        async move {
                            let _ = network::stop_system_forwarding(&shares);
                        },
                        |_| Message::Exit,
                    );
                }
                return iced::window::close(iced::window::Id::MAIN);
            }
            Message::SetCloseBehavior(behavior) => {
                self.close_behavior = behavior;
                self.save_config();
            }
            Message::LanguageChanged(lang) => {
                self.language = lang;
                let status_key = if self.sys_active {
                    "status_active"
                } else {
                    "status_ready"
                };
                self.sys_status = self.language.get(status_key).into();

                for f in &mut self.port_forwarders {
                    if f.is_active {
                        f.status = self.language.get("status_running").into();
                    } else if f.status.contains("Error") || f.status.contains("错误") {
                        f.status = self.language.get("status_stopped").into();
                    } else {
                        f.status = self.language.get("status_ready").into();
                    }
                }
                self.save_config();
            }
            Message::SwitchPage(page) => self.current_page = page,
            Message::RefreshInterfaces => {
                self.interfaces = network::get_interfaces()
                    .into_iter()
                    .filter(|i| {
                        let n = &i.name;
                        n != "lo"
                            && !n.starts_with("veth")
                            && !n.starts_with("docker")
                            && !n.starts_with("br-")
                    })
                    .collect();
            }
            Message::RefreshSystemReport => {
                return Command::perform(
                    async { network::get_system_network_report() },
                    Message::SystemReportReceived,
                );
            }
            Message::SystemReportReceived(report) => {
                self.system_report = Some(report);
            }
            Message::SetRefreshInterval(interval) => {
                self.refresh_interval = interval;
            }
            Message::DetectSystemForward => {
                let (active, wans, failed) = network::detect_system_forward_status();
                if failed {
                    self.sys_status = self.language.get("msg_det_failed").into();
                } else {
                    self.sys_active = active;
                    if active && !wans.is_empty() {
                        self.selected_wans = wans;
                    }
                    let status_key = if active {
                        "status_active"
                    } else {
                        "status_ready"
                    };
                    self.sys_status = self.language.get(status_key).into();
                }
            }
            Message::WanToggled(name, active) => {
                if active {
                    self.selected_wans.push(name);
                } else {
                    self.selected_wans.retain(|n| n != &name);
                }
            }
            Message::LanWanToggled(idx, name, active) => {
                if let Some(share) = self.lan_shares.get_mut(idx) {
                    if active {
                        if !share.config.wans.contains(&name) {
                            share.config.wans.push(name);
                        }
                    } else {
                        share.config.wans.retain(|n| n != &name);
                    }
                }
                self.save_config();
            }
            Message::AddLanShare => {
                self.lan_shares.push(LanShare {
                    config: LanShareConfig::default(),
                    is_active: false,
                    status: self.language.get("status_ready").into(),
                    stop_tx: None,
                });
                self.save_config();
            }
            Message::RemoveLanShare(idx) => {
                if idx < self.lan_shares.len() {
                    self.lan_shares.remove(idx);
                }
                self.save_config();
            }
            Message::UpdateLanShare(idx, field, value) => {
                if let Some(share) = self.lan_shares.get_mut(idx) {
                    match field.as_str() {
                        "interface" => share.config.interface = value,
                        "ip" => share.config.ip = value,
                        "mask" => share.config.mask = value,
                        _ => {}
                    }
                }
                self.save_config();
            }
            Message::ToggleLanShare(idx) => {
                if let Some(share) = self.lan_shares.get_mut(idx) {
                    let active = share.is_active;
                    let wans = if share.config.wans.is_empty() {
                        self.selected_wans.clone()
                    } else {
                        share.config.wans.clone()
                    };
                    let share_data = (
                        share.config.interface.clone(),
                        share.config.ip.clone(),
                        share.config.mask.clone(),
                        wans,
                    );
                    share.status = (if active {
                        self.language.get("msg_stopping")
                    } else {
                        self.language.get("msg_starting")
                    })
                    .into();
                    return Command::perform(
                        async move {
                            let res = if active {
                                network::stop_system_forwarding(&[share_data])
                            } else {
                                network::start_system_forwarding(&[share_data])
                            };
                            res.map_err(|e| e.to_string())
                        },
                        move |res| Message::SysForwardingResult(idx, !active, res),
                    );
                }
            }
            Message::ToggleSysForwarding => {
                let active = self.sys_active;
                let shares: Vec<_> = self
                    .lan_shares
                    .iter()
                    .map(|s| {
                        let wans = if s.config.wans.is_empty() {
                            self.selected_wans.clone()
                        } else {
                            s.config.wans.clone()
                        };
                        (
                            s.config.interface.clone(),
                            s.config.ip.clone(),
                            s.config.mask.clone(),
                            wans,
                        )
                    })
                    .collect();

                if shares.is_empty() || shares.iter().any(|(i, _, _, _)| i.is_empty()) {
                    self.sys_status = self.language.get("msg_select_lan").into();
                    return Command::none();
                }
                if shares.iter().all(|(_, _, _, w)| w.is_empty()) {
                    self.sys_status = self.language.get("msg_select_wan").into();
                    return Command::none();
                }
                self.sys_status = if active {
                    self.language.get("msg_stopping").into()
                } else {
                    self.language.get("msg_starting").into()
                };
                return Command::perform(
                    async move {
                        let res = if active {
                            network::stop_system_forwarding(&shares)
                        } else {
                            network::start_system_forwarding(&shares)
                        };
                        res.map_err(|e| e.to_string())
                    },
                    move |res| Message::SysForwardingResult(usize::MAX, !active, res),
                );
            }
            Message::SysForwardingResult(idx, target, res) => match res {
                Ok(_) => {
                    if idx == usize::MAX {
                        self.sys_active = target;
                        self.sys_status = if target {
                            self.language.get("msg_active_bang").into()
                        } else {
                            self.language.get("msg_stopped").into()
                        };
                    } else if let Some(share) = self.lan_shares.get_mut(idx) {
                        share.is_active = target;
                        share.status = if target {
                            self.language.get("status_running").into()
                        } else {
                            self.language.get("status_stopped").into()
                        };
                    }
                }
                Err(e) => {
                    let msg = format!(
                        "{}: {}",
                        if self.language == Language::Chinese {
                            "错误"
                        } else {
                            "Error"
                        },
                        e
                    );
                    if idx == usize::MAX {
                        self.sys_status = msg.into();
                    } else if let Some(share) = self.lan_shares.get_mut(idx) {
                        share.status = msg.into();
                    }
                }
            },
            Message::AddForwarder => {
                self.port_forwarders.push(PortForwarder {
                    id: uuid::Uuid::new_v4(),
                    protocol: Protocol::Tcp,
                    src_addr: "0.0.0.0".to_string(),
                    src_port: "".to_string(),
                    dst_addr: "127.0.0.1".to_string(),
                    dst_port: "".to_string(),
                    is_active: false,
                    status: self.language.get("status_ready").into(),
                    stop_tx: None,
                });
                self.save_config();
            }
            Message::RemoveForwarder(id) => {
                if let Some(pos) = self.port_forwarders.iter().position(|f| f.id == id) {
                    if self.port_forwarders[pos].is_active
                        && let Some(tx) = self.port_forwarders[pos].stop_tx.take()
                    {
                        let _ = tx.send(true);
                    }
                    self.port_forwarders.remove(pos);
                }
                self.save_config();
            }
            Message::SrcAddrChanged(id, addr) => {
                if let Some(f) = self.port_forwarders.iter_mut().find(|f| f.id == id) {
                    f.src_addr = addr;
                }
                self.save_config();
            }
            Message::SrcPortChanged(id, port) => {
                if let Some(f) = self.port_forwarders.iter_mut().find(|f| f.id == id) {
                    f.src_port = port;
                }
                self.save_config();
            }
            Message::DstAddrChanged(id, addr) => {
                if let Some(f) = self.port_forwarders.iter_mut().find(|f| f.id == id) {
                    f.dst_addr = addr;
                }
                self.save_config();
            }
            Message::DstPortChanged(id, port) => {
                if let Some(f) = self.port_forwarders.iter_mut().find(|f| f.id == id) {
                    f.dst_port = port;
                }
                self.save_config();
            }
            Message::TogglePortForwarding(id) => {
                if let Some(f) = self.port_forwarders.iter_mut().find(|f| f.id == id) {
                    if f.is_active {
                        if let Some(tx) = f.stop_tx.take() {
                            let _ = tx.send(true);
                        }
                        f.is_active = false;
                        f.status = self.language.get("status_stopped").into();
                    } else if let (Ok(sp), Ok(dp)) =
                        (f.src_port.parse::<u16>(), f.dst_port.parse::<u16>())
                    {
                        let (tx, rx) = tokio::sync::watch::channel(false);
                        f.stop_tx = Some(tx);
                        f.is_active = true;
                        f.status = self.language.get("status_running").into();
                        let s = f.src_addr.clone();
                        let d = f.dst_addr.clone();
                        let p = f.protocol;
                        return Command::perform(
                            async move {
                                let res = if p == Protocol::Tcp {
                                    network::start_tcp_forward(s, sp, d, dp, rx).await
                                } else {
                                    network::start_udp_forward(s, sp, d, dp, rx).await
                                };
                                res.map_err(|e| e.to_string())
                            },
                            move |res| Message::PortForwardingResult(id, res),
                        );
                    } else {
                        f.status = self.language.get("status_invalid_port").into();
                    }
                }
            }
            Message::PortForwardingResult(id, res) => {
                if let Some(f) = self.port_forwarders.iter_mut().find(|f| f.id == id)
                    && let Err(e) = res
                {
                    f.is_active = false;
                    f.status = format!(
                        "{}: {}",
                        if self.language == Language::Chinese {
                            "错误"
                        } else {
                            "Error"
                        },
                        e
                    )
                    .into();
                }
            }
            Message::ImportConfig => {
                return Command::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .add_filter("JSON", &["json"])
                            .pick_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::ConfigFileSelected,
                );
            }
            Message::ConfigFileSelected(path) => {
                if let Some(p) = path
                    && let Ok(content) = std::fs::read_to_string(p)
                    && let Ok(configs) = serde_json::from_str::<Vec<PortForwarderConfig>>(&content)
                {
                    for cfg in configs {
                        self.port_forwarders.push(PortForwarder {
                            id: uuid::Uuid::new_v4(),
                            protocol: cfg.protocol,
                            src_addr: cfg.src_addr,
                            src_port: cfg.src_port,
                            dst_addr: cfg.dst_addr,
                            dst_port: cfg.dst_port,
                            is_active: false,
                            status: format!(
                                "{} ({})",
                                self.language.get("status_ready"),
                                self.language.get("status_imported")
                            )
                            .into(),
                            stop_tx: None,
                        });
                    }
                }
            }
            Message::ExportConfig => {
                return Command::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .add_filter("JSON", &["json"])
                            .set_file_name("config.json")
                            .save_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::ConfigFileToExportSelected,
                );
            }
            Message::ConfigFileToExportSelected(path) => {
                if let Some(p) = path {
                    let configs: Vec<PortForwarderConfig> = self
                        .port_forwarders
                        .iter()
                        .map(|f| PortForwarderConfig {
                            protocol: f.protocol,
                            src_addr: f.src_addr.clone(),
                            src_port: f.src_port.clone(),
                            dst_addr: f.dst_addr.clone(),
                            dst_port: f.dst_port.clone(),
                        })
                        .collect();
                    if let Ok(json) = serde_json::to_string_pretty(&configs) {
                        let _ = std::fs::write(p, json);
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        let mut subs: Vec<iced::Subscription<Message>> =
            vec![iced::event::listen_with(|event, _status| match event {
                iced::Event::Window(_, iced::window::Event::CloseRequested) => {
                    Some(Message::CloseRequested)
                }
                _ => None,
            })];

        #[cfg(target_os = "linux")]
        subs.push(iced::subscription::channel(
            std::any::TypeId::of::<ConduitTray>(),
            100,
            |mut output| async move {
                let receiver = tray_icon::TrayIconEvent::receiver();
                loop {
                    if let Ok(_event) = receiver.try_recv() {
                        let _ = output.send(Message::TrayClicked).await;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            },
        ));

        if self.current_page == Page::SystemMonitor {
            subs.push(
                iced::time::every(std::time::Duration::from_secs(self.refresh_interval))
                    .map(|_| Message::RefreshSystemReport),
            );
        }

        iced::Subscription::batch(subs)
    }

    fn view(&self) -> Element<'_, Message> {
        let lang = self.language;

        let sidebar_button = |label: &str, icon: &str, page: Page, current_page: Page| {
            let is_selected = page == current_page;
            button(
                row![
                    container(text(icon)
                        .size(16)
                        .shaping(iced::widget::text::Shaping::Advanced))
                        .width(24)
                        .center_x(),
                    text(label).size(14),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .padding(12)
            .on_press(Message::SwitchPage(page))
            .style(if is_selected {
                theme::Button::Primary
            } else {
                theme::Button::Text
            })
        };

        let sidebar = container(
            column![
                container(image(self.logo_only.clone()).width(50))
                    .width(Length::Fill)
                    .center_x(),
                vertical_space().height(30),
                sidebar_button(
                    lang.get("nav_share"),
                    "🌐",
                    Page::SystemForward,
                    self.current_page
                ),
                sidebar_button(
                    lang.get("nav_forward"),
                    "🔌",
                    Page::PortForward,
                    self.current_page
                ),
                sidebar_button(
                    lang.get("nav_monitor"),
                    "📊",
                    Page::SystemMonitor,
                    self.current_page
                ),
                sidebar_button(
                    lang.get("nav_settings"),
                    "⚙️",
                    Page::Settings,
                    self.current_page
                ),
                sidebar_button(lang.get("nav_about"), "ℹ️", Page::About, self.current_page),
                vertical_space().height(Length::Fill),
                row![
                    button("中")
                        .on_press(Message::LanguageChanged(Language::Chinese))
                        .style(if self.language == Language::Chinese {
                            theme::Button::Primary
                        } else {
                            theme::Button::Secondary
                        })
                        .padding(5),
                    button("EN")
                        .on_press(Message::LanguageChanged(Language::English))
                        .style(if self.language == Language::English {
                            theme::Button::Primary
                        } else {
                            theme::Button::Secondary
                        })
                        .padding(5),
                ]
                .spacing(5)
                .align_items(Alignment::Center)
            ]
            .spacing(10)
            .padding(20),
        )
        .width(200)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(SidebarStyle)));

        let content_area: Element<Message> = match self.current_page {
            Page::Settings => self.view_settings_page(),
            Page::About => self.view_about_page(),
            Page::SystemMonitor => self.view_monitor_page(),
            Page::SystemForward => self.view_share_page(),
            Page::PortForward => self.view_forward_page(),
        };

        row![
            sidebar,
            container(content_area)
                .padding(30)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Container::Custom(Box::new(ContentStyle)))
        ]
        .into()
    }
}

impl ForwarderApp {
    fn save_config(&self) {
        let cfg = AppConfig {
            language: self.language,
            close_behavior: self.close_behavior,
            forwarders: self
                .port_forwarders
                .iter()
                .map(|f| PortForwarderConfig {
                    protocol: f.protocol,
                    src_addr: f.src_addr.clone(),
                    src_port: f.src_port.clone(),
                    dst_addr: f.dst_addr.clone(),
                    dst_port: f.dst_port.clone(),
                })
                .collect(),
            lan_shares: self.lan_shares.iter().map(|s| s.config.clone()).collect(),
        };
        cfg.save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_page_switching() {
        let (mut app, _) = ForwarderApp::new(());
        assert_eq!(app.current_page, Page::SystemForward);

        let _cmd = app.update(Message::SwitchPage(Page::PortForward));
        assert_eq!(app.current_page, Page::PortForward);

        let _cmd = app.update(Message::SwitchPage(Page::SystemMonitor));
        assert_eq!(app.current_page, Page::SystemMonitor);

        let _cmd = app.update(Message::SwitchPage(Page::Settings));
        assert_eq!(app.current_page, Page::Settings);

        let _cmd = app.update(Message::SwitchPage(Page::About));
        assert_eq!(app.current_page, Page::About);
    }

    #[tokio::test]
    async fn test_app_language_switch() {
        let (mut app, _) = ForwarderApp::new(());
        assert_eq!(app.language, Language::Chinese);

        let _cmd = app.update(Message::LanguageChanged(Language::English));
        assert_eq!(app.language, Language::English);

        let _cmd = app.update(Message::AddForwarder);
        assert_eq!(app.port_forwarders[0].status, "Ready");

        let _cmd = app.update(Message::LanguageChanged(Language::Chinese));
        assert_eq!(app.port_forwarders[0].status, "就绪");
    }

    #[tokio::test]
    async fn test_app_forwarder_crud() {
        let (mut app, _) = ForwarderApp::new(());
        assert!(app.port_forwarders.is_empty());

        let _cmd = app.update(Message::AddForwarder);
        assert_eq!(app.port_forwarders.len(), 1);

        let id = app.port_forwarders[0].id;
        let _cmd = app.update(Message::SrcAddrChanged(id, "10.0.0.1".into()));
        assert_eq!(app.port_forwarders[0].src_addr, "10.0.0.1");

        let _cmd = app.update(Message::SrcPortChanged(id, "8080".into()));
        assert_eq!(app.port_forwarders[0].src_port, "8080");

        let _cmd = app.update(Message::DstAddrChanged(id, "192.168.1.1".into()));
        assert_eq!(app.port_forwarders[0].dst_addr, "192.168.1.1");

        let _cmd = app.update(Message::DstPortChanged(id, "80".into()));
        assert_eq!(app.port_forwarders[0].dst_port, "80");

        let _cmd = app.update(Message::RemoveForwarder(id));
        assert!(app.port_forwarders.is_empty());
    }

    #[tokio::test]
    async fn test_app_add_multiple_forwarders() {
        let (mut app, _) = ForwarderApp::new(());
        let _cmd = app.update(Message::AddForwarder);
        let _cmd = app.update(Message::AddForwarder);
        let _cmd = app.update(Message::AddForwarder);
        assert_eq!(app.port_forwarders.len(), 3);

        let ids: std::collections::HashSet<_> = app.port_forwarders.iter().map(|f| f.id).collect();
        assert_eq!(ids.len(), 3);
    }

    #[tokio::test]
    async fn test_app_close_behavior() {
        let (mut app, _) = ForwarderApp::new(());
        assert_eq!(app.close_behavior, CloseBehavior::Quit);

        let _cmd = app.update(Message::SetCloseBehavior(CloseBehavior::Minimize));
        assert_eq!(app.close_behavior, CloseBehavior::Minimize);

        let _cmd = app.update(Message::SetCloseBehavior(CloseBehavior::Quit));
        assert_eq!(app.close_behavior, CloseBehavior::Quit);
    }

    #[tokio::test]
    async fn test_app_wan_selection() {
        let (mut app, _) = ForwarderApp::new(());
        assert!(app.selected_wans.is_empty());

        let _cmd = app.update(Message::WanToggled("eth0".into(), true));
        assert!(app.selected_wans.contains(&"eth0".into()));

        let _cmd = app.update(Message::WanToggled("wlan0".into(), true));
        assert_eq!(app.selected_wans.len(), 2);

        let _cmd = app.update(Message::WanToggled("eth0".into(), false));
        assert!(!app.selected_wans.contains(&"eth0".into()));
        assert_eq!(app.selected_wans.len(), 1);
    }

    #[tokio::test]
    async fn test_app_lan_share_add_remove() {
        let (mut app, _) = ForwarderApp::new(());
        assert_eq!(app.lan_shares.len(), 1);

        let _cmd = app.update(Message::AddLanShare);
        assert_eq!(app.lan_shares.len(), 2);

        let _cmd = app.update(Message::RemoveLanShare(0));
        assert_eq!(app.lan_shares.len(), 1);
    }

    #[tokio::test]
    async fn test_app_lan_share_update() {
        let (mut app, _) = ForwarderApp::new(());
        let _cmd = app.update(Message::UpdateLanShare(
            0,
            "interface".into(),
            "eth1".into(),
        ));
        assert_eq!(app.lan_shares[0].config.interface, "eth1");

        let _cmd = app.update(Message::UpdateLanShare(0, "ip".into(), "10.0.0.1".into()));
        assert_eq!(app.lan_shares[0].config.ip, "10.0.0.1");

        let _cmd = app.update(Message::UpdateLanShare(0, "mask".into(), "16".into()));
        assert_eq!(app.lan_shares[0].config.mask, "16");
    }

    #[tokio::test]
    async fn test_app_lan_share_default_values() {
        let (mut app, _) = ForwarderApp::new(());
        assert_eq!(app.lan_shares[0].config.ip, "192.168.10.1");
        assert_eq!(app.lan_shares[0].config.mask, "24");
    }

    #[tokio::test]
    async fn test_app_forwarder_toggle_validates_port() {
        let (mut app, _) = ForwarderApp::new(());
        let _cmd = app.update(Message::AddForwarder);
        let fwd_id = app.port_forwarders[0].id;

        let _cmd = app.update(Message::TogglePortForwarding(fwd_id));
        assert!(!app.port_forwarders[0].is_active);
        assert!(
            app.port_forwarders[0].status.contains("无效端口")
                || app.port_forwarders[0].status.contains("Invalid port")
        );
    }

    #[tokio::test]
    async fn test_app_forwarder_configures_correctly() {
        let (mut app, _) = ForwarderApp::new(());
        let _cmd = app.update(Message::AddForwarder);
        let id = app.port_forwarders[0].id;

        let _cmd = app.update(Message::SrcAddrChanged(id, "0.0.0.0".into()));
        let _cmd = app.update(Message::SrcPortChanged(id, "12345".into()));
        let _cmd = app.update(Message::DstAddrChanged(id, "10.0.0.1".into()));
        let _cmd = app.update(Message::DstPortChanged(id, "80".into()));

        assert_eq!(app.port_forwarders[0].src_addr, "0.0.0.0");
        assert_eq!(app.port_forwarders[0].src_port, "12345");
        assert_eq!(app.port_forwarders[0].dst_addr, "10.0.0.1");
        assert_eq!(app.port_forwarders[0].dst_port, "80");
    }
}
